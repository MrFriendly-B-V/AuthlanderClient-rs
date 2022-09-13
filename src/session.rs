use serde::Deserialize;
use anyhow::Result;
use crate::{request_deserialized, User};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    id:     String,
    uri:    String
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Check {
    /// Whether the session is still valid. This implies:
    /// - It still exists
    /// - It has not yet expired
    pub session_valid:  bool,
    /// Whether the session is still active
    pub active:         bool
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Description {
    /// Whether the session is active
    pub active:     bool,
    /// The ID of the user associated with the session
    pub user_id:    Option<String>,
    /// The Unix epoch timestamp when the session expires
    pub expiry:     Option<i64>,
    /// The name of the user
    pub name:       Option<String>,
    /// The base64-encoded image of the user
    pub picture:    Option<String>,
    /// The email of the user
    pub email:      Option<String>,
}

const CHECK_ENDPOINT: &str = "/session/check/";
const DESCRIBE_ENDPOINT: &str = "/session/describe/";

impl Session {
    /// Create a new session object
    /// This does not check if the session exists. Use [Self::check] for this.
    pub fn new<K, S>(id: K, server_uri: S) -> Self
    where
        K: AsRef<str>,
        S: AsRef<str>,
    {
        Self {
            id: id.as_ref().to_string(),
            uri: server_uri.as_ref().to_string()
        }
    }

    /// Check that this session actually exists on the server
    ///
    /// It is the resposibility of the caller that this function is called on the Tokio 1.x runtime! (when using Actix 3)
    ///
    /// # Errors
    ///
    /// If the request fails
    pub async fn check(&self) -> Result<Check> {
        request_deserialized(&*self.id, &*self.uri, CHECK_ENDPOINT).await
    }

    /// Get detailed information about the session
    ///
    /// It is the resposibility of the caller that this function is called on the Tokio 1.x runtime! (when using Actix 3)
    ///
    /// # Errors
    ///
    /// If the request fails
    pub async fn describe(&self) -> Result<Description> {
        request_deserialized(&*self.id, &*self.uri, DESCRIBE_ENDPOINT).await
    }

    /// Get the associated user for this session
    ///
    /// It is the resposibility of the caller that this function is called on the Tokio 1.x runtime! (when using Actix 3)
    ///
    /// # Errors
    ///
    /// If the request fails
    pub async fn get_user(&self) -> Result<Option<User>> {
        let description = self.describe().await?;
        match description.user_id {
            Some(user_id) => Ok(Some(User::new(user_id, &self.uri))),
            None => Ok(None)
        }
    }
}