# BeagleBone Black GPIO Control

The gpioctl interface provides an easy way to communicate with the BeagleBone Blacks GPIO pins.

In `/sys/class/gpio/` you can find folders for GPIOs on the BBB. This program gives an easy command line interface for controling the GPIOs, instead of reading and writing to these files directly.

## How it works

Pretty easy: You could just open a GPIO folder, for example `/sys/class/gpio/gpio7`. In that folder, you find a few files. E.g.:

* direction
* value

These files are special files. You can read from them or simply write to some of them to change and read values or directions. GPIOCTL provides an easy command line interface to handle all the (complex) stuff in the background for you. Examples:

* Check, if the GPIO is available for the desired operations on the BBB
* Export and unexport the GPIO before accessing it
* Plausibility checks (e.g. setting a value while the direction is set to "in" makes no sense)

All these features make life easier when working with the GPIOs of the BBB. It's planned, to implement additional helpful modes such as PWM output in GPIOCTL.

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

## Requirements

There are no requirements for GPIOCTL on the BeagleBone Black.

## Building

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
$ cargo build --release --target armv7-unknown-linux-musleabihf
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

Install it on the BeagleBone Black with

```console
$ sudo dpkg -i <package-name>
```

## GPIOs
List all available GPIOs via
```
$ gpioctl list
```

Currently, not all GPIOs are available. In future, we want to provide access to all GPIOs on the BeagleBone Black.
