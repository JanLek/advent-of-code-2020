const { readFileSync } = require("fs");

const requiredFields = [
  "byr",
  "iyr",
  "eyr",
  "hgt",
  "hcl",
  "ecl",
  "pid",
  // 'cid',
];

const answer = readFileSync("./Passports.txt", "utf-8")
  .split("\n\n")
  .map((passportString) => {
    return Object.fromEntries(
      [
        ...passportString.matchAll(/(?<key>[a-z]{3}):(?<value>[^\s]+)/g),
      ].map(({ groups: { key, value } }) => [key, value])
    );
  })
  .filter((passport) =>
    requiredFields.every((requiredField) =>
      Object.keys(passport).includes(requiredField)
    )
  ).length;

console.log(answer);
