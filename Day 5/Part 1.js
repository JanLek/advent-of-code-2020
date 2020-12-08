const { readFileSync } = require("fs");

const answer = readFileSync("./Boarding Passes.txt", "utf-8")
  .split("\n")
  .map((boardingPass) => {
    const asBinary = boardingPass.replace(/F|L/g, 0).replace(/B|R/g, 1);
    const row = parseInt(asBinary.slice(0, 7), 2);
    const column = parseInt(asBinary.slice(7), 2);
    return row * 8 + column;
  });

console.log(Math.max(...answer));
