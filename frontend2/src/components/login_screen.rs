use failure::Error;
use services::client::{ClientService, Login, LoginResponse};
use yew::prelude::*;

use super::error_msg;

pub struct LoginScreen {
    client: ClientService,
    link: ComponentLink<LoginScreen>,
    user: String,
    password: String,
    error: Option<String>,
    on_error: Callback<String>,
}

pub enum Message {
    UserChanged(String),
    PasswordChanged(String),
    Submit,
    OnError(String),
}

impl Component for LoginScreen {
    type Message = Message;
    type Properties = ();

    fn create(props: <Self as Component>::Properties, link: ComponentLink<Self>) -> Self {
        LoginScreen {
            on_error: link.send_back(|x: String| Message::OnError(x)),
            client: ClientService::new(),
            link,
            user: String::new(),
            password: String::new(),
            error: None,
        }
    }

    fn update(&mut self, msg: <Self as Component>::Message) -> bool {
        use self::Message::*;
        match msg {
            UserChanged(value) => {
                self.user = value;
            }
            PasswordChanged(value) => {
                self.password = value;
            }
            Submit => {
                let on_error = self.on_error.clone();
                self.client.login(
                    Login {
                        user: self.user.clone(),
                        password: self.password.clone(),
                    },
                    (move |res: Result<LoginResponse, Error>| {
                        match res {
                            Ok(data) => {

                            },
                            Err(e) => {
                                on_error.emit(e.to_string());
                            }
                        }

                    }).into(),
                );
            }
            OnError(msg) => {
                self.error = Some(msg);
            }
        }
        true
    }
}

impl Renderable<LoginScreen> for LoginScreen {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                // <form>
                    { error_msg::<Self>(self.error.as_ref()) }
                    <div>
                        <label>{ "User" }</label>
                        <input
                            type="text",
                            value=&self.user,
                            oninput=|e|Message::UserChanged(e.value),
                        ></input>
                    </div>

                    <div>
                        <label>{ "Password" }</label>
                        <input
                            type="password",
                            value=&self.password,
                            oninput=|e|Message::PasswordChanged(e.value),
                        ></input>
                    </div>

                    <button
                        onclick=|e|{
                            Message::Submit
                        },
                    >
                        { "Login" }
                    </button>
                // </form>
            </div>
        }
    }
}
