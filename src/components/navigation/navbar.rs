use dioxus::prelude::*;

use crate::routes::everyone::EveryoneRoutes;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: EveryoneRoutes::Home {},
                "Home"
            }
            Link {
                to: EveryoneRoutes::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<EveryoneRoutes> {}
    }
}