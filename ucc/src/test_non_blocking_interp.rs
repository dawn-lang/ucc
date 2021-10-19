// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::interp::HELP;
use crate::non_blocking_interp::NonBlockingInterp;

#[test]
fn test_non_blocking_interp() {
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
        &[
            (
                "[n0] succ",
                "⟨⟩ [n0] succ\n⇓ ⟨[n1]⟩ \n",
            ),
        ][..],
        &[
            (
                "[n0] [n1] add",
                "⟨⟩ [n0] [n1] add\n⇓ ⟨[n1]⟩ \n",
            ),
        ][..],
        &[
            (
                "[n1] [n1] add",
                "⟨⟩ [n1] [n1] add\n⇓ ⟨[n2]⟩ \n",
            ),
        ][..],
        &[
            (
                "[n1] [n1] mul",
                "⟨⟩ [n1] [n1] mul\n⇓ ⟨[n1]⟩ \n",
            ),
        ][..],
        &[
            (
                "[n2] [n2] mul",
                "⟨⟩ [n2] [n2] mul\n⇓ ⟨[n4]⟩ \n",
            ),
        ][..],
        &[
            (
                "[true] foo",
                "⟨⟩ [true] foo\n⇓ ⟨[true]⟩ foo\nUndefinedFn(\"foo\")\n",
            ),
        ][..],
        &[
            (
                ":trace [true] foo",
                "⟨⟩ [true] foo\n⟶ ⟨[true]⟩ foo\nUndefinedFn(\"foo\")\n",
            ),
        ][..],
    ];
    let mut buffer = Vec::with_capacity(4096);
    for session in sessions {
        let mut interp = NonBlockingInterp::default();
        for &(input, expected_output) in session {
            buffer.clear();
            interp.interp_start(input, &mut buffer).unwrap();
            while !interp.is_done() {
                interp.interp_step(&mut buffer).unwrap();
            }
            let output = unsafe { std::str::from_utf8_unchecked(&buffer[..]) };
            assert_eq!(
                output,
                expected_output,
                "Failed on {:?}",
                (input, expected_output)
            );
        }
    }
}
