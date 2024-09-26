#[derive(Clone)]
pub struct Profile {
    pub id: u64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Clone)]
pub struct ProductRegistrationRecord {
    pub registration: ProductRegistration,
    pub children: Vec<ProductRegistration>,
}

#[derive(Clone)]
pub struct ProductRegistration {
    pub id: u64,
    // Foreign Key
    pub profile_id: u64,
    // parent key
    pub parent_id: Option<u64>,
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    pub expiry_at: Option<chrono::DateTime<chrono::Utc>>,
    pub product: String,
    pub serial_code: String,
}
