// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use linefeed::{Interface, ReadResult};
use std::error::Error;
use std::io::stdout;
use ucc::interp::Interp;

fn main() -> Result<(), Box<dyn Error>> {
    let mut interp = Interp::default();

    println!("Untyped Concatenative Calculus Interpreter (UCCI)");
    println!("Type \":help\" to see the available commands.");
    let reader = Interface::new("ucci")?;
    reader.set_prompt(">>> ")?;
    while let ReadResult::Input(input) = reader.read_line()? {
        reader.add_history(input.clone());
        interp.interp(input.as_str(), &mut stdout()).unwrap();
    }
    Ok(())
}
