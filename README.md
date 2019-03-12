
# Experiments with Rust on the xtensa architecture


## Building

### Requirements

    - xargo
    - xtensa-esp32-elf toolchain must be in your `$PATH`

First you will need to build `rustc` and `llvm` you can find rough instructions [here](https://gist.github.com/MabezDev/26e175790f84f2f2b0f9bca4e63275d1).

Modifiy the `build` script to fix the path of the custom `rustc`, then execute the build script build.

## FAQ

    - `error: intermittent IO error while iterating directory` - try creating that directory