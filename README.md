
# Experiments with Rust on the xtensa architecture


## Building

### Requirements

    - xargo or cargo xbuild
    - xtensa-esp32-elf toolchain must be in your `$PATH`

First you will need to build `rustc` and `llvm` you can find rough instructions [here](https://gist.github.com/MabezDev/26e175790f84f2f2b0f9bca4e63275d1).

## Starting a new project

`$ cargo generate --git https://github.com/MabezDev/xtensa-rust-quickstart`

To clone this repo and use it as a template.

## Workflow



## Resources

- The [esp-rs](https://github.com/esp-rs) organization has been formed to develop runtime, pac and hal crates for the esp32 and eventually esp8266.
- Checkout @lexxvir's [project](https://github.com/lexxvir/esp32-hello) for an example of using the esp-idf bindings in a Rust application.

## FAQ

- `error: intermittent IO error while iterating directory` - try creating that directory
- `undefined reference to .L` see [this issue](https://github.com/MabezDev/xtensa-rust-quickstart/issues/1)