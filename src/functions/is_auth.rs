use gloo_net::http::Request;

pub async fn is_auth() -> Result<bool, gloo_net::Error> {
    let url = "http://localhost:8000/api/v1/auth";
    let resp = Request::get(url).send().await?;
    Ok(resp.status() == 200)
}