use crate::accepts::{accepts, Verdict};
use crate::diagnostic::Diagnostic;
use crate::scope::Scope;
use crate::type_of::{eval_statement, line_of, method_return_type};

/// Ruby ソースを型チェックし、診断の Vec を返す。
/// baseline を渡すと「行＋メッセージ」で差し引いた新規診断だけ返す。
pub fn check(source: &[u8], baseline: &[Diagnostic]) -> Vec<Diagnostic> {
    let result = ruby_prism::parse(source);
    let mut diagnostics = Vec::new();
    let program = result.node();
    let program = program.as_program_node().unwrap();
    let mut scope = Scope::new();

    for stmt in program.statements().body().iter() {
        let (_ty, new_scope) = eval_statement(stmt, scope, &mut diagnostics, source);
        scope = new_scope;
    }

    // 戻り型照合（opt-in）: rbs パラメータが渡されたときの相当処理は
    // Rust 版では check_with_rbs を別関数として提供する。

    subtract_baseline(diagnostics, baseline)
}

/// RBS シグネチャ文字列を渡すと戻り型照合も行う。
pub fn check_with_rbs(source: &[u8], rbs_source: &str, baseline: &[Diagnostic]) -> Vec<Diagnostic> {
    use crate::rbs;
    use ruby_prism::parse;

    let result = parse(source);
    let mut diagnostics = Vec::new();
    let program = result.node();
    let program_node = program.as_program_node().unwrap();
    let mut scope = Scope::new();

    for stmt in program_node.statements().body().iter() {
        let (_ty, new_scope) = eval_statement(stmt, scope, &mut diagnostics, source);
        scope = new_scope;
    }

    // 戻り型照合
    let user_sigs = rbs::load(rbs_source);
    for stmt in program_node.statements().body().iter() {
        let Some(def_node) = stmt.as_def_node() else { continue };
        let method_name = String::from_utf8_lossy(def_node.name().as_slice()).into_owned();
        let line = line_of(source, stmt.location().start_offset());

        // クラス無し（トップレベル）は "Object" として照合
        let sig = user_sigs.get(&("Object".to_string(), method_name.clone()))
            .or_else(|| user_sigs.iter().find(|((_, m), _)| m == &method_name).map(|(_, s)| s));

        let Some(sig) = sig else { continue };

        let body_ty = method_return_type(def_node, &scope, &mut Vec::new(), source);

        if accepts(&sig.returns, &body_ty) == Verdict::No {
            diagnostics.push(Diagnostic {
                line,
                message: format!(
                    "戻り型 {} が宣言されていますが {} を返します",
                    sig.returns, body_ty
                ),
            });
        }
    }

    subtract_baseline(diagnostics, baseline)
}

fn subtract_baseline(diagnostics: Vec<Diagnostic>, baseline: &[Diagnostic]) -> Vec<Diagnostic> {
    let seen: Vec<(u32, &str)> = baseline.iter().map(|d| (d.line, d.message.as_str())).collect();
    diagnostics.into_iter().filter(|d| !seen.contains(&(d.line, &d.message))).collect()
}
