#![feature(array_map)]

mod corners;
mod kociemba;
mod turns;

use corners::*;
use cube::search::Search;
use std::time::Instant;

fn main() {
    use crate::turns::FaceTurn::{self, *};

    println!("Generating move table...");
    let now = Instant::now();
    let move_table = Table::new();
    println!("Generated move table in {:?}\n", now.elapsed());

    println!("Generating pruning table...");
    let now = Instant::now();
    let pruning_table = PruningTable::new(&move_table);
    println!("Generated pruning table in {:?}\n", now.elapsed());

    let scramble = vec![U2, R3, U2, R2, U2, R2, U3, R2, U3, R2, U, R];
    let scramble = vec![R2, U3, F2, R, U2, R, U, R2, F3];

    let position: Cube = scramble.iter().cloned().collect();

    println!("Solving scramble {:?}...", scramble);
    let now = Instant::now();
    let path = position.ida_star(
        &pruning_table,
        &move_table,
        10,
        Some(|depth| println!("Depth {:?} complete", depth)),
    );
    println!("Solved in {:?}", now.elapsed());
    let solution: Option<Vec<FaceTurn>> =
        path.map(|v| v.iter().map(|(_, e)| Cube::index(*e)).collect());

    println!("{:?}", solution);
}
