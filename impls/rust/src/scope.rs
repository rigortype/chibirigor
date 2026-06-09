use std::collections::HashMap;
use crate::type_::Type;

/// 変数名 → 型 の不変マップ。束縛を足すと新しい Scope を返す。
#[derive(Debug, Clone, Default)]
pub struct Scope(HashMap<String, Type>);

impl Scope {
    pub fn new() -> Self {
        Scope(HashMap::new())
    }

    /// その名前の型（未束縛なら None）。
    pub fn local(&self, name: &str) -> Option<&Type> {
        self.0.get(name)
    }

    /// 束縛を 1 つ足した新しいスコープ。
    pub fn with_local(&self, name: impl Into<String>, ty: Type) -> Scope {
        let mut map = self.0.clone();
        map.insert(name.into(), ty);
        Scope(map)
    }
}
