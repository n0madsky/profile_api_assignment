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
pub(crate) struct ProfileGetResult {
    pub page: u32,
    pub profiles: Vec<Profile>,
}

#[debug_handler]
pub(crate) async fn profiles_get(
    State(service): State<Arc<ProfileService<InMemoryProfileRepository>>>,
    Query(query): Query<Pagination>,
) -> Result<Json<ProfileGetResult>, ProfileApiError> {
    let page = query.page.unwrap_or(0);

    let res = service.get_profiles(page);

    Ok(Json(ProfileGetResult {
        page,
        profiles: res.into_iter().map(|profile| profile.into()).collect(),
    }))
}

#[debug_handler]
pub(crate) async fn product_registrations_get(
    State(service): State<Arc<ProfileService<InMemoryProfileRepository>>>,
    Path(product_registration_id): Path<u64>,
) -> Result<Json<ProductRegistration>, ProfileApiError> {
    todo!()
}
