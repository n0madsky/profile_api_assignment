
pub struct ProfileServiceConfig {
    pub profile_per_page: u32,
    pub product_registrations_per_page: u32,
}

impl Default for ProfileServiceConfig {
    fn default() -> Self {
        Self {
            profile_per_page: 30,
            product_registrations_per_page: 30,
        }
    }
}