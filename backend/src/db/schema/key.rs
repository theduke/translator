
table!(
  keys(key) {
    id -> Text,
    key -> Text,
    description -> Nullable<Text>,
    created_at -> BigInt,
    created_by -> Nullable<Text>,
  }
);

#[derive(Insertable, Queryable, AsChangeset,
         Serialize, Deserialize, Debug, Clone)]
#[table_name="keys"]
pub struct Key {
    pub id: String,
    pub key: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub created_by: Option<String>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
pub struct NewKey {
    pub key: String,
    pub description: Option<String>,
}
