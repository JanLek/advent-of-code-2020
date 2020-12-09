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

const findInvalidNumber = () => {
  for (let i = 25; i < xmasData.length; ++i) {
    if (!isValidNumber(i)) {
      return { invalidNumber: xmasData[i], invalidNumberIndex: i };
    }
  }
};

const { invalidNumber, invalidNumberIndex } = findInvalidNumber();

for (let rangeStart = 0; rangeStart < invalidNumberIndex - 2; ++rangeStart) {
  for (
    let rangeEnd = rangeStart + 1;
    rangeEnd < invalidNumberIndex - 1;
    ++rangeEnd
  ) {
    const range = xmasData.slice(rangeStart, rangeEnd + 1);
    const sum = range.reduce((x, y) => x + y);
    if (sum === invalidNumber) {
      console.log({
        invalidNumber: invalidNumber.toLocaleString(),
        invalidNumberIndex: invalidNumberIndex.toLocaleString(),
        sum: sum.toLocaleString(),
        rangeStart: rangeStart.toLocaleString(),
        rangeEnd: rangeEnd.toLocaleString(),
        a: Math.min(...range).toLocaleString(),
        b: Math.max(...range).toLocaleString(),
        answer: (Math.min(...range) + Math.max(...range)).toLocaleString(),
        answer: Math.min(...range) + Math.max(...range),
      });
      break;
    }
  }
}

console.log("done");
