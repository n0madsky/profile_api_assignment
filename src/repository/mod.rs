pub mod model;

use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use dashmap::DashMap;
use model::{ProductRegistration, ProductRegistrationRecord, Profile};

///
/// Interface for accessing data
/// An in-memory implementation is provided
/// This can be replaced with a database model too, but the interface will require some slight
/// modification to account for the fact that a db is a remote connection, and can fail
///
pub trait ProfileRepository {
    fn get_profiles(&self, start: u64, count: u64) -> Vec<Profile>;
    fn get_profile(&self, id: u64) -> Option<Profile>;
    fn get_product_registrations_for_profile(
        &self,
        profile_id: u64,
        start: u64,
        count: u64,
    ) -> Vec<ProductRegistrationRecord>;
    fn get_product_registration(&self, id: u64) -> Option<ProductRegistrationRecord>;
    fn product_exists(&self, product: &str) -> bool;
    fn find_missing_products(&self, products: &[String]) -> Vec<String>;
    fn insert_product(&self, product: &str, subproducts: &[String]) -> HashSet<String>;
}

pub struct InMemoryProfileRepository {
    profiles: Vec<Profile>,
    // profile id -> [product registration ids]
    profile_to_product_registrations: HashMap<u64, Vec<u64>>,
    product_registrations: Vec<ProductRegistration>,
    // product registration id
    product_registrations_children: HashMap<u64, Vec<u64>>,
    // product SKU -> set(sub product SKUs)
    products: DashMap<String, HashSet<String>>,
}

impl InMemoryProfileRepository {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            profile_to_product_registrations: HashMap::new(),
            product_registrations: Vec::new(),
            product_registrations_children: HashMap::new(),
            products: DashMap::new(),
        }
    }

    pub fn with_example_data() -> Self {
        let profiles = Vec::from([
            Profile {
                id: 1,
                email: "john.doe@example.com".into(),
                firstname: "John".into(),
                lastname: "Doe".into(),
            },
            Profile {
                id: 2,
                email: "jane.smith@example.com".into(),
                firstname: "Jane".into(),
                lastname: "Smith".into(),
            },
        ]);

        let product_registrations = Vec::from([
            ProductRegistration {
                id: 1,
                parent_id: None,
                profile_id: 1,
                purchase_date: chrono::DateTime::parse_from_rfc3339("2023-01-15T15:04:05Z")
                    .unwrap()
                    .into(),
                expiry_at: Some(
                    chrono::DateTime::parse_from_rfc3339("2024-01-15T15:04:05Z")
                        .unwrap()
                        .into(),
                ),
                product: "ARIE4".into(),
                serial_code: "A1B2C3D4".into(),
            },
            ProductRegistration {
                id: 2,
                parent_id: None,
                profile_id: 1,
                purchase_date: chrono::DateTime::parse_from_rfc3339("2023-03-10T12:00:00Z")
                    .unwrap()
                    .into(),
                expiry_at: None,
                product: "ARCC4".into(),
                serial_code: "L3M4N5O6".into(),
            },
            ProductRegistration {
                id: 3,
                profile_id: 2,
                parent_id: None,
                purchase_date: chrono::DateTime::parse_from_rfc3339("2022-12-25T08:30:00Z")
                    .unwrap()
                    .into(),
                expiry_at: Some(
                    chrono::DateTime::parse_from_rfc3339("2023-12-25T08:30:00Z")
                        .unwrap()
                        .into(),
                ),
                product: "ARCM1".into(),
                serial_code: "Z5X6C7V8".into(),
            },
        ]);

        let products: DashMap<String, HashSet<String>> = DashMap::from_iter([
            (
                "ARIE4".into(),
                HashSet::from(["ARCC4".into(), "AKBL1".into(), "AKDS5".into()]),
            ),
            (
                "ARCC4".into(),
                HashSet::from([
                    "ARAS1".into(),
                    "ARCS1".into(),
                    "ARCH1".into(),
                    "ARCM1".into(),
                ]),
            ),
            ("AKLB1".into(), HashSet::new()),
            ("AKDS5".into(), HashSet::new()),
            ("ARAS1".into(), HashSet::new()),
            ("ARCS1".into(), HashSet::new()),
            ("ARCH1".into(), HashSet::new()),
            ("ARCM1".into(), HashSet::new()),
        ]);

        let mut profile_to_product_registrations: HashMap<u64, Vec<u64>> = HashMap::new();
        let mut product_registrations_children: HashMap<u64, Vec<u64>> = HashMap::new();

        for registration in product_registrations.iter() {
            profile_to_product_registrations
                .entry(registration.profile_id)
                .or_default()
                .push(registration.id);

            if let Some(parent_id) = registration.parent_id {
                product_registrations_children
                    .entry(parent_id)
                    .or_default()
                    .push(registration.id);
            }
        }

        Self {
            profiles,
            profile_to_product_registrations,
            product_registrations,
            product_registrations_children,
            products,
        }
    }
}

