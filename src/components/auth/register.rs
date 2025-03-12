use sycamore::prelude::*;
use sycamore_rstml::html;

#[component]
pub fn register_page() -> View {
    html! {
        <h1> "Register" </h1>
        <input r#type="text" placeholder="Enter your Discord Token" />
        <input r#type="text" placeholder="Enter your email" />
        <button> "Register with Discord" </button>
    }
}