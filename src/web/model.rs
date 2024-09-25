#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct Profile {
    // Note, this is a slight divergence from the spec,
    // spec specifies id should be int
    // but since all the other ids are in uint64, I decided to unify everything as uint64
    pub id: u64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub product_registrations: Vec<ProductRegistration>,
}

impl From<crate::service::model::Profile> for Profile {
    fn from(value: crate::service::model::Profile) -> Self {
        Profile {
            id: value.id,
            email: value.email,
            firstname: value.firstname,
            lastname: value.lastname,
            product_registrations: Vec::new(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct ProductRegistration {
    pub id: u64,
    // set as Unix Epoch, at milliseconds precision
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub expiry_at: chrono::DateTime<chrono::Utc>,
    pub product: Product,
    pub additional_product_registrations: Vec<ProductRegistrationsChild>,
}

impl From<crate::service::model::ProductRegistration> for ProductRegistration {
    fn from(value: crate::service::model::ProductRegistration) -> Self {
        ProductRegistration {
            id: value.id,
            purchase_date: value.purchase_date,
            expiry_at: value.expiry_at,
            product: Product { sku: value.product },
            additional_product_registrations: Vec::new(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct Product {
    pub sku: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct ProductRegistrationsChild {
    pub id: u64,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub expiry_at: chrono::DateTime<chrono::Utc>,
    pub product: Product,
    pub serial_code: String,
}
