// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::interp::*;

#[test]
fn test_interp_interp() {
    let sessions = [
        &[
            (":drop", "Values dropped.\n"),
            (":clear", "Definitions cleared.\n"),
            ("{fn foo = }", "Defined `foo`.\n"),
            (":list", "foo\n"),
            (":show foo", "{fn foo = }\n"),
            ("{fn foo = drop}", "Redefined `foo`.\n"),
            (":show foo", "{fn foo = drop}\n"),
            ("{fn bar = }", "Defined `bar`.\n"),
            (":list", "bar foo\n"),
            (
                "{fn foo = }{fn bar = }",
                "Redefined `foo`.\nRedefined `bar`.\n",
            ),
            (":reset", "Reset.\n"),
            ("[false]", "⟨⟩ [false]\n⇓ ⟨[false]⟩ \n"),
            ("[true]", "⟨[false]⟩ [true]\n⇓ ⟨[false] [true]⟩ \n"),
            (
                "drop\n\n{fn foo =}drop",
                concat!(
                    "⟨[false] [true]⟩ drop\n",
                    "⇓ ⟨[false]⟩ \n",
                    "Defined `foo`.\n",
                    "⟨[false]⟩ drop\n",
                    "⇓ ⟨⟩ \n",
                ),
            ),
        ][..],
        &[
            (
                "[false] [false] and",
                "⟨⟩ [false] [false] and\n⇓ ⟨[false]⟩ \n",
            ),
            ("drop", "⟨[false]⟩ drop\n⇓ ⟨⟩ \n"),
        ][..],
        &[(
            ":trace [false] [false] and",
            concat!(
                "⟨⟩ [false] [false] and\n",
                "⟶ ⟨[false]⟩ [false] and\n",
                "⟶ ⟨[false] [false]⟩ and\n",
                "⟶ ⟨[false] [false]⟩ clone apply\n",
                "⟶ ⟨[false] [false] [false]⟩ apply\n",
                "⟶ ⟨[false] [false]⟩ false\n",
                "⟶ ⟨[false] [false]⟩ swap drop\n",
                "⟶ ⟨[false] [false]⟩ drop\n",
                "⟶ ⟨[false]⟩ \n"
            ),
        )][..],
        &[(":help", HELP)][..],
    ];
    for session in sessions {
        let mut interp = Interp::default();
        let mut output = Vec::default();
        for &(input, expected_output) in session {
            output.clear();
            interp.interp(input, &mut output).unwrap();
            assert_eq!(
                std::str::from_utf8(&output[..]).unwrap(),
                expected_output,
                "Failed on {:?}",
                (input, expected_output)
            );
        }
    }
}
