
# Rust on the xtensa architecture

Need help? Join the esp-rs room on matrix, https://matrix.to/#/#esp-rs:matrix.org.

## Overview
ESP32 and ESP8266 chips come with one or two Xtensa CPUs, which (unlike x86 or ARM) are not supported by the official Rust compiler.

These instructions describe how to build Rust compiler (from [LLVM](https://en.wikipedia.org/wiki/LLVM) and rustc) that supports Xtensa architecture, as well as  how to build and flash blinking LED program to the ESP32.

## Building Rust compiler
### System Requirements
Building Rust compiler is CPU and memory intense process. The compilation can take up to 15 minutes on higher-grade CPUs and even several hours on resource-starved VMs. Besides that, the compilation may fail on systems with less than 6GB of RAM.

The required software components and resulting artefacts can take up to 10GB of disk space.

Make sure you have a working copy of rust already installed. If not, follow the
[Install Rust](https://www.rust-lang.org/tools/install) instructions.

## Recommended build method

The fork now uses the xtensa enabled fork as its llvm submodule, so its now possible to build the entire toolchain in a few commands.

### Recommended build method - UNIX

All that is required to build rustc for linux is the steps below. 

```bash
$ git clone https://github.com/MabezDev/rust-xtensa
$ cd rust-xtensa
$ ./configure --experimental-targets=Xtensa
$ ./x.py build --stage 2
```
Before cross-compiling an xtensa target, you must set the following vars, which are set in the setenv script in this project:
```
XARGO_RUST_SRC=/path/to/rust-xtensa/library
RUSTC=/path/to/rust-xtensa/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
```

### Recommended build method - Windows

After making sure you have Visual Studio Community and python3 installed, in a cmd.exe run:

```cmd
$ git clone https://github.com/MabezDev/rust-xtensa
$ cd rust-xtensa
$ CALL "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
$ python3 src/bootstrap/configure.py --experimental-targets=Xtensa
$ python3 x.py build
```

Before cross-compiling an xtensa target, you must set the following vars either for your system or within your IDE:
```
XARGO_RUST_SRC=\path\to\rust-xtensa\library
RUSTC=\path\to\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc
```

## Manual llvm build instructions

If you would like to build the llvm fork separately, follow the instructions below.

### llvm-xtensa
If you don't have them already, you'll first have to install `ninja-build` and a C++ compiler (such as `g++`).
Please refer to [LLVM project](https://llvm.org/docs/GettingStarted.html) for more information.

    $ git clone https://github.com/MabezDev/llvm-project
    $ cd llvm-project/llvm
    $ mkdir build
    $ cd build
    $ cmake .. -DLLVM_TARGETS_TO_BUILD="X86" -DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD="Xtensa" -DCMAKE_BUILD_TYPE=Release -G "Ninja"
    $ cmake --build .

### rust-xtensa
Please refer to the [rust-xtensa](https://github.com/MabezDev/rust-xtensa) project for authoratative instructions.

Assuming you built llvm in your home directory:

    $ git clone https://github.com/MabezDev/rust-xtensa
    $ cd rust-xtensa
    $ ./configure --llvm-root=$HOME/llvm-project/llvm/build
    $ ./x.py build

## Installing tools
### xtensa-esp32-elf toolchain for esp32 development

Download the latest toolchain file from [here](https://github.com/espressif/crosstool-NG/releases).

Extract it to the directory of your choice. Then add the toolchain's bin/ directory to your `$PATH`. For example:

    $ mkdir ~/esp
    $ tar -xzf ~/Downloads/xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-8.3.0.tar.gz -C ~/esp
    $ PATH="$PATH:$HOME/esp/xtensa-esp32-elf/bin"

Old instructions can be found [on Espressif's web site](https://docs.espressif.com/projects/esp-idf/en/release-v3.0/get-started/linux-setup.html).

### xtensa-lx106-elf  toolchain for esp8266 development
Install the xtensa-lx106-elf toolchain from the [espressif web site](https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/linux-setup.html).

```
$ mkdir ~/esp
$ tar -xzf ~/Downloads/xtensa-lx106-elf-linux64-1.22.0-100-ge567ec7-5.2.0.tar.gz -C ~/esp
$ PATH="$PATH:$HOME/esp/xtensa-lx106-elf/bin"
```

### Cargo Build

<!---
__NOTE:__ Doesn't work for the ESP32 at the moment: https://github.com/rust-lang/wg-cargo-std-aware/issues/53 

Since the introduction of the `build-std` feature of cargo, it is possible to build `core` without any additional tools.
As this feature develops it will become the default, for now you can try it by adding the following to `.cargo/config`:

```
# build core for the xtensa target
[unstable]
build-std = ["core", "alloc"]
```
-->

<!---
Choose either xargo or cargo xbuild.

    $ cargo install xargo

Note: I could not get this to work.

-->

Run

    $ cargo install cargo-xbuild

### esptool

The preferred method of flashing is to use
[`cargo-espflash`](https://github.com/icewind1991/espflash). Otherwise you will
have to invoke Espressif's `esptool.py` to flash the binaries manually.

```bash
    $ cargo install cargo-espflash
```

Esptool is python-based command line tool for flashing ESP32 and ESP8266 chips.
Full installation instructions are available on [the website](https://github.com/espressif/esptool), but it's usually as straightforward as:

Python alternative:

```bash
    $ pip install esptool
```

## Blinking LED
### Starting a new project

```bash
    $ git clone https://github.com/MabezDev/xtensa-rust-quickstart
```

By default this code blinks using GPIO #2. Edit the code under `examples/esp32.rs` or `examples/esp8266.rs` to change the port as required.

For example:

```
let mut led = pins.gpio13.into_push_pull_output();
```

### Workflow
Update `CUSTOM_RUSTC` in `setenv` to point to the version of rust you compiled earlier. Then load the environment variables with:

    $ source setenv

### Building

Note: Flash operation will also conduct a build.

```bash
cargo xbuild --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal"
```

### Flashing

Example for the ESP32, remember to use `target = xtensa-esp32-none-elf` inside `.cargo/config`:

```bash
$ cargo espflash --chip esp32 --example esp32 --speed 460800 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal" /dev/ttyUSB0
```

Example for the ESP8266, remember to use `target = xtensa-esp8266-none-elf` inside `.cargo/config`:

```bash
$ cargo espflash --chip esp8266 --example esp8266 --features="xtensa-lx-rt/lx106 xtensa-lx/lx106 esp8266-hal" /dev/ttyUSB0
```

## Resources

- The [esp-rs](https://github.com/esp-rs) organization has been formed to develop runtime, pac and hal crates for the esp32 and eventually esp8266.
- Checkout @lexxvir's [project](https://github.com/lexxvir/esp32-hello) for an example of using the esp-idf bindings in a Rust application.

## FAQ

- `error: intermittent IO error while iterating directory` - try creating that directory
- `undefined reference to .L` see [this issue](https://github.com/MabezDev/xtensa-rust-quickstart/issues/1)

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
