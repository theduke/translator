

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", content = "data")]
pub enum Command {
    Login {
        username: String,
        password: String,
    },
    CreateUser {
        username: String,
        role: String,
        password: String,
    },
    UpdateUser {
        username: String,
        password: String,
    },
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
