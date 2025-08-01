/// Optimized flat-buffer Game of Life using border padding and unsafe pointer arithmetic
/// - input: finite grid as Vec<u8> (row-major, 0=dead, 1=alive)
/// - width: number of columns
/// - iterations: number of generations
/// returns: Vec<u8> in the same flat format
pub fn gol_two(mut curr: Vec<u8>, width: usize, iterations: usize) -> Vec<u8> {
    let area = curr.len();
    let height = area / width;

    // Dimensions of padded grid (one-cell border)
    let padded_width = width + 2;
    let padded_height = height + 2;

    // Allocate padded buffers initialized to 0 (dead cells)
    let mut padded_curr = vec![0u8; padded_width * padded_height];
    let mut padded_next = vec![0u8; padded_width * padded_height];

    // Copy interior of curr into padded_curr
    for y in 0..height {
        let src_off = y * width;
        let dst_off = (y + 1) * padded_width + 1;
        padded_curr[dst_off..dst_off + width]
            .copy_from_slice(&curr[src_off..src_off + width]);
    }

    let pw = padded_width as isize;
    // Neighbor offsets: 8 surrounding cells
    let offsets: [isize; 8] = [
        -(pw + 1), -pw, -(pw - 1),
        -1,        1,
        pw - 1,   pw, pw + 1,
    ];

    for _ in 0..iterations {
        // Simulation step over interior cells only
        for y in 1..=height {
            let row_off = (y * padded_width) as isize;
            for x in 1..=width {
                let idx = row_off + (x as isize);
                let mut cnt: u8 = 0;
                unsafe {
                    // Count neighbors via pointer offsets
                    let ptr = padded_curr.as_ptr();
                    for &off in &offsets {
                        cnt += *ptr.offset(idx + off);
                    }
                    let cell = *ptr.offset(idx);

                    // Apply Game of Life rules
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
        // Swap buffers for next iteration
        std::mem::swap(&mut padded_curr, &mut padded_next);
    }

    // Extract interior back into curr
    for y in 0..height {
        let dst_off = y * width;
        let src_off = (y + 1) * padded_width + 1;
        curr[dst_off..dst_off + width]
            .copy_from_slice(&padded_curr[src_off..src_off + width]);
    }

    curr
}
