mod config;
mod repository;
mod service;
mod web;

use std::sync::Arc;

use axum::Router;
use envconfig::Envconfig;
use repository::inram::InMemoryProfileRepository;
use service::{ProfileService, ProfileServiceConfig};
use web::controller::{
    product_post, product_registrations_get, product_registrations_post, profile_product_registrations_get, profiles_get
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_line_number(true).init();

    let config = config::Config::init_from_env().unwrap();
    tracing::info!("Starting with the following configs: {:#?}", config);

    let service_config = ProfileServiceConfig {
        profile_per_page: config.profiles_per_page,
        product_registrations_per_page: config.product_registrations_per_page,
    };

    let db = if config.use_sample_data {
        InMemoryProfileRepository::with_example_data()
    } else {
        InMemoryProfileRepository::new()
    };

    let service = Arc::new(ProfileService::new(db, service_config));

    // build our application with a route
    let profile_router = Router::new()
        .route("/profiles", axum::routing::get(profiles_get))
        .route(
            "/profiles/:profile/product_registrations",
            axum::routing::get(profile_product_registrations_get),
        )
        .route(
            "/product_registration/:id",
            axum::routing::get(product_registrations_get),
        )
        .route("/profiles/:profile/product_registrations", axum::routing::post(product_registrations_post))
        .route("/product", axum::routing::post(product_post))
        .with_state(service);

    let app = Router::new().nest("/api/v1", profile_router);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
