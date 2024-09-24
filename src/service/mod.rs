pub mod model;

use model::Profile;

use crate::repository::ProfileRepository;

pub(crate) struct ProfileServiceConfig {
    pub profile_per_page: u32,
}

impl Default for ProfileServiceConfig {
    fn default() -> Self {
        Self {
            profile_per_page: 30,
        }
    }
}

pub struct ProfileService<Repo: ProfileRepository> {
    repo: Repo,
    config: ProfileServiceConfig,
}

impl<Repo: ProfileRepository> ProfileService<Repo> {
    pub fn new(repo: Repo, config: ProfileServiceConfig) -> Self {
        Self { repo, config }
    }

    pub fn get_profiles(&self, page: u32) -> Vec<Profile> {
        let start = page * self.config.profile_per_page;

        let profiles = self
            .repo
            .get_profiles(start.into(), self.config.profile_per_page.into());

        profiles.into_iter().map(|p| p.into()).collect()
    }

    pub fn get_product_registration(&self, product_registration_id: u64) {}
}
