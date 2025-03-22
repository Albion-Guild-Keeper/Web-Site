use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};
use sycamore_rstml::html;

mod components;
mod functions;
mod pages;
mod routes;

use routes::{admin::AdminRoutes, app::AppRoutes, panel::PanelRoutes};

use components::{footer::Footer, header::header::Header};

use pages::{
    about::about_page, admin::home::admin_home_page, home::home_page, not_found::not_found_page, auth::sign_in::sign_in_page, auth::sign_up::sign_up_page,
};

#[component]
fn App() -> View {
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| {
                html! {
                    <Header />
                    <main> {match route.get() {
                        AppRoutes::SignIn => sign_in_page(),
                        AppRoutes::SignUp => sign_up_page(),
                        AppRoutes::Home => home_page(),
                        AppRoutes::About => about_page(),
                        AppRoutes::Admin(route) => {
                            match route {
                                AdminRoutes::Home => admin_home_page(),
                                AdminRoutes::Console => admin_home_page(),
                                AdminRoutes::NotFound => not_found_page(),
                            }
                        }
                        AppRoutes::Panel(route) => {
                            match route {
                                PanelRoutes::Dashboard => home_page(),
                                PanelRoutes::Balance => home_page(),
                                PanelRoutes::Profile => home_page(),
                                PanelRoutes::NotFound => not_found_page(),
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
    sycamore::render(App);
}
