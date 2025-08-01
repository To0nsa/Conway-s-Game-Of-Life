use rayon::prelude::*;

use crate::utils::infinite_grid::extend_flat_grid_to_square64;

fn gol_three_step(curr: Vec<u8>, width: usize) -> Vec<u8>
{
    let area = curr.len();
    let height = area / width;
    let padded_width = width + 2;
    let padded_height = height + 2;

    let mut padded_curr = vec![0u8; padded_width * padded_height];
    let mut padded_next = vec![0u8; padded_width * padded_height];

    for y in 0..height {
        let src_off = y * width;
        let dst_off = (y + 1) * padded_width + 1;
        padded_curr[dst_off..dst_off + width]
            .copy_from_slice(&curr[src_off..src_off + width]);
    }

    let pw = padded_width as isize;
    let offsets: [isize; 8] =
        [-(pw + 1), -pw, -(pw - 1), -1, 1, pw - 1, pw, pw + 1];

    let curr_addr = padded_curr.as_ptr() as usize;
    let next_addr = padded_next.as_mut_ptr() as usize;

    (1..=height).into_par_iter().for_each(move |y| {
        let curr_ptr = curr_addr as *const u8;
        let next_ptr = next_addr as *mut u8;
        let row_off = (y * padded_width) as isize;
        for x in 1..=width {
            let idx = row_off + (x as isize);
            let mut cnt: u8 = 0;
            unsafe {
                for &off in &offsets {
                    cnt += *curr_ptr.offset(idx + off);
                }
                let cell = *curr_ptr.offset(idx);
                let val = if (cell == 1 && (cnt == 2 || cnt == 3))
                    || (cell == 0 && cnt == 3)
                {
                    1
                } else {
                    0
                };
                *next_ptr.offset(idx) = val;
            }
        }
    });

    let mut out = curr;
    for y in 0..height {
        let src_off = (y + 1) * padded_width + 1;
        let dst_off = y * width;
        out[dst_off..dst_off + width]
            .copy_from_slice(&padded_next[src_off..src_off + width]);
    }
    out
}

pub fn gol_three_infinite(
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

        grid = gol_three_step(grid, width);
    }
    (grid, width)
}
