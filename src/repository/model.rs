#[derive(Clone)]
pub struct Profile {
    pub id: u64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Clone)]
pub struct ProductRegistration {
    pub id: u64,
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    pub expiry_at: chrono::DateTime<chrono::Utc>,
    pub product: Product,
    pub serial_code: String,
}

#[derive(Clone)]
pub struct Product {
    pub sku: String,
}
