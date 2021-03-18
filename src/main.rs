#![feature(array_map)]
#![feature(iter_intersperse)]

// mod corners;
mod kociemba;
mod rubiks;

use rubiks::FaceTurn::{self, *};
use std::time::Instant;

fn main() {
    kociemba();
}

fn kociemba() {
    println!("Generating phase 1 move table...");
    let now = Instant::now();
    let move_table_1 = kociemba::Phase1::create_table();
    println!("Generated in {:?}\n", now.elapsed());

    println!("Generating phase 2 move table...");
    let now = Instant::now();
    let move_table_2 = kociemba::Phase2::create_table();
    println!("Generated in {:?}\n", now.elapsed());

    println!("Generating phase 1 pruning table...");
    let now = Instant::now();
    let pruning_table_1 = kociemba::Phase1::create_pruning_table(&move_table_1);
    println!("Generated in {:?}\n", now.elapsed());

    println!("Generating phase 2 pruning table...");
    let now = Instant::now();
    let pruning_table_2 = kociemba::Phase2::create_pruning_table(&move_table_2);
    println!("Generated in {:?}\n", now.elapsed());

    let scramble = vec![
        U, R2, F, B, R, B2, R, U2, L, B2, R, U3, D3, R2, F, R3, L, B2, U2, F2,
    ];
    let _scramble = vec![R2, U3, F2, R, U2, R, U, R2, F3, F, D3, L, U3, R3, F2, D, R];

    println!(
        "Solving scramble [{}]...",
        FaceTurn::format_seq(scramble.iter().cloned()),
    );
    let now = Instant::now();
    let solution = kociemba::solve(
        scramble,
        &move_table_1,
        &move_table_2,
        &pruning_table_1,
        &pruning_table_2,
        Some(22),
    );
    println!("Solved in {:?}", now.elapsed());
    println!("[{}]", FaceTurn::format_seq(solution.into_iter()));
}
