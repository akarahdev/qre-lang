use std::fmt::{Arguments, Debug, Formatter, Write};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Span {
    pub row_start: usize,
    pub row_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub file_name: String,
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[row {}, {}:{} in {}]", self.row_start, self.column_start, self.column_end, self.file_name)
    }
}
