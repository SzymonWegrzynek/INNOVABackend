use actix_web::{http::header::AUTHORIZATION, web, HttpRequest, HttpResponse};

use crate::{
    models::auth::{AuthenticationToken, ExtractToken},
    modules::jwt_token::JwtToken,
    state::AppState,
};

pub struct TokenHandler;

impl TokenHandler {
    pub async fn extract_token(token: AuthenticationToken) -> HttpResponse {
        HttpResponse::Ok().json(ExtractToken { role: token.role })
    }

    pub async fn verify_token(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
        let token = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .map(|auth_str| auth_str.strip_prefix("Bearer ").unwrap_or("").to_string());

        match token {
            Some(token) => {
                let decode = JwtToken::decode_token(&token, &app_state);

                match decode {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(_) => HttpResponse::Unauthorized().finish(),
                }
            }
            None => HttpResponse::Unauthorized().finish(),
        }
    }
}
