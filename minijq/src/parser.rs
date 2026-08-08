use crate::value::Value;

const NULL: &[char] = &['n', 'u', 'l', 'l'];
const TRUE: &[char] = &['t', 'r', 'u', 'e'];
const FALSE: &[char] = &['f', 'a', 'l', 's', 'e'];
pub(crate) struct Parser {
    chars: Vec<char>,
    pos: usize,
}
impl Parser {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    fn bump_n(&mut self, n: usize) {
        self.pos += n;
    }
    fn eat(&mut self, needle: &[char]) -> Result<(), String> {
        if self.chars[self.pos..].starts_with(needle) {
            self.bump_n(needle.len());
            Ok(())
        } else {
            Err(format!(
                "expected {:?} at position {}",
                needle.iter().collect::<String>(),
                self.pos
            ))
        }
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.bump();
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Value, String> {
        self.skip_ws();
        let value = match self.peek() {
            Some('n') => {
                self.eat(NULL)?;
                Value::Null
            }
            Some('t') => {
                self.eat(TRUE)?;
                Value::Bool(true)
            }
            Some('f') => {
                self.eat(FALSE)?;
                Value::Bool(false)
            }
            Some(c) if c.is_ascii_digit() || c == '-' => {
                let float = self.parse_number()?;
                Value::Number(float)
            }
            Some(c) => {
                return Err(format!(
                    "unexpected character {:?} at position {}",
                    c, self.pos
                ));
            }
            None => return Err(format!("end of file at position {}", self.pos)),
        };
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let initial_pos = self.pos;
        let mut number: String = self.chars[self.pos].to_string();
        self.bump();
        while let Some(n) = self.peek() {
            if !(n.is_ascii_digit() || n == '.' || n == 'e' || n == 'E' || n == '+' || n == '-') {
                break;
            }
            number.push(n);
            self.bump();
        }
        let float: f64 = number
            .parse::<f64>()
            .map_err(|_| format!("invalid number {:?} at position {}", number, initial_pos))?;
        Ok(float)
    }
}

#[cfg(test)]
mod tests {
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
}
