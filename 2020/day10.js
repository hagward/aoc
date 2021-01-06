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
  diffs[diff]++;
}

const dp = { 0: 1 };
for (const x of adapters) {
  const ans = (dp[x - 1] || 0) + (dp[x - 2] || 0) + (dp[x - 3] || 0);
  dp[x] = ans;
}

console.log(diffs[1] * diffs[3]);
console.log(dp[adapters[adapters.length - 1]]);
