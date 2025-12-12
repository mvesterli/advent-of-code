#![cfg(test)]

use std::collections::HashMap;

fn find_count(graph: &[Vec<usize>], cache: &mut [i32], node: usize, target: usize) -> i32 {
    if node == target {
        1
    } else if cache[node] >= 0 {
        cache[node]
    } else if cache[node] == -2 {
        0
    } else {
        let mut sum = 0;
        cache[node] = -2;
        for n in &graph[node] {
            sum += find_count(graph, cache, *n, target);
        }
        cache[node] = sum;
        sum
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day11.txt");
    let input: Vec<Vec<_>> = input.lines().map(|l| l.split(':').collect()).collect();

    let mut names = HashMap::new();
    for l in &input {
        names.insert(l[0], names.len());
    }
    names.insert("out", names.len());

    let mut graph: Vec<Vec<usize>> = std::iter::repeat(Vec::new()).take(names.len()).collect();
    for line in &input {
        for to in line[1].trim().split_ascii_whitespace().map(|o| names[o]) {
            graph[to].push(names[line[0]]);
        }
    }

    let mut cache = vec![-1; names.len()];
    let res = find_count(&graph, &mut cache, names["out"], names["you"]);
    assert_eq!(585, res);
}

fn find_count2(
    graph: &[Vec<usize>],
    cache: &mut [i64],
    node: usize,
    mut flags: usize,
    target: usize,
    dac: usize,
    fft: usize,
) -> i64 {
    if node == dac {
        flags |= 2;
    }
    if node == fft {
        flags |= 1;
    }

    let cache_idx = node * 4 + flags;
    if node == target {
        if flags == 3 { 1 } else { 0 }
    } else if cache[cache_idx] >= 0 {
        cache[cache_idx]
    } else if cache[cache_idx] == -2 {
        0
    } else {
        let mut sum = 0;
        cache[cache_idx] = -2;
        for n in &graph[node] {
            sum += find_count2(graph, cache, *n, flags, target, dac, fft);
        }
        cache[cache_idx] = sum;
        sum
    }
}

#[test]
fn part2() {
    let input = include_str!("../input/day11.txt");
    let input: Vec<Vec<_>> = input.lines().map(|l| l.split(':').collect()).collect();

    let mut names = HashMap::new();
    for l in &input {
        names.insert(l[0], names.len());
    }
    names.insert("out", names.len());

    let mut graph: Vec<Vec<usize>> = std::iter::repeat(Vec::new()).take(names.len()).collect();
    for line in &input {
        for to in line[1].trim().split_ascii_whitespace().map(|o| names[o]) {
            graph[to].push(names[line[0]]);
        }
    }

    let mut cache = vec![-1; names.len() * 4];
    let res = find_count2(
        &graph,
        &mut cache,
        names["out"],
        0,
        names["svr"],
        names["dac"],
        names["fft"],
    );
    assert_eq!(349322478796032, res);
}
