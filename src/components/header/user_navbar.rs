use sycamore::prelude::*;
use sycamore_rstml::html;

const LOGIN_URL: &str = "https://discord.com/oauth2/authorize?client_id=1248308695323115543&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fapi%2Fv1%2Fauth%2Fdiscord%2Fcallback&scope=guilds+guilds.members.read+identify";

#[component]
pub fn UserNavbar() -> View {
    html! {
    <section class="user-navbar">
       
    </section>    
    }
}