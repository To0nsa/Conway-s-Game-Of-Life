use std::{thread::sleep, time::Duration};

// grid dimensions
const WIDTH: usize = 32;
const HEIGHT: usize = 32;

// rendering
const ALIVE: char = 'X'; //'█';
const DEAD:  char = ' ';
const CLEAR_SCREEN_SEQUENCE: &str = "\x1B[2J\x1B[1;1H";

// timing
const TICK_DURATION_MS: u64 = 200;
const TICK_DURATION: Duration = Duration::from_millis(TICK_DURATION_MS);

// initial pattern: a “glider”
const GLIDER: &[(usize, usize)] = &[
    (1, 2),
    (2, 3),
    (3, 1),
    (3, 2),
    (3, 3),
];

/// Compute the next generation from the current grid.
fn compute_next_generation(
    current_grid: &[[bool; WIDTH]; HEIGHT]
) -> [[bool; WIDTH]; HEIGHT] {
    let mut next_grid: [[bool; 32]; 32] = [[false; WIDTH]; HEIGHT];

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            // count live neighbors
            let mut live_neighbor_count: i32 = 0;
            for y_offset in [-1isize, 0, 1] {
                for x_offset in [-1isize, 0, 1] {
                    if y_offset == 0 && x_offset == 0 {
                        continue; // skip the cell itself
                    }
                    let neighbor_row: isize = row as isize + y_offset;
                    let neighbor_col: isize = col as isize + x_offset;

                    if neighbor_row >= 0
                        && neighbor_row < HEIGHT as isize
                        && neighbor_col >= 0
                        && neighbor_col < WIDTH as isize
                    {
                        if current_grid[neighbor_row as usize][neighbor_col as usize] {
                            live_neighbor_count += 1;
                        }
                    }
                }
            }

            // apply Conway's rules
            let is_currently_alive: bool = current_grid[row][col];
            next_grid[row][col] = match (is_currently_alive, live_neighbor_count) {
                (true, 2) | (true, 3)  => true,   // survival
                (false, 3)             => true,   // reproduction
                _                      => false,  // death or remains empty
            };
        }
    }

    next_grid
}

/// Clear the terminal screen (ANSI escape).
fn clear_screen() {
    print!("{}", CLEAR_SCREEN_SEQUENCE);
}

fn main() {
    // create an empty grid
    let mut grid: [[bool; 32]; 32] = [[false; WIDTH]; HEIGHT];

    // seed the glider pattern
    for &(row, col) in GLIDER {
        grid[row][col] = true;
    }

    // simulation loop
    loop {
        clear_screen();

        // render current grid
        for row in &grid {
            for &cell in row {
                print!("{}", if cell { ALIVE } else { DEAD });
            }
            println!();
        }

        // advance to next generation
        grid = compute_next_generation(&grid);

        // pause between ticks
        sleep(TICK_DURATION);
    }
}
