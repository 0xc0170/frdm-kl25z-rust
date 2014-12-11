# Copyright (c) 2014, Martin Kojtal (0xc0170)
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

CC      := arm-none-eabi-gcc
AR      := arm-none-eabi-ar
LD      := arm-none-eabi-ld
RUSTC   := rustc
RUSTPKG := rustpkg

OPT  := 2
ARCH := thumbv6m
CPU  := cortex-m0

OBJCPY = arm-none-eabi-objcopy

RUSTFLAGS = -L . --target $(ARCH)-none-eabi -C target-cpu=$(CPU) 
RUSTFLAGS += -C relocation-model=static
RUSTFLAGS += --opt-level $(OPT) -g -Z no-landing-pads 
RUSTFLAGS += -A dead_code -A unused_variables 

LDFLAGS     = -T kl25z.ld
LDFLAGS    += -Map=frdm-kl25z-blinky.map
LDFLAGS    += --gc-sections #-print-gc-sections

.SUFFIXES: .o .rs .c

all: frdm-kl25z-blinky.elf frdm-kl25z-blinky.bin

.rs.o:
	$(RUSTC) $(RUSTFLAGS) --emit obj -o $@ $<

frdm-kl25z-blinky.elf: kl25z.ld main.o
	$(LD) $(LDFLAGS) main.o -o $@

frdm-kl25z-blinky.bin: frdm-kl25z-blinky.elf
	$(OBJCPY) -O binary $< $@

libcore: libcore.rlib
	$(RUSTC) $(RUSTFLAGS) rust/src/libcore/lib.rs

.PHONY: clean

clean:
	rm -f *.o frdm-kl25z-blinky.bin frdm-kl25z-blinky.elf
