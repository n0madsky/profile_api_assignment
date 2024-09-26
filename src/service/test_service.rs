use std::sync::{Once, OnceLock};

use crate::repository::inram::InMemoryProfileRepository;

use super::{model::*, ProfileService, ProfileServiceConfig};

fn registration1() -> &'static ProductRegistrationRecord {
    static REG1: OnceLock<ProductRegistrationRecord> = OnceLock::new();
    REG1.get_or_init(|| ProductRegistrationRecord {
        registration: ProductRegistration {
            id: 1,
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
        children: Vec::new(),
    })
}

static INIT: Once = Once::new();

fn setup() -> ProfileService<InMemoryProfileRepository> {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_line_number(true)
            .init()
    });
    ProfileService::new(
        InMemoryProfileRepository::with_example_data(String::new, || {
            chrono::DateTime::<chrono::Utc>::MIN_UTC
        }),
        ProfileServiceConfig::default(),
    )
}

#[test]
fn test_product_insert_empty_product_name() {
    let service = setup();

    let res = service.create_product("", None, &["foo".into(), "bar".into()]);

    assert!(res.is_err());
}

#[test]
fn test_get_product_registration_for_profile() {
    let service = setup();

    let res = service.get_product_registrations_for_profile(1, 0);

    assert!(res.is_some());
    let registrations = res.unwrap();
    assert_eq!(
        Vec::from([
            registration1().to_owned(),
            ProductRegistrationRecord {
                registration: ProductRegistration {
                    id: 2,
                    purchase_date: chrono::DateTime::parse_from_rfc3339("2023-03-10T12:00:00Z")
                        .unwrap()
                        .into(),
                    expiry_at: None,
                    product: "ARCC4".into(),
                    serial_code: "L3M4N5O6".into(),
                },
                children: Vec::new()
            }
        ]),
        registrations
    );
}

#[test]
fn test_get_product_registration_for_profile_nonexistent_profile() {
    let service = setup();

    let res = service.get_product_registrations_for_profile(1337, 0);

    assert!(res.is_none());
}

#[test]
fn get_product_registration_success() {
    let service = setup();

    let registration_id = 1;
    let res = service.get_product_registration(registration_id);
    assert_eq!(Some(registration1().clone()), res);
}

#[test]
fn get_product_registration_notfound() {
    let service = setup();

    let registration_id = 1337;
    let res = service.get_product_registration(registration_id);
    assert_eq!(None, res);
}

/*
Commented out as due to hashing function randomness, we so far cannot guarantee this always passes
#[test]
fn create_product_registration_success() {
    let service = setup();

    let res = service.create_product_registration(1, "AKB48");
    assert!(res.is_ok());
    let record = res.unwrap();
    assert_eq!(
        ProductRegistrationRecord {
            registration: ProductRegistration {
                id: 4,
                purchase_date: chrono::DateTime::<chrono::Utc>::MIN_UTC,
                expiry_at: None,
                product: "AKB48".into(),
                serial_code: String::new()
            },
            children: Vec::from([
                ProductRegistration {
                    id: 5,
                    purchase_date: chrono::DateTime::<chrono::Utc>::MIN_UTC,
                    expiry_at: None,
                    product: "NMB48".into(),
                    serial_code: String::new()
                },
                ProductRegistration {
                    id: 6,
                    purchase_date: chrono::DateTime::<chrono::Utc>::MIN_UTC,
                    expiry_at: None,
                    product: "SKE48".into(),
                    serial_code: String::new()
                },
            ])
        },
        record
    );
}
*/
