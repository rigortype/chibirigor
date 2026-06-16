use ruby_prism::{CallNode, DefNode, IfNode, Node, StatementsNode};

use crate::diagnostic::Diagnostic;
use crate::dispatch::dispatch;
use crate::narrowing::narrow;
use crate::scope::Scope;
use crate::type_::{union, Type, Value};

/// Convert a byte offset within the source into a 1-based line number.
pub fn line_of(source: &[u8], offset: usize) -> u32 {
    source[..offset.min(source.len())].iter().filter(|&&b| b == b'\n').count() as u32 + 1
}

/// Evaluate a single statement and return (type, updated scope). Only assignments update the scope.
pub fn eval_statement<'pr>(
    node: Node<'pr>,
    scope: Scope,
    diagnostics: &mut Vec<Diagnostic>,
    source: &[u8],
) -> (Type, Scope) {
    if let Some(n) = node.as_local_variable_write_node() {
        let ty = type_of(n.value(), &scope, diagnostics, source);
        let name = String::from_utf8_lossy(n.name().as_slice()).into_owned();
        let new_scope = scope.with_local(name, ty.clone());
        return (ty, new_scope);
    }
    (type_of(node, &scope, diagnostics, source), scope)
}

/// Compute the type of an expression (a Prism node). The heart of the type checker.
pub fn type_of<'pr>(node: Node<'pr>, scope: &Scope, diagnostics: &mut Vec<Diagnostic>, source: &[u8]) -> Type {
    if let Some(n) = node.as_integer_node() {
        let val: i32 = n.value().try_into().unwrap_or(0);
        return Type::Const(Value::Int(i64::from(val)));
    }
    if let Some(n) = node.as_float_node() {
        return Type::Const(Value::Float(n.value()));
    }
    if let Some(n) = node.as_string_node() {
        let s = String::from_utf8_lossy(n.unescaped()).into_owned();
        return Type::Const(Value::Str(s));
    }
    if let Some(n) = node.as_symbol_node() {
        let s = String::from_utf8_lossy(n.unescaped()).into_owned();
        return Type::Const(Value::Symbol(s));
    }
    if node.as_true_node().is_some() {
        return Type::Const(Value::Bool(true));
    }
    if node.as_false_node().is_some() {
        return Type::Const(Value::Bool(false));
    }
    if node.as_nil_node().is_some() {
        return Type::Const(Value::Nil);
    }
    if let Some(n) = node.as_local_variable_read_node() {
        let name = String::from_utf8_lossy(n.name().as_slice()).into_owned();
        return scope.local(&name).cloned().unwrap_or(Type::Dynamic);
    }
    if let Some(n) = node.as_local_variable_write_node() {
        return type_of(n.value(), scope, diagnostics, source);
    }
    if let Some(n) = node.as_hash_node() {
        return type_of_hash(n.elements().iter(), scope, diagnostics, source);
    }
    if let Some(n) = node.as_array_node() {
        let elements: Vec<Type> = n.elements().iter().map(|el| type_of(el, scope, diagnostics, source)).collect();
        return Type::Tuple(elements);
    }
    if let Some(n) = node.as_parentheses_node() {
        return match n.body() {
            None => Type::Const(Value::Nil),
            Some(body) => match body.as_statements_node() {
                Some(stmts) => type_of_stmts(stmts, scope, diagnostics, source),
                None => type_of(body, scope, diagnostics, source),
            },
        };
    }
    if let Some(n) = node.as_call_node() {
        return type_of_call(n, scope, diagnostics, source);
    }
    if let Some(n) = node.as_if_node() {
        return type_of_if(n, scope, diagnostics, source);
    }
    if let Some(n) = node.as_def_node() {
        return type_of_def(n, scope, diagnostics, source);
    }
    Type::Dynamic
}

/// Hash literal → HashShape (only symbol keys are remembered).
fn type_of_hash<'pr>(
    elements: impl Iterator<Item = Node<'pr>>,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
    source: &[u8],
) -> Type {
    let mut fields = Vec::new();
    for el in elements {
        let Some(assoc) = el.as_assoc_node() else { continue };
        let Some(sym) = assoc.key().as_symbol_node() else { continue };
        let key = String::from_utf8_lossy(sym.unescaped()).into_owned();
        let val_ty = type_of(assoc.value(), scope, diagnostics, source);
        fields.push((key, val_ty));
    }
    Type::HashShape(fields)
}

