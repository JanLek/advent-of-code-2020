const { readFileSync } = require("fs");

const answers = readFileSync("./Answers.txt", "utf-8")
  .split("\n\n")
  .map((groupsAnswers) => {
    const answersPerPerson = groupsAnswers.split("\n");
    let count = 0;
    for (let charCode = 97; charCode <= 122; ++charCode) {
      const answer = String.fromCharCode(charCode);
      if (
        answersPerPerson.every((personAnswers) =>
          personAnswers.includes(answer)
        )
      ) {
        count += 1;
      }
    }
    return count;
  })
  .reduce((x, y) => x + y);

console.log(answers);
