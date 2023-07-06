# `arduino-run`

> A cargo runner for [Arduino](https://www.arduino.cc) boards

[![crates.io](https://img.shields.io/crates/v/arduino-run)](https://crates.io/crates/rttmon)
[![github](https://img.shields.io/github/actions/workflow/status/matteocarnelos/arduino-run/cargo.yml?branch=main)](https://github.com/matteocarnelos/arduino-run/actions/workflows/cargo.yml)

## Features

- Based on [`arduino-cli`](https://github.com/arduino/arduino-cli)
- Supports all Arduino boards
- Possibility to open serial monitor after flashing

## Installation

1. [Install `arduino-cli`](https://arduino.github.io/arduino-cli/latest/installation/)
2. Install `arduino-run`:
    ```
    cargo install arduino-run
    ```

## Usage

Add the following in your `.cargo/config.toml`:
```toml
[target.'<target-triple>']
runner = "arduino-run -b <board> -p <port>"
```

### Command reference

```
arduino-run [OPTIONS] --board <BOARD> --port <PORT> <ELF>
```

#### Arguments

- `<ELF>`: The ELF firmware file

#### Options

- `-b`, `--board` `<BOARD>`: The Fully Qualified Board Name (FQBN)
- `-p`, `--port` `<PORT>`: The serial port [env: `ARDUINO_RUN_PORT`]
- `-m`, `--monitor`: Open serial monitor after flashing
- `-r`, `--baudrate` `<BAUDRATE>`: The serial monitor baudrate [default: `9600`]

For more usage info, run `arduino-run --help`.

#### Additional resources

- [Arduino CLI Documentation](https://arduino.github.io/arduino-cli/latest/)

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Copyright © 2023, [Matteo Carnelos](https://github.com/matteocarnelos)
