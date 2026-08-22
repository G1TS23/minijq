use super::*;

#[test]
fn test_query() {
    assert_eq!(
        QueryParser::new(".users").parse(),
        Ok(vec![Query::Field("users".into())])
    );
    assert_eq!(
        QueryParser::new(".user_info").parse(),
        Ok(vec![Query::Field("user_info".into())])
    );
    assert_eq!(
        QueryParser::new("._users").parse(),
        Ok(vec![Query::Field("_users".into())])
    );
}

#[test]
fn test_query_empty_array() {
    assert_eq!(
        QueryParser::new(".users[]").parse(),
        Ok(vec![Query::Field("users".into()), Query::All])
    );
}

#[test]
fn test_query_all() {
    assert_eq!(QueryParser::new(".").parse(), Ok(vec![]));
}

#[test]
fn wrong_queries() {
    assert!(QueryParser::new(".é").parse().is_err());
    assert!(QueryParser::new(".users[0x]").parse().is_err());
    assert!(QueryParser::new("users").parse().is_err());
    assert!(QueryParser::new(".users[-1]x").parse().is_err());
}
