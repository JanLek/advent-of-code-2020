const { readFileSync } = require("fs");

const answers = readFileSync("./Answers.txt", "utf-8")
  .split("\n\n")
  .map((groupsAnswers) => [...new Set(groupsAnswers.match(/[a-z]/g))].length)
  .reduce((x, y) => x + y);

console.log(answers);
