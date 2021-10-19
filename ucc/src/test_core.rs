// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::builtin::FN_DEF_SRCS;
use crate::core::*;
use crate::display::*;
use crate::parse::*;

#[test]
fn test_small_step() {
    let cases = [
        "⟨[e1] [e2]⟩ swap ⟶ ⟨[e2] [e1]⟩",
        "⟨[e1]⟩ clone ⟶ ⟨[e1] [e1]⟩",
        "⟨[e1]⟩ drop ⟶ ⟨⟩",
        "⟨[e1]⟩ quote ⟶ ⟨[[e1]]⟩",
        "⟨[e1] [e2]⟩ compose ⟶ ⟨[e1 e2]⟩",
        "⟨[e1]⟩ apply ⟶ ⟨⟩ e1",
    ];
    for case in cases {
        let mut ctx = Context::default();
        let mut ssa = SmallStepAssertionParser::new()
            .parse(&mut ctx.interner, case)
            .unwrap();
        let result = ctx.small_step(&mut ssa.0, &mut ssa.1);
        assert_eq!(result, Ok(()), "Failed on {}", case);
        assert_eq!(ssa.0, ssa.2, "Failed on {}", case);
        assert_eq!(ssa.1, ssa.3, "Failed on {}", case);
    }
}

#[test]
fn test_define_fn() {
    let mut ctx = Context::default();
    let sym = Symbol(ctx.interner.get_or_intern_static("foo"));
    let fn_def1 = FnDefParser::new()
        .parse(&mut ctx.interner, "{fn foo = e1}")
        .unwrap();
    let e1 = ExprParser::new().parse(&mut ctx.interner, "e1").unwrap();
    let fn_def2 = FnDefParser::new()
        .parse(&mut ctx.interner, "{fn foo = e2}")
        .unwrap();
    let e2 = ExprParser::new().parse(&mut ctx.interner, "e2").unwrap();
    assert_eq!(ctx.fns.get(&sym), None);
    assert_eq!(ctx.define_fn(fn_def1), None);
    assert_eq!(ctx.fns.get(&sym), Some(&e1));
    assert_eq!(ctx.define_fn(fn_def2), Some(FnDef(sym, e1)));
    assert_eq!(ctx.fns.get(&sym), Some(&e2));
}

#[test]
fn test_big_step() {
    let cases = [
        "⟨[e1] [e2]⟩ swap swap ⇓ ⟨[e1] [e2]⟩",
        "⟨[v1] [v2]⟩ true ⇓ ⟨[v1]⟩",
        "⟨[v1] [v2]⟩ false ⇓ ⟨[v2]⟩",
        "⟨[false] [false]⟩ and ⇓ ⟨[false]⟩",
        "⟨[false] [true]⟩ and ⇓ ⟨[false]⟩",
        "⟨[true] [false]⟩ and ⇓ ⟨[false]⟩",
        "⟨[true] [true]⟩ and ⇓ ⟨[true]⟩",
        "⟨[v1] [v2]⟩ quote2 ⇓ ⟨[[v1] [v2]]⟩",
        "⟨[v1] [v2] [v3]⟩ quote3 ⇓ ⟨[[v1] [v2] [v3]]⟩",
        "⟨[v1] [v2] [v3]⟩ rotate3 ⇓ ⟨[v2] [v3] [v1]⟩",
        "⟨[v1] [v2] [v3] [v4]⟩ rotate4 ⇓ ⟨[v2] [v3] [v4] [v1]⟩",
        "⟨[e]⟩ n0 ⇓ ⟨⟩",
        "⟨[e]⟩ n1 ⇓ ⟨⟩ e",
        "⟨[e]⟩ n2 ⇓ ⟨⟩ e e",
        "⟨[e]⟩ n3 ⇓ ⟨⟩ e e e",
        "⟨[e]⟩ n4 ⇓ ⟨⟩ e e e e",
        "⟨[n0]⟩ succ ⇓ ⟨[[clone] n0 [compose] n0 apply]⟩",
        "⟨[n1]⟩ succ ⇓ ⟨[[clone] n1 [compose] n1 apply]⟩",
        "⟨[e] [n0]⟩ apply ⇓ ⟨⟩",
        "⟨[e] [n0]⟩ succ apply ⇓ ⟨⟩ e",
        "⟨[e] [n0]⟩ succ succ apply ⇓ ⟨⟩ e e",
        "⟨[e] [n0] [n0]⟩ add apply ⇓ ⟨⟩",
        "⟨[e] [n0] [n1]⟩ add apply ⇓ ⟨⟩ e",
        "⟨[e] [n1] [n0]⟩ add apply ⇓ ⟨⟩ e",
        "⟨[e] [n1] [n1]⟩ add apply ⇓ ⟨⟩ e e",
        "⟨[e] [n1] [n2]⟩ add apply ⇓ ⟨⟩ e e e",
        "⟨[e] [n2] [n1]⟩ add apply ⇓ ⟨⟩ e e e",
        "⟨[e] [n2] [n2]⟩ add apply ⇓ ⟨⟩ e e e e",
        "⟨[e] [n0] [n0]⟩ mul apply ⇓ ⟨⟩",
        "⟨[e] [n0] [n1]⟩ mul apply ⇓ ⟨⟩",
        "⟨[e] [n1] [n0]⟩ mul apply ⇓ ⟨⟩",
        "⟨[e] [n1] [n1]⟩ mul apply ⇓ ⟨⟩ e",
        "⟨[e] [n1] [n2]⟩ mul apply ⇓ ⟨⟩ e e",
        "⟨[e] [n2] [n1]⟩ mul apply ⇓ ⟨⟩ e e",
        "⟨[e] [n2] [n2]⟩ mul apply ⇓ ⟨⟩ e e e e",
    ];
    let mut ctx = Context::default();
    for fn_def_src in FN_DEF_SRCS.iter() {
        let fn_def = FnDefParser::new()
            .parse(&mut ctx.interner, fn_def_src)
            .unwrap();
        assert_eq!(ctx.define_fn(fn_def), None);
    }
    for case in cases {
        println!("\n{}", case);
        let mut ssa = BigStepAssertionParser::new()
            .parse(&mut ctx.interner, case)
            .unwrap();
        'eval: loop {
            assert_eq!(
                ctx.small_step(&mut ssa.0, &mut ssa.1),
                Ok(()),
                "Failed on {}",
                case
            );
            println!(
                "⟶ {} {}",
                ssa.0.resolve(&ctx.interner),
                ssa.1.resolve(&ctx.interner)
            );
            if ssa.0 == ssa.2 && ssa.1 == ssa.3 {
                break 'eval;
            }
        }
    }
}
