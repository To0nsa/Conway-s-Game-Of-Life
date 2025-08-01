use crate::utils::infinite_grid::extend_flat_grid_to_square64;

fn gol_two_step(curr: Vec<u8>, width: usize) -> Vec<u8>
{
    let height = width;
    let padded_width = width + 2;
    let mut padded_curr = vec![0u8; padded_width * padded_width];
    let mut padded_next = vec![0u8; padded_width * padded_width];

    for y in 0..height {
        let src = y * width;
        let dst = (y + 1) * padded_width + 1;
        padded_curr[dst..dst + width].copy_from_slice(&curr[src..src + width]);
    }

    let pw = padded_width as isize;
    let offs = [-(pw + 1), -pw, -(pw - 1), -1, 1, pw - 1, pw, pw + 1];

    for y in 1..=height {
        let row_off = (y * padded_width) as isize;
        for x in 1..=width {
            let idx = row_off + x as isize;
            let mut cnt = 0u8;
            unsafe {
                let ptr = padded_curr.as_ptr();
                for &o in &offs {
                    cnt += *ptr.offset(idx + o);
                }
                let cell = *ptr.offset(idx);
                let val = if (cell == 1 && (cnt == 2 || cnt == 3))
                    || (cell == 0 && cnt == 3)
                {
                    1
                } else {
                    0
                };
                *padded_next.as_mut_ptr().offset(idx) = val;
            }
        }
    }

    let mut out = curr;
    for y in 0..height {
        let dst = y * width;
        let src = (y + 1) * padded_width + 1;
        out[dst..dst + width].copy_from_slice(&padded_next[src..src + width]);
    }
    out
}

pub fn gol_two_infinite(
    mut grid: Vec<u8>,
    mut width: usize,
    iterations: usize,
) -> (Vec<u8>, usize)
{
    let mut aux = Vec::with_capacity(width * width);
    for _ in 0..iterations {
        let new_w = extend_flat_grid_to_square64(&grid, width, &mut aux);

        if new_w != width {
            std::mem::swap(&mut aux, &mut grid);
            width = new_w;
        }

        grid = gol_two_step(grid, width);
    }
    (grid, width)
}
