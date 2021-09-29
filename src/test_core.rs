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
