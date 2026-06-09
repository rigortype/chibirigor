use std::collections::HashMap;
use std::sync::OnceLock;

use crate::accepts::{accepts, Verdict};
use crate::diagnostic::Diagnostic;
use crate::rbs::{self, Signature};
use crate::type_::{class_of, Type};
use crate::{plugin, rbs::foldable_result};

static METHODS: OnceLock<HashMap<(String, String), Signature>> = OnceLock::new();

fn methods() -> &'static HashMap<(String, String), Signature> {
    METHODS.get_or_init(|| rbs::load(rbs::CORE))
}

/// メソッド送信の型付け。
/// 1. Plugin 登録を優先
/// 2. RBS から生成した METHODS を参照
/// 3. 知らないメソッドは Dynamic（脅かさない）
pub fn dispatch(
    receiver: &Type,
    method: &[u8],
    args: &[Type],
    line: u32,
    diagnostics: &mut Vec<Diagnostic>,
) -> Type {
    let method_str = match std::str::from_utf8(method) {
        Ok(s) => s,
        Err(_) => return Type::Dynamic,
    };

    let klass = match class_of(receiver) {
        Some(k) => k,
        None => return Type::Dynamic, // HashShape / Tuple / Dynamic → 脅かさない
    };

    let sig = plugin::lookup(klass, method_str)
        .or_else(|| methods().get(&(klass.to_string(), method_str.to_string())).cloned());

    let sig = match sig {
        Some(s) => s,
        None => return Type::Dynamic, // 知らないメソッド → 脅かさない
    };

    if args.len() != sig.params.len() {
        diagnostics.push(Diagnostic {
            line,
            message: format!(
                "{method_str} の引数の数が違います（{} 個必要、{} 個渡された）",
                sig.params.len(),
                args.len()
            ),
        });
        return sig.returns.clone();
    }

    for (param, arg) in sig.params.iter().zip(args.iter()) {
        if accepts(param, arg) == Verdict::No {
            diagnostics.push(Diagnostic {
                line,
                message: format!("{param} が必要ですが {arg} が渡されました"),
            });
        }
    }

    // 畳めれば Const に畳む、無理なら表の戻り型。
    foldable_result(receiver, method_str, args).unwrap_or_else(|| sig.returns.clone())
}
