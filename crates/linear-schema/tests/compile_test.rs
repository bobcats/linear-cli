/// Verify the schema module and scalar types are correctly exported.
/// If this file compiles, the schema crate is working.

#[test]
fn test_schema_module_exists() {
    let _ = std::any::type_name::<linear_schema::DateTime>();
    let _ = std::any::type_name::<linear_schema::TimelessDate>();
}

#[test]
fn test_datetime_round_trips_through_serde() {
    let dt = linear_schema::DateTime("2025-01-01T00:00:00Z".to_string());
    let json = serde_json::to_string(&dt).unwrap();
    assert_eq!(json, "\"2025-01-01T00:00:00Z\"");

    let parsed: linear_schema::DateTime = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, dt);
}
