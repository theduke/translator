use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;

use chrono::{Utc};
use serde_json;
use serde_json::value::{Value, to_value};

use ::commands::{Ctx};
use ::error::*;
use ::db::{Db, Connection};
use ::app::App;
use ::db::schema::*;

pub type TranslationsExport = BTreeMap<String, String>;

pub struct Repo {
    app: App,
    db: Option<Db>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Javascript,
}

impl ExportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "json" => Some(ExportFormat::Json),
            "javascript" => Some(ExportFormat::Javascript),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum MutableKeyTree {
    Map(Rc<RefCell<BTreeMap<String, MutableKeyTree>>>),
    Key(String),
}

impl MutableKeyTree {
    pub fn new_map() -> Self {
        MutableKeyTree::Map(Rc::new(RefCell::new(BTreeMap::new())))
    }

    fn insert_nested(&mut self, key: String, mut parts: Vec<String>) {
        let node = match *self {
            MutableKeyTree::Key(_) => { panic!("Can't insert into a key"); },
            MutableKeyTree::Map(ref m) => m,
        };

        // Invariant parts.len() > 0 must hold!
        let name = parts.remove(0);

        if parts.len() < 1 {
            // Last part, so insert as key.
            node.borrow_mut().insert(name.clone(), MutableKeyTree::Key(key));
        } else {
            let mut map = node.borrow_mut();
            let nested = map.entry(name.clone()).or_insert(MutableKeyTree::new_map());
            nested.insert_nested(key, parts);
        }
    }

    pub fn insert(&mut self, key: String) {
        let parts = key.split('.').map(|x| x.to_string()).collect();
        self.insert_nested(key, parts);
    }

    pub fn to_json_value(self) -> Value {
        match self {
            MutableKeyTree::Key(s) => json!(s),
            MutableKeyTree::Map(tree) => {
                let tree = Rc::try_unwrap(tree).unwrap().into_inner();

                let mut map = json!({});
                for (key, val) in tree.into_iter() {
                    map[key] = val.to_json_value();
                }

                map
            },
        }
    }
}

impl Repo {
    pub fn new(app: App) -> Self {
        Repo {
            app,
            db: None,
        }
    }

    pub fn db(&mut self) -> Result<&Db> {
        if self.db.is_none() {
            self.db = Some(self.app.db()?);
        }
        Ok(self.db.as_ref().unwrap())
    }

    pub fn ensure_admin_user(&mut self) -> Result<()> {
        let db = self.db()?;
        // Ensure admin user exists.
        let admin = db.find_user("admin")?;
        if admin.is_none() {
            eprintln!("Creating admin user...");
            db.create_user("admin", Role::Admin, "admin")?;
        }
        Ok(())
    }

    pub fn login<S: AsRef<str>>(&mut self, username: S, password: S) -> Result<ApiToken> {
        let admin_pw = self.app.config().admin_password.clone();
        let db = self.db()?;

        let username = username.as_ref();
        let password = password.as_ref();

        let mut user = match db.find_user(username)? {
            Some(u) => u,
            None => {
                return Err(ErrorKind::UnknownUser.into());
            },
        };

        if user.username == "admin" && admin_pw == Some(password.to_string()) {
            // Superuser detected.
        } else {
            if !user.verify_password(password) {
                return Err(ErrorKind::InvalidPassword.into());
            }
        }

        let token = user.build_session_token()?;
        db.create_api_token(token)
    }


    pub fn translations_export(&mut self, lang: String, format: ExportFormat, pretty: bool)
        -> Result<String>
    {
        // Load all translations for the specified language.
        let translations = self.db()?.translations_by_lang(&lang)?;

        let mut export = TranslationsExport::new();
        for t in translations {
            export.insert(t.key, t.value);
        }

        let mut json = if pretty {
            serde_json::to_string_pretty(&export)?
        } else {
            serde_json::to_string(&export)?
        };

        if format == ExportFormat::Javascript {
            json = format!(
                "// This file was auto-generated. Do not edit by hand!\n\n/* tslint:disable */\n\nexport const translations = {};\n\nexport default translations;\n",
                json);
        }

        Ok(json)
    }

