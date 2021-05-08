# BeagleBone Black GPIO Control

The gpioctl interface provides an easy way to communicate with the BBB's GPIO pins. In `/sys/class/gpio/` you can find folders for GPIOs on the BBB. This program gives an easy command line interface for controling the GPIOs.

## Usage

Call the program by typing `gpioctl` in the console.

```console
$ gpioctl [gpio] [mode] [function] [value]
```

The argument `value` only needs to be used when using the function `set`.


| Mode        | Function | Explanation                                                   |
|-------------|----------|---------------------------------------------------------------|
| `direction` | `get`    | Returns the current direction of the GPIO (in/out)            |
|             | `set`    | Sets the direction of the GPIO (in/out)                       |
| `value`     | `get`    | Returns the current value of the GPIO (0/1)                   |
|             | `set`    | Sets the value of the GPIO (0/1), GPIO direction must be in   |
| `label`     | `get`    | Returns the label of the GPIO                                 |


## Installation

Make sure, you install the gcc ARM compiler and linker first:

```console
$ sudo apt install gcc-arm-linux-gnueabihf
```

You'll also need the corresponding Rust Crate:

```console
$ rustup target add armv7-unknown-linux-musleabihf
```

Before building the program, export the following variable:

```console
$ export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc
```

Finally, build the executable:

```console
$ cargo build --target --release armv7-unknown-linux-musleabihf
```

The executable program is located in `target/armv7-unknown-linux-musleabihf/release`

## Creating Debian Package
To create the Debian Package, install `cargo deb` and `binutils-arm-linux-gnueabi`:

```console
$ cargo install cargo-deb
$ sudo apt install binutils-arm-linux-gnueabi
```

You also need to modify your `~/.cargo/config` file. Add the following lines to the file:

```console
[target.armv7-unknown-linux-musleabihf]
strip = { path = "arm-linux-gnueabihf-strip" }
```

That will point the strip command to the previously installed "stripper" for ARM binaries.

Building the Debian Package is done by the following command by specifying target:

```console
$ cargo deb --target armv7-unknown-linux-musleabihf
```

The package will be located under `target/armv7-unknown-linux-musleabihf/debian`.

Install it with

```console
$ sudo dpkg -i <package-name>
```

## GPIOs
List all available GPIOs via
```
$ gpioctl list
```