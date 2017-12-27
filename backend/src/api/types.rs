use juniper::{FieldResult};

use ::db::schema::*;
use super::Ctx;

graphql_object!(Language: Ctx |&self| {
    field id() -> String {
        self.id.clone()
    }

    field code() -> &str {
        &self.code
    }

    field name() -> String {
        self.name.clone()
    }

    field parent_id() -> Option<String> {
        self.parent_id.clone()
    }

    field created_at() -> f64 {
        self.created_at as f64
    }

    field created_by() -> Option<String> {
        self.created_by.clone()
    }
});

graphql_object!(Translation: Ctx |&self| {

    field id() -> String {
        self.id.clone()
    }

    field language_id() -> String {
        self.language_id.clone()
    }

    field key_id() -> String {
        self.key_id.clone()
    }

    field version() -> i32 {
        self.version as i32
    }

    field value() -> String {
        self.value.clone()
    }

    field created_at() -> f64 {
        self.created_at as f64
    }

    field updated_at() -> f64 {
        self.updated_at as f64
    }

    field created_by() -> Option<String> {
        self.created_by.clone()
    }

});

graphql_object!(User: Ctx |&self| {

    field id() -> String {
        self.id.clone()
    }

    field username() -> String {
        self.username.clone()
    }

    field role() -> String {
        self.role.clone()
    }

    field password_hash() -> String {
        self.password_hash.clone()
    }

    field created_at() -> f64 {
        self.created_at as f64
    }
});

graphql_object!(Key: Ctx |&self| {

    field id() -> String {
        self.id.clone()
    }

    field key() -> String {
        self.key.clone()
    }

    field description() -> Option<String> {
        self.description.clone()
    }

    field created_at() -> f64 {
        self.created_at as f64
    }

    field created_by() -> Option<String> {
        self.created_by.clone()
    }

    field translations(&executor) -> FieldResult<Vec<Translation>> {
        let ctx = executor.context();
        let trans = ctx.repo().translations(&self.id, ctx.user())?;
        Ok(trans)
    }
});

graphql_object!(ApiToken: Ctx |&self| {
    field token() -> String {
        self.token.clone()
    }

    field created_at() -> f64 {
        self.created_at as f64
    }

    field expires_at() -> Option<f64> {
        self.expires_at.map(|x| x as f64)
    }

    field created_by() -> Option<String> {
        self.created_by.clone()
    }
});
