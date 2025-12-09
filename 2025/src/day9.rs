#![cfg(test)]

use std::collections::HashMap;

#[test]
fn part1() {
    let input = include_str!("../input/day9.txt");
    let input: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();
    let mut res = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let x = (input[i][0] - input[j][0]).abs() + 1;
            let y = (input[i][1] - input[j][1]).abs() + 1;
            res = res.max(x * y);
        }
    }
    assert_eq!(4755064176, res);
}

fn is_valid(grid: &[Vec<char>], a: (usize, usize), b: (usize, usize)) -> bool {
    if grid[a.1][b.0] != 'X' || grid[b.1][a.0] != 'X' {
        return false;
    }
    for y in a.1.min(b.1)..=a.1.max(b.1) {
        for x in a.0.min(b.0)..=a.0.max(b.0) {
            if grid[y][x] == '.' {
                return false;
            }
        }
    }
    true
}

#[test]
fn part2() {
    let input = include_str!("../input/day9.txt");
    let input: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();

    let mut distinct_x: Vec<_> = input.iter().map(|n| n[0]).collect();
    distinct_x.sort();
    distinct_x.dedup();

    let mut distinct_y: Vec<_> = input.iter().map(|n| n[1]).collect();
    distinct_y.sort();
    distinct_y.dedup();

    let mut mapping_x = HashMap::new();
    let mut mapping_y = HashMap::new();
    for (idx, v) in distinct_x.iter().enumerate() {
        mapping_x.insert(*v, idx);
    }
    for (idx, v) in distinct_y.iter().enumerate() {
        mapping_y.insert(*v, idx);
    }

    let mut grid = vec![vec!['.'; distinct_x.len()]; distinct_y.len()];
    for i in 0..input.len() {
        let c = &input[i];
        let n = &input[(i + 1) % input.len()];
        let cx = mapping_x[&c[0]];
        let cy = mapping_y[&c[1]];
        let nx = mapping_x[&n[0]];
        let ny = mapping_y[&n[1]];
        if cx == nx {
            for y in cy.min(ny)..=cy.max(ny) {
                grid[y][cx] = 'X';
            }
        } else {
            for x in cx.min(nx)..=cx.max(nx) {
                grid[cy][x] = 'X';
            }
        }
    }

    let mut queue = vec![(150, 120)];
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        if grid[y][x] != 'X' {
            grid[y][x] = 'X';
            queue.push((x - 1, y));
            queue.push((x + 1, y));
            queue.push((x, y - 1));
            queue.push((x, y + 1));
        }
    }

    let mut best = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let c = &input[i];
            let n = &input[j];
            let cx = mapping_x[&c[0]];
            let cy = mapping_y[&c[1]];
            let nx = mapping_x[&n[0]];
            let ny = mapping_y[&n[1]];
            if is_valid(&grid, (cx, cy), (nx, ny)) {
                let x = (distinct_x[cx] as i64 - distinct_x[nx] as i64).abs() + 1;
                let y = (distinct_y[cy] as i64 - distinct_y[ny] as i64).abs() + 1;
                best = best.max(x * y);
            }
        }
    }
    assert_eq!(1613305596, best);
}
