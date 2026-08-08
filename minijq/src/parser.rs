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
}
