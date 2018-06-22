use std::fmt;

use prelude::*;

#[derive(Clone, Debug)]
pub struct TranslationRequest {
    pub id: Uuid,
    pub translation: String,
    pub comment: Option<String>,
    pub language_id: Uuid,
    pub key_id: Uuid,
    pub creator_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<db::types::TranslationRequest> for TranslationRequest {
    fn from(t: db::types::TranslationRequest) -> TranslationRequest {
        TranslationRequest {
            id: t.id.parse().unwrap(),
            translation: t.translation,
            comment: t.comment,
            language_id: t.language_id.parse().unwrap(),
            key_id: t.key_id.parse().unwrap(),
            creator_id: t.creator_id.parse().unwrap(),
            created_at: utc_from_naive(t.created_at),
            updated_at: utc_from_naive(t.updated_at),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TranslationRequestCreate {
    pub translation: String,
    pub comment: Option<String>,
    pub language_id: Uuid,
    pub key_id: Uuid,
}

#[derive(Clone, Debug)]
pub struct TranslationRequestUpdate {
    pub id: Uuid,
    pub translation: Option<String>,
    pub comment: Option<String>,
}

pub struct TranslationRequests {
    app: App,
}

impl TranslationRequests {
    pub fn new(app: App) -> Self {
        TranslationRequests { app }
    }

    pub fn find(&self, id: &Uuid) -> Result<TranslationRequest, Error> {
        let u = self.app.db()?.translation_request(&id.to_string())?;
        Ok(u.into())
    }

    pub fn create(&self, data: TranslationRequestCreate) -> Result<TranslationRequest, Error> {
        let db = self.app.db()?;
        let create = db::types::TranslationRequestCreate {
            id: uuid().to_string(),
            translation: data.translation,
            comment: data.comment,
            language_id: data.language_id.to_string(),
            key_id: data.key_id.to_string(),
            creator_id: self.app.user().id.to_string(),
            created_at: now().naive_utc(),
        };
        Ok(db.translation_request_create(&create)?.into())
    }

    pub fn update(&self, data: TranslationRequestUpdate) -> Result<TranslationRequest, Error> {
        let t = self.find(&data.id)?;
        let update = db::types::TranslationRequestUpdate {
            id: data.id.to_string(),
            translation: data.translation.unwrap_or(t.translation),
            comment: data.comment.or(t.comment),
            updated_at: now().naive_utc(),
        };
        let u = self.app.db()?.translation_request_update(&update)?;
        Ok(u.into())
    }
}
