use actix_web::web;

use crate::handlers::{
    healthcheck_handler::HealthCheck, login_handler::LoginHandler, offers_handler::OffersHandler,
    token_handler::TokenHandler,
};

pub fn healthcheck(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthcheck", web::get().to(HealthCheck::healthcheck));
}

pub fn offers(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/offers", web::get().to(OffersHandler::get_offers));
}

pub fn insert(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/offer", web::post().to(OffersHandler::insert_offer));
}

pub fn user(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/users", web::post().to(LoginHandler::create_user));
}

pub fn auth(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/auth/signin", web::post().to(LoginHandler::signin_user));

    cfg.route(
        "/v1/auth/extract",
        web::get().to(TokenHandler::extract_token),
    );

    cfg.route("/v1/auth/verify", web::get().to(TokenHandler::verify_token));
}
