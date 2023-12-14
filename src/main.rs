use crate::{grid::Grid, sample::TerrainSample, tile::CharTile};
use std::time::Instant;

mod grid;
mod sample;

fn main() {
    let data: Vec<Vec<CharTile>> = include_str!("../sample-2.txt")
        .lines()
        .map(|line| line.chars().map(CharTile::from).collect())
        .collect();
    let terrain_sample = TerrainSample::new(data);
    let mut grid = Grid::new(25, 25, terrain_sample);

    benchmark(|| grid.collapse());

    println!("{grid}");
}

fn benchmark(f: impl FnOnce()) {
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();

    println!("Finished in {}μs:", elapsed.as_micros());
}

mod tile;
