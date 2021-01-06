const numAnswersPart1 = (s) => new Set(groupAnswers(s)).size;

const numAnswersPart2 = (s) => {
  const numPersons = s.split("\n").length;
  const answers = groupAnswers(s).reduce((p, v) => {
    p[v] = p[v] ? p[v] + 1 : 1;
    return p;
  }, {});
  return Object.values(answers).filter((a) => a === numPersons).length;
};

const groupAnswers = (s) => s.split("").filter((c) => c !== "\n");

const sum = (arr, f) => arr.reduce((p, v) => p + f(v), 0);

const fs = require("fs");
const input = fs.readFileSync("inputs/day06.txt", "utf8").trim();
const groups = input.split("\n\n");

console.log(sum(groups, numAnswersPart1));
console.log(sum(groups, numAnswersPart2));
