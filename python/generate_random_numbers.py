import secrets
import asciichartpy

# Constants
ARRAY_LENGTH = 100
MIN_VALUE = 1
MAX_VALUE = 10
CHART_HEIGHT = 10

# Generate 100 cryptographically secure random integers between 1 and 10
numbers = [secrets.randbelow(MAX_VALUE) + MIN_VALUE for _ in range(ARRAY_LENGTH)]

# Print the numbers
print(numbers)

# Plot each number along the x-axis, y-axis is the value (1 to 10)
print(f"\nRandom Numbers (Y: {MIN_VALUE}-{MAX_VALUE}, X: 1-{ARRAY_LENGTH}):")
print(asciichartpy.plot(numbers, {'height': CHART_HEIGHT}))
print(f"X-axis: 1 to {ARRAY_LENGTH} (index of each random number)")
print(f"Y-axis: Value ({MIN_VALUE} to {MAX_VALUE})")
