use sycamore_router::Route;

use crate::routes::{admin::AdminRoutes, panel::PanelRoutes};

#[derive(Route, Clone, Copy)]
pub enum AppRoutes {
    #[to("/sign-in")]
    SignIn,
    #[to("/sign-up")]
    SignUp,
    #[to("/")]
    Home,
    #[to("/about")]
    About,
    #[to("/admin/<_..>")]
    Admin(AdminRoutes),
    #[to("/panel/<_..>")]
    Panel(PanelRoutes),
    #[not_found]
    NotFound,
}