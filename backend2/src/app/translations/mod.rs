use std::fmt;

use prelude::*;

#[derive(Clone, Debug)]
pub struct Translation {
    pub id: Uuid,
    pub version: i32,
    pub translation: String,
    pub comment: Option<String>,
    pub language_id: Uuid,
    pub key_id: Uuid,
    pub creator_id: Uuid,
    pub approver_id: Uuid,
    pub created_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

impl From<db::types::Translation> for Translation {
    fn from(t: db::types::Translation) -> Translation {
        Translation {
            id: t.id.parse().unwrap(),
            version: t.version,
            translation: t.translation,
            comment: t.comment,
            language_id: t.language_id.parse().unwrap(),
            key_id: t.key_id.parse().unwrap(),
            creator_id: t.creator_id.parse().unwrap(),
            approver_id: t.approver_id.parse().unwrap(),
            created_at: utc_from_naive(t.created_at),
            deleted_at: t.deleted_at.map(utc_from_naive),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TranslationCreate {
    pub translation: String,
    pub comment: Option<String>,
    pub language_id: Uuid,
    pub key_id: Uuid,
}

pub struct Translations {
    app: App,
}

impl Translations {
    pub fn new(app: App) -> Self {
        Translations { app }
    }

    pub fn find(&self, id: &Uuid) -> Result<Translation, Error> {
        let u = self.app.db()?.translation(&id.to_string())?;
        Ok(u.into())
    }

    pub fn create(&self, data: TranslationCreate) -> Result<Translation, Error> {
        let db = self.app.db()?;
        let max_version =
            db.translation_for_key(&data.key_id.to_string(), &data.language_id.to_string())?;
        let create = db::types::TranslationCreate {
            id: uuid().to_string(),
            version: max_version.map(|x| x.version).unwrap_or(1),
            translation: data.translation,
            comment: data.comment,
            language_id: data.language_id.to_string(),
            key_id: data.key_id.to_string(),
            creator_id: self.app.user().id.to_string(),
            approver_id: self.app.user().id.to_string(),
            created_at: now().naive_utc(),
        };
        Ok(db.translation_create(&create)?.into())
    }
}
