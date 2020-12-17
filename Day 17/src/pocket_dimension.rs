use std::collections::HashSet;

pub struct PocketDimension {
    active_cubes: HashSet<Coordinate>,
}

impl PocketDimension {
    pub fn from_str(input: &str) -> Self {
        let mut active_cubes = HashSet::new();
        let rows: Vec<&str> = input.split("\n").collect();
        for (row_index, row) in rows.iter().enumerate() {
            for (column_index, character) in row.chars().enumerate() {
                if character == '#' {
                    active_cubes.insert(Coordinate {
                        x: column_index as i32,
                        y: (rows.len() - row_index - 1) as i32,
                        z: 0,
                        w: 0,
                    });
                }
            }
        }
        Self { active_cubes }
    }

    pub fn number_of_active_cubes(&self) -> usize {
        self.active_cubes.len()
    }

    pub fn boot_cycle(self) -> Self {
        let cubes_that_might_next_be_active: HashSet<Coordinate> = self
            .active_cubes
            .iter()
            .flat_map(|coordinate| {
                let mut coordinates = coordinate.adjacent_coordinates();
                coordinates.push(*coordinate);
                coordinates
            })
            .collect();

        let next_active_cubes = cubes_that_might_next_be_active
            .iter()
            .filter(|coordinate| {
                let num_active_neighbours = coordinate
                    .adjacent_coordinates()
                    .iter()
                    .filter(|adjacent_coordinate| self.is_active_cube_at(adjacent_coordinate))
                    .count();

                if self.is_active_cube_at(coordinate) {
                    num_active_neighbours == 2 || num_active_neighbours == 3
                } else {
                    num_active_neighbours == 3
                }
            })
            .map(|&coordinate| coordinate)
            .collect();

        Self {
            active_cubes: next_active_cubes,
        }
    }

    fn is_active_cube_at(&self, coordinate: &Coordinate) -> bool {
        self.active_cubes.contains(coordinate)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coordinate {
    fn adjacent_coordinates(&self) -> Vec<Self> {
        let mut coordinates = Vec::with_capacity(80);
        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                for z in self.z - 1..=self.z + 1 {
                    for w in self.w - 1..=self.w + 1 {
                        let coordinate = Self { x, y, z, w };
                        if &coordinate != self {
                            coordinates.push(coordinate);
                        }
                    }
                }
            }
        }
        coordinates
    }
}
