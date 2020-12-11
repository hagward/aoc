const fs = require("fs");
const v = fs
  .readFileSync("day11.txt", "utf8")
  .trim()
  .split("\n")
  .map((line) => line.split(""));

// const v = [
//   "L.LL.LL.LL",
//   "LLLLLLL.LL",
//   "L.L.L..L..",
//   "LLLL.LL.LL",
//   "L.LL.LL.LL",
//   "L.LLLLL.LL",
//   "..L.L.....",
//   "LLLLLLLLLL",
//   "L.LLLLLL.L",
//   "L.LLLLL.LL",
// ].map((line) => line.split(""));

const isEmpty = (i, j) => v[i] && v[i][j] === "L";
const isOccupied = (i, j) => v[i] && v[i][j] === "#";

const numAdjacent = (i, j, f) =>
  [
    [i - 1, j - 1],
    [i - 1, j],
    [i - 1, j + 1],
    [i, j - 1],
    [i, j + 1],
    [i + 1, j - 1],
    [i + 1, j],
    [i + 1, j + 1],
  ].filter(([i, j]) => f(i, j)).length;

while (true) {
  const changes = [];
  for (let i = 0; i < v.length; i++) {
    for (let j = 0; j < v[i].length; j++) {
      if (isEmpty(i, j) && numAdjacent(i, j, isOccupied) === 0) {
        changes.push({ i, j, change: "#" });
      } else if (isOccupied(i, j) && numAdjacent(i, j, isOccupied) >= 4) {
        changes.push({ i, j, change: "L" });
      }
    }
  }

  if (!changes.length) break;

  for (let change of changes) {
    v[change.i][change.j] = change.change;
  }
}

let numOccupied = 0;
for (let i = 0; i < v.length; i++) {
  for (let j = 0; j < v[i].length; j++) {
    if (isOccupied(i, j)) numOccupied++;
  }
}

console.log(numOccupied);
