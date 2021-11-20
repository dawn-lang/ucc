// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use linefeed::{Interface, ReadResult, Signal};
use std::error::Error;
use std::io::stdout;
use ucc::interp::Interp;

fn main() -> Result<(), Box<dyn Error>> {
    let mut interp = Interp::default();

    println!("Untyped Concatenative Calculus Interpreter (UCCI)");
    println!("Type \":help\" to see the available commands.");
    let reader = Interface::new("ucci")?;
    reader.set_prompt("\n>>> ")?;

    // Report all signals
    reader.set_report_signal(Signal::Break, true);
    reader.set_report_signal(Signal::Continue, true);
    reader.set_report_signal(Signal::Interrupt, true);
    reader.set_report_signal(Signal::Suspend, true);
    reader.set_report_signal(Signal::Quit, true);
    
    while let ReadResult::Input(input) = reader.read_line()? {
        reader.add_history(input.clone());
        interp.interp_start(input.as_str(), &mut stdout()).unwrap();
        while !interp.is_done() {
            interp.interp_step(&mut stdout()).unwrap();
        }
    }
    Ok(())
}
