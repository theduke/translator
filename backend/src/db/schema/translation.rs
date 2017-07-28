
table!(
  translations(language, key) {
    language -> Text,
    key -> Text,
    value -> Text,
    created_at -> BigInt,
    updated_at -> BigInt,
    created_by -> Nullable<Text>,
  }
);

#[derive(Insertable, Queryable, AsChangeset,
Serialize, Deserialize, Debug, Clone)]
#[table_name="translations"]
pub struct Translation {
    pub language: String,
    pub key: String,
    pub value: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub created_by: Option<String>,
}
