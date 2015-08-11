// Copyright 2014-2015 Martin Kojtal (0xc0170)
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

use core::intrinsics::{volatile_load, volatile_store};
use core::cell::UnsafeCell;

pub struct VolatileRW<T> {
    value: UnsafeCell<T>,
}

impl<T> VolatileRW<T> {
    #[inline]
    pub fn get(&self) -> T {
        unsafe {
            volatile_load(self.value.get() as *const T)
        }
    }

    #[inline]
    pub fn set(&self, value: T) {
        unsafe {
            volatile_store(self.value.get(), value);
        }
    }
}

impl VolatileRW<u32> {
    #[inline]
    pub fn bitwise_inc_or(&self, value: u32) {
        let read = self.get();
        self.set(read | value);
    }

    #[inline]
    pub fn bitwise_and(&self, value: u32) {
        let read = self.get();
        self.set(read & value);
    }
}

impl VolatileRW<u8> {
    pub fn bitwise_inc_or_u8(&self, value: u8) {
        let read = self.get();
        self.set(read | value);
    }

    #[inline]
    pub fn bitwise_and_u8(&self, value: u8) {
        let read = self.get();
        self.set(read & value);
    }
}

pub struct VolatileR<T> {
    value: UnsafeCell<T>,
}

impl<T> VolatileR<T> {
    #[inline]
    pub fn get(&self) -> T {
        unsafe {
            volatile_load(self.value.get() as *const T)
        }
    }
}
