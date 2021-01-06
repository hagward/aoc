const fs = require("fs");

const INPUT = fs
  .readFileSync("inputs/day11.txt", "utf8")
  .trim()
  .split("\n")
  .map((line) => line.split(""));

const DIRS = [
  [-1, -1],
  [0, -1],
  [1, -1],
  [-1, 0],
  [1, 0],
  [-1, 1],
  [0, 1],
  [1, 1],
];

const FLOOR = ".";
const EMPTY = "L";
const OCCUPIED = "#";

const findNeighbor = (layout, x, y, dx, dy) => {
  while (true) {
    x += dx;
    y += dy;

    if (!layout[y]) return;

    switch (layout[y][x]) {
      case EMPTY:
      case OCCUPIED:
      case undefined:
        return layout[y][x];
    }
  }
};

const simulate = (calcChange) => {
  const layout = INPUT.map((line) => line.slice());

  while (true) {
    const changes = [];

    for (let i = 0; i < layout.length; i++) {
      for (let j = 0; j < layout[i].length; j++) {
        const change = calcChange(layout, i, j);
        if (change) changes.push({ i, j, change });
      }
    }

    if (!changes.length) break;

    for (const change of changes) {
      layout[change.i][change.j] = change.change;
    }
  }

  return layout;
};

const partOne = () =>
  simulate((layout, i, j) => {
    const seat = layout[i][j];
    if (seat === FLOOR) return;

    const numOccupied = DIRS.map(
      ([dx, dy]) => layout[i + dy] && layout[i + dy][j + dx]
    ).filter((s) => s === OCCUPIED).length;

    if (seat === EMPTY && numOccupied === 0) {
      return OCCUPIED;
    } else if (seat === OCCUPIED && numOccupied >= 4) {
      return EMPTY;
    }
  });

const partTwo = () =>
  simulate((layout, i, j) => {
    const seat = layout[i][j];
    if (seat === FLOOR) return;

    const numOccupied = DIRS.map(([dx, dy]) =>
      findNeighbor(layout, j, i, dx, dy)
    ).filter((s) => s === OCCUPIED).length;

    if (seat === EMPTY && numOccupied === 0) {
      return OCCUPIED;
    } else if (seat === OCCUPIED && numOccupied >= 5) {
      return EMPTY;
    }
  });

[partOne(), partTwo()]
  .map((layout) => layout.flat().filter((seat) => seat === OCCUPIED).length)
  .forEach((numOccupied) => console.log(numOccupied));
