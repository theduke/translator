use failure::Error;
use actix_web::{
    server,
    App as Application,
    HttpRequest,
    Json,
    State,
    http::Method,
};

use super::app::{self, App};

fn index(_req: HttpRequest<App>) -> &'static str {
    "Hello world!"
}

fn auth_login((app, data): (State<App>, Json<app::users::UserLogin>))
                   -> Result<Json<app::users::UserLoginResult>, Error>
{
    Ok(Json(app.users().login(data.clone())?))
}

fn language_create((app, data): (State<App>, Json<app::languages::LanguageCreate>))
    -> Result<Json<app::languages::Language>, Error>
{
    Ok(Json(app.languages().create(data.clone())?))
}

fn language_update((app, data): (State<App>, Json<app::languages::LanguageUpdate>))
                   -> Result<Json<app::languages::Language>, Error>
{
    Ok(Json(app.languages().update(data.clone())?))
}
pub fn run_server(app: App) {
    server::new(move ||
        Application::with_state(app.clone())
            .resource("/", |r| r.f(index))
            .resource("/api/auth/login", |r| r.method(Method::POST).with(auth_login))
            .resource("/api/language", |r| r.method(Method::POST).with(language_create))
    )
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}