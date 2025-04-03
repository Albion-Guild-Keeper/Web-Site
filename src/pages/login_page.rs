use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore_futures::spawn_local;
use sycamore_rstml::html;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CatResponse {
    fact: String,
    length: u32,
}

async fn http_request() -> Result<CatResponse, String> {
    let response = Request::get("https://catfact.ninja/fact")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == 200 {
        let json: CatResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(json)
    } else {
        Err(format!("Error: {}", response.status()))
    }
}

pub fn login_page() -> View {
    let test_response = create_signal("Loading...".to_string());

    spawn_local(async move {
        let result = http_request().await;
        match result {
            Ok(cat_response) => {
                test_response.set(format!("Fact: {}, Length: {}", cat_response.fact, cat_response.length));
            }
            Err(err) => {
                test_response.set(format!("Error: {}", err));
            }
        }
    });

    html! {
        <article>
            <h1> "Login Page" </h1>
            <p> { test_response } </p>
        </article>
    }
}