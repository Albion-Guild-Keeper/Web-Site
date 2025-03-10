use sycamore::prelude::*;
use sycamore_rstml::html;

#[component]
pub fn Footer() -> View {
    html! {
        <footer>
            <p> "This is the footer." </p>
            <p> "Additional footer content goes here." </p>
            <p> "Contact us at example@example.com" </p>
        </footer>
    }
}
