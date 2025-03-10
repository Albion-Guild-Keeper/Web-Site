use sycamore::prelude::*;
use sycamore_rstml::html;

use crate::components::header::user_navbar::UserNavbar;

#[component]
pub fn Navbar() -> View {
    html! {
        <nav>
            <a href="/"> "Home" </a>
            <a href="/about"> "About" </a>
            <a href="/hdfuiahuifd"> "Not Found" </a>
        </nav>
        <UserNavbar />
    }
}
