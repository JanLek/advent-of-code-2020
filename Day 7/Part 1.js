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

const findEnclosingBags = (bags) =>
  Object.fromEntries(
    bags.flatMap((bag) =>
      Object.entries(rules).filter(([enclosingBag, enclosedBags]) =>
        Object.keys(enclosedBags).includes(bag)
      )
    )
  );

const findAllEnclosingBags = (bags) => {
  let previousEnclosingBags = Object.keys(findEnclosingBags(bags));
  const allEnclosingBags = [...previousEnclosingBags];
  while (previousEnclosingBags.length > 0) {
    const enclosingBags = Object.keys(
      findEnclosingBags(previousEnclosingBags)
    ).filter((bag) => !allEnclosingBags.includes(bag));
    allEnclosingBags.push(...enclosingBags);
    previousEnclosingBags = enclosingBags;
  }
  return allEnclosingBags;
};

const allEnclosingBags = findAllEnclosingBags(["shiny gold"]);

console.log(allEnclosingBags.length);