    pub fn keys_export(&mut self, format: ExportFormat, pretty: bool) -> Result<String> {
        let tree = self.build_key_tree()?;
        let data = tree.to_json_value();
        let mut json = if pretty {
            serde_json::to_string_pretty(&data)?
        } else {
            serde_json::to_string(&data)?
        };

        if format == ExportFormat::Javascript {
            json = format!(
                "// This file was auto-generated. Do not edit by hand!\n\n/* tslint:disable */\n\nexport const intlKeys = {};\n\nexport default intlKeys;\n",
                json);
        }
        Ok(json)
    }

    pub fn build_key_tree(&mut self) -> Result<MutableKeyTree> {
        let keys = self.db()?.keys()?;
        let mut t = MutableKeyTree::new_map();

        for key in keys {
            t.insert(key.key);
        }
        Ok(t)
    }

    pub fn languages(&mut self, user: Option<&User>) -> Result<Vec<Language>> {
        self.db()?.languages()
    }

    pub fn language<S: AsRef<str>>(&mut self, id: S, user: Option<&User>)
        -> Result<Option<Language>>
    {
        self.db()?.language(id)
    }

    pub fn create_language(&mut self, lang: NewLanguage, user: Option<&User>) -> Result<Language> {
        let lang = Language {
            id: lang.id,
            name: lang.name,
            parent_id: lang.parent_id,
            created_by: user.map(|u| u.username.clone()),
            created_at: Utc::now().timestamp(),
        };
        eprintln!("Creating lang: {:?}", lang);
        self.db()?.create_language(lang)
    }

    pub fn delete_language<S: AsRef<str>>(&mut self, language: S, user: Option<&User>) -> Result<()> {
        self.db()?.delete_language(language.as_ref())?;
        Ok(())
    }

    pub fn keys(&mut self, user: Option<&User>) -> Result<Vec<Key>> {
        self.db()?.keys()
    }

    pub fn key<S: AsRef<str>>(&mut self, id: S, user: Option<&User>)
        -> Result<Option<Key>>
    {
        self.db()?.key(id)
    }

    pub fn create_key(&mut self, key: NewKey, user: Option<&User>) -> Result<Key> {
        let key = Key{
            key: key.key,
            description: key.description,
            created_at: Utc::now().timestamp(),
            created_by: user.map(|u| u.username.clone()),
        };
        self.db()?.create_key(key)
    }

    pub fn delete_key<S: AsRef<str>>(&mut self, key: S, user: Option<&User>) -> Result<()> {
        self.db()?.delete_key(key.as_ref())?;
        Ok(())
    }

    pub fn translations<S: AsRef<str>>(&mut self, key: S, user: Option<&User>)
        -> Result<Vec<Translation>>
    {
        self.db()?.translations(key)
    }

    pub fn translate(&mut self, translation: NewTranslation, user: Option<&User>)
        -> Result<Translation>
    {
        // Try to find old translation.
        let t = self.db()?.find_translation(&translation.key, &translation.language)?;


        if let Some(mut t) = t {
            self.db()?.update_translation(
                translation.language.as_ref(),
                translation.key.as_ref(),
                translation.value.as_ref())?;
            t.value = translation.value;
            Ok(t)
        } else {
            let now = Utc::now().timestamp();
            let translation = Translation {
                language: translation.language,
                key: translation.key,
                value: translation.value,
                created_at: now,
                updated_at: now,
                created_by: user.map(|u| u.username.clone()),
            };
            self.db()?.create_translation(translation)
        }
    }

    pub fn update_translation(&mut self, translation: NewTranslation, user: Option<&User>)
        -> Result<Translation>
    {
        let now = Utc::now().timestamp();



        let now = Utc::now().timestamp();
        let translation = Translation {
            language: translation.language,
            key: translation.key,
            value: translation.value,
            created_at: now,
            updated_at: now,
            created_by: None,
        };
        Ok(translation)
    }

    pub fn delete_translation<S: AsRef<str>>(&mut self, language: S, key: S, user: Option<&User>)
        -> Result<()>
    {
        self.db()?.delete_translation(language.as_ref(), key.as_ref())?;
        Ok(())
    }


    pub fn users(&mut self, user: Option<&User>) -> Result<Vec<User>> {
        self.db()?.users()
    }
}