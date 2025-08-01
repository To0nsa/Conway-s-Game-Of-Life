fn compute_next_generation(board: &[Vec<u64>], width: usize) -> Vec<Vec<u64>>
{
    let height = board.len();
    let wpr = width / 64;
    let mut next = vec![vec![0u64; wpr]; height];

    for r in 0..height {
        let prev = if r > 0 { &board[r - 1] } else { &vec![0; wpr] };
        let cur = &board[r];
        let nxt = if r + 1 < height {
            &board[r + 1]
        } else {
            &vec![0; wpr]
        };

        for wi in 0..wpr {
            let mut new_word = 0u64;
            let cw = cur[wi];

            for bit in 0..64 {
                let mut count = 0;
                // West
                if bit > 0 {
                    count += ((cw >> (bit - 1)) & 1) as u8;
                } else if wi > 0 {
                    count += ((cur[wi - 1] >> 63) & 1) as u8;
                }
                // East
                if bit < 63 {
                    count += ((cw >> (bit + 1)) & 1) as u8;
                } else if wi + 1 < wpr {
                    count += ((cur[wi + 1] >> 0) & 1) as u8;
                }
                // North
                count += ((prev[wi] >> bit) & 1) as u8;
                // North‐West
                if bit > 0 {
                    count += ((prev[wi] >> (bit - 1)) & 1) as u8;
                } else if wi > 0 {
                    count += ((prev[wi - 1] >> 63) & 1) as u8;
                }
                // North‐East
                if bit < 63 {
                    count += ((prev[wi] >> (bit + 1)) & 1) as u8;
                } else if wi + 1 < wpr {
                    count += ((prev[wi + 1] >> 0) & 1) as u8;
                }
                // South
                count += ((nxt[wi] >> bit) & 1) as u8;
                // South‐West
                if bit > 0 {
                    count += ((nxt[wi] >> (bit - 1)) & 1) as u8;
                } else if wi > 0 {
                    count += ((nxt[wi - 1] >> 63) & 1) as u8;
                }
                // South‐East
                if bit < 63 {
                    count += ((nxt[wi] >> (bit + 1)) & 1) as u8;
                } else if wi + 1 < wpr {
                    count += ((nxt[wi + 1] >> 0) & 1) as u8;
                }

                let alive = ((cw >> bit) & 1) == 1;
                if alive {
                    if count == 2 || count == 3 {
                        new_word |= 1 << bit;
                    }
                } else if count == 3 {
                    new_word |= 1 << bit;
                }
            }

            next[r][wi] = new_word;
        }
    }

    next
}

pub fn gol_four(
    initial: Vec<Vec<u64>>,
    width: usize,
    iterations: usize,
) -> Vec<Vec<u64>>
{
    let mut board = initial;
    for _ in 0..iterations {
        board = compute_next_generation(&board, width);
    }
    board
}
