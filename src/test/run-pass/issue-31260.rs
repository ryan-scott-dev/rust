// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct Struct<K: 'static> {
    pub field: K,
}

// Partial fix for #31260, doesn't work without {...}.
static STRUCT: Struct<&'static [u8]> = Struct {
    field: {&[1]}
};

fn main() {}
