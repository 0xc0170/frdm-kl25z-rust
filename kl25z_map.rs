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

use io::VolatileRW;

const BASE_SIM : u32 = 0x40047000;

pub struct Sim {
    pub sopt1 : VolatileRW<u32>,
    pub sopt1cfg : VolatileRW<u32>,
    pub reserved_0 : [u8, ..4092],
    pub sopt2 : VolatileRW<u32>,
    pub reserved_1 : [u8, ..4],
    pub sopt4 : VolatileRW<u32>,
    pub sopt5 : VolatileRW<u32>,
    pub reserved_2 : [u8, ..4],
    pub sopt7 : VolatileRW<u32>,
    pub reserved_3 : [u8, ..8],
    pub sdid : VolatileRW<u32>,
    pub reserved_4 : [u8, ..12],
    pub scgc4 : VolatileRW<u32>,
    pub scgc5 : VolatileRW<u32>,
    pub scgc6 : VolatileRW<u32>,
    pub scgc7 : VolatileRW<u32>,
    pub clkdiv1 : VolatileRW<u32>,
    pub reserved_5 : [u8, ..4],
    pub fcfg1 : VolatileRW<u32>,
    pub fcfg2 : VolatileRW<u32>,
    pub reserved_6 : VolatileRW<u32>,
    pub uidmh : VolatileRW<u32>,
    pub uidml : VolatileRW<u32>,
    pub uidl : VolatileRW<u32>,
    pub reserved_7 : [u8, ..156],
    pub copc : VolatileRW<u32>,
    pub srvcop : VolatileRW<u32>,
}

impl Sim {
    pub fn get() -> &'static Sim {
        unsafe {
            &*(BASE_SIM as *const Sim)
        }
    }
}

const BASE_PORTA : u32 = 0x40049000;

pub struct Port {
    pub pcr : [VolatileRW<u32>, ..32],
    pub gpclr : VolatileRW<u32>,
    pub gpchr : VolatileRW<u32>,
    pub reserved_0 : [u8, ..24],
    pub isfr : VolatileRW<u32>,
}

impl Port {
    pub fn get(port: u32) -> &'static Port {
        unsafe {
            &*((BASE_PORTA + (port*0x1000)) as *const Port)
        }
    }
}

const BASE_MCG : u32 = 0x40064000;

pub struct Mcg {
    pub c1 : VolatileRW<u8>,
    pub c2 : VolatileRW<u8>,
    pub c3 : VolatileRW<u8>,
    pub c4 : VolatileRW<u8>,
    pub c5 : VolatileRW<u8>,
    pub c6 : VolatileRW<u8>,
    pub s : VolatileRW<u8>,
    pub reserved_0 : [u8, ..1],
    pub sc : VolatileRW<u8>,
    pub reserved_1 : [u8, ..1],
    pub atcvh : VolatileRW<u8>,
    pub atcvl : VolatileRW<u8>,
    pub c7 : VolatileRW<u8>,
    pub c8 : VolatileRW<u8>,
    pub c9 : VolatileRW<u8>,
    pub c10 : VolatileRW<u8>,
}

impl Mcg {
    pub fn get() -> &'static Mcg {
        unsafe {
            &*(BASE_MCG as *const Mcg)
        }
    }
}

const BASE_OSC0 : u32 = 0x40065000;

pub struct Osc0 {
    pub cr : VolatileRW<u8>,
}

impl Osc0 {
    pub fn get() -> &'static Osc0 {
        unsafe {
            &*(BASE_OSC0 as *const Osc0)
        }
    }
}

const BASE_PTA :u32 = 0x400FF040;

pub struct Gpio {
    pub pdor : VolatileRW<u32>,
    pub psor : VolatileRW<u32>,
    pub pcor : VolatileRW<u32>,
    pub ptor : VolatileRW<u32>,
    pub pdir : VolatileRW<u32>,
    pub pddr : VolatileRW<u32>,
}

impl Gpio {
    pub fn get(port : u32) -> &'static Gpio {
        unsafe {
            &*((BASE_PTA + (port*0x40)) as *const Gpio)
        }
    }
}
