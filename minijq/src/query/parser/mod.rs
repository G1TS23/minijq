use crate::query::Query;

#[cfg(test)]
mod tests;

pub(crate) struct QueryParser {
    chars: Vec<char>,
    pos: usize,
}

impl QueryParser {
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

    fn parse_step(&mut self) -> Result<Query, String> {
        let query: Query = match self.peek() {
            Some('[') => self.parse_array()?,
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let string = self.parse_ident(c);
                Query::Field(string)
            }
            Some(c) => {
                return Err(format!("unexpected character: {}", c));
            }
            None => {
                return Err("end of query".to_string());
            }
        };
        Ok(query)
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Query>, String> {
        if self.chars == ['.'] {
            return Ok(vec![]);
        }
        let mut queries = vec![];
        loop {
            let query = match self.peek() {
                Some('.') => {
                    self.bump();
                    self.parse_step()?
                }
                Some('[') => self.parse_step()?,
                Some(c) => return Err(format!("unexpected character: {}", c)),
                None => break,
            };
            queries.push(query);
        }
        Ok(queries)
    }
    fn parse_ident(&mut self, c: char) -> String {
        let mut ident = String::new();
        ident.push(c);
        self.bump();
        loop {
            match self.peek() {
                Some(c) if c.is_ascii_alphanumeric() || c == '_' => {
                    ident.push(c);
                    self.bump();
                }
                _ => break,
            }
        }
        ident
    }

    fn parse_array(&mut self) -> Result<Query, String> {
        self.bump();
        let query = match self.peek() {
            Some('"') => {
                self.bump();
                let mut string = String::new();
                loop {
                    match self.peek() {
                        Some('"') => {
                            self.bump();
                            break;
                        }
                        Some(c) => {
                            string.push(c);
                            self.bump();
                        }
                        None => {
                            return Err("unexpected end of query".to_string());
                        }
                    }
                }
                Query::Field(string)
            }
            Some(c) if c.is_ascii_digit() || c == '-' => {
                let mut string = String::new();
                string.push(c);
                self.bump();
                loop {
                    match self.peek() {
                        Some(c) if c.is_ascii_digit() => {
                            string.push(c);
                            self.bump();
                        }
                        Some(_) => {
                            break;
                        }
                        None => {
                            return Err("end of query".to_string());
                        }
                    }
                }
                Query::Index(string.parse().map_err(|_| "invalid index".to_string())?)
            }
            Some(']') => Query::All,
            Some(c) => {
                return Err(format!("unexpected character: {}", c));
            }
            None => {
                return Err("unexpected end of query".to_string());
            }
        };
        self.eat(&[']'])?;
        Ok(query)
    }
}
