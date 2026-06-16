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

/// Typing of a method send.
/// 1. Prefer plugin registrations
/// 2. Fall back to METHODS generated from RBS
/// 3. Unknown methods become Dynamic (don't scare the user)
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
        None => return Type::Dynamic, // HashShape / Tuple / Dynamic → don't scare the user
    };

    let sig = plugin::lookup(klass, method_str)
        .or_else(|| methods().get(&(klass.to_string(), method_str.to_string())).cloned());

    let sig = match sig {
        Some(s) => s,
        None => return Type::Dynamic, // unknown method → don't scare the user
    };

    if args.len() != sig.params.len() {
        diagnostics.push(Diagnostic {
            line,
            message: format!(
                "wrong number of arguments for {method_str} ({} expected, {} given)",
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
                message: format!("expected {param} but got {arg}"),
            });
        }
    }

    // Fold to a Const if possible; otherwise use the table's return type.
    foldable_result(receiver, method_str, args).unwrap_or_else(|| sig.returns.clone())
}
