mod pocket_dimension;

use pocket_dimension::PocketDimension;

const INPUT: &str = include_str!("initial-state.txt");

fn main() {
    let mut pocket_dimension = PocketDimension::from_str(&INPUT);
    for _ in 0..6 {
        pocket_dimension = pocket_dimension.boot_cycle();
    }
    println!(
        "number of active cubes = {}",
        pocket_dimension.number_of_active_cubes()
    );
}
