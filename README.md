
# Rust on the xtensa architecture

## Building

### Requirements

    - xargo or cargo xbuild
    - xtensa-esp32-elf toolchain must be in your `$PATH`
    - esptool, if targeting the esp32 or esp8266

First you will need to build `rustc` and `llvm` you can find rough instructions [here](https://gist.github.com/MabezDev/26e175790f84f2f2b0f9bca4e63275d1).

## Starting a new project

`$ cargo generate --git https://github.com/MabezDev/xtensa-rust-quickstart`

Requires cargo generate.

## Workflow

Update `setenv` to use your xtensa enabled rustc, then simply run `source setenv`.
From then on, you can just call `xargo build` or use the built in `flash` script to build and flash to the esp.


## Resources

- The [esp-rs](https://github.com/esp-rs) organization has been formed to develop runtime, pac and hal crates for the esp32 and eventually esp8266.
- Checkout @lexxvir's [project](https://github.com/lexxvir/esp32-hello) for an example of using the esp-idf bindings in a Rust application.

## FAQ

- `error: intermittent IO error while iterating directory` - try creating that directory
- `undefined reference to .L` see [this issue](https://github.com/MabezDev/xtensa-rust-quickstart/issues/1)