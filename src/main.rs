use dioxus::prelude::*;

mod components;
mod routes;
use routes::everyone::EveryoneRoutes;


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<EveryoneRoutes> {}
    }
}
