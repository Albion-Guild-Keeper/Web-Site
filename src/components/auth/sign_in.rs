use sycamore::prelude::*;
use sycamore_rstml::html;

const LOGIN_URL: &str = "https://discord.com/oauth2/authorize?client_id=1248308695323115543&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fapi%2Fv1%2Fauth%2Fdiscord%2Fcallback&scope=guilds+guilds.members.read+identify";


#[component]
pub async fn SignIn() -> View {
    html! {
        <div class="sign-in-container">
            <dialog style="display: block">
            <div class="login-with-discord">
                <h1>You must Login to access the dashboard</h1>
                <a href=LOGIN_URL>
                    <img src="https://img.icons8.com/ios11/512/FFFFFF/discord-logo.png" alt="Discord Logo" />
                    LOGIN WITH DISCORD
                </a>
                <p>Terms of Service - Privacy</p>
                </div>
            </dialog>
        </div>
    }
}
