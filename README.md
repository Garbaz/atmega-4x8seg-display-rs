# Driving a 4x8 Segment Display with ATmega328 in Rust

Using an ATmega328 (aka Arduino Nano), we drive a 4x8 segment display like this one:

![pinout](https://www.dotnetlovers.com/Images/4digit7segmentdisplay85202024001AM.jpg)

Connect the display's anode pins **through resistors (say ~1kΩ)** as follows:

| Display Pin | ATmega Pin | Arduino Nano Pin |
| ----------- | ---------- | ---------------- |
| D1          | PB0        | D8               |
| D2          | PB1        | D9               |
| D3          | PB2        | D10              |
| D4          | PD7        | D7               |

And connect the display's cathode pins as follows:

| Display Pin | ATmega Pin | Arduino Nano Pin |
| ----------- | ---------- | ---------------- |
| A           | PB3        | D11              |
| B           | PB5        | D13              |
| C           | PD3        | D3               |
| D           | PD5        | D5               |
| E           | PD6        | D6               |
| F           | PB4        | D12              |
| G           | PD2        | D2               |
| Dp          | PD4        | D4               |

**Don't forget the resistors, and double-check the wiring, or things might go pop (._.)**

If you want to use different pins, just change them in [main.rs](/src/main.rs).

## Requirements

See the [`avr-hal` README](https://github.com/Rahix/avr-hal?tab=readme-ov-file#quickstart).

## Build & Run

Try

```sh
cargo run --release
```

If that doesn't work (*`Not able to guess port`*), try

```sh
cargo run --release -- -P /dev/ttyUSB0
```

If that doesn't work (*`/dev/ttyUSB0: Permission denied`*), try

```sh
# For Arch Linux
sudo usermod -a -G uucp <YOUR USERNAME>

# For other Linux distros
sudo usermod -a -G tty <YOUR USERNAME>
sudo usermod -a -G dialout <YOUR USERNAME>
```

then **log out and back in** and try again

```sh
cargo run --release -- -P /dev/ttyUSB0
```

## Usage

Once flashed onto the ATmega, send four digit numbers via serial to be displayed on the display.

The code is quite modular, so have a go at hacking around to make it do what you want. Or steal parts for your own projects :)