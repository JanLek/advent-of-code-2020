use std::{
    fmt::{Display, Formatter, Result},
    mem::{swap, take},
    ops::Index,
};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!(
        "Part 1 - Occupied Seats = {}",
        WaitingArea::from_input(adjacent_seats, 4)
            .rounds()
            .last()
            .unwrap()
            .num_occupied_seats()
    );

    println!(
        "Part 2 - Occupied Seats = {}",
        WaitingArea::from_input(visible_seats, 5)
            .rounds()
            .last()
            .unwrap()
            .num_occupied_seats()
    );
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Seat {
    is_occupied: bool,
}

#[derive(Clone)]
struct WaitingArea {
    seats: Vec<Option<Seat>>,
    num_rows: usize,
    num_columns: usize,
    proximate_seats: fn(&WaitingArea, usize) -> Vec<Seat>,
    occupancy_tolerance: usize,
}

impl WaitingArea {
    fn from_input(
        proximate_seats: fn(&Self, usize) -> Vec<Seat>,
        occupancy_tolerance: usize,
    ) -> Self {
        Self {
            seats: INPUT
                .chars()
                .filter(|&c| c != '\n')
                .map(|c| match c {
                    '#' => Some(Seat { is_occupied: true }),
                    'L' => Some(Seat { is_occupied: false }),
                    '.' => None,
                    other_character => panic!(
                        "Unknown character in waiting area input: {}",
                        other_character
                    ),
                })
                .collect(),
            num_rows: INPUT.matches("\n").count() + 1,
            num_columns: INPUT.find("\n").unwrap(),
            proximate_seats,
            occupancy_tolerance,
        }
    }

    fn rounds(self) -> RoundsIterator {
        RoundsIterator::new(self)
    }

    fn to_position(&self, seat_index: usize) -> Position {
        (seat_index / self.num_columns, seat_index % self.num_columns)
    }

    fn num_occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .filter_map(|&seat| seat)
            .filter(|seat| seat.is_occupied)
            .count()
    }
}

impl Display for WaitingArea {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        for row_index in 0..self.num_rows {
            for column_index in 0..self.num_columns {
                write!(
                    formatter,
                    "{}",
                    match self[(row_index, column_index)] {
                        Some(Seat { is_occupied: true }) => '#',
                        Some(Seat { is_occupied: false }) => 'L',
                        None => '.',
                    }
                )?
            }
            write!(formatter, "{}", '\n')?
        }
        Ok(())
    }
}

impl Index<Position> for WaitingArea {
    type Output = Option<Seat>;

    fn index(&self, (row, column): Position) -> &Self::Output {
        &self.seats[row * self.num_columns + column]
    }
}

impl Default for WaitingArea {
    fn default() -> Self {
        Self {
            num_columns: 0,
            num_rows: 0,
            seats: Vec::with_capacity(0),
            occupancy_tolerance: 0,
            proximate_seats: adjacent_seats,
        }
    }
}

type Position = (usize, usize);

enum RoundIteratorState {
    Running,
    LastOne,
    Done,
}

struct RoundsIterator {
    waiting_area: WaitingArea,
    state: RoundIteratorState,
}

impl RoundsIterator {
    fn new(waiting_area: WaitingArea) -> Self {
        Self {
            waiting_area,
            state: RoundIteratorState::Running,
        }
    }

    fn next_round(&self) -> WaitingArea {
        WaitingArea {
            seats: (0..self.waiting_area.seats.len())
                .map(|index| match self.waiting_area.seats[index] {
                    Some(Seat { is_occupied: false }) => Some(Seat {
                        is_occupied: !(self.waiting_area.proximate_seats)(
                            &self.waiting_area,
                            index,
                        )
                        .into_iter()
                        .any(|seat| seat.is_occupied),
                    }),
                    Some(Seat { is_occupied: true }) => Some(Seat {
                        is_occupied: (self.waiting_area.proximate_seats)(&self.waiting_area, index)
                            .into_iter()
                            .filter(|seat| seat.is_occupied)
                            .count()
                            < self.waiting_area.occupancy_tolerance,
                    }),
                    None => None,
                })
                .collect(),
            ..self.waiting_area
        }
    }
}

impl Iterator for RoundsIterator {
    type Item = WaitingArea;

    fn next(&mut self) -> Option<Self::Item> {
        use RoundIteratorState::*;
        match &self.state {
            Running => {
                let mut next_waiting_area = self.next_round();
                if next_waiting_area.seats == self.waiting_area.seats {
                    self.state = LastOne;
                }
                swap(&mut self.waiting_area, &mut next_waiting_area);
                Some(next_waiting_area)
            }
            LastOne => {
                self.state = Done;
                Some(take(&mut self.waiting_area))
            }
            Done => None,
        }
    }
}

fn adjacent_seats(waiting_area: &WaitingArea, seat_index: usize) -> Vec<Seat> {
    let (row, column) = waiting_area.to_position(seat_index);
    let min_row = if row == 0 { row } else { row - 1 };
    let max_row = if row == waiting_area.num_rows - 1 {
        row
    } else {
        row + 1
    };
    let min_column = if column == 0 { column } else { column - 1 };
    let max_column = if column == waiting_area.num_columns - 1 {
        column
    } else {
        column + 1
    };

    (min_row..=max_row)
        .flat_map(|r| {
            (min_column..=max_column)
                .filter(move |&c| !(r == row && c == column))
                .map(move |c| waiting_area[(r, c)])
                .filter_map(|seat| seat)
        })
        .collect()
}

fn visible_seats(waiting_area: &WaitingArea, seat_index: usize) -> Vec<Seat> {
    let position = waiting_area.to_position(seat_index);
    let row = position.0 as i8;
    let column = position.1 as i8;
    let num_rows = waiting_area.num_rows as i8;
    let num_columns = waiting_area.num_columns as i8;
    let directions: Vec<(i8, i8)> = vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    let mut seats = Vec::with_capacity(directions.len());

    for (row_delta, column_delta) in directions {
        for step in 1.. {
            let seat_row = row + step * row_delta;
            let seat_column = column + step * column_delta;

            if seat_row < 0 || seat_row >= num_rows || seat_column < 0 || seat_column >= num_columns
            {
                break;
            }

            if let Some(seat) = waiting_area[(seat_row as usize, seat_column as usize)] {
                seats.push(seat);
                break;
            }
        }
    }

    seats
}
