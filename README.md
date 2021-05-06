# BeagleBone Black GPIO Control

The gpioctl interface provides an easy way to communicate with the BBB's GPIO pins.

## Usage

Call the program by typing `gpioctl` in the console.

```console
$ gpioctl [gpio] [mode] [function] [value]
```

The argument `value` is optional and only needs to be used when using the function `set`.


| Mode        | Function | Explanation                                                   |
|-------------|----------|---------------------------------------------------------------|
| `direction` | `get`    | Returns the current direction of the GPIO (in/out)            |
|             | `set`    | Sets the direction of the GPIO (in/out)                       |
| `value`     | `get`    | Returns the current value of the GPIO (0/1)                   |
|             | `set`    | Sets the value of the GPIO (0/1), GPIO direction must be `in` |
| `label`     | `get`    | Returns the label of the GPIO                                 |



## Installation

tbd...

## GPIOs

tbd...