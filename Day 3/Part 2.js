const { readFileSync } = require("fs");

const map = readFileSync("./Tree Map.txt", "utf-8")
  .split("\n")
  .map((line) => [...line].map((character) => character === "#"));
const isTreeAt = ({ x, y }) => map[x][y % map[0].length];

const deltas = [
  { x: 1, y: 1 },
  { x: 1, y: 3 },
  { x: 1, y: 5 },
  { x: 1, y: 7 },
  { x: 2, y: 1 },
];

const answer = deltas
  .map((delta) => {
    const steps = [{ x: 0, y: 0 }];
    const lastStep = () => steps[steps.length - 1];
    while (lastStep().x < map.length - 1) {
      steps.push({
        x: lastStep().x + delta.x,
        y: lastStep().y + delta.y,
      });
    }

    return steps.filter(isTreeAt).length;
  })
  .reduce((x, y) => x * y);

console.log(answer);
