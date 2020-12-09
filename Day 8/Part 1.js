const { readFileSync } = require("fs");

const instructions = [
  ...readFileSync("Program.txt", "utf-8").matchAll(
    /(?<operation>[a-z]{3}) (?<argumentSign>\+|-)(?<argumentNumber>\d+)\n/g
  ),
].map(({ groups: { operation, argumentSign, argumentNumber } }) => ({
  operation,
  argument: (argumentSign === "-" ? -1 : 1) * parseInt(argumentNumber),
}));

const executionHistory = [];
let instructionIndex = 0;
let accumulator = 0;
while (true) {
  const instruction = instructions[instructionIndex];

  if (executionHistory.includes(instruction)) {
    console.log("Loop detected", {
      atIndex: executionHistory.indexOf(instruction),
      instruction,
      accumulator,
    });
    break;
  }

  switch (instruction.operation) {
    case "acc":
      accumulator += instruction.argument;
      instructionIndex += 1;
      break;
    case "jmp":
      instructionIndex += instruction.argument;
      break;
    case "nop":
      instructionIndex += 1;
      break;
    default:
      throw new Error(`Unsupported instruction: ${instruction}`);
  }

  executionHistory.push(instruction);
}
