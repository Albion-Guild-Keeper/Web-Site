use dioxus::prelude::*;

use crate::components::auth::login::Login;
use crate::components::hero::Hero;

#[component]
pub fn Home() -> Element {
    rsx! {
        if true {
            Login {}
        } else {
            Hero {}
        }
    }
}
