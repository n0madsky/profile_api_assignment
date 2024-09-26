use std::{cmp::min, collections::HashSet, sync::Mutex};

use super::{
    model::{ProductRegistration, ProductRegistrationRecord, Profile},
    ProfileRepository,
};
use dashmap::DashMap;
use rand::Rng;

pub struct InMemoryProfileRepository {
    profiles: Vec<Profile>,
    // profile id -> [product registration ids]
    profile_to_product_registrations: DashMap<u64, Vec<u64>>,
    product_registrations: Mutex<Vec<ProductRegistration>>,
    // product registration id
    product_registrations_children: DashMap<u64, Vec<u64>>,
    // product SKU -> set(sub product SKUs)
    products: DashMap<String, HashSet<String>>,
    // Product SKU -> expiry time, if it is not in the map, the product does not expire
    product_active_for: DashMap<String, u64>,
}

fn registration_is_active(
    registration: &ProductRegistration,
    timestamp: chrono::DateTime<chrono::Utc>,
) -> bool {
    match registration.expiry_at {
        None => true,
        Some(expires_at) => timestamp < expires_at,
    }
}

impl InMemoryProfileRepository {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            profile_to_product_registrations: DashMap::new(),
            product_registrations: Mutex::new(Vec::new()),
            product_registrations_children: DashMap::new(),
            products: DashMap::new(),
            product_active_for: DashMap::new(),
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
            (
                "AKB48".into(),
                HashSet::from(["SKE48".into(), "NMB48".into()]),
            ),
            ("AKBL1".into(), HashSet::new()),
            ("AKDS5".into(), HashSet::new()),
            ("ARAS1".into(), HashSet::new()),
            ("ARCS1".into(), HashSet::new()),
            ("ARCH1".into(), HashSet::new()),
            ("ARCM1".into(), HashSet::new()),
            ("SKE48".into(), HashSet::new()),
            ("NMB48".into(), HashSet::new()),
        ]);

        let profile_to_product_registrations: DashMap<u64, Vec<u64>> = DashMap::new();
        let product_registrations_children: DashMap<u64, Vec<u64>> = DashMap::new();

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
            product_registrations: Mutex::new(product_registrations),
            product_registrations_children,
            products,
            product_active_for: DashMap::new(),
        }
    }

    fn get_active_registered_products(&self, profile_id: u64) -> HashSet<String> {
        let Some(product_registration_ids) = self.profile_to_product_registrations.get(&profile_id)
        else {
            tracing::error!("Did not find profile_id:{}", profile_id);
            return HashSet::new();
        };

        let mut existing_products = HashSet::new();

        let now = chrono::Utc::now();
        for id in product_registration_ids.value() {
            let Some(registration_record) = self.get_product_registration(*id) else {
                continue;
            };

            if registration_is_active(&registration_record.registration, now) {
                existing_products.insert(registration_record.registration.product);
            }

            for child_record in registration_record.children.iter() {
                if registration_is_active(child_record, now) {
                    existing_products.insert(child_record.product.clone());
                }
            }
        }

        existing_products
    }

    fn append_product_registration(
        &self,
        registrations: &mut Vec<ProductRegistration>,
        profile_id: u64,
        parent_id: Option<u64>,
        purchase_date: chrono::DateTime<chrono::Utc>,
        product_sku: &str,
    ) -> ProductRegistration {
        let new_registration_id = (registrations.len() + 1) as u64;
        let product_expiration = self.product_active_for.get(product_sku);
        let registration = ProductRegistration {
            id: new_registration_id,
            profile_id,
            parent_id,
            purchase_date,
            expiry_at: product_expiration.map(|expires_in| {
                purchase_date + chrono::Duration::seconds(*expires_in.value() as i64)
            }),
            product: product_sku.into(),
            serial_code:
                crate::repository::inram::InMemoryProfileRepository::generate_random_serial(),
        };
        registrations.push(registration.clone());

        registration
    }

    fn generate_random_serial() -> String {
        let mut rng = rand::thread_rng();
        (0..15)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect()
    }
}

impl ProfileRepository for InMemoryProfileRepository {
    fn get_profiles(&self, start: u64, count: usize) -> Vec<Profile> {
        let start = start as usize;

        if start >= self.profiles.len() {
            return Vec::new();
        }

        let end = min(start + count, self.profiles.len());

        self.profiles.get(start..end).unwrap_or_default().to_vec()
    }

