use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};

mod components;
mod routes;

use routes::app::AppRoutes;

use components::{
    footer::Footer, header::Header, home::HomePage, navigation::navbar::NavBar,
    not_found::NotFoundPage,
};

#[component]
fn App() -> View {
    let test = create_signal(false);
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=move |route: ReadSignal<AppRoutes>| {
                let current_route = route.get();
                view! {
                    Header {}
                    div(id="body") {
                        NavBar {}
                        main {
                            h1 {
                                "Rotta: " (test.get().to_string())
                            }
                            (if test.get() == true {
                                view! { p { "Signal is true!" } }
                            } else {
                                view! { p { "Signal is false!" } }
                            })
                            (match current_route {
                                AppRoutes::Home => view! {
                                    HomePage(signal=test) {}
                                },
                                _ => view! {
                                    NotFoundPage()
                                }
                            })
                        }
                    }
                    Footer {}
                }
            }
        )
    }
}

fn main() {
    sycamore::render(App);
}
