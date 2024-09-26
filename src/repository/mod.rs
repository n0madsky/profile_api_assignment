pub mod model;

use std::{cmp::min, collections::HashMap};

use model::{ProductRegistration, Profile};

pub trait ProfileRepository {
    fn get_profiles(&self, start: u64, count: u64) -> Vec<Profile>;
    fn get_profile(&self, id: u64) -> Option<Profile>;
    fn get_product_registrations_for_profile(
        &self,
        profile_id: u64,
        start: u64,
        count: u64,
    ) -> Vec<ProductRegistration>;
    fn get_product_registration(&self, id: u64) -> Option<ProductRegistration>;
}

pub struct InMemoryProfileRepository {
    profiles: Vec<Profile>,
    // profile id -> [product registration ids]
    profile_to_product_registrations: HashMap<u64, Vec<u64>>,
    product_registrations: Vec<ProductRegistration>,
    // product registration id
    product_registration_children: HashMap<u64, Vec<u64>>,
}

impl InMemoryProfileRepository {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            profile_to_product_registrations: HashMap::new(),
            product_registrations: Vec::new(),
            product_registration_children: HashMap::new(),
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

        let mut profile_to_product_registrations: HashMap<u64, Vec<u64>> = HashMap::new();
        for registration in product_registrations.iter() {
            profile_to_product_registrations
                .entry(registration.profile_id)
                .or_default()
                .push(registration.id);
        }

        Self {
            profiles,
            profile_to_product_registrations,
            product_registrations,
            product_registration_children: HashMap::new(),
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
    ) -> Vec<ProductRegistration> {
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
    fn get_product_registration(&self, id: u64) -> Option<ProductRegistration> {
        self.product_registrations.get((id - 1) as usize).cloned()
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
