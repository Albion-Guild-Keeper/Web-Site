use sycamore::prelude::*;
use sycamore_rstml::html;

#[component]
pub fn UserNavbar() -> View {
    html! {
    <section class="user-navbar">
        <a href="https://discord.com/oauth2/authorize?client_id=1248308695323115543&response_type=code&redirect_uri=http%3A%2F%2F127.0.0.1%3A8000%2Fauth%2Fdiscord&scope=identify">
            <img src="https://img.icons8.com/ios11/512/FFFFFF/discord-logo.png" alt="Discord Logo" />
            <span>Login with Discord</span>
        </a>
    </section>    
    }
}