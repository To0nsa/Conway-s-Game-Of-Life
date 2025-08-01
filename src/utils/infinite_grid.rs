pub fn extend_flat_grid_to_square64(
    grid: &Vec<u8>,
    width: usize,
    out_buf: &mut Vec<u8>,
) -> usize
{
    const MARGIN: usize = 5;
    let height = width;
    debug_assert_eq!(grid.len(), width * height);
    debug_assert_eq!(width % 64, 0);

    let top_band = &grid[0..width * MARGIN.min(height)];
    let bot_start = width * height.saturating_sub(MARGIN);
    let bot_band = &grid[bot_start..bot_start + width];
    let left_near = grid
        .chunks(width)
        .skip(MARGIN)
        .take(height - 2 * MARGIN)
        .any(|row| row[..MARGIN].iter().any(|&v| v == 1));
    let right_near = grid
        .chunks(width)
        .skip(MARGIN)
        .take(height - 2 * MARGIN)
        .any(|row| row[width - MARGIN..].iter().any(|&v| v == 1));

    let near_edge = top_band.iter().any(|&v| v == 1)
        || bot_band.iter().any(|&v| v == 1)
        || left_near
        || right_near;

    if !near_edge {
        return width;
    }

    let base_pad = 1;
    let intermediate = width + 2 * base_pad;
    let new_size = ((intermediate + 63) / 64) * 64;
    let extra = new_size - intermediate;
    let pad_each = base_pad + (extra / 2);

    out_buf.clear();
    out_buf.resize(new_size * new_size, 0);

    for (row_idx, row) in grid.chunks_exact(width).enumerate() {
        let dst = (row_idx + pad_each) * new_size + pad_each;
        out_buf[dst..dst + width].copy_from_slice(row);
    }

    new_size
}
