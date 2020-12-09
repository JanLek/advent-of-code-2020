const { readFileSync } = require("fs");

const xmasData = readFileSync("XMAS Data.txt", "utf-8")
  .split("\n")
  .map((n) => parseInt(n));

const isValidNumber = (i) => {
  for (let j = i - 25; j < i; ++j) {
    for (let k = i - 25; k < i; ++k) {
      if (k === j) continue;
      if (xmasData[j] + xmasData[k] === xmasData[i]) return true;
    }
  }
  return false;
};

for (let i = 25; i < xmasData.length; ++i) {
  if (!isValidNumber(i)) {
    console.log("invalid number", i, xmasData[i]);
    break;
  }
}

console.log("done");
