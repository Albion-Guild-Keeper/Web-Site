use sycamore::{prelude::*, web::rt::web_sys};
use sycamore_rstml::html;

#[component]
pub fn LoginButton() -> View {
    html! {
        <button
            on:click=|_| {
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href("https://discord.com/oauth2/authorize?client_id=1248308695323115543&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fapi%2Fv1%2Fauth%2Fcallback&scope=guilds+identify+guilds.members.read")
                    .unwrap();
            }
            class="login-button"
        >
            "Login"
        </button>
    }
}
