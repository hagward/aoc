const isValidPart1 = (line) => {
  const [min, max, char, password] = parseLine(line);
  const matches = [...password].filter((c) => c === char).length;
  return matches >= min && matches <= max;
};

const isValidPart2 = (line) => {
  const [i, j, char, password] = parseLine(line);
  return Boolean((password[i - 1] === char) ^ (password[j - 1] === char));
};

const parseLine = (line) => {
  const [, x, y, char, password] = /^(\d+)-(\d+) ([a-z]): ([a-z]+)$/.exec(line);
  return [Number(x), Number(y), char, password];
};

const fs = require("fs");
const data = fs.readFileSync("inputs/day02.txt", "utf8").trim().split("\n");
console.log(data.filter(isValidPart1).length);
console.log(data.filter(isValidPart2).length);
