use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;

use chrono::{Utc};
use serde_json;
use serde_json::value::{Value, to_value};
use uuid::Uuid;

use ::commands::{Ctx};
use ::error::*;
use ::db::{self, Db, Connection};
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

type Tree = Rc<RefCell<BTreeMap<String, MutableKeyTree>>>;

#[derive(Debug, Clone)]
pub enum MutableKeyTree {
    Map(Tree),
    Key(String),
}

impl MutableKeyTree {
    pub fn new_map() -> Self {
        MutableKeyTree::Map(Rc::new(RefCell::new(BTreeMap::new())))
    }

    pub fn as_key(&self) -> Option<&String> {
        match self {
            &MutableKeyTree::Key(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn is_key(&self) -> bool {
        self.as_key().is_some()
    }

    pub fn as_map(&self) -> Option<&Tree> {
        match self {
            &MutableKeyTree::Map(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn is_map(&self) -> bool {
        self.as_map().is_some()
    }

    pub fn get(&self, name: &str) -> Option<MutableKeyTree> {
        self.as_map()
            .and_then(|t| {
                t.borrow().get(name).map(|x| x.clone())
            })
    }

    pub fn contains_key(&self, name: &str) -> bool {
        return self.get(name).is_some()
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
        let admin = db.user_by_username("admin")?;
        if admin.is_none() {
            eprintln!("Creating admin user...");
            db.create_user("admin".to_string(), Role::Admin, "admin".to_string())?;
        }
        Ok(())
    }

    pub fn login<S: AsRef<str>>(&mut self, username: S, password: S) -> Result<ApiToken> {
        let admin_pw = self.app.config().admin_password.clone();
        let db = self.db()?;

        let username = username.as_ref();
        let password = password.as_ref();

        let user = match db.user_by_username(username)? {
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


    pub fn translations_export(&mut self, lang_id: String, format: ExportFormat, pretty: bool)
        -> Result<String>
    {
        // Load all translations for the specified language.
        let translations = self.db()?.translations_with_keys(&lang_id)?;

        let mut export = TranslationsExport::new();
        for (t, k) in translations {
            export.insert(k.key, t.value);
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

    pub fn language(&mut self, id: &str, user: Option<&User>) -> Result<Option<Language>> {
        self.db()?.language_by_id(id)
    }

    pub fn validate_key(&mut self, key: &str) -> Result<()> {
        if !key::validate_key(key) {
            return Err("Invalid key format".into());
        }

        let mut tree = self.build_key_tree()?;
        let mut parts: Vec<_> = key.split('.').collect();

        while parts.len() > 0 {
            let part = parts.remove(0);

            match tree.get(&part) {
                Some(subtree) => {
                    if parts.len() == 0 {
                        if subtree.is_key() {
                            return Err("Duplicate key".into());
                        } else {
                            return Err("Invalid nested key: can't create a key inside an existing hierarchy".into());
                        }
                    } else if subtree.is_key() {
                        return Err("Invalid nested key: can't create a key under an existing key".into());
                    } else {
                        tree = subtree;
                    }
                },
                None => {
                    return Ok(())
                },
            }

            let subtree = tree.get(&part);
        }

        Ok(())
    }

    pub fn create_language(&mut self, lang: NewLanguage, user: Option<&User>) -> Result<Language> {
        let lang = Language {
            id: Uuid::new_v4().to_string(),
            code: lang.code,
            name: lang.name,
            parent_id: lang.parent_id,
            created_by: user.map(|u| u.id.clone()),
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

    pub fn key_by_id(&mut self, id: &str, user: Option<&User>) -> Result<Option<Key>> {
        self.db()?.key_by_id(id)
    }

    pub fn key_by_key(&mut self, key: &str, user: Option<&User>) -> Result<Option<Key>> {
        self.db()?.key_by_key(key)
    }

    pub fn create_key(&mut self, key: NewKey, user: Option<&User>) -> Result<Key> {
        self.validate_key(&key.key)?;
        let key = Key{
            id: Uuid::new_v4().to_string(),
            key: key.key,
            description: key.description,
            created_at: Utc::now().timestamp(),
            created_by: user.map(|u| u.username.clone()),
        };
        self.db()?.create_key(key)
    }

    pub fn rename_key(&mut self, id: &str, new_key: &str, user: Option<&User>) -> Result<Key> {
        self.validate_key(new_key)?;
        let db = self.db()?;
        db.rename_key(id, new_key)?;
        db.must_get_key(id)
    }

    pub fn delete_key<S: AsRef<str>>(&mut self, key: S, user: Option<&User>) -> Result<()> {
        self.db()?.delete_key(key.as_ref())?;
        Ok(())
    }

    pub fn translations(&mut self, key_id: &str, user: Option<&User>)
        -> Result<Vec<Translation>>
    {
        self.db()?.translations(key_id)
    }

    pub fn translate(&mut self, translation: NewTranslation, user: Option<&User>)
        -> Result<Translation>
    {
        // Try to find old translation.
        let key_id = translation.key_id.to_string();
        let language_id = translation.language_id.to_string();
        let t = self.db()?.find_translation(&key_id, &language_id)?;

        if let Some(mut t) = t {
            self.db()?.update_translation(&t.id, &translation.value)?;
            t.value = translation.value;
            t.version += 1;
            Ok(t)
        } else {
            self.create_translation(translation, user.map(|u| u.id.clone()))
        }
    }

    pub fn create_translation(&mut self, translation: NewTranslation, user_id: Option<String>)
        -> Result<Translation>
    {
        let now = Utc::now().timestamp();
        let translation = Translation {
            id: Uuid::new_v4().to_string(),
            language_id: translation.language_id.to_string(),
            key_id: translation.key_id.to_string(),
            value: translation.value,
            created_at: now,
            updated_at: now,
            created_by: user_id,
            version: 1,
        };
        self.db()?.create_translation(translation)
    }

    pub fn update_translation(&mut self, translation: NewTranslation, user: Option<&User>)
        -> Result<Translation>
    {
        unimplemented!();
    }

    pub fn delete_translation(&mut self, id: &str, user: Option<&User>) -> Result<()> {
        self.db()?.delete_translation(id)?;
        Ok(())
    }


    pub fn users(&mut self, user: Option<&User>) -> Result<Vec<User>> {
        self.db()?.users()
    }

    pub fn export(&mut self) -> Result<db::Export> {
        self.db()?.export()
    }
}