use std::{collections::HashSet, sync::OnceLock};

use super::{
    model::{ProductRegistrationRecord, Profile},
    ProfileServiceConfig,
};
use crate::repository::ProfileRepository;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum ProfileServiceError {
    BadRequest(String),
    InternalServiceError(String),
}

pub struct ProfileService<Repo: ProfileRepository> {
    repo: Repo,
    config: ProfileServiceConfig,
}

fn product_verification_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new("^[A-Z0-9]+$").unwrap())
}

fn is_product_sku_valid(product: &str) -> Result<(), &'static str> {
    if product.is_empty() {
        return Err("Empty strings are not allowed");
    }

    let regex = product_verification_regex();
    if !regex.is_match(product) {
        return Err("Product SKU can only contain alphanumeric characters");
    }

    Ok(())
}

impl<Repo: ProfileRepository> ProfileService<Repo> {
    pub fn new(repo: Repo, config: ProfileServiceConfig) -> Self {
        Self { repo, config }
    }

    pub fn get_profiles(&self, page: u32) -> Vec<Profile> {
        let start = page * self.config.profile_per_page as u32;

        let profiles = self
            .repo
            .get_profiles(start.into(), self.config.profile_per_page);

        profiles.into_iter().map(|p| p.into()).collect()
    }

    pub fn get_product_registrations_for_profile(
        &self,
        profile_id: u64,
        page: u32,
    ) -> Option<Vec<ProductRegistrationRecord>> {
        let _ = self.repo.get_profile(profile_id)?;
        let start = page * self.config.product_registrations_per_page as u32;

        let profile_registrations = self.repo.get_product_registrations_for_profile(
            profile_id,
            start.into(),
            self.config.product_registrations_per_page,
        );

        Some(
            profile_registrations
                .into_iter()
                .map(|product_registration| product_registration.into())
                .collect(),
        )
    }

    pub fn get_product_registration(
        &self,
        product_registration_id: u64,
    ) -> Option<ProductRegistrationRecord> {
        self.repo
            .get_product_registration(product_registration_id)
            .map(|registration| registration.into())
    }

    pub fn create_product(
        &self,
        product: &str,
        active_for: Option<u64>,
        subproducts: &[String],
    ) -> Result<HashSet<String>, ProfileServiceError> {
        if let Err(msg) = is_product_sku_valid(product) {
            tracing::warn!(
                "Unable to insert product: {}, subproducts: {:?}, product name is invalid",
                product,
                subproducts
            );

            return Err(ProfileServiceError::BadRequest(String::from(msg)));
        }

        for p in subproducts.iter() {
            if let Err(msg) = is_product_sku_valid(p) {
                tracing::warn!("Unable to insert product: {}, subproducts: {:?}, subproduct name {} is invalid", product, subproducts, p);

                return Err(ProfileServiceError::BadRequest(String::from(msg)));
            }
        }

        let mut missing_products = Vec::new();
        for p in subproducts.iter() {
            if !self.repo.product_exists(product) {
                missing_products.push(p);
            }
        }

        if !missing_products.is_empty() {
            tracing::warn!(
                "Unable to create product {}, as products {:?} does not exist in the db",
                product,
                missing_products
            );

            return Err(ProfileServiceError::BadRequest(format!(
                "Products {:?} does not exist",
                missing_products
            )));
        }

        if self.repo.product_exists(product) {
            tracing::warn!("Unable to create product {}, as product exists", product);

            return Err(ProfileServiceError::BadRequest(format!(
                "Product {} exists",
                product
            )));
        }

        let products = self.repo.insert_product(product, subproducts, active_for);

        Ok(products)
    }

    pub fn create_product_registration(
        &self,
        profile_id: u64,
        product_sku: &str,
    ) -> Result<ProductRegistrationRecord, ProfileServiceError> {
        let Some(_) = self.repo.get_profile(profile_id) else {
            return Err(ProfileServiceError::BadRequest(format!(
                "profile_id:{} does not exist",
                profile_id
            )));
        };

        if !self.repo.product_exists(product_sku) {
            return Err(ProfileServiceError::BadRequest(format!(
                "product:{} does not exist",
                product_sku
            )));
        }

        let res = self
            .repo
            .insert_product_registration(profile_id, product_sku);
        match res {
            Ok(reg) => Ok(reg.into()),
            Err(err) => Err(ProfileServiceError::InternalServiceError(format!(
                "Unable to create registration as this will create a duplicate registration:{:?}",
                err
            ))),
        }
    }
}
