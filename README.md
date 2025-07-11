# Random Number Generator

This repository contains two implementations of a cryptographically secure random number generator that creates 100 random integers between 1 and 10, with ASCII chart visualization.

## Implementations

### Python Version (`python/`)
- Uses `secrets.randbelow()` for cryptographic security
- ASCII charts with `asciichartpy` library
- Clean, concise code with list comprehensions
- [See Python README](python/README.md)

### JavaScript Version (`javascript/`)
- Uses `crypto.randomInt()` for cryptographic security  
- Professional ASCII charts with `asciichart` library
- Modern ES6+ features and smooth curve visualization
- [See JavaScript README](javascript/README.md)

## Quick Start

### Python
```bash
cd python
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
python generate_random_numbers.py
```

### JavaScript
```bash
cd javascript
npm install
node generate_random_numbers.js
```

## Features

Both versions provide:
- **Cryptographically secure** random number generation
- **ASCII line chart** visualization
- **Configurable parameters** via constants
- **Clean, readable code** with modern best practices

## Output Example

Both scripts generate 100 random numbers and display them as:
1. Raw data array/list
2. Beautiful ASCII line chart with labeled axes

The visualization helps identify patterns, trends, and distribution in the randomly generated data.
