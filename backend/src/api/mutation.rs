use juniper::{FieldResult as Res};

pub use super::{Ctx};
use ::db::schema::*;

pub struct Mutation;

graphql_object!(Mutation: Ctx |&self| {

    field login(&executor, user: String, password: String) -> Res<ApiToken> {
        let ctx = executor.context();
        let token = ctx.repo()
                       .login(user, password)?;
        Ok(token)
    }

    field create_language(&executor, lang: NewLanguage) -> Res<Language> {
        let ctx = executor.context();
        let lang = ctx.repo()
                       .create_language(lang, ctx.user())?;
        Ok(lang)
    }

    field delete_language(&executor, lang: String) -> Res<String> {
        let ctx = executor.context();
            ctx.repo()
               .delete_language(lang.clone(), ctx.user())?;
        Ok(lang)
    }

    field create_key(&executor, key: NewKey) -> Res<Key> {
        let ctx = executor.context();
        let key = ctx.repo()
                       .create_key(key, ctx.user())?;
        Ok(key)
    }

    field delete_key(&executor, key: String) -> Res<bool> {
        let ctx = executor.context();
            ctx.repo()
               .delete_key(key, ctx.user())?;
        Ok(true)
    }

    field translate(&executor, translation: NewTranslation) -> Res<Translation> {
        let ctx = executor.context();
        let translation = ctx.repo()
                       .translate(translation, ctx.user())?;
        Ok(translation)
    }

    field update_translation(&executor, translation: NewTranslation) -> Res<Translation> {
        let ctx = executor.context();
        let translation = ctx.repo()
                       .update_translation(translation, ctx.user())?;
        Ok(translation)
    }

    field delete_translation(&executor, language: String, key: String) -> Res<bool> {
        let ctx = executor.context();
            ctx.repo()
               .delete_translation(language, key, ctx.user())?;
        Ok(true)
    }

});
