const { readFileSync } = require("fs");

const validPasswords = [
  ...readFileSync("./Corrupted Passwords.txt", "utf-8").matchAll(
    /(?<firstPosition>\d+)-(?<secondPosition>\d+) (?<character>[a-z]): (?<password>[a-z]+)\n/g
  ),
].filter(
  ({
    [0]: match,
    groups: { firstPosition, secondPosition, character, password },
  }) => {
    const isValid =
      (password[parseInt(firstPosition) - 1] === character) ^
      (password[parseInt(secondPosition) - 1] === character);
    console.log({
      firstPosition,
      secondPosition,
      character,
      password,
      isValid,
      match,
    });
    return isValid;
  }
);

console.log(validPasswords.length);
