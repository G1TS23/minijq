use std::collections::BTreeMap;
use crate::value::Value;
use super::*;
#[test]
fn lit_data_json() {
    let data = include_str!("tests/test_data.json");
    let mut object: BTreeMap<String, Value> = BTreeMap::new();
    let mut array: Vec<Value> = Vec::new();
    array.push(Value::Object(BTreeMap::from([
        ("name".to_string(), Value::String("Alice".to_string())),
        ("age".to_string(), Value::Number(30.0)),
        ("admin".to_string(), Value::Bool(true)),
    ])));
    array.push(Value::Object(BTreeMap::from([
        ("name".to_string(), Value::String("Bob".to_string())),
        ("age".to_string(), Value::Number(25.0)),
        ("admin".to_string(), Value::Bool(false)),
    ])));
    object.insert("users".to_string(), Value::Array(array));
    assert_eq!(Parser::new(data).parse(), Ok(Value::Object(object)));
}