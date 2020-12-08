const { readFileSync } = require("fs");

const occupiedSeats = readFileSync("./Boarding Passes.txt", "utf-8")
  .split("\n")
  .map((boardingPass) => {
    const asBinary = boardingPass.replace(/F|L/g, 0).replace(/B|R/g, 1);
    const row = parseInt(asBinary.slice(0, 7), 2);
    const column = parseInt(asBinary.slice(7), 2);
    return row * 8 + column;
  });

for (const seatId of occupiedSeats.sort((a, b) => a - b)) {
  const column = seatId % 8;
  const row = (seatId - column) / 8;
  // console.log("occupied seat", { row, column, seatId });
}

for (let row = 3; row < 103; row++) {
  for (let column = 0; column < 8; column++) {
    const seatId = row * 8 + column;
    if (!occupiedSeats.includes(seatId)) {
      console.log("free seat", { row, column, seatId });
    }
  }
}