/// Type inference for method sends.
fn type_of_call<'pr>(node: CallNode<'pr>, scope: &Scope, diagnostics: &mut Vec<Diagnostic>, source: &[u8]) -> Type {
    let recv_ty = node.receiver().map(|r| type_of(r, scope, diagnostics, source)).unwrap_or(Type::Dynamic);

    let arg_types: Vec<Type> = node
        .arguments()
        .map(|a| a.arguments().iter().map(|arg| type_of(arg, scope, diagnostics, source)).collect())
        .unwrap_or_default();

    let line = line_of(source, node.location().start_offset());
    dispatch(&recv_ty, node.name().as_slice(), &arg_types, line, diagnostics)
}

/// Type inference for an if expression. Union the types of both branches and narrow per branch.
fn type_of_if<'pr>(node: IfNode<'pr>, scope: &Scope, diagnostics: &mut Vec<Diagnostic>, source: &[u8]) -> Type {
    let pred = node.predicate();
    let _ = type_of(pred, scope, diagnostics, source); // type-check the condition too

    let then_scope = narrow(scope, node.predicate(), true);
    let else_scope = narrow(scope, node.predicate(), false);

    let then_ty = node
        .statements()
        .map(|s| type_of_stmts(s, &then_scope, diagnostics, source))
        .unwrap_or(Type::Const(Value::Nil));

    let else_ty = node
        .subsequent()
        .and_then(|n| n.as_else_node())
        .and_then(|e| e.statements())
        .map(|s| type_of_stmts(s, &else_scope, diagnostics, source))
        .unwrap_or(Type::Const(Value::Nil));

    union(vec![then_ty, else_ty])
}

/// Type inference for a method definition. Type-check the body and collect diagnostics.
fn type_of_def<'pr>(node: DefNode<'pr>, scope: &Scope, diagnostics: &mut Vec<Diagnostic>, source: &[u8]) -> Type {
    let body_scope = node
        .parameters()
        .map(|p| {
            p.requireds().iter().fold(scope.clone(), |s, r| {
                let name = r
                    .as_required_parameter_node()
                    .map(|rp| String::from_utf8_lossy(rp.name().as_slice()).into_owned())
                    .unwrap_or_default();
                s.with_local(name, Type::Dynamic)
            })
        })
        .unwrap_or_else(|| scope.clone());

    if let Some(body) = node.body().and_then(|b| b.as_statements_node()) {
        let _ = type_of_stmts(body, &body_scope, diagnostics, source);
    }

    // The value of a def expression is the method-name symbol
    let name = String::from_utf8_lossy(node.name().as_slice()).into_owned();
    Type::Const(Value::Symbol(name))
}

/// Evaluate a StatementsNode and return the type of the last statement. Thread the scope through the statements.
pub fn type_of_stmts<'pr>(
    stmts: StatementsNode<'pr>,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
    source: &[u8],
) -> Type {
    let mut cur = scope.clone();
    let mut last = Type::Const(Value::Nil);
    for stmt in stmts.body().iter() {
        let (ty, new_scope) = eval_statement(stmt, cur, diagnostics, source);
        cur = new_scope;
        last = ty;
    }
    last
}

/// Synthesize a method's return type from its body (for return-type checking).
pub fn method_return_type<'pr>(
    node: DefNode<'pr>,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
    source: &[u8],
) -> Type {
    let body_scope = node
        .parameters()
        .map(|p| {
            p.requireds().iter().fold(scope.clone(), |s, r| {
                let name = r
                    .as_required_parameter_node()
                    .map(|rp| String::from_utf8_lossy(rp.name().as_slice()).into_owned())
                    .unwrap_or_default();
                s.with_local(name, Type::Dynamic)
            })
        })
        .unwrap_or_else(|| scope.clone());

    node.body()
        .and_then(|b| b.as_statements_node())
        .map(|s| type_of_stmts(s, &body_scope, diagnostics, source))
        .unwrap_or(Type::Const(Value::Nil))
}
