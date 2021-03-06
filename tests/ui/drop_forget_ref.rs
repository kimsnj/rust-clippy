// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(tool_lints)]


#![warn(clippy::drop_ref, clippy::forget_ref)]
#![allow(clippy::toplevel_ref_arg, clippy::similar_names, clippy::needless_pass_by_value)]

use std::mem::{drop, forget};

struct SomeStruct;

fn main() {
    drop(&SomeStruct);
    forget(&SomeStruct);

    let mut owned1 = SomeStruct;
    drop(&owned1);
    drop(&&owned1);
    drop(&mut owned1);
    drop(owned1); //OK
    let mut owned2 = SomeStruct;
    forget(&owned2);
    forget(&&owned2);
    forget(&mut owned2);
    forget(owned2); //OK

    let reference1 = &SomeStruct;
    drop(reference1);
    forget(&*reference1);

    let reference2 = &mut SomeStruct;
    drop(reference2);
    let reference3 = &mut SomeStruct;
    forget(reference3);

    let ref reference4 = SomeStruct;
    drop(reference4);
    forget(reference4);
}

#[allow(dead_code)]
fn test_generic_fn_drop<T>(val: T) {
    drop(&val);
    drop(val); //OK
}

#[allow(dead_code)]
fn test_generic_fn_forget<T>(val: T) {
    forget(&val);
    forget(val); //OK
}

#[allow(dead_code)]
fn test_similarly_named_function() {
    fn drop<T>(_val: T) {}
    drop(&SomeStruct); //OK; call to unrelated function which happens to have the same name
    std::mem::drop(&SomeStruct);
    fn forget<T>(_val: T) {}
    forget(&SomeStruct); //OK; call to unrelated function which happens to have the same name
    std::mem::forget(&SomeStruct);
}
