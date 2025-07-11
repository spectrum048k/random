# Rust Random Number Generator

A Rust program that generates 100 cryptographically secure random integers between 1 and 10, then displays them as both a list and an ASCII line chart.

## Features

- **Cryptographic Security**: Uses `rand::rngs::OsRng` for secure random number generation
- **Visual Output**: ASCII line chart using `textplots` crate
- **Configurable**: Easy to modify constants for different ranges and array sizes

## Requirements

- Rust (edition 2021)

## Installation

1. Install Rust (if not already):
   https://www.rust-lang.org/tools/install

2. Build dependencies:
   ```bash
   cd rust
   cargo build
   ```

## Usage

```bash
cargo run
```

## Output

The program will display:
1. A list of 100 random numbers
2. An ASCII line chart visualization
3. Axis labels for easy interpretation

## Configuration

You can modify these constants in `src/main.rs`:
- `ARRAY_LENGTH`: Number of random numbers to generate (default: 100)
- `MIN_VALUE`: Minimum value (default: 1)
- `MAX_VALUE`: Maximum value (default: 10)

## Example Output

```
[7, 3, 9, 1, 5, ...]

Random Numbers (Y: 1-10, X: 1-100):
 10.0 ┤   ╭╮     ╭╮
  9.0 ┤╭╮ ││  ╭╮ ││
  8.0 ┤││ ││  ││ ││
  ...
X-axis: 1 to 100 (index of each random number)
Y-axis: Value (1 to 10)
```
