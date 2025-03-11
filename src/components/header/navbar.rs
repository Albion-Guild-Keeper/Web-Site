use sycamore::prelude::*;
use sycamore_rstml::html;
use sycamore::web::View;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use sycamore_web::Suspense;

use crate::components::header::user_navbar::UserNavbar;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct AdminCheck {
    is_admin: bool,
}

impl Default for AdminCheck {
    fn default() -> Self {
        Self { is_admin: false }
    }
}

async fn fetch_is_admin() -> Result<AdminCheck, gloo_net::Error> {
    let url = "http://localhost:8000/api/v1/"; // Replace with your API endpoint
    let resp = Request::get(url).send().await?;

    resp.json::<AdminCheck>().await
}

#[component]
pub async fn Navbar() -> View {
    let admin_check = fetch_is_admin().await.unwrap_or_default();
    console_log!("{:?}", admin_check.is_admin);

    html! {
        <nav>
            <a href="/"> "Home" </a>
            <a href="/about"> "About" </a>
            <a href="/hdfuiahuifd"> "Not Found" </a>
            {
                if admin_check.is_admin {
                    html! { <a href="/admin"> "Admin Panel" </a> }
                } else {
                    html! {}
                }
            }
        </nav>
        <UserNavbar />
    }
}