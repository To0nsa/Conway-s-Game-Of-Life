use std::time::{Duration, Instant};

/* pub fn benchmark<G, F>(
    grid: &G,
    iterations: usize,
    mut simulate: F,
) -> (G, Duration)
where
    G: Clone,
    F: FnMut(G, usize) -> G,
{
    let input = grid.clone();
    let start = Instant::now();
    let final_grid = simulate(input, iterations);
    let elapsed = start.elapsed();
    (final_grid, elapsed)
}

pub fn benchmark_flat_grid_infinite<F>(
    flat: &Vec<u8>,
    width: usize,
    iterations: usize,
    mut simulate: F,
) -> ((Vec<u8>, usize), Duration)
where
    F: FnMut(Vec<u8>, usize, usize) -> (Vec<u8>, usize),
{
    let input = flat.clone();
    let start = Instant::now();
    let result = simulate(input, width, iterations);
    let elapsed = start.elapsed();
    (result, elapsed)
}

pub fn benchmark_bitboard<F>(
    initial: Vec<Vec<u64>>,
    width: usize,
    iterations: usize,
    life_fn: F,
) -> (Vec<Vec<u64>>, std::time::Duration)
where
    F: FnOnce(Vec<Vec<u64>>, usize, usize) -> Vec<Vec<u64>>,
{
    let start = Instant::now();
    let final_state = life_fn(initial, width, iterations);
    (final_state, start.elapsed())
} */

pub fn benchmark<Input, Output, F>(input: &Input, mut simulate: F) -> (Output, Duration)
where
    Input: Clone,
    F: FnMut(Input) -> Output,
{
    let start = Instant::now();
    let result = simulate(input.clone());
    let elapsed = start.elapsed();
    (result, elapsed)
}

