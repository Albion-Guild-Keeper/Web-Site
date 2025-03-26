use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use gloo_net::http::Request;

#[component]
pub async fn Header() -> View {
    let guilds = get_guilds().await.unwrap_or_default();
    
    tracing::info!("Guilds: {:?}", guilds);
    // spawn_local(async move {
    //     let guilds = get_guilds().await;
    // });

    // println!("{:#?}", guilds);

    view! {
        header(id="header") {
            section(id="logo") {
                select(id="logo") {
                    option { "Sycamore" }
                    option { "Sycamore" }
                    option { "Sycamore" }
                    option { "Sycamore" }
                }
                a(href="https://discord.com/oauth2/authorize?client_id=1248308695323115543&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fapi%2Fv1%2Fauth%2Fdiscord%2Fcallback&scope=guilds+identify+guilds.members.read") { "Login With Discord" }
            }
            // UserBar {}
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetGuilds {
    id: String,
}

async fn get_guilds() -> Result<Vec<GetGuilds>, gloo_net::Error> {
    let url = "http://localhost:8000/api/v1/guilds/@me"; // Updated endpoint for guilds
    let resp = Request::get(url).send().await?;

    tracing::info!("Response: {:?}", resp);

    resp.json::<Vec<GetGuilds>>().await
}