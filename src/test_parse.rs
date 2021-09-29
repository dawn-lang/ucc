// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::core::*;
use crate::parse::*;

#[test]
fn test_parse_fn_def() {
    let interner = &mut Interner::default();
    let cases = &[("{fn foo = }", "foo", "")];
    for (fn_def_src, sym_src, e_src) in cases {
        let fn_def = FnDefParser::new().parse(interner, fn_def_src).unwrap();
        let sym = Symbol(interner.get_or_intern_static(sym_src));
        let e = ExprParser::new().parse(interner, e_src).unwrap();
        assert_eq!(fn_def, FnDef(sym, e), "{}", fn_def_src);
    }
}

#[test]
fn test_parse_expr_empty() {
    let interner = &mut Interner::default();
    let input = "";
    let e = ExprParser::new().parse(interner, input).unwrap();
    assert_eq!(e, Expr::Empty);
}

#[test]
fn test_parse_expr_intrinsic() {
    let cases = [
        ("swap", Expr::Intrinsic(Intrinsic::Swap)),
        ("clone", Expr::Intrinsic(Intrinsic::Clone)),
        ("drop", Expr::Intrinsic(Intrinsic::Drop)),
        ("quote", Expr::Intrinsic(Intrinsic::Quote)),
        ("compose", Expr::Intrinsic(Intrinsic::Compose)),
        ("apply", Expr::Intrinsic(Intrinsic::Apply)),
    ];
    for (e_src, e_expected) in cases {
        let interner = &mut Interner::default();
        let e = ExprParser::new().parse(interner, e_src).unwrap();
        assert_eq!(e, e_expected);
    }
}

#[test]
fn test_parse_expr_call() {
    let interner = &mut Interner::default();
    let input = "foo";
    let e = ExprParser::new().parse(interner, input).unwrap();
    assert_eq!(e, Expr::Call(Symbol(interner.get("foo").unwrap())));
}

#[test]
fn test_parse_expr_call2() {
    let interner = &mut Interner::default();
    let inputs = &["foo bar", "(foo bar)", "((foo bar))"];
    for input in inputs {
        let e = ExprParser::new().parse(interner, input).unwrap();
        let e2 = Expr::Compose(
            Box::new(Expr::Call(Symbol(interner.get("foo").unwrap()))),
            Box::new(Expr::Call(Symbol(interner.get("bar").unwrap()))),
        );
        assert_eq!(e, e2);
    }
}

#[test]
fn test_parse_expr_quote_call() {
    let interner = &mut Interner::default();
    let inputs = &["[foo]", "[(foo)]", "[((foo))]"];
    for input in inputs {
        let e = ExprParser::new().parse(interner, input).unwrap();
        let e2 = Expr::Quote(Box::new(Expr::Call(Symbol(interner.get("foo").unwrap()))));
        assert_eq!(e, e2);
    }
}

#[test]
fn test_parse_expr_quote_call2() {
    let interner = &mut Interner::default();
    let inputs = &["[foo bar]", "[(foo bar)]", "[((foo bar))]"];
    for input in inputs {
        let e = ExprParser::new().parse(interner, input).unwrap();
        let e2 = Expr::Quote(Box::new(Expr::Compose(
            Box::new(Expr::Call(Symbol(interner.get("foo").unwrap()))),
            Box::new(Expr::Call(Symbol(interner.get("bar").unwrap()))),
        )));
        assert_eq!(e, e2);
    }
}
