
table!{
    api_tokens(token) {
        token -> Text,
        kind -> Text,
        created_at -> BigInt,
        expires_at -> Nullable<BigInt>,
        created_by -> Nullable<Text>,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Session,
    Api,
}

impl TokenKind {
    pub fn to_str(&self) -> &'static str {
        match *self {
            TokenKind::Session => "session",
            TokenKind::Api => "api",
        }
    }
}

#[derive(Insertable, Queryable, AsChangeset,
Serialize, Deserialize, Debug, Clone)]
#[table_name="api_tokens"]
pub struct ApiToken {
    pub token: String,
    pub kind: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub created_by: Option<String>,
}
