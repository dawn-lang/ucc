// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::core::*;
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
