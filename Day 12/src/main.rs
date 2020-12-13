use regex::Regex;
use Instruction::*;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let instructions = parse_instructions();
    println!(
        "Part 1 - Manhattan distance = {}",
        execute_instructions_incorrectly(&instructions).round()
    );

    println!(
        "Part 2 - Manhattan distance = {}",
        execute_instructions_correctly(&instructions).round()
    );
}

fn parse_instructions() -> Vec<Instruction> {
    let instruction_pattern: Regex = Regex::new(r"(?P<action>[A-Z])(?P<value>\d+)").unwrap();
    INPUT
        .split("\n")
        .map(|line| {
            let captures = instruction_pattern.captures(line).unwrap();
            let action = captures.name("action").unwrap().as_str();
            let value = captures
                .name("value")
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap();

            match action {
                "N" => MoveNorth { distance: value },
                "S" => MoveSouth { distance: value },
                "E" => MoveEast { distance: value },
                "W" => MoveWest { distance: value },
                "L" => TurnLeft { degrees: value },
                "R" => TurnRight { degrees: value },
                "F" => MoveForward { distance: value },
                _ => panic!("Unsupported action: {}", action),
            }
        })
        .collect()
}

fn execute_instructions_incorrectly(instructions: &Vec<Instruction>) -> f64 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut rotation = 0.0;

    for instruction in instructions {
        match instruction {
            MoveNorth { distance } => y += distance,
            MoveSouth { distance } => y -= distance,
            MoveEast { distance } => x += distance,
            MoveWest { distance } => x -= distance,
            TurnLeft { degrees } => rotation += degrees,
            TurnRight { degrees } => rotation -= degrees,
            MoveForward { distance } => {
                x += rotation.to_radians().cos() * distance;
                y += rotation.to_radians().sin() * distance;
            }
        }
    }

    manhattan_distance(x, y)
}

fn execute_instructions_correctly(instructions: &Vec<Instruction>) -> f64 {
    let mut waypoint_x = 10.0;
    let mut waypoint_y = 1.0;
    let mut ship_x = 0.0;
    let mut ship_y = 0.0;

    for instruction in instructions {
        match instruction {
            MoveNorth { distance } => waypoint_y += distance,
            MoveSouth { distance } => waypoint_y -= distance,
            MoveEast { distance } => waypoint_x += distance,
            MoveWest { distance } => waypoint_x -= distance,
            TurnLeft { degrees } => rotate_waypoint(&mut waypoint_x, &mut waypoint_y, *degrees),
            TurnRight { degrees } => rotate_waypoint(&mut waypoint_x, &mut waypoint_y, -degrees),
            MoveForward { distance } => {
                ship_x += waypoint_x * distance;
                ship_y += waypoint_y * distance;
            }
        }
    }

    manhattan_distance(ship_x, ship_y)
}

fn rotate_waypoint(waypoint_x: &mut f64, waypoint_y: &mut f64, degrees: f64) {
    let current_rotation = (*waypoint_y).atan2(*waypoint_x);
    let new_rotation = current_rotation + degrees.to_radians();
    let distance_to_waypoint = (waypoint_x.powi(2) + waypoint_y.powi(2)).sqrt();

    *waypoint_x = new_rotation.cos() * distance_to_waypoint;
    *waypoint_y = new_rotation.sin() * distance_to_waypoint;
}

fn manhattan_distance(x: f64, y: f64) -> f64 {
    x.abs() + y.abs()
}

#[derive(Debug)]
enum Instruction {
    MoveNorth { distance: f64 },
    MoveSouth { distance: f64 },
    MoveEast { distance: f64 },
    MoveWest { distance: f64 },
    TurnLeft { degrees: f64 },
    TurnRight { degrees: f64 },
    MoveForward { distance: f64 },
}

#[cfg(test)]
mod tests {
    use crate::rotate_waypoint;

    #[test]
    fn rotate_minus_90_degrees() {
        // Arrange
        let mut waypoint_x = 10.0;
        let mut waypoint_y = 4.0;

        // Act
        rotate_waypoint(&mut waypoint_x, &mut waypoint_y, -90.0);

        // Assert
        assert_eq!(waypoint_x.round(), 4.0);
        assert_eq!(waypoint_y.round(), -10.0);
    }

    #[test]
    fn rotate_plus_90_degrees() {
        // Arrange
        let mut waypoint_x = 10.0;
        let mut waypoint_y = 4.0;

        // Act
        rotate_waypoint(&mut waypoint_x, &mut waypoint_y, 90.0);

        // Assert
        assert_eq!(waypoint_x.round(), -4.0);
        assert_eq!(waypoint_y.round(), 10.0);
    }

    #[test]
    fn rotate_plus_90_degrees_twice() {
        // Arrange
        let mut waypoint_x = 10.0;
        let mut waypoint_y = 4.0;

        // Act 1
        rotate_waypoint(&mut waypoint_x, &mut waypoint_y, 90.0);

        // Assert 1
        assert_eq!(waypoint_x.round(), -4.0);
        assert_eq!(waypoint_y.round(), 10.0);

        // Act 2
        rotate_waypoint(&mut waypoint_x, &mut waypoint_y, 90.0);

        // Assert 2
        assert_eq!(waypoint_x.round(), -10.0);
        assert_eq!(waypoint_y.round(), -4.0);
    }

    #[test]
    fn rotate_minus_180_degrees() {
        // Arrange
        let mut waypoint_x = 10.0;
        let mut waypoint_y = 4.0;

        // Act
        rotate_waypoint(&mut waypoint_x, &mut waypoint_y, -180.0);

        // Assert
        assert_eq!(waypoint_x.round(), -10.0);
        assert_eq!(waypoint_y.round(), -4.0);
    }

    #[test]
    fn rotate_plus_180_degrees() {
        // Arrange
        let mut waypoint_x = 10.0;
        let mut waypoint_y = 4.0;

        // Act
        rotate_waypoint(&mut waypoint_x, &mut waypoint_y, 180.0);

        // Assert
        assert_eq!(waypoint_x.round(), -10.0);
        assert_eq!(waypoint_y.round(), -4.0);
    }

    #[test]
    fn rotate_360_degrees() {
        // Arrange
        let mut waypoint_x = 10.0;
        let mut waypoint_y = 4.0;

        // Act
        rotate_waypoint(&mut waypoint_x, &mut waypoint_y, 360.0);

        // Assert
        assert_eq!(waypoint_x.round(), 10.0);
        assert_eq!(waypoint_y.round(), 4.0);
    }
}
