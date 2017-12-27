use uuid::Uuid;

use super::key::keys;

table!(
  translations {
    id -> Text,
    language_id -> Text,
    key_id -> Text,
    version -> Int4,
    value -> Text,
    created_at -> BigInt,
    updated_at -> BigInt,
    created_by -> Nullable<Text>,
  }
);

joinable!(translations -> super::key::keys (key_id));
allow_tables_to_appear_in_same_query!(translations, keys);

#[derive(Insertable, Queryable, AsChangeset,
Serialize, Deserialize, Debug, Clone)]
#[table_name="translations"]
pub struct Translation {
    pub id: String,
    pub language_id: String,
    pub key_id: String,
    pub version: i32,
    pub value: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub created_by: Option<String>,
}

#[derive(GraphQLInputObject, Debug)]
pub struct NewTranslation {
    pub language_id: Uuid,
    pub key_id: Uuid,
    pub value: String,
}
