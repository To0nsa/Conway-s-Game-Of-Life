use std::{sync::Arc, thread};

fn compute_next_generation(board: &[Vec<u64>], width: usize) -> Vec<Vec<u64>>
{
    let height = board.len();
    let wpr = width / 64;

    let threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let chunk_size = (height + threads - 1) / threads;

    let board = Arc::new(board.to_vec());
    let mut handles = Vec::with_capacity(threads.min(height));

    for chunk_start in (0..height).step_by(chunk_size) {
        let start = chunk_start;
        let end = (start + chunk_size).min(height);
        let board = Arc::clone(&board);

        let handle = thread::spawn(move || {
            let mut local = Vec::with_capacity(end - start);
            for r in start..end {
                let prev = if r > 0 { &board[r - 1] } else { &vec![0; wpr] };
                let cur = &board[r];
                let next = if r + 1 < board.len() {
                    &board[r + 1]
                } else {
                    &vec![0; wpr]
                };

                let mut row_out = vec![0u64; wpr];
                for wi in 0..wpr {
                    let cw = cur[wi];
                    let mut new_word = 0u64;

                    for bit in 0..64 {
                        let mut count = 0u8;
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
                            count += ((cur[wi + 1]) & 1) as u8;
                        }
                        // North
                        count += ((prev[wi] >> bit) & 1) as u8;
                        // North-West
                        if bit > 0 {
                            count += ((prev[wi] >> (bit - 1)) & 1) as u8;
                        } else if wi > 0 {
                            count += ((prev[wi - 1] >> 63) & 1) as u8;
                        }
                        // North-East
                        if bit < 63 {
                            count += ((prev[wi] >> (bit + 1)) & 1) as u8;
                        } else if wi + 1 < wpr {
                            count += ((prev[wi + 1]) & 1) as u8;
                        }
                        // South
                        count += ((next[wi] >> bit) & 1) as u8;
                        // South-West
                        if bit > 0 {
                            count += ((next[wi] >> (bit - 1)) & 1) as u8;
                        } else if wi > 0 {
                            count += ((next[wi - 1] >> 63) & 1) as u8;
                        }
                        // South-East
                        if bit < 63 {
                            count += ((next[wi] >> (bit + 1)) & 1) as u8;
                        } else if wi + 1 < wpr {
                            count += ((next[wi + 1]) & 1) as u8;
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

                    row_out[wi] = new_word;
                }

                local.push(row_out);
            }
            (start, local)
        });

        handles.push(handle);
    }

    let mut next = vec![vec![0u64; wpr]; height];
    for handle in handles {
        let (start, rows) = handle.join().expect("thread panicked");
        for (i, row) in rows.into_iter().enumerate() {
            next[start + i] = row;
        }
    }

    next
}

pub fn gol_five(
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
