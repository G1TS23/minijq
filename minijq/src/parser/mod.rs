use crate::value::Value;
use std::collections::BTreeMap;

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
            Some('{') => {
                let object = self.parse_object()?;
                Value::Object(object)
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
        self.skip_ws();
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
        self.skip_ws();
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
                    Some('u') => self.parse_unicode()?,
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

    fn parse_unicode(&mut self) -> Result<char, String> {
        //TODO: emoji
        // \uD83D\uDE00 — un émoji. JSON encode les caractères hors du plan de base sur deux
        // échappements consécutifs (une « paire de substitution »). char::from_u32(0xD83D)
        // rend None, donc ton code refusera l'émoji avec « invalid code point ».
        // Quand le code lu est entre D800 et DBFF, il faut lire l'échappement suivant et combiner les deux.
        self.bump(); // consomme le 'u'
        let hex: String = self
            .chars
            .get(self.pos..self.pos + 4)
            .ok_or_else(|| format!("incomplete \\u escape at position {}", self.pos))?
            .iter()
            .collect();
        let code = u32::from_str_radix(&hex, 16)
            .map_err(|_| format!("invalid \\u escape {hex:?} at position {}", self.pos))?;
        let decoded = char::from_u32(code)
            .ok_or_else(|| format!("invalid code point U+{hex} at position {}", self.pos))?;
        self.bump_n(3);
        Ok(decoded)
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

    fn parse_object(&mut self) -> Result<BTreeMap<String, Value>, String> {
        let initial_pos = self.pos;
        self.bump();
        self.skip_ws();
        let mut object: BTreeMap<String, Value> = BTreeMap::new();
        if self.peek() == Some('}') {
            self.bump();
            return Ok(object);
        }
        loop {
            self.skip_ws();
            let key = match self.peek() {
                Some('"') => self.parse_string()?,
                Some(_) => return Err(format!("expected '\"' at position {}", self.pos)),
                None => return Err(format!("unterminated Object at {}", initial_pos)),
            };
            self.skip_ws();
            let value = match self.peek() {
                Some(':') => {
                    self.bump();
                    self.skip_ws();
                    self.parse()?
                }
                Some(_) => return Err(format!("expected ':' at position {}", self.pos)),
                None => {
                    return Err(format!("unterminated Object started at {}", initial_pos));
                }
            };
            object.insert(key, value);
            self.skip_ws();
            match self.peek() {
                Some(',') => {
                    self.bump();
                }
                Some('}') => {
                    self.bump();
                    break;
                }
                Some(_) => {
                    return Err(format!("expected {} or , at {}", '}', self.pos));
                }
                None => {
                    return Err(format!("unterminated Object started at {}", initial_pos));
                }
            };
        }
        Ok(object)
    }
}
