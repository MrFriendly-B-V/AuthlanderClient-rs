mod session;

use serde::de::DeserializeOwned;
pub use session::*;

mod user;
pub use user::*;

#[cfg(any(feature = "actix-web-3", feature = "actix-web-4"))]
mod actix;
#[cfg(any(feature = "actix-web-3", feature = "actix-web-4"))]
pub use actix::*;

lazy_static::lazy_static! {
    pub(crate) static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

/// Describe
async fn request_deserialized<T>(id: &str, uri: &str, endpoint: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned
{
    Ok(CLIENT
        .get(format!("{uri}{endpoint}{id}"))
        .send()
        .await?
        .json()
        .await?)
}