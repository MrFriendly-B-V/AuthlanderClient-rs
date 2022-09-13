use serde::Deserialize;
use anyhow::Result;
use crate::{CLIENT, request_deserialized};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id:     String,
    uri:    String
}

#[derive(Debug, Deserialize)]
pub struct Scopes {
    pub scopes:     Vec<String>,
    pub is_active:  bool
}

#[derive(Debug, Deserialize)]
pub struct Description {
    pub active:     bool,
    pub name:       Option<String>,
    pub email:      Option<String>,
    pub picture:    Option<String>

}

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token:   Option<String>,
    pub expiry:         Option<i64>,
    pub active:         bool
}

const SCOPES_ENDPOINT: &str = "/user/scopes/";
const DESCRIBE_ENDPOINT: &str = "/user/describe/";
const TOKEN_ENDPOINT: &str = "/token/get/";

impl User {
    /// Create a new user object
    ///
    /// It is the responsibility of the caller to make sure the user actually exists.
    /// [Self::get_scopes], [Self::token] and [Self::describe] return `Err` if the user does not exist.
    pub fn new<K, S>(id: K, server_uri: S) -> Self
        where   K: AsRef<str>,
                S: AsRef<str> {
        Self {
            id: id.as_ref().to_string(),
            uri: server_uri.as_ref().to_string()
        }
    }

    /// Get the scopes for the User
    ///
    /// It is the resposibility of the caller that this function is called on the Tokio 1.x runtime! (when using Actix 3)
    ///
    /// # Errors
    ///
    /// If the request fails
    pub async fn get_scopes(&self) -> Result<Scopes> {
        request_deserialized(&*self.id, &*self.uri, SCOPES_ENDPOINT).await
    }

    /// Get the user profile details
    ///
    /// It is the resposibility of the caller that this function is called on the Tokio 1.x runtime! (when using Actix 3)
    ///
    /// # Errors
    ///
    /// If the request fails
    pub async fn describe(&self) -> Result<Description> {
        request_deserialized(&*self.id, &*self.uri, DESCRIBE_ENDPOINT).await

    }

    /// Get a Google API OAuth2 access token for the user
    ///
    /// It is the resposibility of the caller that this function is called on the Tokio 1.x runtime! (when using Actix 3)
    ///
    /// # Errors
    ///
    /// If the request fails
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