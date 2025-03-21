use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web, App, HttpServer,
};
use dotenv::dotenv;
use std::env;

mod database;
mod extractor;
mod handlers;
mod middleware;
mod models;
mod modules;
mod routes;
mod state;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let db_conn = database::conn::DatabaseConn::create_pool().await;
    let pool = db_conn.pool().clone();

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app_state = state::AppState { pool, jwt_secret };

    let server_ip = env::var("SERVER_IP").expect("SERVER_IP must be set");
    let server_port = env::var("SERVER_PORT")
        .expect("SERVER_PORT must be set")
        .parse::<u16>()
        .expect("Invalid port number");

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(100)
        .burst_size(1000)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("https://innova-puce.vercel.app/")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Governor::new(&governor_conf))
            .wrap(middleware::auth_header_middleware::AuthHeader)
            .configure(routes::routes::healthcheck)
            .configure(routes::routes::offers)
            .configure(routes::routes::insert)
            .configure(routes::routes::user)
            .configure(routes::routes::auth)
    })
    .bind((server_ip, server_port))?
    .run()
    .await?;

    Ok(())
}
