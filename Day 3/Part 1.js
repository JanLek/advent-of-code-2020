const { readFileSync } = require("fs");

const map = readFileSync("./Tree Map.txt", "utf-8")
  .split("\n")
  .map((line) => [...line].map((character) => character === "#"));
const isTreeAt = ({ x, y }) => map[x][y % map[0].length];

const steps = [{ x: 0, y: 0 }];
const lastStep = () => steps[steps.length - 1];
while (lastStep().x < map.length - 1) {
  steps.push({
    x: lastStep().x + 1,
    y: lastStep().y + 3,
  });
}

console.log(steps.length);
console.log(steps.filter(isTreeAt).length);
