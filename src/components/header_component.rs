use sycamore::prelude::*;
use sycamore_rstml::html;

#[component]
pub fn Header() -> View {
    html! {
        <header>
            <h1>"This is the header content."</h1>
        </header>
    }
}