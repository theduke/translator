pub mod login_screen;
pub use self::login_screen::LoginScreen;

use yew::prelude::{Html, Component};

pub fn error_msg<T: Component>(msg: Option<&String>) -> Html<T> {
    match msg {
        Some(msg) => {
            html!{
                    <div>
                        { msg }
                    </div>
                }

        }
        None => {
            html!{<span />}
        }
    }
}
