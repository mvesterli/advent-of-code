#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day4.txt");
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut res = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == '@' {
                let mut adjacent = 0;
                for delta_row in -1..=1 {
                    for delta_col in -1..=1 {
                        let r = row as i32 + delta_row;
                        let c = col as i32 + delta_col;
                        if r >= 0
                            && r < input.len() as i32
                            && c >= 0
                            && c < input[row].len() as i32
                            && input[r as usize][c as usize] == '@'
                        {
                            adjacent += 1;
                        }
                    }
                }
                if adjacent < 5 {
                    res += 1;
                }
            }
        }
    }
    assert_eq!(1372, res);
}

#[test]
fn part2() {
    let input = include_str!("../input/day4.txt");
    let mut input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut res = 0;
    let mut progress = true;
    while progress {
        progress = false;
        for row in 0..input.len() {
            for col in 0..input[row].len() {
                if input[row][col] == '@' {
                    let mut adjacent = 0;
                    for delta_row in -1..=1 {
                        for delta_col in -1..=1 {
                            let r = row as i32 + delta_row;
                            let c = col as i32 + delta_col;
                            if r >= 0
                                && r < input.len() as i32
                                && c >= 0
                                && c < input[row].len() as i32
                                && input[r as usize][c as usize] == '@'
                            {
                                adjacent += 1;
                            }
                        }
                    }
                    if adjacent < 5 {
                        progress = true;
                        res += 1;
                        input[row][col] = '.';
                    }
                }
            }
        }
    }
    assert_eq!(7922, res);
}
