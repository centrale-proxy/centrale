use actix_web::{
    Error, FromRequest, HttpMessage, HttpRequest, dev::Payload, error::ErrorUnauthorized,
};
use std::future::{Ready, ready};

#[derive(Clone, Debug)]
pub struct CentraleUser {
    pub user_id: i64,
    pub subdomain: String,
    pub role: String,
    pub pass: String,
    pub url: String,
}

// Implement FromRequest for CentraleUser
impl FromRequest for CentraleUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.extensions().get::<CentraleUser>() {
            Some(user) => ready(Ok(user.clone())),
            None => ready(Err(ErrorUnauthorized("User not authenticated"))),
        }
    }
}
