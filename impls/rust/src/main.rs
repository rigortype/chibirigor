mod accepts;
mod annotator;
mod checker;
mod diagnostic;
mod dispatch;
mod narrowing;
mod plugin;
mod rbs;
mod scope;
mod type_;
mod type_of;

#[cfg(test)]
mod tests {
    use ruby_prism::parse;

    use crate::annotator::annotate;
    use crate::checker::{check, check_with_rbs};
    use crate::diagnostic::Diagnostic;
    use crate::rbs::Signature;
    use crate::scope::Scope;
    use crate::type_::{Type, Value};
    use crate::type_of::type_of;

    fn first_type(source: &[u8]) -> (Type, Vec<Diagnostic>) {
        let result = parse(source);
        let program = result.node();
        let node = program.as_program_node().unwrap().statements().body().iter().next().unwrap();
        let mut diags = vec![];
        let ty = type_of(node, &Scope::new(), &mut diags, source);
        (ty, diags)
    }

    // --- リテラル ---

    #[test]
    fn int_literal() {
        let (ty, _) = first_type(b"42");
        assert_eq!(ty, Type::Const(Value::Int(42)));
    }

    #[test]
    fn string_literal() {
        let (ty, _) = first_type(b"\"hello\"");
        assert_eq!(ty, Type::Const(Value::Str("hello".into())));
    }

    #[test]
    fn symbol_literal() {
        let (ty, _) = first_type(b":foo");
        assert_eq!(ty, Type::Const(Value::Symbol("foo".into())));
    }

    #[test]
    fn hash_literal() {
        let (ty, _) = first_type(b"{foo: 1, bar: \"x\"}");
        assert!(matches!(ty, Type::HashShape(_)));
        if let Type::HashShape(fields) = ty {
            assert_eq!(fields[0].0, "foo");
            assert_eq!(fields[0].1, Type::Const(Value::Int(1)));
        }
    }

    #[test]
    fn array_literal() {
        let (ty, _) = first_type(b"[1, \"a\"]");
        assert_eq!(
            ty,
            Type::Tuple(vec![Type::Const(Value::Int(1)), Type::Const(Value::Str("a".into()))])
        );
    }

    // --- 演算 ---

    #[test]
    fn int_plus_int_folds_const() {
        let (ty, diags) = first_type(b"1 + 2");
        assert_eq!(ty, Type::Const(Value::Int(3))); // 畳まれる
        assert!(diags.is_empty());
    }

    #[test]
    fn string_plus_string_folds() {
        let (ty, _) = first_type(b"\"hello\" + \" world\"");
        assert_eq!(ty, Type::Const(Value::Str("hello world".into())));
    }

    #[test]
    fn int_plus_string_error() {
        let (ty, diags) = first_type(b"1 + \"x\"");
        // 型エラーでも宣言された戻り型（Integer）を返す（Ruby 版と同じ挙動）
        assert_eq!(ty, Type::Nominal("Integer".into()));
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("が必要ですが"));
    }

    #[test]
    fn wrong_arg_count_error() {
        let (_, diags) = first_type(b"1.to_s(42)");
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("引数の数"));
    }

    // --- if / narrowing ---

    #[test]
    fn if_no_else_gives_nil_union() {
        let (ty, _) = first_type(b"if true\n  1\nend");
        match ty {
            Type::Union(ms) => {
                assert!(ms.contains(&Type::Const(Value::Int(1))));
                assert!(ms.contains(&Type::Const(Value::Nil)));
            }
            other => panic!("expected Union, got {other:?}"),
        }
    }

    #[test]
    fn narrowing_nil_check() {
        // x が Integer | nil のとき、if x.nil? 真枝では NilClass に絞れる
        let source = b"x = if true; 1; else; nil; end\nif x.nil?\n  x\nend";
        let diags = check(source, &[]);
        assert!(diags.is_empty());
    }

    // --- checker ---

    #[test]
    fn checker_assignment_flows() {
        let diags = check(b"x = 1\nx + \"bad\"", &[]);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn checker_def_body_checked() {
        let diags = check(b"def foo\n  1 + \"bad\"\nend", &[]);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn checker_baseline_subtracts() {
        let source = b"1 + \"bad\"";
        let diags_full = check(source, &[]);
        assert_eq!(diags_full.len(), 1);
        // 同じ診断を baseline に渡すと差し引かれる
        let diags_sub = check(source, &diags_full);
        assert!(diags_sub.is_empty());
    }

    // --- return type check ---

    #[test]
    fn return_type_ok() {
        let source = b"def foo\n  1\nend";
        let rbs = "class Object\n  def foo: () -> Integer\nend\n";
        let diags = check_with_rbs(source, rbs, &[]);
        assert!(diags.is_empty());
    }

    #[test]
    fn return_type_mismatch() {
        let source = b"def foo\n  \"oops\"\nend";
        let rbs = "class Object\n  def foo: () -> Integer\nend\n";
        let diags = check_with_rbs(source, rbs, &[]);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("Integer"));
    }

    // --- plugin ---

    #[test]
    fn plugin_register() {
        crate::plugin::reset();
        crate::plugin::register_method(
            "Integer",
            "double",
            Signature { params: vec![], returns: Type::Nominal("Integer".into()) },
        );
        let (ty, diags) = first_type(b"1.double");
        assert_eq!(ty, Type::Nominal("Integer".into()));
        assert!(diags.is_empty());
        crate::plugin::reset();
    }

    // --- annotator ---

    #[test]
    fn annotate_literals() {
        let anns = annotate(b"x = 1\nx + 2");
        assert_eq!(anns.len(), 2);
        assert_eq!(anns[0].type_, Type::Const(Value::Int(1)));
        assert_eq!(anns[1].type_, Type::Const(Value::Int(3))); // 畳まれる
    }

    #[test]
    fn annotate_def_signature() {
        let anns = annotate(b"def foo(a)\n  1\nend");
        assert_eq!(anns.len(), 1);
        if let Type::Const(Value::Str(sig)) = &anns[0].type_ {
            assert!(sig.contains("def foo:"));
            assert!(sig.contains("untyped"));
        } else {
            panic!("expected Str signature");
        }
    }
}

fn main() {
    println!("chibirigor Rust 試作 — `cargo test` でテストを実行してください。");
}
