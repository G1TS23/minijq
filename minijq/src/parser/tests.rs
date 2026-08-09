use super::*;

//NULL
#[test]
fn lit_null() {
    assert_eq!(Parser::new(" null").parse(), Ok(Value::Null));
}

#[test]
fn lit_with_spaces() {
    assert_eq!(Parser::new("  null").parse(), Ok(Value::Null));
}

//BOOL
#[test]
fn lit_true() {
    assert_eq!(Parser::new("true").parse(), Ok(Value::Bool(true)));
}
#[test]
fn lit_false() {
    assert_eq!(Parser::new("false").parse(), Ok(Value::Bool(false)));
}

#[test]
fn refuse_gibberish() {
    assert!(Parser::new("nope").parse().is_err());
}

#[test]
fn refuse_empty() {
    assert!(Parser::new("").parse().is_err());
}

#[test]
fn refuse_empty_with_spaces() {
    assert!(Parser::new("   ").parse().is_err());
}

#[test]
fn refuse_other_chars() {
    assert!(Parser::new("a").parse().is_err());
}

//NUMBER
#[test]
fn lit_number() {
    assert_eq!(Parser::new("42").parse(), Ok(Value::Number(42.0)));
}
#[test]
fn lit_number_with_spaces() {
    assert_eq!(Parser::new(" 42").parse(), Ok(Value::Number(42.0)));
}

#[test]
fn lit_number_with_decimals() {
    assert_eq!(Parser::new("42.34").parse(), Ok(Value::Number(42.34)));
}

#[test]
fn lit_negative_number() {
    assert_eq!(Parser::new("-42").parse(), Ok(Value::Number(-42.0)));
}

#[test]
fn lit_negative_number_with_decimals() {
    assert_eq!(Parser::new("-42.34").parse(), Ok(Value::Number(-42.34)));
}

#[test]
fn lit_exponential_notation() {
    for (input, expected) in [
        ("42e10", 42e10),
        ("42E-10", 42e-10),
        ("42.34e+10", 42.34e10),
    ] {
        assert_eq!(
            Parser::new(input).parse(),
            Ok(Value::Number(expected)),
            "input : {input}"
        );
    }
}

#[test]
fn lit_wrong_numbers() {
    for input in ["42..", "42.0.3", "42.0.", "-", "1e", "1-2"] {
        assert!(Parser::new(input).parse().is_err(), "input : {input}");
    }
}

//STRING
#[test]
fn lit_string() {
    assert_eq!(
        Parser::new("\"abc\"").parse(),
        Ok(Value::String("abc".to_string()))
    );
}

#[test]
fn lit_empty_string() {
    assert_eq!(
        Parser::new("\"\"").parse(),
        Ok(Value::String("".to_string()))
    );
}

#[test]
fn lit_string_with_spaces() {
    assert_eq!(
        Parser::new(" \"abc\"").parse(),
        Ok(Value::String("abc".to_string()))
    );
}

#[test]
fn lit_string_with_escaped_quotes() {
    assert_eq!(
        Parser::new("\"abc\\\"def\"").parse(),
        Ok(Value::String("abc\"def".to_string()))
    );
}

#[test]
fn lit_string_with_escaped_backslash() {
    assert_eq!(
        Parser::new("\"abc\\\\def\"").parse(),
        Ok(Value::String("abc\\def".to_string()))
    );
}

#[test]
fn lit_string_with_escaped_newline() {
    assert_eq!(
        Parser::new("\"abc\\ndef\"").parse(),
        Ok(Value::String("abc\ndef".to_string()))
    );
}

#[test]
fn lit_string_with_escaped_tab() {
    assert_eq!(
        Parser::new("\"abc\\tdef\"").parse(),
        Ok(Value::String("abc\tdef".to_string()))
    );
}

#[test]
fn lit_string_with_escaped_return() {
    assert_eq!(
        Parser::new("\"abc\\rdef\"").parse(),
        Ok(Value::String("abc\rdef".to_string()))
    );
}

#[test]
fn lit_string_with_escaped_backspace() {
    assert_eq!(
        Parser::new("\"abc\\bdef\"").parse(),
        Ok(Value::String("abc\u{0008}def".to_string()))
    );
}

