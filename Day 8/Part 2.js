const { readFileSync } = require("fs");

const instructions = [
  ...readFileSync("Program.txt", "utf-8").matchAll(
    /(?<operation>[a-z]{3}) (?<argumentSign>\+|-)(?<argumentNumber>\d+)\n/g
  ),
].map(({ groups: { operation, argumentSign, argumentNumber } }) => ({
  operation,
  argument: (argumentSign === "-" ? -1 : 1) * parseInt(argumentNumber),
}));

const tryProgram = () => {
  const executionHistory = [];
  let instructionIndex = 0;
  let accumulator = 0;
  while (true) {
    if (instructionIndex === instructions.length)
      return { loopDetected: false, accumulator };

    const instruction = instructions[instructionIndex];

    if (executionHistory.includes(instruction)) {
      return { loopDetected: true, accumulator };
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
};

const flipJmpNop = (instruction) => {
  instruction.operation = instruction.operation === "jmp" ? "nop" : "jmp";
};

for (const instruction of instructions) {
  if (instruction.operation !== "jmp" && instruction.operation !== "nop")
    continue;

  flipJmpNop(instruction);
  const { loopDetected, accumulator } = tryProgram();
  if (loopDetected) {
    flipJmpNop(instruction);
  } else {
    console.log({ loopDetected, accumulator });
    break;
  }
}

console.log("no fix was found");
