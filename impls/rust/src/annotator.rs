use crate::diagnostic::Diagnostic;
use crate::scope::Scope;
use crate::type_::{Type, Value};
use crate::type_of::{eval_statement, method_return_type, line_of};

/// 各トップレベル文の推論結果を返す。
pub struct Annotation {
    pub line: u32,
    pub type_: Type,
}

/// Ruby ソースの各文を型推論し、行番号と型のペアを返す。
/// メソッド定義は RBS 風シグネチャ、それ以外は推論型。
pub fn annotate(source: &[u8]) -> Vec<Annotation> {
    let result = ruby_prism::parse(source);
    let mut ignored: Vec<Diagnostic> = Vec::new();
    let program = result.node();
    let program_node = program.as_program_node().unwrap();
    let mut scope = Scope::new();
    let mut out = Vec::new();

    for stmt in program_node.statements().body().iter() {
        let line = line_of(source, stmt.location().start_offset());

        if let Some(def_node) = stmt.as_def_node() {
            let name = String::from_utf8_lossy(def_node.name().as_slice()).into_owned();
            let params = def_node
                .parameters()
                .map(|p| {
                    p.requireds()
                        .iter()
                        .map(|_| "untyped".to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            let ret_ty = method_return_type(def_node, &scope, &mut ignored, source);
            // シグネチャを Str 型として格納（UI 層で使う）
            let sig = format!("def {name}: ({params}) -> {ret_ty}");
            out.push(Annotation { line, type_: Type::Const(Value::Str(sig)) });
        } else {
            let (ty, new_scope) = eval_statement(stmt, scope, &mut ignored, source);
            scope = new_scope;
            out.push(Annotation { line, type_: ty });
        }
    }

    out
}
