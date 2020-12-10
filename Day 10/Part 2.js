const { readFileSync } = require("fs");

const adapters = readFileSync("Adapter Joltages.txt", "utf-8")
  .split("\n")
  .map((line) => parseInt(line))
  .sort((a, b) => a - b);

const cache = { [0]: 1 }; // Wall starts at 0 jolts

for (const adapter of adapters) {
  let numPathsToAdapter = 0;
  for (const joltageDelta of [1, 2, 3]) {
    const previousAdapter = adapter - joltageDelta;
    if (previousAdapter in cache) numPathsToAdapter += cache[previousAdapter];
  }
  cache[adapter] = numPathsToAdapter;
}

console.log(cache[adapters[adapters.length - 1]]);
