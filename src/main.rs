use cookie_rs::{cookie::SameSite, Cookie, CookieJar};
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};
use sycamore_rstml::html;

mod components;
mod functions;
mod pages;
mod routes;

use routes::{admin::AdminRoutes, app::AppRoutes};

use components::{footer::Footer, header::header::Header};

use pages::{
    about::about_page, admin::home::admin_home_page, home::home_page, not_found::not_found_page,
};

fn test_cookie() {
    let cookie = Cookie::new("test", "test").with_path("/");
    let mut jar = CookieJar::default();

    jar.add(cookie.clone());

    cookie.with_same_site(SameSite::Lax);
}   

#[component]
fn App() -> View {
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| {
                html! {
                    <Header />
                    <main class="app"> {match route.get() {
                        AppRoutes::Home => home_page(),
                        AppRoutes::About => about_page(),
                        AppRoutes::Admin(route) => {
                            match route {
                                AdminRoutes::Home => admin_home_page(),
                                AdminRoutes::Console => admin_home_page(),
                                AdminRoutes::NotFound => not_found_page(),
                            }
                        }
                        AppRoutes::NotFound => not_found_page(),
                    }}</main>
                    <Footer />
                }
            }
        )
    }
}

fn main() {
    test_cookie();
    sycamore::render(App);
}
