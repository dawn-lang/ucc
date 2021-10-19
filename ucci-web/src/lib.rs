// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use ucc::non_blocking_interp::NonBlockingInterp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ucci {
    interp: NonBlockingInterp,
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl Ucci {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            interp: NonBlockingInterp::default(),
            buffer: Vec::with_capacity(4096),
        }
    }

    pub fn is_done(&self) -> bool {
        self.interp.is_done()
    }

    pub fn interp_start(&mut self, input: &str, write_output: &js_sys::Function) {
        self.buffer.clear();
        self.interp.interp_start(input, &mut self.buffer).unwrap();
        let output = unsafe { std::str::from_utf8_unchecked(&self.buffer[..]) };
        write_output
            .call1(&JsValue::null(), &JsValue::from(output))
            .unwrap();
    }

    pub fn interp_step(&mut self, write_output: &js_sys::Function) {
        self.buffer.clear();
        self.interp.interp_step(&mut self.buffer).unwrap();
        let output = unsafe { std::str::from_utf8_unchecked(&self.buffer[..]) };
        write_output
            .call1(&JsValue::null(), &JsValue::from(output))
            .unwrap();
    }
}
