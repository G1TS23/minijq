#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_with_level(0))
    }
}

impl Value {
    fn string_with_level(&self, mut level: usize) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => self.escape_string(s),
            Value::Array(a) => {
                if a.is_empty() {
                    "[]".to_string()
                } else {
                    let mut str: String = "[\n".to_string();
                    level += 1;
                    let mut it = a.iter().peekable();
                    while let Some(v) = it.next() {
                        if it.peek().is_some() {
                            str.push_str(
                                format!("{}{},\n", "  ".repeat(level), v.string_with_level(level))
                                    .as_str(),
                            );
                        } else {
                            str.push_str(
                                format!("{}{}\n", "  ".repeat(level), v.string_with_level(level))
                                    .as_str(),
                            );
                            level -= 1;
                        }
                    }
                    str.push_str(format!("{}]", "  ".repeat(level)).as_str());
                    str
                }
            }
            Value::Object(o) => {
                if o.is_empty() {
                    "{}".to_string()
                } else {
                    let mut str: String = "{\n".to_string();
                    level += 1;
                    let mut it = o.iter().peekable();
                    while let Some((k, v)) = it.next() {
                        if it.peek().is_some() {
                            str.push_str(
                                format!(
                                    "{}\"{}\": {},\n",
                                    "  ".repeat(level),
                                    k,
                                    v.string_with_level(level)
                                )
                                .as_str(),
                            );
                        } else {
                            str.push_str(
                                format!(
                                    "{}\"{}\": {}\n",
                                    "  ".repeat(level),
                                    k,
                                    v.string_with_level(level)
                                )
                                .as_str(),
                            );
                            level -= 1;
                        }
                    }
                    str.push_str(format!("{}}}", "  ".repeat(level)).as_str());
                    str
                }
            }
        }
    }

    fn escape_string(&self, str: &str) -> String {
        let mut string: String = "\"".to_string();
        for c in str.chars() {
            match c {
                '"' => string.push_str("\\\""),
                '\\' => string.push_str("\\\\"),
                '\n' => string.push_str("\\n"),
                '\t' => string.push_str("\\t"),
                '\r' => string.push_str("\\r"),
                '\u{0008}' => string.push_str("\\b"),
                '\u{000C}' => string.push_str("\\f"),
                c if (c as u32) < 0x20 => {
                    let hex = format!("\\u{:04x}", c as u32);
                    string.push_str(&hex);
                }
                _ => string.push(c),
            }
        }
        string.push('"');
        string
    }
}
