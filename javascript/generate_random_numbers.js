const crypto = require('crypto');

// Constants
const MIN_VALUE = 1;
const MAX_VALUE = 10;
const ARRAY_LENGTH = 100;
const CHART_HEIGHT = 10;

// Generate 100 cryptographically secure random integers between 1 and 10
const numbers = Array.from({ length: ARRAY_LENGTH }, () =>
  crypto.randomInt(MIN_VALUE, MAX_VALUE + 1)
);

console.log(numbers);

// ASCII line chart using asciichart library
function plotLine(data) {
  const asciichart = require('asciichart');
  console.log(asciichart.plot(data, { 
    height: 10,
    padding: '      ',
    format: (x) => x.toFixed(1).padStart(6)
  }));
}

console.log(`\nRandom Numbers (Y: ${MIN_VALUE}-${MAX_VALUE}, X: 1-${ARRAY_LENGTH}):`);
plotLine(numbers);
