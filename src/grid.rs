use crate::{sample::TerrainSample, tile::Tile};
use rand::{
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use std::fmt::Display;

// pub struct Grid<T: Tile>(Vec<Vec<T>>);

// pub struct Grid<T: Tile>(Vec<Vec<Option<usize>>>);

pub struct Grid<T: Tile> {
    inner: Vec<Vec<Option<usize>>>,
    sample: TerrainSample<T>,
}

impl<T: Tile + Clone + PartialEq> Grid<T> {
    pub fn new(width: usize, height: usize, sample: TerrainSample<T>) -> Self {
        // Self(vec![vec![fill; height]; width])
        // Self(vec![vec![None; height]; width])

        Self {
            inner: vec![vec![None; height]; width],
            sample,
        }
    }

    pub fn collapse(&mut self) {
        // let seed: u64 = 127;
        let seed = rand::random();
        let mut rng = StdRng::seed_from_u64(seed);
        let (width, height) = (self.width(), self.height());

        let mut available_coordinates: Vec<(usize, usize)> = (0..width)
            .flat_map(|x| (0..height).map(|y| (x, y)).collect::<Vec<(usize, usize)>>())
            .collect();

        while available_coordinates.len() > 0 {
            // while !self.is_collapsed() {
            let grid_index = rng.gen_range(0..available_coordinates.len());
            let (x, y) = available_coordinates[grid_index];
            let tile_index = rng.gen_range(0..self.sample.tileset.len());

            available_coordinates.remove(grid_index);

            // let value = self.sample.get(tile_index).unwrap();
            // self.collapse_cell(x, y, value);
            self.collapse_cell(x, y);
            // self.propagate_constraints(&sample);
        }
    }

    pub fn generate(&mut self) -> Option<Vec<Vec<T>>> {
        if !self.is_collapsed() {
            // self.collapse();
            return None;
        }

        todo!()
    }

    fn width(&self) -> usize {
        self.inner.len()
    }

    fn height(&self) -> usize {
        self.inner.get(0).map_or(0, |column| column.len())
    }

    fn is_collapsed(&self) -> bool {
        self.inner.iter().all(|row| row.iter().all(Option::is_some))
    }

    // fn collapse_cell(&mut self, x: usize, y: usize, value: &T) {
    //     let Some(row) = self.inner.get_mut(x) else {
    //         return;
    //     };

    //     let Some(cell) = row.get_mut(y) else {
    //         return;
    //     };

    //     *cell = value.clone();
    // }

    fn collapse_cell(&mut self, x: usize, y: usize) {
        let mut rng = rand::thread_rng();
        let valid_neighbors_indices = self.get_valid_neighbors(x, y);

        // Ensure there are valid neighbors
        if !valid_neighbors_indices.is_empty() {
            // let new_value = *valid_neighbors_indices
            //     .choose(&mut rand::thread_rng())
            //     .unwrap();

            // Choose a random valid neighbor
            self.inner[x][y] = Some(*valid_neighbors_indices.choose(&mut rng).unwrap());

            return;

            // self.inner[x][y] = Some(self.sample.unwrap().get(new_value).cloned().unwrap());
        }

        self.inner[x][y] = Some((0..self.sample.tileset.len()).choose(&mut rng).unwrap());

        self.inner[x][y] = match valid_neighbors_indices.is_empty() {
            // Choose a random valid neighbor constraint
            true => Some((0..self.sample.tileset.len()).choose(&mut rng).unwrap()),
            // Otherwise, choose a random tile if there none
            false => Some(*valid_neighbors_indices.choose(&mut rng).unwrap()),
        };
    }

    fn get_valid_neighbors(&self, x: usize, y: usize) -> Vec<usize> {
        let mut valid_neighbors = self.get_neighbors(x, y);

        // Filter valid neighbors based on constraints
        valid_neighbors.retain(|&neighbor_index| {
            let constraints = &self.sample.constraints[neighbor_index];
            // self.sample.tileset.iter().position( |tile| tile == )
            // let tile_index = self.sample.get_tile_index(x, y).unwrap();
            // let constraints = &self.sample.constraints[tile_index];

            constraints.contains(&neighbor_index)
        });

        valid_neighbors
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let (width, height) = (self.inner.len(), self.inner[0].len());

        if x > 0 {
            if let Some(neighbor) = self.inner[x - 1][y] {
                neighbors.push(neighbor)
            }
        }

        if y > 0 {
            if let Some(neighbor) = self.inner[x][y - 1] {
                neighbors.push(neighbor)
            }
        }

        if x < width - 1 {
            if let Some(neighbor) = self.inner[x + 1][y] {
                neighbors.push(neighbor)
            }
        }

        if y < height - 1 {
            if let Some(neighbor) = self.inner[x][y + 1] {
                neighbors.push(neighbor)
            }
        }

        neighbors
    }

    // fn propagate_constraints(&mut self, terrain_sample: &TerrainSample<T>) {
    //     for (x, row) in self.inner.iter_mut().enumerate() {
    //         for (y, cell) in row.iter_mut().enumerate() {
    //             let tile_index = match terrain_sample.get_tile_index(x, y) {
    //                 Some(index) => index,
    //                 None => continue, // skip cells without a valid tile index
    //             };

    //             let valid_neighbors = terrain_sample.constraints[tile_index].clone();

    //             // Filter valid neighbors based on constraints
    //             let valid_neighbors_values: Vec<T> = valid_neighbors
    //                 .iter()
    //                 .filter_map(|&neighbor_index| terrain_sample.get(neighbor_index))
    //                 .cloned()
    //                 .collect();

    //             // Ensure the current cell's value is a valid neighbor
    //             if !valid_neighbors_values.contains(cell) {
    //                 // If not, choose a valid neighbor randomly
    //                 let new_value = *valid_neighbors
    //                     .choose(&mut rand::thread_rng())
    //                     .unwrap_or(&tile_index);

    //                 *cell = terrain_sample.get(new_value).cloned().unwrap();
    //             }
    //         }
    //     }
    // }
}

impl<T: Tile + Clone + PartialEq + Display + Default> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self
            .inner
            .iter()
            .map(|row| {
                row.iter()
                    .map(|index| index.map(|index| self.sample.get(index)).flatten())
                    .map(|into_tile| into_tile.map(Tile::value).unwrap_or_default())
                    .fold("".to_owned(), |row, cell| format!("{row}{cell}"))
            })
            .fold("".to_owned(), |message, row| format!("{message}{row}\n"));

        write!(f, "{message}")
    }
}
