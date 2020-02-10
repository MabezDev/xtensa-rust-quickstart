
# Rust on the xtensa architecture

Need help? Join the esp-rs room on matrix, https://matrix.to/#/#esp-rs:matrix.org.

## Building

### Requirements

#### llvm-xtensa
Please refer to [Espressif's llvm](https://github.com/espressif/llvm-project) project for authoratative instructions.

    $ git clone https://github.com/espressif/llvm-project
    $ cd llvm-project/llvm
    $ mkdir build
    $ cd build
    $ cmake .. -DLLVM_TARGETS_TO_BUILD="Xtensa;X86" -DCMAKE_BUILD_TYPE=Release -G "Ninja"
    $ cmake --build .

Calling make with an appropriate number of threads will speed the process considerably.

Many use the guideline `n + 1`, where `n` is the number of processor cores on your machine. For example, for a processor with 4 logical cores:
    
    $ make -j5

#### rust-xtensa
Please refer to the [rust-xtensa](https://github.com/MabezDev/rust-xtensa) project for authoratative instructions.

Assuming you built llvm in your home directory:

    $ git clone https://github.com/MabezDev/rust-xtensa
    $ cd rust-xtensa
    $ git checkout xtensa-target
    $ ./configure --llvm-root=$HOME/llvm-project/llvm/build
    $ ./x.py build

#### xtensa-esp32-elf toolchain
Instructions can be found [on Espressif's web site](https://docs.espressif.com/projects/esp-idf/en/release-v3.0/get-started/linux-setup.html).

Download the archived toolchain file, and extract it to the directory of your choice. Then add the toolchain's bin/ directory to your `$PATH`. For example:

    $ mkdir ~/esp
    $ tar -xzf ~/Downloads/xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-5.2.0.tar.gz -C ~/esp
    $ PATH="$PATH:$HOME/esp/xtensa-esp32-elf/bin"

#### xargo or cargo xbuild
    $ cargo install xargo

or

    $ cargo install xbuild

#### esptool
    $ pip install esptool

### Starting a new project
    $ git clone https://github.com/MabezDev/xtensa-rust-quickstart

### Workflow
Update `CUSTOM_RUSTC` in `setenv` to point to the version of rust you compiled earlier. Then load the environment variables with `source setenv`.

If you installed `xbuild` instead of `xargo`, you will need to update `flash` and `flash_release` accordingly.

You should now be able to call xargo (or cargo xbuild) to build the project. You can also run the flash script to both build the project, and flash it to the ESP32

You will need to change the parameter `BLINKY_GPIO` to match your board's LED pin. Unfortunately, this may require adjustments to the chip's IO_MUX peripheral, which will mean consulting the ESP32 Technical Reference Manual. See [this issue](https://github.com/MabezDev/idf2svd/issues/11) for more information.

## Resources

- The [esp-rs](https://github.com/esp-rs) organization has been formed to develop runtime, pac and hal crates for the esp32 and eventually esp8266.
- Checkout @lexxvir's [project](https://github.com/lexxvir/esp32-hello) for an example of using the esp-idf bindings in a Rust application.

## FAQ

- `error: intermittent IO error while iterating directory` - try creating that directory
- `undefined reference to .L` see [this issue](https://github.com/MabezDev/xtensa-rust-quickstart/issues/1)
