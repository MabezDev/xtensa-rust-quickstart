
# Experiments with Rust on the xtensa architecture


## Building

First you will need to build `rustc` and `llvm` you can find rough instructions [here](https://gist.github.com/MabezDev/26e175790f84f2f2b0f9bca4e63275d1).

Currently the xtensa llvm fork does not support object generation, hence we must make rustc emit asm, then use the `xtensa-elf-as` to produce our binaries from the assembly -- see this `gen_asm` script for a rough idea of how to do this (not this requires xargo to build libcore on the fly).