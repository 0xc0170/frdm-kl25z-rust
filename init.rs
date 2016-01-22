// Copyright 2014-2016 Martin Kojtal (0xc0170)
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

// extern crate core;

use kl25z_map::Sim;
use kl25z_map::Port;
use kl25z_map::Osc0;
use kl25z_map::Mcg;

use core::option::*;

extern {
    fn __StackTop();

    static     __etext : u32;
    static mut __data_start__  : u32;
    static mut __data_end__ : u32;
    static mut __bss_start__   : u32;
    static mut __bss_end__  : u32;
}


#[link_section=".vector_table"]
pub static ISR_VECTOR: [Option<unsafe extern fn()>; 16] = [
  Option::Some(__StackTop),
  Option::Some(reset_handler as unsafe extern fn()),
  Option::Some(nmi_handler as unsafe extern fn()),
  Option::Some(hardfault_handler as unsafe extern fn()),
  Option::None,
  Option::None,
  Option::None,
  Option::None,
  Option::None,
  Option::None,
  Option::None,
  Option::Some(svc_handler as unsafe extern fn()),
  Option::None,
  Option::None,
  Option::Some(pendsv_handler as unsafe extern fn()),
  Option::Some(systick_handler as unsafe extern fn()),
];

#[link_section=".flash_configuration"]
pub static FLASH_CONFIG_FIELD: [u32; 4] = [
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFE,
];

#[no_mangle]
pub unsafe extern "C" fn reset_handler() {
    let sim = Sim::get();
    sim.copc.set(0x0);

    let mut bss  = &mut __bss_start__  as *mut u32;
    let ebss = &mut __bss_end__ as *mut u32;

    while bss < ebss {
        *bss = 0;
        bss = ((bss as u32) + 4) as *mut u32;
    }

    let mut data  = &mut __data_start__  as *mut u32;
    let     edata = &mut __data_end__ as *mut u32;
    let mut etext = &    __etext as *const u32;

    while data < edata {
        *data = *etext;
        data  = ((data as u32)  + 4) as *mut u32;
        etext = ((etext as u32) + 4) as *const u32;
    }

    system_init();
    ::main();
}

#[no_mangle]
pub unsafe extern "C" fn nmi_handler() {
    abort();
}

#[no_mangle]
pub unsafe extern "C" fn hardfault_handler() {
    abort();
}

#[no_mangle]
pub unsafe extern "C" fn svc_handler() {
    abort();
}

#[no_mangle]
pub unsafe extern "C" fn pendsv_handler() {
    abort();
}

#[no_mangle]
pub unsafe extern "C" fn systick_handler() {
    abort();
}

pub fn system_init()
{
    let sim = Sim::get();
    sim.scgc5.bitwise_inc_or(0x0200);
    sim.clkdiv1.set(0x10010000);
    let porta = Port::get(0);
    porta.pcr[18].bitwise_and(!0x01000700u32);
    porta.pcr[19].bitwise_and(!0x01000700u32);
    let osc0 = Osc0::get();
    osc0.cr.set(0x89);
    let mcg = Mcg::get();
    mcg.c2.set(0x24);
    mcg.c1.set(0x9A);
    mcg.c4.bitwise_and_u8(!0xE0);
    mcg.c5.set(0x1);
    mcg.c6.set(0x0);
    while (mcg.s.get() & 0x10) != 0x0 {};
    while (mcg.s.get() & 0x0C) != 0x08 {};
    mcg.c6.set(0x40);
    while (mcg.s.get() & 0x0C) != 0x08 {};
    while (mcg.s.get() & 0x40) == 0x00 {};
    mcg.c1.set(0x1A);
    while (mcg.s.get() & 0x0C) != 0x0C {};
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {
    abort();
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() {
    abort();
}

#[no_mangle]
pub extern fn abort() -> ! {
    loop {}
}
