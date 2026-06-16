use std::collections::HashMap;
use crate::type_::Type;

/// An immutable map from variable name → type. Adding a binding returns a new Scope.
#[derive(Debug, Clone, Default)]
pub struct Scope(HashMap<String, Type>);

impl Scope {
    pub fn new() -> Self {
        Scope(HashMap::new())
    }

    /// The type for that name (None if unbound).
    pub fn local(&self, name: &str) -> Option<&Type> {
        self.0.get(name)
    }

    /// A new scope with one binding added.
    pub fn with_local(&self, name: impl Into<String>, ty: Type) -> Scope {
        let mut map = self.0.clone();
        map.insert(name.into(), ty);
        Scope(map)
    }
}
