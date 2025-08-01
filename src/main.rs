// src/main.rs

use std::env;
use utils::{
    benchmark::benchmark,
    conversion::{flatten_grid, to_bitboards},
    initialize_grid::{load_grid, parse_arg},
};

mod gol_five;
mod gol_four;
mod gol_one;
mod gol_three;
mod gol_three_infinite;
mod gol_two;
mod gol_two_infinite;
mod gol_zero;
mod utils;

use crate::{
    gol_five::gol_five,
    gol_four::gol_four,
    gol_one::gol_one,
    gol_three::gol_three,
    gol_three_infinite::gol_three_infinite,
    gol_two::gol_two,
    gol_two_infinite::gol_two_infinite,
    gol_zero::gol_zero,
};

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <initial_state_file> <iterations>", args[0]);
        std::process::exit(1);
    }

    let file_path: String = parse_arg(&args, 1, "initial_state_file");
    let iterations: usize = parse_arg(&args, 2, "iterations");

    // Load & preprocess
    let initial_grid = load_grid(&file_path);
    let width = initial_grid[0].len();
    let flat_grid = flatten_grid(&initial_grid);
    let (bitboards, _bb_width) = to_bitboards(&initial_grid);

    // Benchmark simulations
    let (_, elapsed) = benchmark(&initial_grid, |grid| gol_zero(grid, iterations));
    println!("Simulation with gol_zero took {:?}", elapsed);

    let (_, elapsed) = benchmark(&initial_grid, |grid| gol_one(grid, iterations));
    println!("Simulation with gol_one took {:?}", elapsed);

    let (_, elapsed) = benchmark(&flat_grid, |flat| gol_two(flat, width, iterations));
    println!("Simulation with gol_two (flat) took {:?}", elapsed);

    let ((_, _), elapsed) = benchmark(&(flat_grid, width), |(flat, w)| gol_two_infinite(flat, w, iterations));
    println!("Simulation with gol_two_infinite (flat) took {:?}", elapsed);

    let flat_grid = flatten_grid(&initial_grid);
    let (_, elapsed) = benchmark(&flat_grid, |flat| gol_three(flat, width, iterations));
    println!("Simulation with gol_three (flat) took {:?}", elapsed);

    let ((_, _), elapsed) = benchmark(&(flat_grid, width), |(flat, w)| gol_three_infinite(flat, w, iterations));
    println!("Simulation with gol_three_infinite (flat) took {:?}", elapsed);

    let (_, elapsed) = benchmark(&(bitboards.clone(), width), |(bb, w)| gol_four(bb, w, iterations));
    println!("Simulation with gol_four (bitboard) took {:?}", elapsed);

    let (_, elapsed) = benchmark(&(bitboards, width), |(bb, w)| gol_five(bb, w, iterations));
    println!("Simulation with gol_five (bitboard) took {:?}", elapsed);
}