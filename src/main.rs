use console::style;

fn main() -> Result<(), std::num::ParseIntError> {
    let rows: usize = read_line().parse()?;
    let cols: usize = read_line().parse()?;

    let mut data = vec![];
    for _ in 0..rows {
        let mut line: Vec<_> = read_line().chars().collect();
        data.append(&mut line);
    }

    let mut min_distance = std::usize::MAX;
    let mut min_pos = vec![];

    let targets = data.iter().filter(|&&x| x == '.').count();

    for row in 0..rows {
        for col in 0..cols {
            if let Some(distance) =
                find_distance(&data, rows, cols, min_distance, row, col, targets)
            {
                if distance < min_distance {
                    min_distance = distance;
                    min_pos.clear();
                }
                if distance == min_distance {
                    min_pos.push((row, col));
                }
            }
        }
    }

    if !min_pos.is_empty() {
        println!("Min distance {}", min_distance);
        for (y, x) in min_pos {
            data[y * cols + x] = if data[y * cols + x] == ' ' { 'o' } else { 'O' };
        }
        let mut buf = data.iter();
        for _ in 0..rows {
            for _ in 0..cols {
                let ch = buf.next().unwrap();
                let style = style(ch).bold();
                let colored = match ch {
                    'o' | 'O' => style.green().blink(),
                    '.' => style.cyan(),
                    'X' => style.yellow(),
                    _ => style.white(),
                };
                print!("{}", colored);
            }
            println!();
        }
    } else {
        println!("No solution");
    }

    Ok(())
}

fn find_distance(
    data: &Vec<char>,
    rows: usize,
    cols: usize,
    min_distance: usize,
    row: usize,
    col: usize,
    targets: usize,
) -> Option<usize> {
    let mut level = 0;
    let mut data = data.clone();
    let mut pending = vec![(row, col)];
    let mut targets_left = targets;
    let mut distance = 0;
    while !pending.is_empty() {
        let mut next_level = vec![];

        for &(y, x) in &pending {
            let c = data[cols * y + x];

            if c == '.' {
                distance += level;
                targets_left -= 1;
                if targets_left == 0 {
                    return Some(distance);
                }
            }
            if c != 'X' {
                data[cols * y + x] = 'X';
                if y > 0 {
                    if x > 0 {
                        next_level.push((y - 1, x - 1));
                    }
                    next_level.push((y - 1, x));
                    if x < cols - 1 {
                        next_level.push((y - 1, x + 1));
                    }
                }
                if x > 0 {
                    next_level.push((y, x - 1));
                }
                if x < cols - 1 {
                    next_level.push((y, x + 1));
                }
                if y < rows - 1 {
                    if x > 0 {
                        next_level.push((y + 1, x - 1));
                    }
                    next_level.push((y + 1, x));
                    if x < cols - 1 {
                        next_level.push((y + 1, x + 1));
                    }
                }
            }
        }

        level += 1;
        if level > min_distance {
            return None;
        }

        pending = next_level;
    }
    None
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
