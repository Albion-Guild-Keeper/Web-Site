use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        dialog {
            id: "login",
            open: true,
            h1 { "You must login to access the dashboard" }
            a {
                href: "https://discord.com/oauth2/authorize?client_id=1248308695323115543&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fapi%2Fv1%2Fauth%2Fdiscord%2Fcallback&scope=guilds+guilds.members.read+identify",
                img {
                    src: "https://img.icons8.com/ios11/512/FFFFFF/discord-logo.png",
                    alt: "Discord Logo",
                }
                span {
                    "LOGIN WITH DISCORD"
                }
            }
            p { "Terms of Service - Privacy" }
        }
    }
}
