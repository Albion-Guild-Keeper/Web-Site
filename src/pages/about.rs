use sycamore::prelude::*;
use sycamore_rstml::html;

pub fn about_page() -> View {
    html! {
        <h1> "About Us" </h1>
        <p> "This is the content of the about page." </p>
        <p> "We are excited to share more information with you soon!" </p>
    }
}