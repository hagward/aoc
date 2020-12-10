const fs = require("fs");
const adapters = fs
  .readFileSync("day10.txt", "utf8")
  .trim()
  .split("\n")
  .map(Number)
  .sort((a, b) => a - b);

const diffs = [0, 1, 0, 1];

for (let i = 1; i < adapters.length; i++) {
  const diff = adapters[i] - adapters[i - 1];
  // console.log(`${adapters[i]}-${adapters[i - 1]}=${diff}`);
  diffs[diff]++;
}

console.log(diffs);
console.log(diffs[1] * diffs[3]);
