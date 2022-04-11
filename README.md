
# Rust on the xtensa architecture

⚠️ **NOTE**: This repo is now obsolete, there is now have a quickstart repo for [bare-metal](https://github.com/esp-rs/esp-template) and a template for [using the standard library](https://github.com/esp-rs/esp-idf-template).

Need help? Join the esp-rs room on matrix, https://matrix.to/#/#esp-rs:matrix.org.

## Supported chips

|name|arch|rust-target| custom compiler required |
|-|-|-|-|
|esp32|Xtensa|`xtensa-esp32-none-elf`| yes |
|esp8266|Xtensa|`xtensa-esp8266-none-elf`| yes |

## Installing the compiler

For Xtensa targets a forked compiler must be installed, follow the instructions from [the book](https://esp-rs.github.io/book/getting-started/installing-rust.html).

## Installing tools

Currently the xtensa targets do not have LLD support. Therefore the GCC toolchain is required for linking.

### xtensa-esp32-elf toolchain for esp32 development

Instructions can be found [on Espressif's web site](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started-legacy/linux-setup.html#toolchain-setup) and the latest tool toolchain can be downloaded from [here](https://github.com/espressif/crosstool-NG/releases).

Extract it to the directory of your choice. Then add the toolchain's bin/ directory to your `$PATH`. For example:

    $ mkdir ~/esp
    $ tar -xzf ~/Downloads/xtensa-esp32-elf-gcc8_4_0-esp-2020r3-linux-amd64.tar.gz -C ~/esp
    $ PATH="$PATH:$HOME/esp/xtensa-esp32-elf/bin"

Old instructions can be found [on Espressif's web site](https://docs.espressif.com/projects/esp-idf/en/release-v3.0/get-started/linux-setup.html).

### xtensa-lx106-elf toolchain for esp8266 development
Install the xtensa-lx106-elf toolchain from the [espressif web site](https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/linux-setup.html).

```
$ mkdir ~/esp
$ tar -xzf ~/Downloads/xtensa-lx106-elf-linux64-1.22.0-100-ge567ec7-5.2.0.tar.gz -C ~/esp
$ PATH="$PATH:$HOME/esp/xtensa-lx106-elf/bin"
```

## Cargo Build

Since the introduction of the `build-std` feature of cargo, it is possible to build `core` without any additional tools.

## Flashing

Install espflash:

```bash
$ cargo install cargo-espflash
```

Example for the ESP32, remember to use `target = xtensa-esp32-none-elf` inside `.cargo/config`:

```bash
$ cargo espflash --chip esp32 --example esp32 --speed 460800 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal" /dev/ttyUSB0
```

Example for the ESP8266, remember to use `target = xtensa-esp8266-none-elf` inside `.cargo/config`:

```bash
$ cargo espflash --chip esp8266 --example esp8266 --features="xtensa-lx-rt/lx106 xtensa-lx/lx106 esp8266-hal" /dev/ttyUSB0
```

### esptool

The preferred method of flashing is to use
[`cargo-espflash`](https://github.com/icewind1991/espflash), but you can use Espressif's `esptool.py` to flash the binaries manually. Esptool is python-based command line tool for flashing Espressif's chips. Full installation instructions are available on [the website](https://github.com/espressif/esptool).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
