use crate::value::Value;

#[cfg(test)]
mod tests;

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
            Some('"') => {
                let string = self.parse_string()?;
                Value::String(string)
            }
            Some('[') => {
                let array = self.parse_array()?;
                Value::Array(array)
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

    fn parse_string(&mut self) -> Result<String, String> {
        let initial_pos = self.pos;
        self.bump();
        let mut string = String::new();
        let mut terminated = false;
        while let Some(c) = self.peek() {
            if c == '"' {
                terminated = true;
                self.bump();
                break;
            } else if c == '\\' {
                self.bump();
                let decoded = match self.peek() {
                    Some('"') => '"',
                    Some('\\') => '\\',
                    Some('/') => '/',
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('b') => '\u{0008}',
                    Some('f') => '\u{000C}',
                    //TODO: add support for Unicode escape
                    Some(other) => {
                        return Err(format!("invalid escape \\{other} at position {}", self.pos));
                    }
                    None => return Err(format!("unterminated string started at {initial_pos}")),
                };
                string.push(decoded);
                self.bump();
            } else {
                string.push(c);
                self.bump();
            }
        }
        if !terminated {
            return Err(format!("unterminated string started at {}", initial_pos));
        }
        Ok(string)
    }

    fn parse_array(&mut self) -> Result<Vec<Value>, String> {
        let initial_pos = self.pos;
        self.bump();
        self.skip_ws();
        let mut array = Vec::new();
        match self.peek() {
            Some(']') => {
                self.bump();
                return Ok(array);
            }
            Some(',') => return Err(format!("unexpected comma at position {}", self.pos)),
            Some(_) => (),
            None => return Err(format!("unterminated array started at {}", initial_pos)),
        }
        loop {
            array.push(self.parse()?);
            self.skip_ws();
            match self.peek() {
                Some(']') => {
                    self.bump();
                    break;
                }
                Some(',') => {
                    self.bump();
                    self.skip_ws();
                    match self.peek() {
                        Some(',') => {
                            return Err(format!("unexpected comma at position {}", self.pos));
                        }
                        Some(']') => {
                            return Err(format!("unexpected bracket at position {}", self.pos));
                        }
                        Some(_) => (),
                        None => {
                            return Err(format!("unterminated array started at {}", initial_pos));
                        }
                    }
                }
                Some(_) => return Err(format!("expected ] or , at position {}", self.pos)),
                None => return Err(format!("unterminated array started at {}", initial_pos)),
            }
        }
        Ok(array)
    }
}
