#[derive(Clone, Debug)]
pub struct Span {
    pub row_start: usize,
    pub row_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub file_name: String,
}
