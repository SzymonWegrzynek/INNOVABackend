use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    http::header::{HeaderValue, AUTHORIZATION},
    web, Error as ActixWebError, FromRequest, HttpRequest,
};
use std::future::{ready, Ready};

use crate::{models::auth::AuthenticationToken, modules::jwt_token::JwtToken, state::AppState};

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        let auth_header: Option<&HeaderValue> = req.headers().get(AUTHORIZATION);

        if auth_header.is_none() {
            return ready(Err(ErrorUnauthorized("No authentication token sent")));
        }

        let auth_token: String = auth_header.unwrap().to_str().unwrap_or("").to_string();

        let auth_token = auth_token.strip_prefix("Bearer ").unwrap_or("").to_string();

        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized(
                "Authentication token has foreign chars",
            )));
        }

        let app_state = match req.app_data::<web::Data<AppState>>() {
            Some(state) => state,
            None => return ready(Err(ErrorUnauthorized("Missing app state"))),
        };

        let decode = JwtToken::decode_token(&auth_token, &app_state);

        match decode {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.id,
                role: token.claims.role,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Invalid authentication token sent"))),
        }
    }
}
