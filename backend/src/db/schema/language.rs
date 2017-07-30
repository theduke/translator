
table!(
  languages {
    id -> Text,
    name -> Text,
    parent_id -> Nullable<Text>,
    created_at -> BigInt,
    created_by -> Nullable<Text>,
  }
);

#[derive(Insertable, Queryable, AsChangeset,
Serialize, Deserialize, Debug, Clone)]
#[table_name="languages"]
pub struct Language {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: i64,
    pub created_by: Option<String>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
pub struct NewLanguage {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}