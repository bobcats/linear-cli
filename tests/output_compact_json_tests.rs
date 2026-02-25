use linear_cli::output::{generic_json_formatter, generic_json_list_formatter};
use serde::Serialize;

#[derive(Serialize)]
struct Payload {
    id: String,
    value: i32,
}

#[test]
fn test_generic_json_formatter_defaults_to_compact_json() {
    let payload = Payload {
        id: "abc".to_string(),
        value: 42,
    };

    let json = generic_json_formatter(&payload).expect("json formatting should succeed");

    assert_eq!(json, "{\"id\":\"abc\",\"value\":42}");
    assert!(!json.contains('\n'));
}

#[test]
fn test_generic_json_list_formatter_defaults_to_compact_json() {
    let payloads = vec![
        Payload {
            id: "a".to_string(),
            value: 1,
        },
        Payload {
            id: "b".to_string(),
            value: 2,
        },
    ];

    let json = generic_json_list_formatter(&payloads).expect("json list formatting should succeed");

    assert_eq!(
        json,
        "[{\"id\":\"a\",\"value\":1},{\"id\":\"b\",\"value\":2}]"
    );
    assert!(!json.contains('\n'));
}
