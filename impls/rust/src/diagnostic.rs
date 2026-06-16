/// Diagnostic information for a type error.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub line: u32,
    pub message: String,
}
