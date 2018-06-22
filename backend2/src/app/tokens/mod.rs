use std::fmt;

use prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum TokenKind {
    Auth,
}

impl TokenKind {
    pub fn from_str(value: &str) -> Self {
        match value {
            "auth" => TokenKind::Auth,
            _ => {
                panic!("Unknown token kind: {}", value);
            },
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &TokenKind::Auth => "auth",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token {
    pub id: Uuid,
    pub kind: TokenKind,
    pub token: String,
    pub valid_until: Option<DateTime>,
    pub created_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub user_id: Uuid,
}

impl From<db::types::Token> for Token {
    fn from(l: db::types::Token) -> Token {
        Token {
            id: l.id.parse().unwrap(),
            kind: TokenKind::from_str(&l.kind),
            token: l.token,
            valid_until: l.valid_until.map(utc_from_naive),
            created_at: utc_from_naive(l.created_at),
            deleted_at: l.deleted_at.map(utc_from_naive),
            user_id: l.user_id.parse().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenCreate {
    pub kind: TokenKind,
    pub token: String,
    pub valid_until: Option<DateTime>,
}


pub struct Tokens {
    app: App,
}

impl Tokens {
    pub fn new(app: App) -> Self {
        Tokens { app }
    }

    pub fn find(&self, id: &Uuid) -> Result<Token, Error> {
        let u = self.app.db()?.token(&id.to_string())?;
        Ok(u.into())
    }

    pub fn create(&self, data: TokenCreate) -> Result<Token, Error> {
        let create = db::types::TokenCreate {
            id: uuid().to_string(),
            kind: data.kind.to_str().to_string(),
            token: data.token,
            valid_until: data.valid_until.map(|x| x.naive_utc()),
            deleted_at: None,
            user_id: self.app.user().id.to_string(),
        };
        Ok(self.app.db()?.token_create(&create)?.into())
    }
}
