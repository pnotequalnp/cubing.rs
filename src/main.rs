fn main() {
    kociemba();
}

#[allow(dead_code)]
fn random() {
    use cubing::algorithms::kociemba;
    use cubing::metric::Htm;

    let tables = kociemba::generate_tables();
    let position = cubing::puzzle::Cube3x3::random_state();
    let mut solution = position.kociemba(&tables, None);
    solution.reverse();
    println!("{}", Htm::format_seq(solution.into_iter()));
}

#[allow(dead_code)]
fn kociemba() {
    use cubing::algorithms::kociemba;
    use cubing::puzzle::positions;
    use std::time::Instant;

    println!("Generating Kociemba tables...");
    let now = Instant::now();
    let tables = kociemba::generate_tables();
    println!("Generated in {:?}\n", now.elapsed());

    let position = positions::SUPER_FLIP;

    use kociemba::Phase1;
    let mut solver = cubing::core::search::dfs_iter(
        Phase1::from(&position),
        Phase1::default(),
        &tables.1,
        &tables.0,
        11,
    );

    let now = Instant::now();
    let _sol = solver.next();
    let time = now.elapsed();
    println!(
        "Nodes: {:?}, time: {:?}, NPS: {:?}\n",
        solver.nodes(),
        time,
        solver.nodes() as f64 / time.as_secs_f64(),
    );

    println!("Solving superflip...",);
    let now = Instant::now();
    let solution = position.kociemba(&tables, Some(22));
    println!("Solved in {:?}", now.elapsed());
    println!(
        "[{}]",
        cubing::metric::Htm::format_seq(solution.into_iter())
    );
}
