#![cfg(test)]

fn solve(input: &str) -> (u32, u64) {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut counts: Vec<u64> = vec![0; input[0].len()];
    let start = input[0].iter().enumerate().find(|v| *v.1 == 'S').unwrap().0;
    counts[start] = 1;

    let mut num_splits = 0;
    for row in &input[1..] {
        let mut new_counts = vec![0; input[0].len()];
        for col in 0..row.len() {
            if counts[col] != 0 {
                if row[col] == '.' {
                    new_counts[col] += counts[col];
                } else {
                    if col > 0 {
                        new_counts[col - 1] += counts[col];
                    }
                    if col + 1 < row.len() {
                        new_counts[col + 1] += counts[col];
                    }
                    num_splits += 1;
                }
            }
        }
        counts = new_counts;
    }
    (num_splits, counts.iter().sum())
}

#[test]
fn part11() {
    let input = include_str!("../input/day7.txt");
    assert_eq!(1630, solve(input).0);
}

#[test]
fn part2() {
    let input = include_str!("../input/day7.txt");
    assert_eq!(47857642990160, solve(input).1);
}
