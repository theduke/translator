use juniper::{FieldResult as Res, ResultExt};

pub use super::{Ctx};
use ::db::schema::*;

pub struct Mutation;

graphql_object!(Mutation: Ctx |&self| {

    field login(&executor, user: String, password: String) -> Res<ApiToken> {
        let ctx = executor.context();
        let token = ctx.repo()
                       .login(user, password)
                       .to_field_err()?;
        Ok(token)
    }

    field create_language(&executor, lang: NewLanguage) -> Res<Language> {
        let ctx = executor.context();
        let lang = ctx.repo()
                       .create_language(lang, ctx.user())
                       .to_field_err()?;
        Ok(lang)
    }

    field delete_language(&executor, lang: String) -> Res<String> {
        let ctx = executor.context();
            ctx.repo()
               .delete_language(lang.clone(), ctx.user())
               .to_field_err()?;
        Ok(lang)
    }

    field create_key(&executor, key: NewKey) -> Res<Key> {
        let ctx = executor.context();
        let key = ctx.repo()
                       .create_key(key, ctx.user())
                       .to_field_err()?;
        Ok(key)
    }

    field delete_key(&executor, key: String) -> Res<bool> {
        let ctx = executor.context();
            ctx.repo()
               .delete_key(key, ctx.user())
               .to_field_err()?;
        Ok(true)
    }

    field translate(&executor, translation: NewTranslation) -> Res<Translation> {
        let ctx = executor.context();
        let translation = ctx.repo()
                       .translate(translation, ctx.user())
                       .to_field_err()?;
        Ok(translation)
    }

    field update_translation(&executor, translation: NewTranslation) -> Res<Translation> {
        let ctx = executor.context();
        let translation = ctx.repo()
                       .update_translation(translation, ctx.user())
                       .to_field_err()?;
        Ok(translation)
    }

    field delete_translation(&executor, language: String, key: String) -> Res<bool> {
        let ctx = executor.context();
            ctx.repo()
               .delete_translation(language, key, ctx.user())
               .to_field_err()?;
        Ok(true)
    }

});

