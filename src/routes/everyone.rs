use dioxus::prelude::*;

use crate::components::navigation::navbar::Navbar;
use crate::components::home::Home;
use crate::components::auth::login::Login;
use crate::components::blog::Blog;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum EveryoneRoutes {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/login")]
    Login {},
}