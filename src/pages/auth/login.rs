use sycamore::prelude::*;
use sycamore_rstml::html;

use crate::components::auth::login::Login;

pub fn login_page() -> View {
    html! {
        <h1> "Login" </h1>
        <Login />
    }
}