use std::fmt::Debug;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use ::error::*;
use ::app::App;
use db::{Db};
use ::db::schema::{User};
use ::repo::Repo;

pub struct Ctx {
    app: App,
    user: Option<User>,
}

impl Ctx {
    pub fn new(app: App, user: Option<User>) -> Self {
        Ctx { app, user }
    }

    pub fn repo(&self) -> Repo {
        self.app.repo()
    }

    pub fn db(&self) -> Result<Db> {
        self.app.db()
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }
}

pub trait CommandExecutor: Serialize + DeserializeOwned + Debug {
    fn execute(&self, ctx: Ctx) -> Result<Value>;
}

/*

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct CreateUser {
    pub username: String,
    pub role: String,
    pub password: String,
}


#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", content = "data")]
pub enum Command {
    Login(Login),
    CreateUser(CreateUser),
    UpdateUser(UpdateUser),
    DeleteUser {
        username: String,
    },
    CreateLanguage {
        id: String,
        name: String,
        parent_id: Option<String>,
    },
    DeleteLanguage { id: String },
    CreateKey {
        key: String,
        description: Option<String>,
    },
    DeleteKey {
        key: String,
    },
    CreateTranslation {
        lang: String,
        key: String,
        value: String,
    },
    UpdateTranslation {
        lang: String,
        key: String,
        value: String,
    },
    DeleteTranslation {
        lang: String,
        key: String,
    },
}
*/
