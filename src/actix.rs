#[cfg(feature = "actix-web-3")]
use actix_web_3::{
    HttpRequest,
    http::{HeaderMap, HeaderValue},
};

#[cfg(feature = "actix-web-4")]
use actix_web_4::{
    HttpRequest,
    http::header::{HeaderMap, HeaderValue},
};

use crate::{Session, User};

#[derive(Debug)]
pub enum Error {
    /// The request is missing the Authorization header
    MissingAuthHeader,
    /// The Authorization header is present, but contains non-ASCII characters
    AuthHeaderNonAscii,
    /// Internal error to the Authlander server
    InternalError(Box<dyn std::error::Error>),
    /// The session is invalid. This implies:
    /// - That it does not exist
    /// - Or that it has expired
    /// - Or that it is not active
    InvalidSession,
    /// The user is missing one or more of the required scopes
    MissingScopes
}

#[derive(Debug, Clone)]
pub struct SessionCheck {
    pub session: Session,
    pub user: User,
}

/// Check that a session exists
///
/// # Errors
///
/// Refer to [Error] variants
pub async fn check_session(req: HttpRequest, server_uri: &str, scopes: Vec<&'static str>) -> Result<SessionCheck, Error> {
    let hm: &HeaderMap = req.headers();
    let hv: &HeaderValue = match hm.get("authorization") {
        Some(hv) => hv,
        None => return Err(Error::MissingAuthHeader)
    };

    let str = match hv.to_str() {
        Ok(s) => s,
        Err(_) => return Err(Error::AuthHeaderNonAscii)
    };

    let session = Session::new(str, server_uri);
    let valid = session.check()
        .await
        .map_err(|e| Error::InternalError(e.into()))?;

    if !valid.session_valid || !valid.active {
        return Err(Error::InvalidSession)
    }

    let user = session.get_user()
        .await
        .map_err(|e| Error::InternalError(e.into()))?
        .expect("User no longer exists after checking that the session exists");
    let user_scopes = user.get_scopes().await.map_err(|e| Error::InternalError(e.into()))?;

    let scopes_ref: Vec<&str> = user_scopes.scopes.iter().map(AsRef::as_ref).collect();
    let contains_all = scopes.iter().all(|item| scopes_ref.contains(item));
    if !contains_all {
        return Err(Error::MissingScopes)
    }

    Ok(SessionCheck {
        user,
        session,
    })
}