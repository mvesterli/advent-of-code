#![cfg(test)]

use std::collections::VecDeque;

fn parse_line(line: &str) -> (i32, Vec<Vec<usize>>, Vec<usize>) {
    let mut iter = line.split_ascii_whitespace().map(|s| &s[1..s.len() - 1]);
    let mut req = 0;
    for (idx, c) in iter.next().unwrap().chars().enumerate() {
        if c == '#' {
            req |= 1 << idx;
        }
    }
    let mut rows: Vec<_> = iter
        .map(|s| s.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    let jolt = rows.pop().unwrap();
    (req, rows, jolt)
}

#[test]
fn part1() {
    let input = include_str!("../input/day10.txt");
    let input: Vec<_> = input.lines().map(parse_line).collect();
    let mut sum = 0;
    for (req, buttons, _) in input {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((0, 0));
        sum += loop {
            let (state, dist) = q.pop_front().unwrap();
            if state == req {
                break dist;
            }
            for button in &buttons {
                let mut new_state = state;
                for i in button {
                    new_state ^= 1 << i;
                }
                q.push_back((new_state, dist + 1));
            }
        };
    }
    assert_eq!(571, sum);
}
