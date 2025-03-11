use sycamore_router::Route;

use crate::routes::admin::AdminRoutes;

#[derive(Route, Clone, Copy)]
pub enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/about")]
    About,
    #[to("/admin/<_..>")]
    Admin(AdminRoutes),
    #[not_found]
    NotFound,
}