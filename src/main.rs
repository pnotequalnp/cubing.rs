#![feature(array_map)]
#![feature(iter_intersperse)]

mod corners;
mod kociemba;
mod turns;

use crate::turns::FaceTurn::{self, *};
use cube::search::Search;
use kociemba::Phase1;
use std::time::Instant;

fn main() {
    kociemba();
}

fn kociemba() {
    println!("Generating move table...");
    let now = Instant::now();
    let move_table = Phase1::create_table();
    println!("Generated move table in {:?}\n", now.elapsed());

    println!("Generating pruning table...");
    let now = Instant::now();
    let pruning_table = Phase1::create_pruning_table(&move_table);
    println!("Generated move table in {:?}\n", now.elapsed());

    let superflip = vec![U, R2, F, B, R, B2, R, U2, L, B2, R, U3, D3, R2, F, R3, L, B2, U2, F2];
    // let superflip = vec![L, B, U, R2, R, R, U2, R, U3, R2, F, R3, U2, F2];
    // let scramble = vec![R2, U3, F2, R, U2, R, U, R2, F3];
    let scramble = superflip;
    let position = scramble.iter().cloned().collect::<Phase1>();

    println!("Solving scramble [{}]...\n({:?})", FaceTurn::format_seq(scramble.into_iter()), position);
    let now = Instant::now();
    if let Some((_path, moves)) = position.ida_star(
        &pruning_table,
        &move_table,
        10,
        Some(|depth| println!("Depth {:?} complete in {:?}", depth, now.clone().elapsed())),
    ) {
        println!("Solved in {:?}", now.elapsed());
        let solution: Vec<FaceTurn> =
            moves.into_iter().map(FaceTurn::from).collect();

        println!("[{}]", FaceTurn::format_seq(solution.into_iter()));
    } else {
        println!("No solution found in {:?}", now.elapsed());
    }
}

#[allow(dead_code)]
fn corner_cube() {
    println!("Generating move table...");
    let now = Instant::now();
    let move_table = corners::Table::new();
    println!("Generated move table in {:?}\n", now.elapsed());

    println!("Generating pruning table...");
    let now = Instant::now();
    let pruning_table = corners::PruningTable::new(&move_table);
    println!("Generated pruning table in {:?}\n", now.elapsed());

    let scramble = vec![R2, U3, F2, R, U2, R, U, R2, F3];

    let position: corners::Cube = scramble.iter().cloned().collect();

    println!("Solving scramble {:?}...", scramble);
    let now = Instant::now();
    let path = position.ida_star(
        &pruning_table,
        &move_table,
        10,
        Some(|depth| println!("Depth {:?} complete", depth)),
    );
    println!("Solved in {:?}", now.elapsed());
    // let solution: Option<Vec<FaceTurn>> =
    //     path.map(|v| v.iter().map(|(_, e)| corners::Cube::index(*e)).collect());

    // println!("{:?}", solution);
}
