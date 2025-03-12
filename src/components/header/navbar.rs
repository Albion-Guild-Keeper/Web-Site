use sycamore::prelude::*;
use sycamore_rstml::html;

use crate::components::header::user_navbar::UserNavbar;
use crate::functions::is_admin::is_admin;

#[component]
pub async fn Navbar() -> View {
    let result = is_admin().await.unwrap_or_default();
    console_log!("{:?}", result);

    html! {
        <nav>
            <a href="/"> "Home" </a>
            <a href="/about"> "About" </a>
            <a href="/hdfuiahuifd"> "Not Found" </a>
            {
                if result.is_admin {
                    html! { <a href="/admin"> "Admin Panel" </a> }
                } else {
                    html! {}
                }
            }
        </nav>
        <UserNavbar />
    }
}
