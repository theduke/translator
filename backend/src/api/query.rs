use juniper::{FieldResult as Res};

use ::db::schema::*;
use super::Ctx;

pub struct Query;

graphql_object!(Query: Ctx |&self| {

    field languages(&executor) -> Res<Vec<Language>> {
        let ctx = executor.context();
        let langs = ctx.repo()
                       .languages(ctx.user())?;
        Ok(langs)
    }

    field language(&executor, id: String) -> Res<Option<Language>> {
        let ctx = executor.context();
        let lang = ctx.repo()
                       .language(id, ctx.user())?;
        Ok(lang)
    }

    field users(&executor) -> Res<Vec<User>> {
        let ctx = executor.context();
        let langs = ctx.repo()
                       .users(ctx.user())?;
        Ok(langs)
    }

      field keys(&executor) -> Res<Vec<Key>> {
        let ctx = executor.context();
        let keys = ctx.repo()
                       .keys(ctx.user())?;
        Ok(keys)
    }

    field key(&executor, key: String) -> Res<Option<Key>> {
        let ctx = executor.context();
        let key = ctx.repo()
                       .key(key, ctx.user())?;
        Ok(key)
    }

    field translations(&executor, key: String) -> Res<Vec<Translation>> {
        let ctx = executor.context();
        let langs = ctx.repo()
                       .translations(key, ctx.user())?;
        Ok(langs)
    }

    field id() -> String {
        "a".to_string()
    }
});
