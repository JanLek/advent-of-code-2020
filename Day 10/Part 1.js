const { readFileSync } = require("fs");

const adapterJoltages = readFileSync("Adapter Joltages.txt", "utf-8")
  .split("\n")
  .map((line) => parseInt(line))
  .sort((a, b) => a - b);

const joltageDifferences = adapterJoltages.map((adapterJoltage, index) => {
  const previousAdapterJoltage = index === 0 ? 0 : adapterJoltages[index - 1];
  return adapterJoltage - previousAdapterJoltage;
});
joltageDifferences.push(3); // Device joltage difference

console.log(
  joltageDifferences.filter((a) => a === 1).length *
    joltageDifferences.filter((a) => a === 3).length
);
