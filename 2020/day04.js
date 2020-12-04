const difference = (setA, setB) => {
  let _difference = new Set(setA);
  for (let elem of setB) {
    _difference.delete(elem);
  }
  return _difference;
};

const rules = {
  byr: (v) => v > 1919 && v < 2003,
  iyr: (v) => v > 2009 && v < 2021,
  eyr: (v) => v > 2019 && v < 2031,
  hgt: (v) => {
    const length = v.slice(0, -2);
    const unit = v.slice(-2);
    return (
      (unit === "cm" && length > 149 && length < 194) ||
      (unit === "in" && length > 58 && length < 77)
    );
  },
  hcl: (v) => /^#[0-9a-f]{6}$/.test(v),
  ecl: (v) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(v),
  pid: (v) => /^[0-9]{9}$/.test(v),
  cid: () => true,
};

const fields = difference(new Set(Object.keys(rules)), new Set(["cid"]));

const isValid = (passport, checkRules = false) => {
  const _fields = passport.split(/\s+/).map((f) => f.split(":"));
  const fieldNames = _fields.map(([f]) => f);
  const hasAllFields = difference(fields, new Set(fieldNames)).size === 0;

  if (!checkRules) return hasAllFields;

  return hasAllFields && _fields.every(([f, v]) => rules[f](v));
};

const fs = require("fs");
const passports = fs.readFileSync("day04.txt", "utf8").trim().split("\n\n");
console.log(passports.filter((p) => isValid(p)).length);
console.log(passports.filter((p) => isValid(p, true)).length);
