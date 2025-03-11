use sycamore::prelude::*;
use sycamore_rstml::html;

pub fn admin_home_page() -> View {
    html! {
        <h1> "Admin Home Page" </h1>
        <p> "This is the content of the home page." </p>
        <p> "We are excited to share more with you soon!" </p>
    }
}