#[test]
fn lit_string_with_escaped_formfeed() {
    assert_eq!(
        Parser::new("\"abc\\fdef\"").parse(),
        Ok(Value::String("abc\u{000C}def".to_string()))
    );
}

#[test]
fn lit_string_with_unexpected_escape() {
    assert!(Parser::new("\"abc\\gdef\"").parse().is_err());
}

#[test]
fn lit_unfinished_string() {
    assert!(Parser::new("\"abc").parse().is_err());
}

//ARRAY
#[test]
fn lit_array() {
    assert_eq!(Parser::new("[]").parse(), Ok(Value::Array(Vec::new())));
}

#[test]
fn lit_array_with_whitespaces() {
    let array = vec![Value::Number(1.0), Value::Number(2.0)];
    assert_eq!(
        Parser::new("[ 1 , 2 ]").parse(),
        Ok(Value::Array(array.clone()))
    );
    assert_eq!(
        Parser::new("[1 , 2]").parse(),
        Ok(Value::Array(array.clone()))
    );
    assert_eq!(
        Parser::new("[ 1,2 ]").parse(),
        Ok(Value::Array(array.clone()))
    );
}

#[test]
fn lit_array_with_line_breaks() {
    let array = vec![Value::Number(1.0), Value::Number(2.0)];
    assert_eq!(
        Parser::new("[\n  1,\n  2\n]").parse(),
        Ok(Value::Array(array))
    );
}

#[test]
fn lit_array_of_string() {
    let array = vec![
        Value::String("abc".to_string()),
        Value::String("def".to_string()),
    ];
    assert_eq!(
        Parser::new("[\"abc\",\"def\"]").parse(),
        Ok(Value::Array(array))
    );
}

#[test]
fn lit_array_of_float() {
    let array = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
    assert_eq!(Parser::new("[1,2,3]").parse(), Ok(Value::Array(array)));
}

#[test]
fn lit_array_of_bool() {
    let array = vec![Value::Bool(true), Value::Bool(false), Value::Bool(true)];
    assert_eq!(
        Parser::new("[true,false,true]").parse(),
        Ok(Value::Array(array))
    );
}

#[test]
fn lit_array_in_array() {
    let array = vec![
        Value::Array(Vec::new()),
        Value::Array(Vec::new()),
        Value::Array(Vec::new()),
    ];
    assert_eq!(Parser::new("[[],[],[]]").parse(), Ok(Value::Array(array)));
}

#[test]
fn lit_empty_array_in_array() {
    let array = vec![Value::Array(Vec::new())];
    assert_eq!(Parser::new("[[]]").parse(), Ok(Value::Array(array)));
}

#[test]
fn lit_array_with_deep_imbrication() {
    let array = vec![
        Value::Number(1.0),
        Value::Array(vec![
            Value::Number(2.0),
            Value::Array(vec![Value::Number(3.0)]),
        ]),
    ];
    assert_eq!(Parser::new("[1,[2,[3]]]").parse(), Ok(Value::Array(array)));
}

#[test]
fn lit_array_with_different_value() {
    let array = vec![
        Value::Array(Vec::new()),
        Value::Number(42.0),
        Value::String("abc".to_string()),
        Value::Null,
        Value::Bool(true),
    ];
    assert_eq!(
        Parser::new("[[], 42, \"abc\", null, true]").parse(),
        Ok(Value::Array(array))
    );
}

#[test]
fn lit_with_unclosed_array() {
    assert!(Parser::new("[").parse().is_err());
}

#[test]
fn lit_with_malformed_arrays() {
    assert!(Parser::new("[,]").parse().is_err());
    assert!(Parser::new("[,2]").parse().is_err());
    assert!(Parser::new("[,,,]").parse().is_err());
    assert!(Parser::new("[2 2]").parse().is_err());
    assert!(Parser::new("[2,]").parse().is_err());
    assert!(Parser::new("[1,,2]").parse().is_err());
    assert!(Parser::new("[1,").parse().is_err());
    assert!(Parser::new("[1").parse().is_err());
    assert!(Parser::new("[[1]").parse().is_err());
    let err = Parser::new("[1 2]").parse().unwrap_err();
    assert!(err.contains("expected ] or ,"), "message inattendu : {err}");
}
