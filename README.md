Blinky demo for FRDM-KL25Z
===============

Simple blinky demo for FRDM KL25Z written in Rust. Red LED should be blinking.
I have tested it with the rust nightly (rustc 1.7.0-nightly (7dce32e65 2016-01-20)). I would like to use rust stable, but currently it does not support asm and other features for this example. Please once asm becomes stable, no_std already is, core should also be, not certain if all types are there, send PR updating this example.

The latest version of the Rust compiler can be installed from http://www.rust-lang.org/install.html

To be able to run this demo, libcore library is needed. Clone the rust repository
from github, and run make libcore (rust should be one directory up as this demo (for libcore this is used: ../rust/src/libcore/lib.rs)). The nightly version
of rustc and rust repo should be in synch, otherwise you might end up with errors. To fix this, go to the rust
repository, this should checkout the version in synch with rustc: git checkout `rustc --version|cut -f 3 -d ' '|sed 's/^.//'`

```
git clone https://github.com/0xc0170/frdm-kl25z-rust.git
git clone https://github.com/rust-lang/rust.git
cd frdm-kl25z-rust
make libcore
```

Then run make, to build this demo.

The size of this demo with optimization set to 1 (using <1.5, the demo had size of 1056, now with the 1.7 it's 1064):
```
   text    data     bss     dec     hex filename
   1064       0       0    1064     428 frdm-kl25z-blinky.elf
   1064       0       0    1064     428 (TOTALS)
```

EnjoY
