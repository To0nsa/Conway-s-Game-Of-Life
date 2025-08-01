// src/main.rs

mod gol_zero;
mod gol_one;
mod gol_two;
mod gol_three;
mod utils;

use std::env;
use utils::{parse_arg, load_grid, benchmark, print_grid, flatten_grid, print_flat_grid};
use utils::{render_grid};

use crate::gol_zero::gol_zero;
use crate::gol_one::gol_one;
use crate::gol_two::gol_two;
use crate::gol_three::gol_three;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <initial_state_file> <iterations>", args[0]);
        std::process::exit(1);
    }

    let file_path: String  = parse_arg(&args, 1, "initial_state_file");
    let iterations: usize  = parse_arg(&args, 2, "iterations");

    // 1) Load the 2D grid
    let initial_grid = load_grid(&file_path);

    // 2) Run & display the gol_zero and gol_one benchmarks
    render_grid(&initial_grid, iterations, gol_one::compute_next_generation);
    print_grid(&initial_grid);

    let (final_state, elapsed) = benchmark(&initial_grid, iterations, gol_zero);
    println!("Simulation with gol_zero of {} generations took {:?}", iterations, elapsed);
    print_grid(&final_state);

    let (final_state, elapsed) = benchmark(&initial_grid, iterations, gol_one);
    println!("Simulation with gol_one of {} generations took {:?}", iterations, elapsed);
    print_grid(&final_state);

    // 3) Prepare for flat‐buffer gol_two
    let width     = initial_grid[0].len();
    let flat_grid = flatten_grid(&initial_grid);

    // 4) Benchmark the new flat version by wrapping gol_two in a closure
    let (final_flat, elapsed_flat) = benchmark(&flat_grid, iterations, |buf, iters| {
        gol_two(buf, width, iters)
    });
    println!("Simulation with gol_two (flat) of {} generations took {:?}", iterations, elapsed_flat);
    print_flat_grid(&final_flat, width);

    let (final_flat, elapsed_flat) = benchmark(&flat_grid, iterations, |buf, iters| {
        gol_three(buf, width, iters)
    });
    println!("Simulation with gol_three (flat) of {} generations took {:?}", iterations, elapsed_flat);
    print_flat_grid(&final_flat, width);

}
