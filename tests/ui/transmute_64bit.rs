// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(tool_lints)]

//ignore-x86
//no-ignore-x86_64



#[warn(clippy::wrong_transmute)]
fn main() {
    unsafe {
        let _: *const usize = std::mem::transmute(6.0f64);

        let _: *mut usize = std::mem::transmute(6.0f64);
    }
}
