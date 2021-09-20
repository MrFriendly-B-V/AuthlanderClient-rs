mod session;
pub use session::*;

mod user;
pub use user::*;

#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "actix")]
pub use actix::*;

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::new();
}
