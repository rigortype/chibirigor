/// 型キャリア。Ruby 実装の `Chibirigor::Type` に対応。
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// リテラル値そのものを表す型。例: Const(Value::Int(1))
    Const(Value),
    /// 名前付きクラス。例: Nominal("Integer")
    Nominal(String),
    /// 「知らない」型。gradual typing の要。
    Dynamic,
    /// Union 型。例: Integer | String
    Union(Vec<Type>),
    /// ハッシュの構造。キーごとの型を覚える。例: {foo: 1, bar: "a"}
    HashShape(Vec<(String, Type)>),
    /// 配列を「位置ごとの型」で覚える。例: [1, "a"]
    Tuple(Vec<Type>),
}

/// Const 型が保持できる値の種類（Ruby の主なリテラルに対応）。
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

/// 複数の型を 1 つに畳む。入れ子 Union をならし、重複を消す。
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

/// 型を Ruby クラス名に丸める。Dispatch テーブルのキー照合に使う。
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
