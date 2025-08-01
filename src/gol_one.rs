// src/gol_one.rs

/// Compute the next generation for any rectangular grid.
pub fn compute_next_generation(current: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let height = current.len();
    let width  = current.get(0).map_or(0, |row| row.len());
    let mut next = vec![vec![false; width]; height];

    for y in 0..height {
        // determine vertical bounds
        let y_start = if y == 0        { 0 } else { y - 1 };
        let y_end   = if y + 1 < height { y + 1 } else { height - 1 };

        for x in 0..width {
            // determine horizontal bounds
            let x_start = if x == 0       { 0 } else { x - 1 };
            let x_end   = if x + 1 < width { x + 1 } else { width - 1 };

            // count live neighbours
            let mut live_neighbors = 0;
            for ny in y_start..=y_end {
                for nx in x_start..=x_end {
                    if (ny != y || nx != x) && current[ny][nx] {
                        live_neighbors += 1;
                    }
                }
            }

            // apply Game-of-Life rules
            next[y][x] = match (current[y][x], live_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3)            => true,
                _                     => false,
            };
        }
    }

    next
}

/// Advance the given `grid` for `iterations` steps,
/// performing only in-memory updates (no I/O or rendering).
/// Returns the final grid state.
pub fn gol_one(mut grid: Vec<Vec<bool>>, iterations: usize) -> Vec<Vec<bool>> {
    for _ in 0..iterations {
        grid = compute_next_generation(&grid);
    }
    grid
}