    fn get_profile(&self, id: u64) -> Option<Profile> {
        self.profiles.get((id - 1) as usize).cloned()
    }

    fn get_product_registrations_for_profile(
        &self,
        profile_id: u64,
        start: u64,
        count: usize,
    ) -> Vec<ProductRegistrationRecord> {
        let Some(product_registration_ids) = self.profile_to_product_registrations.get(&profile_id)
        else {
            return Vec::new();
        };

        let start = start as usize;
        let end = min(start + count, product_registration_ids.len());
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

    fn get_product_registration(&self, id: u64) -> Option<ProductRegistrationRecord> {
        let guard = self.product_registrations.lock().unwrap();

        let registration = guard.get((id - 1) as usize)?.to_owned();
        let product_registration_children: Vec<ProductRegistration> = self
            .product_registrations_children
            .get(&registration.id)
            .map(|subregistrations| {
                subregistrations
                    .iter()
                    .filter_map(|child_id| guard.get((child_id - 1) as usize))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        Some(ProductRegistrationRecord {
            registration,
            children: product_registration_children,
        })
    }

    fn product_exists(&self, product: &str) -> bool {
        self.products.contains_key(product)
    }

    fn insert_product(
        &self,
        product: &str,
        subproducts: &[String],
        active_for: Option<u64>,
    ) -> HashSet<String> {
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
        if let Some(active_seconds) = active_for {
            self.product_active_for
                .insert(product.to_owned(), active_seconds);
        }

        products_to_add
    }

    fn insert_product_registration(
        &self,
        profile_id: u64,
        product_sku: &str,
    ) -> Result<ProductRegistrationRecord, HashSet<String>> {
        let registered_products = self.get_active_registered_products(profile_id);

        let mut visited_products = HashSet::new();
        let mut products_to_add = HashSet::new();

        find_subproduct_dfs(
            product_sku,
            &self.products,
            &mut visited_products,
            &mut products_to_add,
        );

        let intersection: HashSet<String> = products_to_add
            .intersection(&registered_products)
            .cloned()
            .collect();
        if !intersection.is_empty() {
            return Err(intersection);
        }

        let now = chrono::Utc::now();
        let mut registrations = self.product_registrations.lock().unwrap();
        let parent_registration = self.append_product_registration(
            &mut registrations,
            profile_id,
            None,
            now,
            product_sku,
        );
        self.profile_to_product_registrations
            .entry(profile_id)
            .or_default()
            .push(parent_registration.id);

        let mut child_registrations = Vec::new();
        for child in products_to_add {
            let child_registration = self.append_product_registration(
                &mut registrations,
                profile_id,
                Some(parent_registration.id),
                now,
                &child,
            );
            self.product_registrations_children
                .entry(parent_registration.id)
                .or_default()
                .push(child_registration.id);
            child_registrations.push(child_registration);
        }
        Ok(ProductRegistrationRecord {
            registration: parent_registration,
            children: child_registrations,
        })
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

    #[test]
    fn insert_product_dfs_ok() {
        let expected = HashSet::from([
            "AKBL1".into(),
            "AKDS5".into(),
            "ARAS1".into(),
            "ARCS1".into(),
            "ARCH1".into(),
            "ARCM1".into(),
        ]);

        let repo = InMemoryProfileRepository::with_example_data();
        assert!(!repo.product_exists("foo"));
        let actual = repo.insert_product("foo", &["ARIE4".into()], None);

        assert_eq!(expected, actual);
        assert!(repo.product_exists("foo"));
    }

    #[test]
    fn insert_product_repeated_insert_ok() {
        let expected = HashSet::from([
            "AKBL1".into(),
            "AKDS5".into(),
            "ARAS1".into(),
            "ARCS1".into(),
            "ARCH1".into(),
            "ARCM1".into(),
            "SKE48".into(),
            "NMB48".into(),
        ]);

        let repo = InMemoryProfileRepository::with_example_data();
        let _ = repo.insert_product("foo", &["ARIE4".into()], None);
        let actual = repo.insert_product("bar", &["foo".into(), "AKB48".into()], None);

        assert_eq!(expected, actual);
    }
}
