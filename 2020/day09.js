const fs = require("fs");
const numbers = fs
  .readFileSync("day09.txt", "utf8")
  .trim()
  .split("\n")
  .map(Number);

const findInvalidNumber = (numbers, preambleSize) => {
  const preamble = numbers.slice(0, preambleSize);
  const preambleSet = new Set(preamble);

  loop: for (let n of numbers.slice(preambleSize)) {
    for (let k of preamble) {
      if (preambleSet.has(n - k)) {
        const firstElement = preamble.shift();
        preambleSet.delete(firstElement);
        preamble.push(n);
        preambleSet.add(n);
        continue loop;
      }
    }

    return n;
  }

  return -1;
};

const findEncryptionWeakness = (numbers, invalidNumber) => {
  for (let i = 0; i < numbers.length; i++) {
    let j = i + 1;
    let sum = numbers[i] + numbers[j];
    while (sum < invalidNumber) sum += numbers[++j];
    if (sum === invalidNumber) {
      const range = numbers.slice(i, j + 1);
      return Math.min(...range) + Math.max(...range);
    }
  }

  return -1;
};

const invalidNumber = findInvalidNumber(numbers, 25);
const weakness = findEncryptionWeakness(numbers, invalidNumber);

console.log("Invalid number:", invalidNumber);
console.log("Encryption weakness:", weakness);
