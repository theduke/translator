use std::fmt;

use prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Language {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub parent_id: Option<Uuid>,
}

impl From<db::types::Language> for Language {
    fn from(l: db::types::Language) -> Language {
        Language {
            id: l.id.parse().unwrap(),
            code: l.code,
            name: l.name,
            description: l.description,
            created_at: utc_from_naive(l.created_at),
            updated_at: utc_from_naive(l.updated_at),
            parent_id: l.parent_id.map(|x| x.parse().unwrap()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LanguageCreate {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LanguageUpdate {
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

pub struct Languages {
    app: App,
}

impl Languages {
    pub fn new(app: App) -> Self {
        Languages { app }
    }

    pub fn find(&self, id: &Uuid) -> Result<Language, Error> {
        let u = self.app.db()?.language(&id.to_string())?;
        Ok(u.into())
    }

    pub fn create(&self, data: LanguageCreate) -> Result<Language, Error> {
        let create = db::types::LanguageCreate {
            id: uuid().to_string(),
            code: data.code,
            name: data.name,
            description: data.description,
            parent_id: data.parent_id.map(|x| x.to_string()),
        };
        Ok(self.app.db()?.language_create(&create)?.into())
    }

    pub fn update(&self, data: LanguageUpdate) -> Result<Language, Error> {
        let language = self.find(&data.id)?;
        let update = db::types::LanguageUpdate {
            id: data.id.to_string(),
            code: data.code.unwrap_or(language.code),
            name: data.name.unwrap_or(language.name),
            description: data.description.or(language.description),
            parent_id: data
                .parent_id
                .map(|x| x.to_string())
                .or(language.parent_id.map(|x| x.to_string())),
            updated_at: now().naive_utc(),
        };
        let u = self.app.db()?.language_update(&update)?;
        Ok(u.into())
    }
}
