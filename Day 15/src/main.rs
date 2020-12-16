use std::collections::HashMap;

fn main() {
    let input = vec![14, 3, 1, 0, 9, 5];
    println!("Part 1 - {}", play_memory_game(&input, 2020));
    println!("Part 2 - {}", play_memory_game(&input, 30_000_000));
}

fn play_memory_game(starting_numbers: &Vec<usize>, end_turn: usize) -> usize {
    let mut game_history = HashMap::new();
    let mut number_last_spoken = 0;

    for turn in 0..starting_numbers.len() {
        let number = starting_numbers[turn];
        number_last_spoken = number;
        game_history.insert(number, vec![turn]);
    }

    for turn in starting_numbers.len()..end_turn {
        let history_for_number = game_history.get_mut(&number_last_spoken).unwrap();
        let next_number = match history_for_number.len() {
            1 => 0, // Number was not spoken before
            _ => (turn - 1) - history_for_number[history_for_number.len() - 2],
        };

        if let Some(history_for_next_number) = game_history.get_mut(&next_number) {
            history_for_next_number.push(turn);
        } else {
            game_history.insert(next_number, vec![turn]);
        }

        number_last_spoken = next_number;
    }

    number_last_spoken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_memory_game_test() {
        assert_eq!(play_memory_game(&vec![0, 3, 6], 2020), 436);
        assert_eq!(play_memory_game(&vec![1, 3, 2], 2020), 1);
        assert_eq!(play_memory_game(&vec![2, 1, 3], 2020), 10);
        assert_eq!(play_memory_game(&vec![1, 2, 3], 2020), 27);
        assert_eq!(play_memory_game(&vec![2, 3, 1], 2020), 78);
        assert_eq!(play_memory_game(&vec![3, 2, 1], 2020), 438);
        assert_eq!(play_memory_game(&vec![3, 1, 2], 2020), 1836);
    }
}
