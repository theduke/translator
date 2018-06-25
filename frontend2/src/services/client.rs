use failure::Error;
use yew::{
    callback::Callback,
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: Token,
}

pub struct ClientService {
    fetch: FetchService,
}

impl ClientService {
    pub fn new() -> Self {
        ClientService {
            fetch: FetchService::new(),
        }
    }

    pub fn login(&mut self, data: Login, cb: Callback<Result<LoginResponse, Error>>) -> FetchTask {
        let handler = move |response: Response<Json<Result<LoginResponse, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                cb.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                cb.emit(Err(format_err!(
                    "{}: error getting profile https://gravatar.com/",
                    meta.status
                )))
            }
        };
        let request = Request::post("/api/login")
            .header("content-type", "application/json")
            .body(Json(&data))
            .expect("Could not construct request");
        self.fetch.fetch(request, handler.into())
    }
}
