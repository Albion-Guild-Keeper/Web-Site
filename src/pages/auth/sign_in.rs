use sycamore::prelude::*;
use sycamore_rstml::html;

use crate::components::auth::sign_in::SignIn;

pub fn sign_in_page() -> View {
    html! {
        <h1> "Sign In" </h1>
        <SignIn />
    }
}