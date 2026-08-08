use super::*;
#[test]
fn lit_null() {
    assert_eq!(Parser::new(" null").parse(), Ok(Value::Null));
}
#[test]
fn lit_true() {
    assert_eq!(Parser::new("true").parse(), Ok(Value::Bool(true)));
}
#[test]
fn lit_false() {
    assert_eq!(Parser::new("false").parse(), Ok(Value::Bool(false)));
}
#[test]
fn lit_with_spaces() {
    assert_eq!(Parser::new("  null").parse(), Ok(Value::Null));
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
    for (input) in ["42..", "42.0.3", "42.0.", "-", "1e", "1-2"] {
        assert!(Parser::new("input").parse().is_err(), "input : {input}");
    }
}

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
