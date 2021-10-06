// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use linefeed::{Interface, ReadResult};
use std::error::Error;
use ucc::interp::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut interp = Interp::default();
    let mut out_buf = String::default();

    println!("Untyped Concatenative Calculus Interpreter (UCCI)");
    println!("Type \":help\" to see the available commands.");
    let reader = Interface::new("ucci")?;
    reader.set_prompt("ucci> ")?;
    while let ReadResult::Input(input) = reader.read_line()? {
        reader.add_history(input.clone());
        out_buf.clear();
        interp.interp(input.as_str(), &mut out_buf).unwrap();
        println!("{}", out_buf);
    }
    Ok(())
}
