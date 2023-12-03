use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DOT: u8 = '.' as u8;
const ZERO: u8 = '0' as u8;
const NINE: u8 = '9' as u8;
const STAR: u8 = '*' as u8;

fn is_digit(b: u8) -> bool {
    b >= ZERO && b <= NINE
}

fn is_part(left: usize, right: usize, row: usize, matrix: &Vec<&[u8]>) -> (bool, Option<(usize, usize)>) {
    let (m, n) = (matrix.len(), matrix[0].len());
    let r = matrix[row];
    let part_but_not_star = (true, None);

    if left > 0 && r[left - 1] != DOT {
        if r[left - 1] == STAR {return (true, Some((row, left - 1)))}
        return part_but_not_star;
    }

    if right + 1 < n && r[right + 1] != DOT {
        if r[right + 1] == STAR {return (true, Some((row, right + 1)))}
        return part_but_not_star;
    }

    let l_end = if left > 0 {left - 1} else {left};
    let r_end = if right + 1 < n {right + 1} else {right};

    if row > 0 {
        let r_n = row - 1;
        let mut i = l_end;
        let r = matrix[r_n];

        while i  <= r_end {
            if r[i] != DOT {
                if r[i] == STAR {
                    if r[i] == STAR {return (true, Some((r_n, i)))}
                    return part_but_not_star;
                }
                return part_but_not_star;
            }
            i += 1;
        }
    }

    if row + 1 < m {
        let r_n = row + 1;
        let mut i = l_end;
        let r = matrix[r_n];

        while i  <= r_end {
            if r[i] != DOT {
                if r[i] == STAR {return (true, Some((r_n, i)))}
                return part_but_not_star;

            }
            i += 1;
        }
    }

    (false, None)
}

fn main() -> io::Result<()> {
    let path = Path::new("input/day3.dat");
    let file = File::open(&path)?;

    let reader = io::BufReader::new(file);
    let mut matrix = Vec::new();

    for line in reader.lines() {
        if let Ok(row) = line {
            matrix.push(row);
        }
    }

    let matrix: Vec<&[u8]> = matrix.iter().map(|r| r.as_bytes()).collect();

    let mut row = 0;
    let mut sum_parts = 0;
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut star_map: HashMap<(usize, usize), u128> = HashMap::new();
    let mut gear_sum: u128 = 0;

    while row < m {
        let r = matrix[row];
        let mut left = 0;
        while left < n {
            if is_digit(r[left]) {
                let mut right = left + 1;
                while right < n && is_digit(r[right]) {right += 1;}
                let (part, star) = is_part(left, right - 1, row, &matrix);
                if part {
                    let mut f = 1;
                    let mut j = right - 1;
                    let mut num = 0;
                    while left <= j {
                        num += f * (r[j] as u128 - ZERO as u128);
                        f *= 10;
                        if j > 0 {
                            j -= 1;
                        } else {
                            break;
                        }
                    }
                    sum_parts += num;
                    if let Some(point) = star {
                        match star_map.get(&point) {
                            Some(v) => gear_sum += v * num,
                            None => {star_map.insert(point, num);}
                        }
                    }
                }
                left = right;
            } else {
                left += 1;
            }
        }
        row += 1;
    }
    println!("{sum_parts}");
    println!("{gear_sum}");

    Ok(())

}