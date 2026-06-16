use std::collections::HashMap;

use crate::type_::{Type, Value};

/// A method signature.
#[derive(Debug, Clone)]
pub struct Signature {
    pub params: Vec<Type>,
    pub returns: Type,
}

/// Signatures of the core types (same content as Rbs::CORE in the Ruby version).
pub const CORE: &str = "
class Integer
  def +: (Integer) -> Integer
  def -: (Integer) -> Integer
  def *: (Integer) -> Integer
  def to_s: () -> String
end
class String
  def +: (String) -> String
  def *: (Integer) -> String
  def length: () -> Integer
  def upcase: () -> String
end
";

/// Turn RBS text into a `(class, method) → Signature` table.
/// A direct port of the Ruby version's `Rbs.load`.
pub fn load(source: &str) -> HashMap<(String, String), Signature> {
    let mut table = HashMap::new();
    let mut current: Option<String> = None;

    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("class ") {
            current = Some(rest.trim().to_string());
        } else if let Some(klass) = &current {
            if let Some(sig) = parse_def_line(trimmed) {
                table.insert((klass.clone(), sig.0), sig.1);
            }
        }
    }

    table
}

/// Parse `def name: (T, ...) -> R` and return `(method_name, Signature)`.
fn parse_def_line(line: &str) -> Option<(String, Signature)> {
    let rest = line.strip_prefix("def ")?;
    let colon = rest.find(": (")?;
    let name = rest[..colon].to_string();
    let rest = &rest[colon + 3..]; // skip the 3 characters of ": ("

    let close = rest.find(") -> ")?;
    let params_str = &rest[..close];
    let ret_str = rest[close + 5..].trim();

    let params: Vec<Type> = params_str
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_type)
        .collect();

    let returns = parse_type(ret_str);

    Some((name, Signature { params, returns }))
}

fn parse_type(s: &str) -> Type {
    match s {
        "untyped" => Type::Dynamic,
        other => Type::Nominal(other.to_string()),
    }
}

/// Decide whether dispatch can "fold the value" and return the folded result.
/// Fold only when both operands are Const. Return None if it gets too large.
pub fn foldable_result(recv: &Type, method: &str, args: &[Type]) -> Option<Type> {
    use Type::Const;
    use Value::{Int, Str};

    match (recv, method, args) {
        (Const(Int(a)), "+", [Const(Int(b))]) => {
            let r = a.checked_add(*b)?;
            if r.unsigned_abs() > 1_000_000 { None } else { Some(Const(Int(r))) }
        }
        (Const(Int(a)), "-", [Const(Int(b))]) => {
            let r = a.checked_sub(*b)?;
            if r.unsigned_abs() > 1_000_000 { None } else { Some(Const(Int(r))) }
        }
        (Const(Int(a)), "*", [Const(Int(b))]) => {
            let r = a.checked_mul(*b)?;
            if r.unsigned_abs() > 1_000_000 { None } else { Some(Const(Int(r))) }
        }
        (Const(Str(a)), "+", [Const(Str(b))]) => {
            let r = format!("{a}{b}");
            if r.len() > 100 { None } else { Some(Const(Str(r))) }
        }
        (Const(Str(a)), "*", [Const(Int(n))]) => {
            if *n < 0 { return Some(Const(Str(String::new()))); }
            let r = a.repeat(*n as usize);
            if r.len() > 100 { None } else { Some(Const(Str(r))) }
        }
        _ => None,
    }
}
