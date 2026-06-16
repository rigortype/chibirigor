/// The type carrier. Corresponds to `Chibirigor::Type` in the Ruby implementation.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// A type representing a literal value itself. Example: Const(Value::Int(1))
    Const(Value),
    /// A named class. Example: Nominal("Integer")
    Nominal(String),
    /// The "unknown" type. The crux of gradual typing.
    Dynamic,
    /// A Union type. Example: Integer | String
    Union(Vec<Type>),
    /// The shape of a hash. Remembers the type per key. Example: {foo: 1, bar: "a"}
    HashShape(Vec<(String, Type)>),
    /// Remembers an array as a "type per position". Example: [1, "a"]
    Tuple(Vec<Type>),
}

/// The kinds of value a Const type can hold (corresponding to Ruby's main literals).
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Symbol(String),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Str(s) => write!(f, "{s:?}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Symbol(s) => write!(f, ":{s}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Const(v) => write!(f, "{v}"),
            Type::Nominal(n) => write!(f, "{n}"),
            Type::Dynamic => write!(f, "untyped"),
            Type::Union(members) => {
                let parts: Vec<_> = members.iter().map(|t| t.to_string()).collect();
                write!(f, "{}", parts.join(" | "))
            }
            Type::HashShape(fields) => {
                let parts: Vec<_> = fields.iter().map(|(k, v)| format!("{k}: {v}")).collect();
                write!(f, "{{{}}}", parts.join(", "))
            }
            Type::Tuple(elements) => {
                let parts: Vec<_> = elements.iter().map(|t| t.to_string()).collect();
                write!(f, "[{}]", parts.join(", "))
            }
        }
    }
}

/// Fold multiple types into one. Flatten nested Unions and remove duplicates.
pub fn union(types: Vec<Type>) -> Type {
    let mut flat = Vec::new();
    for t in types {
        match t {
            Type::Union(members) => flat.extend(members),
            other => flat.push(other),
        }
    }
    flat.dedup();
    if flat.is_empty() {
        return Type::Dynamic;
    }
    if flat.iter().any(|t| t == &Type::Dynamic) {
        return Type::Dynamic;
    }
    if flat.len() == 1 {
        flat.remove(0)
    } else {
        Type::Union(flat)
    }
}

/// Reduce a type to a Ruby class name. Used to match keys in the dispatch table.
pub fn class_of(t: &Type) -> Option<&str> {
    match t {
        Type::Const(Value::Int(_)) => Some("Integer"),
        Type::Const(Value::Float(_)) => Some("Float"),
        Type::Const(Value::Str(_)) => Some("String"),
        Type::Const(Value::Bool(true)) => Some("TrueClass"),
        Type::Const(Value::Bool(false)) => Some("FalseClass"),
        Type::Const(Value::Symbol(_)) => Some("Symbol"),
        Type::Const(Value::Nil) => Some("NilClass"),
        Type::Nominal(n) => Some(n.as_str()),
        _ => None,
    }
}
