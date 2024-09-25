use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, Query, State},
    Json,
};

use crate::{repository::InMemoryProfileRepository, service::ProfileService};

use super::{
    error::ProfileApiError,
    model::{ProductRegistration, Profile},
};

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
) -> Result<Json<PagedResult<ProductRegistration>>, ProfileApiError> {
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
) -> Result<Json<ProductRegistration>, ProfileApiError> {
    todo!()
}
