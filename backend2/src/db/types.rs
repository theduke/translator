use chrono::NaiveDateTime;

use super::schema::*;

#[derive(Queryable, Identifiable, Associations)]
pub struct User {
    pub id: String,
    pub role: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserCreate {
    pub id: String,
    pub role: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserUpdate {
    pub id: String,
    pub role: String,
    pub email: String,
    pub username: String,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserUpdatePassword {
    pub id: String,
    pub password_hash: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations)]
pub struct Token {
    pub id: String,
    pub kind: String,
    pub token: String,
    pub valid_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub user_id: String,
}

#[derive(Insertable)]
#[table_name = "tokens"]
pub struct TokenCreate {
    pub id: String,
    pub kind: String,
    pub token: String,
    pub valid_until: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub user_id: String,
}

#[derive(Queryable, Identifiable, Associations)]
pub struct Language {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub parent_id: Option<String>,
}

#[derive(Insertable)]
#[table_name = "languages"]
pub struct LanguageCreate {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "languages"]
pub struct LanguageUpdate {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations)]
pub struct Key {
    pub id: String,
    pub key: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub creator_id: String,
}

#[derive(Insertable)]
#[table_name = "keys"]
pub struct KeyCreate {
    pub id: String,
    pub key: String,
    pub description: Option<String>,
    pub creator_id: String,
}

#[derive(AsChangeset)]
#[table_name = "keys"]
pub struct KeyUpdate {
    pub id: String,
    pub key: String,
    pub description: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations)]
pub struct Translation {
    pub id: String,
    pub version: i32,
    pub translation: String,
    pub comment: Option<String>,
    pub language_id: String,
    pub key_id: String,
    pub creator_id: String,
    pub approver_id: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "translations"]
pub struct TranslationCreate {
    pub id: String,
    pub version: i32,
    pub translation: String,
    pub comment: Option<String>,
    pub language_id: String,
    pub key_id: String,
    pub creator_id: String,
    pub approver_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "translations"]
pub struct TranslationDelete {
    pub id: String,
    pub deleted_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations)]
pub struct TranslationRequest {
    pub id: String,
    pub translation: String,
    pub comment: Option<String>,
    pub language_id: String,
    pub key_id: String,
    pub creator_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "translation_requests"]
pub struct TranslationRequestCreate {
    pub id: String,
    pub translation: String,
    pub comment: Option<String>,
    pub language_id: String,
    pub key_id: String,
    pub creator_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "translation_requests"]
pub struct TranslationRequestUpdate {
    pub id: String,
    pub translation: String,
    pub comment: Option<String>,
    pub updated_at: NaiveDateTime,
}
