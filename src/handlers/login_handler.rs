use actix_web::{
    cookie::{time::OffsetDateTime, Cookie, SameSite},
    web, HttpResponse,
};
use chrono::{Duration, Utc};

use crate::{
    models::login::{CreateUserData, CreateUserResponse, SignInResponse, SignInUserData, User},
    modules::{hash_password::HashPassword, jwt_token::JwtToken},
    state::AppState,
};

pub struct LoginHandler;

impl LoginHandler {
    pub async fn create_user(
        app_state: web::Data<AppState>,
        payload: web::Json<CreateUserData>,
    ) -> HttpResponse {
        let hash_password = match HashPassword::hash_password(&payload.password) {
            Ok(hash) => hash,
            Err(_) => {
                return HttpResponse::InternalServerError().json(CreateUserResponse {
                    message: "Hash password error".to_string(),
                })
            }
        };

        let result = sqlx::query_file!(
            "src/queries/insert_user.sql",
            &payload.username,
            &payload.email,
            hash_password,
        )
        .execute(&app_state.pool)
        .await;

        match result {
            Ok(_) => HttpResponse::Created().json(CreateUserResponse {
                message: "User created successfully".to_string(),
            }),
            Err(_) => HttpResponse::BadRequest().json(CreateUserResponse {
                message: "Failed to create user".to_string(),
            }),
        }
    }

    pub async fn signin_user(
        app_state: web::Data<AppState>,
        payload: web::Json<SignInUserData>,
    ) -> HttpResponse {
        let user = sqlx::query_file_as!(User, "src/queries/get_user.sql", &payload.email)
            .fetch_one(&app_state.pool)
            .await
            .ok();

        let user = match user {
            Some(user) => user,
            None => {
                return HttpResponse::Unauthorized().json(SignInResponse {
                    message: "Incorrect email or password".to_string(),
                });
            }
        };

        if !HashPassword::verify_password(&payload.password, &user.password).unwrap_or(false) {
            return HttpResponse::Unauthorized().json(SignInResponse {
                message: "Incorrect email or password".to_string(),
            });
        }

        let token = match JwtToken::encode_token(
            user.user_id.try_into().unwrap(),
            user.role.to_string(),
            &app_state,
        ) {
            Ok(token) => token,
            Err(_) => {
                return HttpResponse::InternalServerError().json(SignInResponse {
                    message: "Error generating token".to_string(),
                });
            }
        };

        let expire_time = Utc::now() + Duration::hours(24);
        let expire_time = OffsetDateTime::from_unix_timestamp(expire_time.timestamp()).unwrap();

        HttpResponse::Ok()
            .cookie(
                Cookie::build("UserToken", token)
                    .http_only(true)
                    .secure(false)
                    .same_site(SameSite::Lax)
                    .expires(expire_time)
                    .finish(),
            )
            .json(SignInResponse {
                message: "Successfully logged in".to_string(),
            })
    }
}
