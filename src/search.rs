use crate::permutations::*;
use alloc::vec::Vec;
use core::fmt::Write;

pub fn search<const COUNT: u8, const GENERATORS: usize>(
    output: &mut dyn Write,
    position: Coordinate<COUNT>,
    generators: &Generators<GENERATORS>,
    table: &MoveTable<COUNT, GENERATORS>,
) -> Vec<usize>
where
    [u8; upscale(COUNT)]: Sized,
{
    (0..)
        .find_map(|depth| {
            let res = search_depth(position, generators, table, depth);
            let _ = write!(output, "Depth {} complete", depth);
            res
        })
        .unwrap()
}

fn search_depth<const COUNT: u8, const GENERATORS: usize>(
    position: Coordinate<COUNT>,
    generators: &Generators<GENERATORS>,
    table: &MoveTable<COUNT, GENERATORS>,
    depth: u32,
) -> Option<Vec<usize>>
where
    [u8; upscale(COUNT)]: Sized,
{
    match depth {
        0 => position.is_zero().then(|| Vec::new()),
        _ => generators.0.iter().find_map(|gen| {
            let pos = table.0[position.0 as usize][*gen];
            let res = search_depth(pos, generators, table, depth - 1);
            match res {
                None => None,
                Some(mut v) => {
                    v.push(*gen);
                    Some(v)
                }
            }
        }),
    }
}