impl ProfileRepository for InMemoryProfileRepository {
    fn get_profiles(&self, start: u64, count: u64) -> Vec<Profile> {
        let start = start as usize;

        if start >= self.profiles.len() {
            return Vec::new();
        }

        let end = min(start + count as usize, self.profiles.len());

        self.profiles.get(start..end).unwrap_or_default().to_vec()
    }

    fn get_profile(&self, id: u64) -> Option<Profile> {
        self.profiles.get((id - 1) as usize).cloned()
    }

    fn get_product_registrations_for_profile(
        &self,
        profile_id: u64,
        start: u64,
        count: u64,
    ) -> Vec<ProductRegistrationRecord> {
        let Some(product_registration_ids) = self.profile_to_product_registrations.get(&profile_id)
        else {
            return Vec::new();
        };

        let start = start as usize;
        let end = min(start + count as usize, product_registration_ids.len());
        if start >= product_registration_ids.len() {
            return Vec::new();
        }

        product_registration_ids
            .get(start..end)
            .unwrap_or_default()
            .iter()
            .filter_map(|id| self.get_product_registration(*id))
            .collect()
    }

    // TODO - fill out children logic
    fn get_product_registration(&self, id: u64) -> Option<ProductRegistrationRecord> {
        let registration = self
            .product_registrations
            .get((id - 1) as usize)?
            .to_owned();
        let product_registration_children: Vec<ProductRegistration> = self
            .product_registrations_children
            .get(&registration.id)
            .map(|subregistrations| {
                subregistrations
                    .iter()
                    .filter_map(|child_id| self.product_registrations.get((child_id - 1) as usize))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        Some(ProductRegistrationRecord {
            registration,
            children: product_registration_children,
        })
    }

    fn find_missing_products(&self, products: &[String]) -> Vec<String> {
        let mut p = Vec::new();
        for product in products {
            if !self.products.contains_key(product) {
                p.push(product.clone());
            }
        }
        p
    }

    fn product_exists(&self, product: &str) -> bool {
        self.products.contains_key(product)
    }

    fn insert_product(&self, product: &str, subproducts: &[String]) -> HashSet<String> {
        let mut products_to_add = HashSet::new();
        let mut visited_products = HashSet::new();

        for subproduct in subproducts {
            find_subproduct_dfs(
                subproduct,
                &self.products,
                &mut visited_products,
                &mut products_to_add,
            );
        }

        self.products
            .insert(product.to_owned(), products_to_add.clone());

        products_to_add
    }
}

fn find_subproduct_dfs(
    product: &str,
    existing_products: &DashMap<String, HashSet<String>>,
    visited_products: &mut HashSet<String>,
    products_to_add: &mut HashSet<String>,
) {
    if visited_products.contains(product) {
        return;
    }

    visited_products.insert(product.to_owned());

    let Some(subproducts) = existing_products.get(product) else {
        tracing::error!("Unable to find {} in existing products", product);
        return;
    };

    if subproducts.is_empty() {
        products_to_add.insert(product.to_owned());
    } else {
        for p in subproducts.value() {
            find_subproduct_dfs(p, existing_products, visited_products, products_to_add);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_is_valid() {
        let _ = InMemoryProfileRepository::with_example_data();
    }
}
