use actix_web::HttpRequest;
use actix_web::http::{HeaderMap, HeaderValue};
use crate::Session;

#[derive(Debug)]
pub enum Error {
    MissingAuthHeader,
    AuthHeaderNonAscii,
    InternalError,
    InvalidSession,
    MissingScopes
}

pub async fn check_session(req: HttpRequest, server_uri: &str, scopes: Vec<&'static str>) -> Result<(), Error> {
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
    let valid = match session.check().await {
        Ok(c) => c,
        Err(_) => return Err(Error::InternalError)
    };

    if !valid.session_valid || !valid.active {
        return Err(Error::InvalidSession)
    }

    let user = match session.get_user().await {
        Ok(Some(u)) => u,
        Ok(None) | Err(_) => return Err(Error::InternalError)
    };

    let user_scopes = match user.get_scopes().await {
        Ok(s) => s,
        Err(e) => return Err(Error::InternalError)
    };

    let scopes_ref: Vec<&str> = user_scopes.scopes.iter().map(AsRef::as_ref).collect();
    let contains_all = scopes.iter().all(|item| scopes_ref.contains(item));
    if !contains_all {
        return Err(Error::MissingScopes)
    }

    Ok(())
}