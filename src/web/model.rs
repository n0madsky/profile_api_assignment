#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct Profile {
    // Note, this is a slight divergence from the spec,
    // spec specifies id should be int
    // but since all the other ids are in uint64, I decided to unify everything as uint64
    pub id: u64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

impl From<crate::service::model::Profile> for Profile {
    fn from(value: crate::service::model::Profile) -> Self {
        Profile {
            id: value.id,
            email: value.email,
            firstname: value.firstname,
            lastname: value.lastname,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct ProductRegistration {
    pub id: u64,
    // set as Unix Epoch, at milliseconds precision
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub expiry_at: Option<chrono::DateTime<chrono::Utc>>,
    pub product: Product,
    pub serial_code: String,
}

impl From<crate::service::model::ProductRegistration> for ProductRegistration {
    fn from(value: crate::service::model::ProductRegistration) -> Self {
        ProductRegistration {
            id: value.id,
            purchase_date: value.purchase_date,
            expiry_at: value.expiry_at,
            product: Product { sku: value.product },
            serial_code: value.serial_code,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct ProductRegistrationRecord {
    #[serde(flatten)]
    pub registration: ProductRegistration,
    pub additional_product_registrations: Vec<ProductRegistration>,
}

impl From<crate::service::model::ProductRegistrationRecord> for ProductRegistrationRecord {
    fn from(value: crate::service::model::ProductRegistrationRecord) -> Self {
        ProductRegistrationRecord {
            registration: value.registration.into(),
            additional_product_registrations: value
                .children
                .into_iter()
                .map(|a| a.into())
                .collect(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct Product {
    pub sku: String,
}
