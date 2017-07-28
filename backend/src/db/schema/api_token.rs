
table!{
    api_tokens(token) {
        token -> Text,
        created_at -> BigInt,
        created_by -> Nullable<Text>,
    }
}

#[derive(Insertable, Queryable, AsChangeset,
Serialize, Deserialize, Debug, Clone)]
#[table_name="api_tokens"]
pub struct ApiToken {
    pub token: String,
    pub created_at: i64,
    // User.
    pub created_by: Option<String>,
}
