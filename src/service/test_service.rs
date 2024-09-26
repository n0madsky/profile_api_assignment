use crate::repository::inram::InMemoryProfileRepository;

use super::{ProfileService, ProfileServiceConfig};

fn setup() -> ProfileService<InMemoryProfileRepository> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_line_number(true)
        .init();
    ProfileService::new(InMemoryProfileRepository::with_example_data(), ProfileServiceConfig::default())
}

#[test]
fn test_product_insert_empty_product_name() {
    let service = setup();

    let res = service.create_product("", None, &["foo".into(), "bar".into()]);

    assert!(res.is_err());
}
