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

    fn peak(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    pub(crate) fn parse_query(&self) -> Result<Query, String> {
        todo!()
    }
}
