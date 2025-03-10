use sycamore::prelude::*;
use sycamore_rstml::html;

use crate::components::header::navbar::Navbar;

#[component]
pub fn Header() -> View {
    html! {
        <header>
            <Navbar />
        </header>
    }
}
