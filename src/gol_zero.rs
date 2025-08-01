/// src/gol_zero.rs

/// A 2D grid of cells: `true` = alive, `false` = dead.
type Grid = Vec<Vec<bool>>;

/// Context for querying and updating a specific cell within the grid.
struct CellContext<'a> {
    grid: &'a Grid,
    row: usize,
    col: usize,
    height: usize,
    width: usize,
}

impl<'a> CellContext<'a> {
    /// Check if the neighbor at offset (dr, dc) is within the grid bounds.
    fn in_bounds(&self, dr: i32, dc: i32) -> bool {
        let nr = self.row as i32 + dr;
        let nc = self.col as i32 + dc;
        (0 <= nr && nr < self.height as i32) && (0 <= nc && nc < self.width as i32)
    }

    /// Return whether the neighbor at offset (dr, dc) is alive (and in bounds).
    fn is_alive(&self, dr: i32, dc: i32) -> bool {
        if self.in_bounds(dr, dc) {
            let nr = (self.row as i32 + dr) as usize;
            let nc = (self.col as i32 + dc) as usize;
            self.grid[nr][nc]
        } else {
            false
        }
    }

    /// Count how many of the eight surrounding neighbors are alive.
    fn count_live_neighbors(&self) -> usize {
        let mut count = 0;
        for &dr in &[-1, 0, 1] {
            for &dc in &[-1, 0, 1] {
                if dr == 0 && dc == 0 {
                    continue;
                }
                if self.is_alive(dr, dc) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Determine the next state of this cell according to the Game of Life rules.
    fn next_state(&self) -> bool {
        let live_neighbors = self.count_live_neighbors();
        match (self.grid[self.row][self.col], live_neighbors) {
            (true, 2) | (true, 3) => true,
            (false, 3)            => true,
            _                     => false,
        }
    }
}

/// Compute the next generation for the entire grid.
pub fn compute_next_generation(current: &Grid) -> Grid {
    let height = current.len();
    let width  = current.get(0).map_or(0, |r| r.len());
    let mut next = vec![vec![false; width]; height];

    for row in 0..height {
        for col in 0..width {
            let ctx = CellContext { grid: current, row, col, height, width };
            next[row][col] = ctx.next_state();
        }
    }

    next
}

/// Advance the grid by a number of iterations.
/// Returns the final grid state.
pub fn gol_zero(mut grid: Grid, iterations: usize) -> Grid {
    for _ in 0..iterations {
        grid = compute_next_generation(&grid);
    }
    grid
}
