use reqwest;

#[allow(dead_code)]
pub async fn is_admin() -> bool {
    let client = reqwest::Client::new();

    let response = client.get("https://rust-guild-api-kvdl.shuttle.app/") // Replace with your actual endpoint
        .send()
        .await;

    match response {
        Ok(response) => {
            let body = response.text().await.unwrap_or_default();
            let result = body == "admin";
            result
        }
        Err(_) => false,
    }
}