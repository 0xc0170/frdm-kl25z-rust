// Copyright 2014 Martin Kojtal (0xc0170)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]
#![crate_type = "rlib"]
#![feature(globs, lang_items, asm)]

#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate core;

pub mod kl25z_map;
pub mod io;
pub mod init;

#[no_mangle]
pub fn main() 
{
    use kl25z_map::*;

    let sim = Sim::get();
    sim.scgc5.bitwise_inc_or(0x400);
    let portb = Port::get(2);
    portb.pcr[18].set(1 << 8);

    let ptb = Gpio::get(2);
    ptb.pddr.set(1 << 18);

    ptb.psor.set(1 << 18);

    loop {}
}
