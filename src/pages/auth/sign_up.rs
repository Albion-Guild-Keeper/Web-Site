use sycamore::prelude::*;
use sycamore_rstml::html;

use crate::components::auth::sign_up::SignUp;

pub fn sign_up_page() -> View {
    html! {
        <h1> "Sign Up" </h1>
        <SignUp />
    }
}