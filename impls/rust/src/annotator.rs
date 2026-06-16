use crate::diagnostic::Diagnostic;
use crate::scope::Scope;
use crate::type_::{Type, Value};
use crate::type_of::{eval_statement, method_return_type, line_of};

/// The inference result for each top-level statement.
pub struct Annotation {
    pub line: u32,
    pub type_: Type,
}

/// Infer the type of each statement in the Ruby source and return (line number, type) pairs.
/// Method definitions get an RBS-style signature; everything else gets its inferred type.
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
            // Store the signature as a Str type (used by the UI layer)
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
