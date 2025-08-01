/* use std::{thread::sleep, time::Duration};

const ALIVE: char = 'X';
const DEAD: char = '.';
const CLEAR_SCREEN_SEQUENCE: &str = "\x1B[2J\x1B[1;1H";
const TICK_DURATION_MS: u64 = 100;

pub fn print_grid(grid: &[Vec<bool>])
{
    for row in grid {
        for &cell in row {
            let c = if cell { ALIVE } else { DEAD };
            print!("{}", c);
        }
        println!();
    }
}

pub fn print_flat_grid(grid: &[u8], width: usize)
{
    let height = grid.len() / width;
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let c = if grid[idx] != 0 { ALIVE } else { DEAD };
            print!("{}", c);
        }
        println!();
    }
}

pub fn print_bitboard(grid: &[Vec<u64>], width: usize)
{
    let words_per_row = width / 64;
    for row in grid {
        for word_idx in 0..words_per_row {
            let word = row[word_idx];
            for bit_idx in 0..64 {
                let c = if ((word >> bit_idx) & 1) == 1 {
                    ALIVE
                } else {
                    DEAD
                };
                print!("{}", c);
            }
        }
        println!();
    }
}

pub fn render_grid<F>(
    grid: &Vec<Vec<bool>>,
    iterations: usize,
    mut compute_next: F,
) where
    F: FnMut(&[Vec<bool>]) -> Vec<Vec<bool>>,
{
    let mut grid = grid.to_vec();
    let tick = Duration::from_millis(TICK_DURATION_MS);

    for _ in 0..iterations {
        print!("{}", CLEAR_SCREEN_SEQUENCE);

        for row in &grid {
            for &cell in row {
                print!("{}", if cell { ALIVE } else { DEAD });
            }
            println!();
        }

        grid = compute_next(&grid);
        sleep(tick);
    }
}
 */