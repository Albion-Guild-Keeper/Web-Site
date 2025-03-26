use sycamore_router::Route;

#[derive(Route, Clone, Copy)]
pub enum AdminRoutes {
    #[to("/")]
    Home,
    #[to("/console")]
    Console,
    #[not_found]
    NotFound,
}