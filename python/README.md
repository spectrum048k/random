# Python Random Number Generator

A Python script that generates 100 cryptographically secure random integers between 1 and 10, then displays them as both a list and an ASCII line chart.

## Features

- **Cryptographic Security**: Uses `secrets.randbelow()` for secure random number generation
- **Visual Output**: ASCII line chart using `asciichartpy` library
- **Configurable**: Easy to modify constants for different ranges and array sizes

## Requirements

- Python 3.6+
- asciichartpy library

## Installation

1. Create a virtual environment (recommended):
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate  # On Windows: .venv\Scripts\activate
   ```

2. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```

## Usage

```bash
python generate_random_numbers.py
```

## Output

The script will display:
1. A list of 100 random numbers
2. An ASCII line chart visualization
3. Axis labels for easy interpretation

## Configuration

You can modify these constants in the script:
- `ARRAY_LENGTH`: Number of random numbers to generate (default: 100)
- `MIN_VALUE`: Minimum value (default: 1)
- `MAX_VALUE`: Maximum value (default: 10)
- `CHART_HEIGHT`: Height of the ASCII chart (default: 10)

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
