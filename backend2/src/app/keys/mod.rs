use std::fmt;

use prelude::*;

#[derive(Clone, Debug)]
pub struct Key {
    pub id: Uuid,
    pub key: String,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub creator_id: Uuid,
}

impl From<db::types::Key> for Key {
    fn from(l: db::types::Key) -> Key {
        Key {
            id: l.id.parse().unwrap(),
            key: l.key,
            description: l.description,
            created_at: utc_from_naive(l.created_at),
            updated_at: utc_from_naive(l.updated_at),
            deleted_at: l.deleted_at.map(utc_from_naive),
            creator_id: l.creator_id.parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeyCreate {
    pub key: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct KeyUpdate {
    pub id: Uuid,
    pub key: Option<String>,
    pub description: Option<String>,
}

pub struct Keys {
    app: App,
}

impl Keys {
    pub fn new(app: App) -> Self {
        Keys { app }
    }

    pub fn find(&self, id: &Uuid) -> Result<Key, Error> {
        let u = self.app.db()?.key(&id.to_string())?;
        Ok(u.into())
    }

    pub fn create(&self, data: KeyCreate) -> Result<Key, Error> {
        let create = db::types::KeyCreate {
            id: uuid().to_string(),
            key: data.key,
            description: data.description,
            creator_id: self.app.user().id.to_string(),
        };
        Ok(self.app.db()?.key_create(&create)?.into())
    }

    pub fn update(&self, data: KeyUpdate) -> Result<Key, Error> {
        let key = self.find(&data.id)?;
        let update = db::types::KeyUpdate {
            id: data.id.to_string(),
            key: data.key.unwrap_or(key.key),
            description: data.description.or(key.description),
            updated_at: now().naive_utc(),
        };
        let u = self.app.db()?.key_update(&update)?;
        Ok(u.into())
    }
}
