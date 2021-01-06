const parse = (s) => {
  const parents = {};
  const children = {};
  for (let line of s.split("\n")) {
    const p = line.split(" ");
    const parent = `${p[0]} ${p[1]}`;
    children[parent] = [];
    if (line.endsWith("no other bags.")) continue;
    for (let i = 4; i < p.length; i += 4) {
      const count = p[i];
      const child = `${p[i + 1]} ${p[i + 2]}`;
      if (!parents[child]) parents[child] = [];
      parents[child].push(parent);
      children[parent].push({ child, count });
    }
  }
  return [parents, children];
};

const countParents = (bag, parents, visited) => {
  if (visited.has(bag)) return 0;
  visited.add(bag);
  let sum = 1;
  for (let p of parents[bag] || []) {
    sum += countParents(p, parents, visited);
  }
  return sum;
};

const countChildren = (bag, children) => {
  let sum = 1;
  for (let c of children[bag] || []) {
    sum += c.count * countChildren(c.child, children);
  }
  return sum;
};

const fs = require("fs");
const input = fs.readFileSync("inputs/day07.txt", "utf8").trim();
const [parents, children] = parse(input);
console.log(countParents("shiny gold", parents, new Set()) - 1);
console.log(countChildren("shiny gold", children) - 1);
