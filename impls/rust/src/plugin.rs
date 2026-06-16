use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::rbs::Signature;

type Registry = HashMap<(String, String), Signature>;

static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();

fn registry() -> &'static Mutex<Registry> {
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Register a method signature from outside (takes precedence over the dispatch table).
pub fn register_method(klass: impl Into<String>, name: impl Into<String>, sig: Signature) {
    registry().lock().unwrap().insert((klass.into(), name.into()), sig);
}

/// Look up a registered signature.
pub fn lookup(klass: &str, name: &str) -> Option<Signature> {
    registry().lock().unwrap().get(&(klass.to_string(), name.to_string())).cloned()
}

/// Reset for tests.
#[cfg(test)]
pub fn reset() {
    registry().lock().unwrap().clear();
}
