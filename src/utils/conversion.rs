pub fn flatten_grid(grid: &[Vec<bool>]) -> Vec<u8>
{
    let h = grid.len();
    let w = grid.get(0).map_or(0, |r| r.len());
    let mut flat = Vec::with_capacity(h * w);
    for row in grid {
        flat.extend(row.iter().map(|&b| b as u8));
    }
    flat
}

pub fn to_bitboards(grid: &[Vec<bool>]) -> (Vec<Vec<u64>>, usize)
{
    let height = grid.len();
    let width = grid.first().map(|r| r.len()).unwrap_or(0);
    assert!(width % 64 == 0, "bitboard width must be a multiple of 64");
    let words_per_row = width / 64;

    let mut board = Vec::with_capacity(height);
    for row in grid {
        assert!(row.len() == width, "ragged rows not supported");
        let mut words = vec![0u64; words_per_row];
        for (col, &alive) in row.iter().enumerate() {
            if alive {
                let wi = col / 64;
                let bi = col % 64;
                words[wi] |= 1 << bi;
            }
        }
        board.push(words);
    }
    (board, width)
}
