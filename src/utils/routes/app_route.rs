use sycamore_router::Route;

use crate::utils::routes::admin_route::AdminRoutes;

#[derive(Route, Clone, Copy)]
pub enum AppRoutes {
    #[to("/login")]
    Login,
    #[to("/")]
    Home,
    #[to("/admin/<_..>")]
    Admin(AdminRoutes),
    #[not_found]
    NotFound,
}