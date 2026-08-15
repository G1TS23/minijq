use crate::value::Value;
use std::collections::BTreeMap;

#[test]
fn empty_array() {
    let v = Value::Array(vec![]);
    let mut object: BTreeMap<String, Value> = BTreeMap::new();
    object.insert("users".to_string(), v.clone());
    assert_eq!(v.to_string(), "[]".to_string());
    assert_eq!(
        Value::Object(object).to_string(),
        "{\n  \"users\": []\n}".to_string()
    );
}

#[test]
fn string_with_escaped_chars() {
    assert_eq!(
        Value::String(r#""def""#.to_string()).to_string(),
        r#""\"def\"""#
    );
}

#[test]
fn string_with_non_ascii_chars() {
    assert_eq!(Value::String("Ƽdef".to_string()).to_string(), r#""Ƽdef""#);
}

#[test]
fn string_with_control_chars() {
    assert_eq!(
        Value::String("\u{0001}def".to_string()).to_string(),
        r#""\u0001def""#
    );
}

#[test]
fn string_with_multiple_control_chars() {
    assert_eq!(
        Value::String("\n\t\r\u{0008}\u{000C}".to_string()).to_string(),
        r#""\n\t\r\b\f""#
    );
}
