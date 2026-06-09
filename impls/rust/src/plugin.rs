use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::rbs::Signature;

type Registry = HashMap<(String, String), Signature>;

static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();

fn registry() -> &'static Mutex<Registry> {
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 外部からメソッドシグネチャを登録する（Dispatch テーブルより優先）。
pub fn register_method(klass: impl Into<String>, name: impl Into<String>, sig: Signature) {
    registry().lock().unwrap().insert((klass.into(), name.into()), sig);
}

/// 登録済みシグネチャを参照する。
pub fn lookup(klass: &str, name: &str) -> Option<Signature> {
    registry().lock().unwrap().get(&(klass.to_string(), name.to_string())).cloned()
}

/// テスト用リセット。
#[cfg(test)]
pub fn reset() {
    registry().lock().unwrap().clear();
}
