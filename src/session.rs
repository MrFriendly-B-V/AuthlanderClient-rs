use serde::Deserialize;
use anyhow::Result;
use crate::{CLIENT, User};

pub struct Session {
    id:     String,
    uri:    String
}

#[derive(Deserialize)]
pub struct Check {
    pub session_valid:  bool,
    pub active:         bool
}

#[derive(Deserialize)]
pub struct Description {
    pub active:     bool,
    pub user_id:    Option<String>,
    pub expiry:     Option<i64>,
    pub name:       Option<String>,
    pub picture:    Option<String>,
    pub email:      Option<String>,
}

const CHECK_ENDPOINT: &str = "/session/check/";
const DESCRIBE_ENDPOINT: &str = "/session/describe/";

impl Session {
    pub fn new<K, S>(id: K, server_uri: S) -> Self
    where   K: AsRef<str>,
            S: AsRef<str> {
        Self {
            id: id.as_ref().to_string(),
            uri: server_uri.as_ref().to_string()
        }
    }

    pub async fn check(&self) -> Result<Check> {
        Ok(CLIENT
            .get(format!("{}{}{}", &self.uri, CHECK_ENDPOINT, &self.id))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn describe(&self) -> Result<Description> {
        Ok(CLIENT
            .get(format!("{}{}{}", &self.uri, DESCRIBE_ENDPOINT, &self.id))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_user(&self) -> Result<Option<crate::User>> {
        let description = self.describe().await?;
        match description.user_id {
            Some(user_id) => Ok(Some(User::new(user_id, &self.uri))),
            None => Ok(None)
        }
    }
}