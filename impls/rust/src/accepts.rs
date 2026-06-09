use crate::type_::{class_of, Type};

/// 受理判定の三値。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Verdict {
    Yes,
    No,
    Maybe,
}

/// 「expected の所に actual を渡して合うか」を三値で答える。
/// untyped が絡んだら Maybe（gradual の核）。
pub fn accepts(expected: &Type, actual: &Type) -> Verdict {
    if matches!(expected, Type::Dynamic) || matches!(actual, Type::Dynamic) {
        return Verdict::Maybe;
    }

    // actual が Union：全メンバが通って初めて Yes。一番弱い結論を採る。
    if let Type::Union(members) = actual {
        return weakest(members.iter().map(|m| accepts(expected, m)));
    }

    // expected が Union：どれか 1 つに合えばよい。一番強い結論を採る。
    if let Type::Union(members) = expected {
        return strongest(members.iter().map(|m| accepts(m, actual)));
    }

    // 素朴な部分型：クラスが一致するか。
    if class_of(expected) == class_of(actual) {
        Verdict::Yes
    } else {
        Verdict::No
    }
}

/// :no があれば No、無ければ :maybe があれば Maybe、でなければ Yes。
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

/// :yes があれば Yes、無ければ :maybe があれば Maybe、でなければ No。
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
