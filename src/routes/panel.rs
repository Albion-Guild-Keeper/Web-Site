use sycamore_router::Route;

#[derive(Route, Clone, Copy)]
pub enum PanelRoutes {
    #[to("/dashboard")]
    Dashboard,
    #[to("/balance")]
    Balance,
    #[to("/profile")]
    Profile,
    #[not_found]
    NotFound,
}