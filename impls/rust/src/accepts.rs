use crate::type_::{class_of, Type};

/// The three-valued acceptance verdict.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Verdict {
    Yes,
    No,
    Maybe,
}

/// Answer, in three values, "does passing actual where expected is wanted fit?"
/// If untyped is involved, the answer is Maybe (the core of gradual typing).
pub fn accepts(expected: &Type, actual: &Type) -> Verdict {
    if matches!(expected, Type::Dynamic) || matches!(actual, Type::Dynamic) {
        return Verdict::Maybe;
    }

    // actual is a Union: Yes only when every member passes. Take the weakest conclusion.
    if let Type::Union(members) = actual {
        return weakest(members.iter().map(|m| accepts(expected, m)));
    }

    // expected is a Union: matching any one is enough. Take the strongest conclusion.
    if let Type::Union(members) = expected {
        return strongest(members.iter().map(|m| accepts(m, actual)));
    }

    // Naive subtyping: do the classes match?
    if class_of(expected) == class_of(actual) {
        Verdict::Yes
    } else {
        Verdict::No
    }
}

/// No if any :no; otherwise Maybe if any :maybe; otherwise Yes.
fn weakest(mut results: impl Iterator<Item = Verdict>) -> Verdict {
    let mut seen_maybe = false;
    for v in &mut results {
        match v {
            Verdict::No => return Verdict::No,
            Verdict::Maybe => seen_maybe = true,
            Verdict::Yes => {}
        }
    }
    if seen_maybe { Verdict::Maybe } else { Verdict::Yes }
}

/// Yes if any :yes; otherwise Maybe if any :maybe; otherwise No.
fn strongest(mut results: impl Iterator<Item = Verdict>) -> Verdict {
    let mut seen_maybe = false;
    for v in &mut results {
        match v {
            Verdict::Yes => return Verdict::Yes,
            Verdict::Maybe => seen_maybe = true,
            Verdict::No => {}
        }
    }
    if seen_maybe { Verdict::Maybe } else { Verdict::No }
}
