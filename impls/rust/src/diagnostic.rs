/// 型エラーの診断情報。
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub line: u32,
    pub message: String,
}
