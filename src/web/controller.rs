use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, Query, State},
    Json,
};

use crate::{
    repository::InMemoryProfileRepository, service::ProfileService,
    web::model::ProductRegistrationRecord,
};

use super::{error::ProfileApiError, model::Profile};

#[derive(serde::Deserialize)]
pub(crate) struct Pagination {
    pub page: Option<u32>,
}

#[derive(serde::Serialize)]
pub(crate) struct PagedResult<T> {
    pub page: u32,
    pub items: Vec<T>,
}

#[debug_handler]
pub(crate) async fn profiles_get(
    State(service): State<Arc<ProfileService<InMemoryProfileRepository>>>,
    Query(query): Query<Pagination>,
) -> Result<Json<PagedResult<Profile>>, ProfileApiError> {
    let page = query.page.unwrap_or(0);

    let res = service.get_profiles(page);

    Ok(Json(PagedResult {
        page,
        items: res.into_iter().map(|profile| profile.into()).collect(),
    }))
}

#[debug_handler]
pub(crate) async fn profile_product_registrations_get(
    State(service): State<Arc<ProfileService<InMemoryProfileRepository>>>,
    Path(profile_id): Path<u64>,
    Query(query): Query<Pagination>,
) -> Result<Json<PagedResult<ProductRegistrationRecord>>, ProfileApiError> {
    let page = query.page.unwrap_or(0);

    let res = service.get_product_registrations_for_profile(profile_id, page);

    match res {
        None => Err(ProfileApiError::NotFound),
        Some(registrations) => Ok(Json(PagedResult {
            page,
            items: registrations
                .into_iter()
                .map(|registration| registration.into())
                .collect(),
        })),
    }
}

#[debug_handler]
pub(crate) async fn product_registrations_get(
    State(service): State<Arc<ProfileService<InMemoryProfileRepository>>>,
    Path(product_registration_id): Path<u64>,
) -> Result<Json<ProductRegistrationRecord>, ProfileApiError> {
    let product_registration = service.get_product_registration(product_registration_id);
    match product_registration {
        Some(registration) => Ok(Json(registration.into())),
        None => Err(ProfileApiError::NotFound),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct ProductPostRequest {
    pub sku: String,
    pub bundled_products: Vec<String>,
}

#[derive(serde::Serialize)]
pub(crate) struct ProductPostResponse {
    pub sku_added: String,
    pub bundled_products: Vec<String>,
}

#[debug_handler]
pub(crate) async fn product_post(
    State(service): State<Arc<ProfileService<InMemoryProfileRepository>>>,
    Json(req): Json<ProductPostRequest>,
) -> Result<Json<ProductPostResponse>, ProfileApiError> {
    let res = service.create_product(&req.sku, &req.bundled_products);
    match res {
        Ok(products) => Ok(Json(ProductPostResponse {
            sku_added: req.sku,
            bundled_products: products.into_iter().collect(),
        })),
        Err(err) => match err {
            crate::service::ProfileServiceError::BadRequest(r) => {
                Err(ProfileApiError::BadRequest(r))
            }
        },
    }
}
