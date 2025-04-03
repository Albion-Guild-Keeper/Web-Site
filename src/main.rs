use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};
use sycamore_rstml::html;

mod components;
mod pages;
mod utils;

use components::{
    footer_component::Footer, header_component::Header, login_dialog_component::LoginDialog,
};

use pages::{home_page::HomePage, login_page::login_page, not_found_page::not_found_page};

use utils::routes::{admin_route::AdminRoutes, app_route::AppRoutes};

#[component]
fn App() -> View {
    let logged = create_signal(false);
    view! {
        Router(
            integration = HistoryIntegration::new(),
            view = move |route: ReadSignal<AppRoutes>| {
                html! {
                    <Header />
                    <main> {match route.get() {
                        AppRoutes::Home => view! {
                            HomePage(signal=logged) {}
                        },
                        AppRoutes::Login => login_page(),
                        AppRoutes::Admin(route) => {
                            match route {
                                AdminRoutes::Home=>html!{<h1>"Admin Home Page"</h1>},
                                AdminRoutes::Console=>html!{<h1>"Admin Console Page"</h1>},
                                AdminRoutes::NotFound=>html!{<h1>"Admin Not Found"</h1>},
                            }
                        }
                        AppRoutes::NotFound => not_found_page(),
                    }}
                     {
                        if logged.get() {
                            view! {
                                LoginDialog(open=true) {}
                            }
                        } else {
                            view! {
                                LoginDialog(open=false) {}
                            }
                        }
                    }
                    </main>
                    <Footer />
                }
            }
        )
    }
}

fn main() {
    sycamore::render(App);
}
