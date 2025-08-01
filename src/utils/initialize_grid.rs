use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
    str::FromStr,
};

pub fn parse_arg<T: FromStr>(args: &[String], index: usize, name: &str) -> T
where
    T::Err: std::fmt::Display,
{
    if args.len() <= index {
        eprintln!("Missing argument: '{}'", name);
        process::exit(1);
    }
    let val = &args[index];
    val.parse::<T>().unwrap_or_else(|e| {
        eprintln!("Invalid {} ('{}'): {}", name, val, e);
        process::exit(1);
    })
}

fn pad_grid_to_multiple_of_64(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>>
{
    let height = grid.len();
    let width = grid.first().map(|r| r.len()).unwrap_or(0);

    let target_h = ((height + 63) / 64) * 64;
    let target_w = ((width + 63) / 64) * 64;

    let new_size = target_h.max(target_w);

    let pad_vert = new_size - height;
    let pad_horiz = new_size - width;

    let pad_top = pad_vert / 2;
    let pad_bottom = pad_vert - pad_top;
    let pad_left = pad_horiz / 2;
    let pad_right = pad_horiz - pad_left;

    let mut out = Vec::with_capacity(new_size);
    let dead_row = vec![false; new_size];

    for _ in 0..pad_top {
        out.push(dead_row.clone());
    }

    for mut row in grid {
        let mut new_row = Vec::with_capacity(new_size);
        new_row.extend(std::iter::repeat(false).take(pad_left));
        new_row.append(&mut row);
        new_row.extend(std::iter::repeat(false).take(pad_right));
        out.push(new_row);
    }

    for _ in 0..pad_bottom {
        out.push(dead_row.clone());
    }

    out
}

pub fn load_grid(path: &str) -> Vec<Vec<bool>>
{
    const ALIVE: char = 'X';
    const DEAD: char = '.';

    let file = File::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open {}: {}", path, e);
        process::exit(1);
    });
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|r| {
            r.unwrap_or_else(|e| {
                eprintln!("I/O error: {}", e);
                process::exit(1)
            })
        })
        .collect();

    let max_w = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let grid: Vec<Vec<bool>> = lines
        .into_iter()
        .map(|line| {
            let mut row = Vec::with_capacity(max_w);
            for ch in line.chars().take(max_w) {
                row.push(match ch {
                    ALIVE => true,
                    DEAD => false,
                    other => {
                        eprintln!(
                            "Unexpected character '{}' in {}",
                            other, path
                        );
                        process::exit(1);
                    }
                });
            }
            row.resize(max_w, false);
            row
        })
        .collect();

    pad_grid_to_multiple_of_64(grid)
}
