pub mod model;

use std::{cmp::min, collections::HashMap};

use model::{ProductRegistration, Profile};

pub trait ProfileRepository {
    fn get_profiles(&self, start: u64, count: u64) -> Vec<Profile>;
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

    // TODO - fill out children logic
    fn get_product_registration(&self, id: u64) -> Option<ProductRegistration> {
        self.product_registrations.get(id as usize).cloned()
    }
}
