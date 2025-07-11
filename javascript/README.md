# JavaScript Random Number Generator

A Node.js script that generates 100 cryptographically secure random integers between 1 and 10, then displays them as both an array and an ASCII line chart.

## Features

- **Cryptographic Security**: Uses `crypto.randomInt()` for secure random number generation
- **Professional Visualization**: ASCII line chart using `asciichart` library
- **Modern JavaScript**: Uses ES6+ features like `Array.from()` and arrow functions
- **Configurable**: Easy to modify constants for different ranges and array sizes

## Requirements

- Node.js 14+
- asciichart library

## Installation

1. Install Node.js dependencies:
   ```bash
   npm install
   ```

   Or install asciichart directly:
   ```bash
   npm install asciichart
   ```

## Usage

```bash
node generate_random_numbers.js
```

## Output

The script will display:
1. An array of 100 random numbers
2. A smooth ASCII line chart visualization

## Configuration

You can modify these constants in the script:
- `ARRAY_LENGTH`: Number of random numbers to generate (default: 100)
- `MIN_VALUE`: Minimum value (default: 1)
- `MAX_VALUE`: Maximum value (default: 10)
- `CHART_HEIGHT`: Height of the ASCII chart (default: 10)

## Example Output

```javascript
[7, 3, 9, 1, 5, 8, 2, ...]

Random Numbers (Y: 1-10, X: 1-100):
  10.0 ┤   ╭╮     ╭╮
   9.1 ┤╭╮ ││  ╭╮ ││
   8.2 ┤││ ││  ││ ││
   7.3 ┤││ ╰╯  ││ ││
   ...
```

## Features Comparison

This JavaScript version provides:
- Smooth curved line connections
- Professional-looking charts with rounded corners
- Built-in formatting options
- High-quality ASCII art visualization
