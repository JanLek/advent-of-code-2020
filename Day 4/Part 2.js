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
  .filter(
    (passport) =>
      requiredFields.every((requiredField) =>
        Object.keys(passport).includes(requiredField)
      ) &&
      // byr (Birth Year) - four digits; at least 1920 and at most 2002.
      /^\d{4}$/.test(passport.byr) &&
      passport.byr >= 1920 &&
      passport.byr <= 2002 &&
      // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
      /^\d{4}$/.test(passport.iyr) &&
      passport.iyr >= 2010 &&
      passport.iyr <= 2020 &&
      // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
      /^\d{4}$/.test(passport.eyr) &&
      passport.eyr >= 2020 &&
      passport.eyr <= 2030 &&
      // hgt (Height) - a number followed by either cm or in:
      // If cm, the number must be at least 150 and at most 193.
      // If in, the number must be at least 59 and at most 76.
      (() => {
        const height = parseInt(passport.hgt);
        if (/\d{3}cm/.test(passport.hgt)) return 150 <= height && height <= 193;
        if (/\d{2}in/.test(passport.hgt)) return 59 <= height && height <= 76;
        return false;
      })() &&
      // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
      /^#[0-9a-f]{6}$/.test(passport.hcl) &&
      // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
      /^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$/.test(passport.ecl) &&
      // pid (Passport ID) - a nine-digit number, including leading zeroes.
      /^\d{9}$/.test(passport.pid) &&
      // cid (Country ID) - ignored, missing or not.
      true
  ).length;

console.log(answer);
