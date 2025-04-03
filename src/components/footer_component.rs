use sycamore::prelude::*;
use sycamore_rstml::html;

#[component]
pub fn Footer() -> View {
    html! {
        <footer>
            <p>"This is the footer content."</p>
        </footer>
    }
}