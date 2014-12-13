Blinky demo for FRDM-KL25Z
===============

Simple blinky demo for FRDM KL25Z written in Rust. Red LED should be blinking.
I have tested it with the rust nightly (rustc 0.13.0-nightly (193390d0e 2014-12-11 22:56:54 +0000)).

To be able to run this demo, libcore library is needed. Clone the rust repository
from github, and run make libcore (rust should be in the same directory as this demo).

```
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
