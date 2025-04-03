use sycamore::prelude::*;
use sycamore_rstml::html;

pub fn not_found_page() -> View {
    html! {
        <h1> "404 Not Found" </h1>
        <p> "The page you are looking for does not exist." </p>
        <p> "Please check the URL or return to the home page." </p>
    }
}