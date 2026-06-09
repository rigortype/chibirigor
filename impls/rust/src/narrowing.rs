use ruby_prism::{CallNode, Node};

use crate::scope::Scope;
use crate::type_::{class_of, Type, Value};

/// 条件分岐の枝ごとに変数の型を絞る。
/// 絞れない条件はスコープをそのまま返す（脅かさない）。
pub fn narrow<'pr>(scope: &Scope, cond: Node<'pr>, truthy: bool) -> Scope {
    let Some(call) = cond.as_call_node() else { return scope.clone() };
    let Some(recv) = call.receiver() else { return scope.clone() };
    let Some(var) = recv.as_local_variable_read_node() else { return scope.clone() };

    let var_name = String::from_utf8_lossy(var.name().as_slice()).into_owned();
    let Some(current) = scope.local(&var_name).cloned() else { return scope.clone() };

    let method = String::from_utf8_lossy(call.name().as_slice()).into_owned();

    match narrow_type(&current, &method, &call, truthy) {
        Some(narrowed) => scope.with_local(var_name, narrowed),
        None => scope.clone(),
    }
}

/// 絞れたら新しい型を、絞れなければ None を返す。
fn narrow_type<'pr>(current: &Type, method: &str, call: &CallNode<'pr>, truthy: bool) -> Option<Type> {
    match method {
        "nil?" => {
            if truthy {
                Some(Type::Nominal("NilClass".to_string()))
            } else {
                remove_nil(current)
            }
        }
        "is_a?" | "kind_of?" | "instance_of?" => {
            let klass = class_argument(call)?;
            // 真の枝だけ絞る。そのクラスが current に含まれるときだけ。
            if truthy && possible(current, &klass) {
                Some(Type::Nominal(klass))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// current 型で klass があり得るか。
fn possible(current: &Type, klass: &str) -> bool {
    if matches!(current, Type::Dynamic) {
        return false;
    }
    let members: Vec<&Type> = if let Type::Union(ms) = current {
        ms.iter().collect()
    } else {
        vec![current]
    };
    members.iter().any(|m| class_of(m) == Some(klass))
}

/// Union から NilClass を除く。
fn remove_nil(ty: &Type) -> Option<Type> {
    if let Type::Union(members) = ty {
        let filtered: Vec<Type> = members
            .iter()
            .filter(|m| class_of(m) != Some("NilClass") && *m != &Type::Const(Value::Nil))
            .cloned()
            .collect();
        if filtered.len() == members.len() {
            None // 変わらない
        } else {
            Some(crate::type_::union(filtered))
        }
    } else {
        None
    }
}

/// `is_a?(ClassName)` の引数からクラス名を取り出す。
fn class_argument<'pr>(call: &CallNode<'pr>) -> Option<String> {
    let arg = call
        .arguments()?
        .arguments()
        .iter()
        .next()?;
    let const_node = arg.as_constant_read_node()?;
    Some(String::from_utf8_lossy(const_node.name().as_slice()).into_owned())
}
