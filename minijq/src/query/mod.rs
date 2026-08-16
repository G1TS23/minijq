#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Query {
    Field(String),
    Index(i32),
}
