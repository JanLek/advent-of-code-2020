const { readFileSync } = require("fs");

const validPasswords = [
  ...readFileSync("./Corrupted Passwords.txt", "utf-8").matchAll(
    /(?<minOccurences>\d+)-(?<maxOccurences>\d+) (?<character>[a-z]): (?<password>[a-z]+)\n/g
  ),
].filter(
  ({
    [0]: match,
    groups: { minOccurences, maxOccurences, character, password },
  }) => {
    const pattern = RegExp(character, "g");
    const numOccurences = (password.match(pattern) ?? []).length;
    const isValid =
      minOccurences <= numOccurences && numOccurences <= maxOccurences;
    if (!isValid) {
      console.log({
        minOccurences,
        maxOccurences,
        numOccurences,
        character,
        password,
        isValid,
        match,
      });
    }
    return isValid;
  }
);

console.log(validPasswords.length);
