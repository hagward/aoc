const seatId = (s) => row(s) * 8 + col(s);
const row = (boardingPass) =>
  binarySearch(
    128,
    boardingPass
      .split("")
      .slice(0, 7)
      .map((c) => c === "F")
  );
const col = (boardingPass) =>
  binarySearch(
    8,
    boardingPass
      .split("")
      .slice(7, 10)
      .map((c) => c === "L")
  );

const binarySearch = (n, v) => {
  let low = 0;
  let high = n - 1;
  for (let e of v) {
    const mid = (low + high) / 2;
    if (e) high = Math.floor(mid);
    else low = Math.ceil(mid);
  }
  return low;
};

const fs = require("fs");
const boardingPasses = fs.readFileSync("day05.txt", "utf8").trim().split("\n");
const seatIds = boardingPasses.map(seatId).sort();

const maxSeatId = seatIds.reduce((p, v) => (v > p ? v : p));
console.log("Highest seat ID:", maxSeatId);

for (let i = 1; i < seatIds.length; i++) {
  if (seatIds[i] - seatIds[i - 1] > 1) {
    console.log("Your seat ID:", seatIds[i] - 1);
    break;
  }
}
