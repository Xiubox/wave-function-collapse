use crate::tile::Tile;
use std::collections::BTreeSet;

pub struct TerrainSample<T: Tile> {
    pub tileset: Vec<T>,
    map: Vec<Vec<usize>>,
    pub constraints: Vec<Vec<usize>>,
}

impl<T: Tile + Clone + PartialEq> TerrainSample<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
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

    pub fn get_valid_neighbors(&self, x: usize, y: usize) -> Vec<usize> {
        let mut valid_neighbors = Self::get_neighbors(&self.map, x, y);

        // Filter valid neighbors based on constraints
        valid_neighbors.retain(|&neighbor_index| {
            let tile_index = self.get_tile_index(x, y).unwrap();
            let constraints = &self.constraints[tile_index];

            constraints.contains(&neighbor_index)
        });

        valid_neighbors
    }

    fn generate_constraints(tileset: &Vec<T>, map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut constraints = vec![BTreeSet::new(); tileset.len()];

        map.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, &tile_index)| {
                constraints[tile_index].extend(Self::get_neighbors(map, x, y))
            });
        });

        constraints
            .into_iter()
            .map(|set| set.into_iter().collect::<Vec<usize>>())
            .collect()
    }

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
