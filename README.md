Blinky demo for FRDM-KL25Z
===============

Simple blinky demo for FRDM KL25Z written in Rust. Red LED should be blinking.
I have tested it with the rust nightly (rustc 1.4.0-nightly (5aca49c69 2015-08-10)). I would like to use rust stable, but currently it does not support no_std, asm and other features for this example.

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

The size of this demo with optimization set to 1:
```
   text    data     bss     dec     hex filename
   1056       0       0    1056     420 frdm-kl25z-blinky.elf
   1056       0       0    1056     420 (TOTALS)
```

EnjoY
