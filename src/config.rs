use envconfig::Envconfig;

#[derive(Debug, envconfig::Envconfig)]
pub(crate) struct Config {
    #[envconfig(from = "APP_HOST", default = "0.0.0.0")]
    pub host: String,
    #[envconfig(from = "APP_HOST", default = "3000")]
    pub port: u16,
    #[envconfig(from = "APP_PROFILES_PER_PAGE", default = "30")]
    pub profiles_per_page: u32,
    #[envconfig(from = "APP_USE_SAMPLE_DATA", default = "true")]
    pub use_sample_data: bool,
}
