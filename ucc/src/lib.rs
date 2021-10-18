// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod core;
#[cfg(test)]
mod test_core;

pub mod display;

pub mod interp;
#[cfg(test)]
mod test_interp;

pub mod interp_step;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parse);
#[cfg(test)]
mod test_parse;
