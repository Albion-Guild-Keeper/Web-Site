use sycamore::prelude::*;

#[component]
pub fn NotFoundPage() -> View {
    view! {
        section(id="not-found") {
            h1 { "404 Not Found" }
            p { "The page you are looking for does not exist." }
        }
    }
}