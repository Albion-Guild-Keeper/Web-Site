use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct IsAdmin {
    pub is_admin: bool,
}

impl Default for IsAdmin {
    fn default() -> Self {
        Self { is_admin: false }
    }
}

pub async fn is_admin() -> Result<IsAdmin, gloo_net::Error> {
    let url = "http://localhost:8000/api/v1/";
    let resp = Request::get(url).send().await?;

    resp.json::<IsAdmin>().await
}