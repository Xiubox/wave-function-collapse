use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::vec::Vec;
use std::{
    collections::{BTreeSet, HashSet},
    fmt::{Debug, Display},
    time::Instant,
};

fn main() {
    let data: Vec<Vec<CharTile>> = include_str!("../sample-2.txt")
        .lines()
        .map(|line| line.chars().map(CharTile::from).collect())
        .collect();
    let terrain_sample = TerrainSample::new(data);

    let mut grid = Grid::new(25, 25, CharTile::default());

    let now = Instant::now();
    grid.collapse(terrain_sample);
    let elapsed = now.elapsed();

    println!("\nFinished in {}μs:\n{}", elapsed.as_micros(), grid);
}

trait Tile {
    type Output: Display;

    fn is_collapsed(&self) -> bool;

    fn value(&self) -> Self::Output;
}

struct Tileset<T: Tile>(Vec<T>);

impl<T: Tile + Display> Tileset<T> {
    pub fn new(tiles: Vec<T>) -> Self {
        Self(tiles)
    }
}

impl<T: Tile + Display> Display for Tileset<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self
            .0
            .iter()
            .fold("".to_owned(), |message, tile| format!("{message}{tile}"));

        writeln!(f, "{message}")
    }
}

struct Grid<T: Tile>(Vec<Vec<T>>);

impl<T: Tile + Clone + PartialEq + Debug> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        Self(vec![vec![fill; height]; width])
    }

    pub fn collapse(&mut self, sample: TerrainSample<T>) {
        // let seed: u64 = 127;
        let seed = rand::random();
        let mut rng = StdRng::seed_from_u64(seed);
        let (width, height) = (self.width(), self.height());

        let mut available_coordinates: Vec<(usize, usize)> = (0..width)
            .flat_map(|x| (0..height).map(|y| (x, y)).collect::<Vec<(usize, usize)>>())
            .collect();

        while !self.is_collapsed() {
            let grid_index = rng.gen_range(0..available_coordinates.len());
            let (x, y) = available_coordinates[grid_index];
            let tile_index = rng.gen_range(0..sample.tileset.len());

            available_coordinates.remove(grid_index);

            let value = sample.get(tile_index).unwrap();
            self.collapse_cell(x, y, value);
            self.propagate_constraints(&sample);
        }
    }

    fn width(&self) -> usize {
        self.0.len()
    }

    fn height(&self) -> usize {
        self.0.get(0).map_or(0, |column| column.len())
    }

    fn is_collapsed(&self) -> bool {
        self.0.iter().all(|row| row.iter().all(Tile::is_collapsed))
    }

    fn collapse_cell(&mut self, x: usize, y: usize, value: &T) {
        let Some(row) = self.0.get_mut(x) else {
            return;
        };

        let Some(cell) = row.get_mut(y) else {
            return;
        };

        *cell = value.clone();
    }

    fn propagate_constraints(&mut self, terrain_sample: &TerrainSample<T>) {
        for (x, row) in self.0.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                let tile_index = match terrain_sample.get_tile_index(x, y) {
                    Some(index) => index,
                    None => continue, // skip cells without a valid tile index
                };

                let valid_neighbors = terrain_sample.constraints[tile_index].clone();

                // Filter valid neighbors based on constraints
                let valid_neighbors_values: Vec<T> = valid_neighbors
                    .iter()
                    .filter_map(|&neighbor_index| terrain_sample.get(neighbor_index))
                    .cloned()
                    .collect();

                // Ensure the current cell's value is a valid neighbor
                if !valid_neighbors_values.contains(cell) {
                    // If not, choose a valid neighbor randomly
                    let new_value = *valid_neighbors
                        .choose(&mut rand::thread_rng())
                        .unwrap_or(&tile_index);

                    *cell = terrain_sample.get(new_value).cloned().unwrap();
                }
            }
        }
    }
}

impl<T: Tile + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(Tile::value)
                    .fold("".to_owned(), |row, cell| format!("{row}{cell}"))
            })
            .fold("".to_owned(), |message, row| format!("{message}{row}\n"));

        write!(f, "{message}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct CharTile(char);

impl Default for CharTile {
    fn default() -> Self {
        Self('.')
    }
}

impl From<char> for CharTile {
    fn from(value: char) -> Self {
        Self(value)
    }
}

impl Tile for CharTile {
    type Output = char;

    fn is_collapsed(&self) -> bool {
        self.0 != '.'
    }

    fn value(&self) -> Self::Output {
        self.0
    }
}

impl Display for CharTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for CharTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct TerrainSample<T: Tile> {
    tileset: Vec<T>,
    map: Vec<Vec<usize>>,
    constraints: Vec<Vec<usize>>,
}

impl<T: Tile + Clone + PartialEq + Debug> TerrainSample<T> {
    pub fn new(mut data: Vec<Vec<T>>) -> Self {
        let mut tileset = Vec::new();
        let map: Vec<Vec<usize>> = data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| {
                        tileset.iter().position(|t| t == tile).unwrap_or_else(|| {
                            let index = tileset.len();
                            tileset.push(tile.clone());

                            index
                        })
                    })
                    .collect()
            })
            .collect();
        let constraints = Self::generate_constraints(&tileset, &map);

        Self {
            tileset,
            map,
            constraints,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.tileset.get(index)
    }

    pub fn get_tile_index(&self, x: usize, y: usize) -> Option<usize> {
        self.map.get(x).and_then(|row| row.get(y).cloned())
    }

    fn generate_constraints(tileset: &Vec<T>, map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut constraints = vec![BTreeSet::new(); tileset.len()];

        map.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, &tile_index)| {
                constraints[tile_index].extend(Self::get_neighbors(map, x, y))
            });

            // constraints
        });

        println!("{tileset:?}\n{constraints:?}");

        constraints
            .into_iter()
            .map(|set| set.into_iter().collect::<Vec<usize>>())
            .collect()
    }

    // fn generate_constraints(tileset: &Vec<T>, map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    //     map.iter()
    //         .enumerate()
    //         .fold(vec![vec![]; tileset.len()], |mut constraints, (x, row)| {
    //             row.iter().enumerate().for_each(|(y, &tile_index)| {
    //                 let valid_neighbors = Self::get_neighbors(map, x, y);
    //                 let valid_neighbors_indices: Vec<usize> = valid_neighbors
    //                     .iter()
    //                     .filter_map(|&(neighbor_x, neighbor_y)| {
    //                         map.get(neighbor_x).and_then(|r| r.get(neighbor_y).cloned())
    //                     })
    //                     .collect();

    //                 constraints[tile_index].extend(valid_neighbors_indices)
    //             });

    //             constraints
    //         });

    //     map.iter()
    //         .enumerate()
    //         .fold(vec![vec![]; tileset.len()], |mut constraints, (x, row)| {
    //             row.iter().enumerate().for_each(|(y, &tile_index)| {
    //                 let valid_neighbors = Self::get_neighbors(map, x, y);
    //                 let valid_neighbor_indicies = valid_neighbors.iter().filter_map(|&|)

    //                 constraints[tile_index].extend(valid_neighbor_indicies);
    //             });

    //             constraints
    //         })
    // }

    fn get_neighbors(map: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();

        if x > 0 {
            neighbors.push(map[x - 1][y]);
        }

        if y > 0 {
            neighbors.push(map[x][y - 1]);
        }

        if x < map.len() - 1 {
            neighbors.push(map[x + 1][y]);
        }

        if y < map[0].len() - 1 {
            neighbors.push(map[x][y + 1]);
        }

        neighbors
    }
}
