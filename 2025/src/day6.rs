#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day6.txt");
    let mut input: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect())
        .collect();
    let ops = input.pop().unwrap();
    let nums: Vec<Vec<i64>> = input
        .iter()
        .map(|l| l.iter().map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut res = 0;
    for col in 0..ops.len() {
        res += nums.iter().map(|row| row[col]).skip(1).fold(
            nums[0][col],
            if ops[col] == "*" {
                |a, b| a * b
            } else {
                |a, b| a + b
            },
        );
    }
    assert_eq!(5552221122013, res);
}

#[test]
fn part2() {
    let input = include_str!("../input/day6.txt");
    let ops: Vec<&str> = input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    let mut input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    input.pop();

    let mut splits = Vec::new();
    for i in 0..input[0].len() {
        if input.iter().all(|l| l[i] == ' ') {
            splits.push(i);
        }
    }
    splits.push(input[0].len());

    let mut total: u64 = 0;
    let mut start = 0;
    for (end, op) in splits.into_iter().zip(ops.into_iter()) {
        let mut res: u64 = if op == "*" { 1 } else { 0 };
        for y in start..end {
            let mut num = 0;
            for row in &input {
                if row[y] != ' ' {
                    let d = row[y].to_digit(10).unwrap();
                    num = num * 10 + d as u64;
                }
            }
            if op == "*" {
                res = res * num
            } else {
                res = res + num
            };
        }
        total += res;

        start = end + 1;
    }
    assert_eq!(11371597126232, total);
}
