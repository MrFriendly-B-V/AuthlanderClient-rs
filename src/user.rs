use serde::Deserialize;
use anyhow::Result;
use crate::CLIENT;

pub struct User {
    id:     String,
    uri:    String
}

#[derive(Deserialize)]
pub struct Scopes {
    pub scopes:     Vec<String>,
    pub is_active:  bool
}

#[derive(Deserialize)]
pub struct Description {
    pub active:     bool,
    pub name:       Option<String>,
    pub email:      Option<String>,
    pub picture:    Option<String>

}

#[derive(Deserialize)]
pub struct AccessToken {
    pub access_token:   Option<String>,
    pub expiry:         Option<i64>,
    pub active:         bool
}

const SCOPES_ENDPOINT: &str = "/user/scopes/";
const DESCRIBE_ENDPOINT: &str = "/user/describe/";
const TOKEN_ENDPOINT: &str = "/token/get/";

impl User {
    pub fn new<K, S>(id: K, server_uri: S) -> Self
        where   K: AsRef<str>,
                S: AsRef<str> {
        Self {
            id: id.as_ref().to_string(),
            uri: server_uri.as_ref().to_string()
        }
    }

    /// Get the scopes for the User
    pub async fn get_scopes(&self) -> Result<Scopes> {
        Ok(CLIENT
            .get(format!("{}{}{}", &self.uri, SCOPES_ENDPOINT, &self.id))
            .send()
            .await?
            .json()
            .await?)
    }

    /// Get the user profile details
    pub async fn describe(&self) -> Result<Description> {
        Ok(CLIENT
            .get(format!("{}{}{}", &self.uri, DESCRIBE_ENDPOINT, &self.id))
            .send()
            .await?
            .json()
            .await?)
    }

    /// Get a Google API OAuth2 access token for the user
    pub async fn token(&self, access_token: &str) -> Result<AccessToken> {
        Ok(CLIENT
            .get(format!("{}{}{}", &self.uri, TOKEN_ENDPOINT, &self.id))
            .header("Authorization", access_token)
            .send()
            .await?
            .json()
            .await?)
    }
}