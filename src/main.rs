use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};
use sycamore_rstml::html;

mod pages;
mod components;

use components::{
    header::header::Header,
    footer::Footer,
};

use pages::{
    about::about_page,
    home::home_page,
    not_found::not_found_page,
};

#[derive(Route, Clone, Copy)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
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
