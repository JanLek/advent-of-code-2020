const { readFileSync } = require("fs");

const rules = Object.fromEntries(
  [
    ...readFileSync("Rules.txt", "utf-8").matchAll(
      /(?<enclosingBag>[a-z]+ [a-z]+) bags contain (?<enclosedBags>.+)\n/g
    ),
  ].map(({ groups: { enclosingBag, enclosedBags } }) => [
    enclosingBag,
    Object.fromEntries(
      [
        ...enclosedBags.matchAll(
          /(?<numEnclosedBags>\d+) (?<enclosedBagColour>[a-z]+ [a-z]+) bags?/g
        ),
      ].map(({ groups: { numEnclosedBags, enclosedBagColour } }) => [
        enclosedBagColour,
        numEnclosedBags,
      ])
    ),
  ])
);

const numEnclosedBags = (rule) => {
  if (Object.entries(rule).length === 0) return 0;

  return Object.entries(rule).reduce(
    (total, [bag, numBags]) =>
      total + parseInt(numBags) * (numEnclosedBags(rules[bag]) + 1),
    0
  );
};

console.log(numEnclosedBags(rules["shiny gold"]));
