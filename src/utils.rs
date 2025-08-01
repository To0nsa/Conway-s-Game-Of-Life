// src/utils.rs

use std::{fs::File, io::{BufRead, BufReader}, process};
use std::str::FromStr;
use std::{thread::sleep, time::Duration, time::Instant};

/// ANSI sequence to clear the terminal and move the cursor home
const CLEAR_SCREEN_SEQUENCE: &str = "\x1B[2J\x1B[1;1H";

/// Symbols for alive/dead cells
pub const ALIVE: char = 'X';
pub const DEAD:  char = '.';

/// How long to pause between frames (in milliseconds).
/// Tweak this constant to speed up / slow down the animation.
const TICK_DURATION_MS: u64 = 100;

/// Animate a Game‐of‐Life run on the terminal.
///
/// - `initial_grid`: your starting pattern  
/// - `iterations`:   how many generations to run  
/// - `compute_next`: callback that, given a `&[Vec<bool>]`, returns the next grid
pub fn render_grid<F>(
    grid: &Vec<Vec<bool>>,
    iterations: usize,
    mut compute_next: F,
) 
where
    F: FnMut(&[Vec<bool>]) -> Vec<Vec<bool>>,
{
    let mut grid = grid.to_vec();
    let tick = Duration::from_millis(TICK_DURATION_MS);

    for _ in 0..iterations {
        // 1) clear the screen
        print!("{}", CLEAR_SCREEN_SEQUENCE);

        // 2) render
        for row in &grid {
            for &cell in row {
                print!("{}", if cell { ALIVE } else { DEAD });
            }
            println!();
        }

        // 3) advance and pause
        grid = compute_next(&grid);
        sleep(tick);
    }
}

/// Print a single grid state to stdout (no clearing, no delay).
pub fn print_grid(grid: &[Vec<bool>]) {
    for row in grid {
        for &cell in row {
            // use the shared symbols
            print!("{}", if cell { ALIVE } else { DEAD });
        }
        println!();
    }
}

pub fn print_flat_grid(flat: &[u8], width: usize) {
    let height = flat.len() / width;
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            // use the same symbols as your existing print_grid
            print!("{}", if flat[idx] != 0 { ALIVE } else { DEAD });
        }
        println!();
    }
}

/// Flatten a 2D Vec<bool> into a single Vec<u8> (0 = dead, 1 = alive)
pub fn flatten_grid(grid: &[Vec<bool>]) -> Vec<u8> {
    let h = grid.len();
    let w = grid.get(0).map_or(0, |r| r.len());
    let mut flat = Vec::with_capacity(h * w);
    for row in grid {
        flat.extend(row.iter().map(|&b| b as u8));
    }
    flat
}

pub fn benchmark<G, F>(
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

/// Parse an argument from the `args` slice at position `index`,
/// giving it the name `name` for error messages.
/// Exits if missing or unparsable.
pub fn parse_arg<T: FromStr>(args: &[String], index: usize, name: &str) -> T
where
    T::Err: std::fmt::Display,
{
    if args.len() <= index {
        eprintln!("Missing argument: '{}'", name);
        process::exit(1);
    }
    let val = &args[index];
    val.parse::<T>().unwrap_or_else(|e| {
        eprintln!("Invalid {} ('{}'): {}", name, val, e);
        process::exit(1);
    })
}

/// Load an initial grid from a text file:
/// The file may have variable width per line; pads with DEAD as needed.
/// Alive cells are 'X', dead are '.', other chars cause an error.
pub fn load_grid(path: &str) -> Vec<Vec<bool>> {
    let file = File::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open {}: {}", path, e);
        process::exit(1);
    });
    let reader = BufReader::new(file);

    // Read all lines first
    let lines: Vec<String> = reader
        .lines()
        .map(|r| r.unwrap_or_else(|e| { eprintln!("I/O error: {}", e); process::exit(1) }))
        .collect();

    // Determine the maximum width
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Build the grid, padding short lines with dead cells
    lines.into_iter().map(|line| {
        let mut row = Vec::with_capacity(max_width);
        for ch in line.chars().take(max_width) {
            match ch {
                ALIVE => row.push(true),
                DEAD  => row.push(false),
                other => {
                    eprintln!("Unexpected character '{}' in {}", other, path);
                    process::exit(1);
                }
            }
        }
        while row.len() < max_width {
            row.push(false);
        }
        row
    }).collect()
}
