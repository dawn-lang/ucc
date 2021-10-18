// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use ucc::interp::Interp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ucci(Interp);

#[wasm_bindgen]
impl Ucci {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Interp::default())
    }

    pub fn interp(&mut self, input: &str, write_output: &js_sys::Function) {
        let mut buffer = Vec::new();
        self.0.interp(input, &mut buffer).unwrap();
        let output = unsafe { std::str::from_utf8_unchecked(&buffer[..]) };
        write_output
            .call1(&JsValue::null(), &JsValue::from(output))
            .unwrap();
    }
}
