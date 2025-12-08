#![cfg(test)]

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn dist_sq(&self, o: &Point) -> f64 {
        let dx = self.x - o.x;
        let dy = self.y - o.y;
        let dz = self.z - o.z;
        dx * dx + dy * dy + dz * dz
    }

    fn parse(input: &str) -> Vec<Point> {
        input
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<f64>>()
            })
            .map(|l| Point {
                x: l[0],
                y: l[1],
                z: l[2],
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
struct Edge {
    a: usize,
    b: usize,
    weight: f64,
}

impl Edge {
    fn create(points: &[Point]) -> Vec<Edge> {
        (0..points.len())
            .flat_map(|i| {
                (i + 1..points.len())
                    .map(|j| Edge {
                        a: i,
                        b: j,
                        weight: points[i].dist_sq(&points[j]),
                    })
                    .collect::<Vec<Edge>>()
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
struct DisjointSetNode {
    parent: usize,
    size: u32,
}

struct DisjointSet {
    nodes: Vec<DisjointSetNode>,
    num_sets: usize,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            nodes: (0..size)
                .map(|i| DisjointSetNode { parent: i, size: 1 })
                .collect(),
            num_sets: size,
        }
    }

    fn find_id(&mut self, idx: usize) -> usize {
        if self.nodes[idx].parent == idx {
            idx
        } else {
            let id = self.find_id(self.nodes[idx].parent);
            self.nodes[idx].parent = id;
            id
        }
    }

    fn merge(&mut self, a: usize, b: usize) {
        let id_a = self.find_id(a);
        let id_b = self.find_id(b);
        if id_a != id_b {
            self.nodes[id_a].parent = id_b;
            self.nodes[id_b].size += self.nodes[id_a].size;
            self.num_sets -= 1;
        }
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day8.txt");
    let input = Point::parse(input);
    let mut edges = Edge::create(&input);
    edges.sort_by(|a, b| a.weight.total_cmp(&b.weight));

    let mut set = DisjointSet::new(input.len());
    for edge in edges.iter().take(1000) {
        set.merge(edge.a, edge.b);
    }
    let mut sizes: Vec<u32> = set
        .nodes
        .iter()
        .enumerate()
        .filter(|(idx, n)| n.parent == *idx)
        .map(|(_, n)| n.size)
        .collect();
    sizes.sort();
    let res: u32 = sizes.iter().rev().take(3).product();
    assert_eq!(163548, res);
}

fn get_last_edge(points: &[Point], edges: &[Edge]) -> Option<Edge> {
    let mut set = DisjointSet::new(points.len());
    for edge in edges {
        set.merge(edge.a, edge.b);
        if set.num_sets == 1 {
            return Some(*edge);
        }
    }
    None
}

#[test]
fn part2() {
    let input = include_str!("../input/day8.txt");
    let input = Point::parse(input);
    let mut edges = Edge::create(&input);
    edges.sort_by(|a, b| a.weight.total_cmp(&b.weight));

    let edge = get_last_edge(&input, &edges).unwrap();
    let res = input[edge.a].x * input[edge.b].x;
    assert_eq!(772452514.0, res);
}
