mod repository;
mod service;
mod web;

use std::sync::Arc;

use axum::Router;
use repository::InMemoryProfileRepository;
use service::{ProfileService, ProfileServiceConfig};
use web::controller::{product_registrations_get, profiles_get};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // TODO - Add code to initialize config from config file/environment variables
    let config = ProfileServiceConfig::default();

    let service = Arc::new(ProfileService::new(
        InMemoryProfileRepository::with_example_data(),
        config,
    ));

    // build our application with a route
    let profile_router = Router::new()
        .route("/profiles", axum::routing::get(profiles_get))
        .route(
            "/product_registration/:id",
            axum::routing::get(product_registrations_get),
        )
        .with_state(service);

    let app = Router::new().nest("/api/v1", profile_router);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
