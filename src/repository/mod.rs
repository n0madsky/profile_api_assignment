use std::collections::HashSet;

use model::{ProductRegistrationRecord, Profile};

pub mod inram;
pub mod model;

///
/// Interface for accessing data
/// An in-memory implementation is provided
/// This can be replaced with a database model too, but the interface will require some slight
/// modification to account for the fact that a db is a remote connection, and can fail
///
pub trait ProfileRepository {
    fn get_profiles(&self, start: u64, count: usize) -> Vec<Profile>;
    fn get_profile(&self, id: u64) -> Option<Profile>;
    fn get_product_registrations_for_profile(
        &self,
        profile_id: u64,
        start: u64,
        count: usize,
    ) -> Vec<ProductRegistrationRecord>;
    fn get_product_registration(&self, id: u64) -> Option<ProductRegistrationRecord>;
    fn insert_product_registration(&self, profile_id: u64, product_sku: &str) -> Result<ProductRegistrationRecord, HashSet<String>>;
    fn product_exists(&self, product: &str) -> bool;
    fn insert_product(&self, product: &str, subproducts: &[String], active_for: Option<u64>) -> HashSet<String>;
}
