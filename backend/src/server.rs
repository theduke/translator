use std::io::Cursor;

use rocket::{self, State, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use rocket_contrib::Json;
use rocket::response::Content;
use rocket::response::content;
use serde_json::{self};
use juniper_rocket;

use ::error::*;
use ::db::{self, Db, BaseData, TranslationData};
use ::commands::{Ctx};
use ::api::{self, Schema};
use ::app::App;
use ::repo::{ExportFormat};

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type,, Authorization, Access-Control-Allow-Headers"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandError {
    code: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum CommandResult {
    Ok {
        data: ::serde_json::Value,
    },
    Error {
        error: CommandError,
    },
}

#[get("/")]
fn index() -> Content<&'static str> {
    let index_file = include_bytes!("../../frontend/dist/index.html");
    let content = ::std::str::from_utf8(&index_file[0..]).unwrap();

    Content(ContentType::HTML, content)
}

#[get("/assets/bundle.js")]
fn assets_js() -> Content<&'static str> {
    let js_bundle = include_bytes!("../../frontend/dist/assets/bundle.js");
    let content = ::std::str::from_utf8(&js_bundle[0..]).unwrap();

    Content(ContentType::JavaScript, content)
}

#[derive(FromForm)]
struct ExportArgs {
    format: Option<String>,
    pretty: Option<bool>,
    token: Option<String>,
}

#[get("/export/translations/<lang>?<args>")]
fn export_translations(lang: String, args: ExportArgs, app: State<App>) -> Result<Content<String>> {
    let format = args.format
                     .and_then(|x| ExportFormat::from_str(&x))
                     .unwrap_or(ExportFormat::Json);
    let pretty = args.pretty.unwrap_or(false);


    let export = app.repo().translations_export(lang, format, pretty)?;
    Ok(Content(ContentType::JSON, export))
}

#[get("/export/keys?<args>")]
fn export_keys(args: ExportArgs, app: State<App>) -> Result<Content<String>> {
    let format = args.format.and_then(|x| ExportFormat::from_str(&x)).unwrap_or(ExportFormat::Json);
    let pretty = args.pretty.unwrap_or(false);

    let export = app.repo().keys_export(format, pretty)?;
    Ok(Content(ContentType::JSON, export))
}


#[get("/export/all")]
fn export_all(app: State<App>) -> Result<Json<db::Export>> {
    let export = app.repo().export()?;
    Ok(Json(export))
}



#[get("/api/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/api/graphql")
}


#[options("/api/graphql")]
fn get_graphql_options() -> &'static str {
    ""
}

#[get("/api/graphql?<request>")]
fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    app: State<App>,
) -> juniper_rocket::GraphQLResponse {
    let ctx = Ctx::new(app.clone(), None);
    request.execute(&schema, &ctx)
}

#[post("/api/graphql", data="<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    app: State<App>,
) -> juniper_rocket::GraphQLResponse {
    let ctx = Ctx::new(app.clone(), None);
    request.execute(&schema, &ctx)
}

/*

#[get("/api/base-data")]
fn api_base_data(db: State<Db>) -> Result<Json<BaseData>> {
  let data = db.base_data()
    .map(|d| Json(d))?;
    Ok(data)
}

#[get("/api/translations/<key>")]
fn api_translations(key: String, db: State<Db>)
  -> Result<Json<TranslationData>>
{
  let data = db.translations(key)
    .map(|d| Json(d))?;
    Ok(data)
}

#[post("/api/command", data="<cmd>")]
fn api_command(cmd: Json<Command>, db: State<Db>)
  -> Json<CommandResult>
{
    let res = match db.command(&*cmd) {
        Ok(data) => CommandResult::Ok {
            data: json!(data),
        },
        Err(e) => CommandResult::Error{
            error: CommandError{
                code: e.to_string(),
            },
        },
    };
    Json(res)
}



*/

pub fn run(app: ::app::App) {
    use rocket::config::*;


    let config = {
        let c = app.config();
        ConfigBuilder::new(Environment::Production)
            .port(c.port)
            .log_level(LoggingLevel::Debug)
            .secret_key(c.secret.as_str())
            .workers(8)
            .unwrap()
    };

    let schema = api::new_schema();

    rocket::custom(config, true)
        .attach(CORS)
        .manage(app)
        .manage(schema)
        .mount("/", routes![
            index,
            export_translations,
            export_keys,
            export_all,
            assets_js,
            // Juniper graphql handlers.
            graphiql,
            get_graphql_handler,
            get_graphql_options,
            post_graphql_handler,
        ])
        .launch();
}
