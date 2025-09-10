use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use std::cmp;

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i32,
    end: i32,
}

struct IntervalTreeNode {
    interval: Interval,
    max: i32,
    left: Option<Box<IntervalTreeNode>>,
    right: Option<Box<IntervalTreeNode>>,
}

impl IntervalTreeNode {
    fn new(interval: Interval) -> Self {
        IntervalTreeNode {
            interval,
            max: interval.end,
            left: None,
            right: None,
        }
    }
}

struct IntervalTree {
    root: Option<Box<IntervalTreeNode>>,
}

impl IntervalTree {
    fn new() -> Self {
        IntervalTree { root: None }
    }

    fn insert(&mut self, interval: Interval) {
        self.root = Self::insert_node(self.root.take(), interval);
    }

    fn insert_node(
        node: Option<Box<IntervalTreeNode>>,
        interval: Interval,
    ) -> Option<Box<IntervalTreeNode>> {
        match node {
            None => Some(Box::new(IntervalTreeNode::new(interval))),
            Some(mut n) => {
                if interval.start < n.interval.start {
                    n.left = Self::insert_node(n.left, interval);
                } else {
                    n.right = Self::insert_node(n.right, interval);
                }
                n.max = cmp::max(n.max, interval.end);
                Some(n)
            }
        }
    }

    fn find_intersecting(&self, point: i32) -> Vec<Interval> {
        let mut results = Vec::new();
        Self::search_node(&self.root, point, &mut results);
        results
    }

    fn search_node(node: &Option<Box<IntervalTreeNode>>, point: i32, results: &mut Vec<Interval>) {
        if let Some(n) = node {
            if point >= n.interval.start && point <= n.interval.end {
                results.push(n.interval);
            }

            if let Some(ref left) = n.left {
                if left.max >= point {
                    Self::search_node(&n.left, point, results);
                }
            }

            if point >= n.interval.start {
                Self::search_node(&n.right, point, results);
            }
        }
    }
}

fn generate_random_intervals(count: usize, max_value: i32) -> Vec<Interval> {
    let mut rng = rand::thread_rng();
    let mut intervals = Vec::with_capacity(count);

    for _ in 0..count {
        let start = rng.gen_range(0..max_value);
        let length = rng.gen_range(1..=1000);
        intervals.push(Interval {
            start,
            end: start + length,
        });
    }

    intervals
}

fn generate_random_points(count: usize, max_value: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut points = Vec::with_capacity(count);

    for _ in 0..count {
        points.push(rng.gen_range(0..max_value));
    }

    points
}

fn bench_build_tree(c: &mut Criterion) {
    let intervals = generate_random_intervals(1000, 100000);

    c.bench_function("build_interval_tree", |b| {
        b.iter(|| {
            let mut tree = IntervalTree::new();
            for interval in &intervals {
                tree.insert(black_box(*interval));
            }
            tree
        })
    });
}

fn bench_query_points(c: &mut Criterion) {
    let intervals = generate_random_intervals(1000, 100000);
    let mut tree = IntervalTree::new();
    for interval in &intervals {
        tree.insert(*interval);
    }

    let mut group = c.benchmark_group("query_points");

    for size in [100, 1000, 10000].iter() {
        let points = generate_random_points(*size, 100000);
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut total = 0;
                for point in &points {
                    let results = tree.find_intersecting(black_box(*point));
                    total += results.len();
                }
                total
            })
        });
    }
    group.finish();
}

fn bench_single_query(c: &mut Criterion) {
    let intervals = generate_random_intervals(1000, 100000);
    let mut tree = IntervalTree::new();
    for interval in &intervals {
        tree.insert(*interval);
    }

    let points = generate_random_points(10000, 100000);
    let mut point_iter = points.iter().cycle();

    c.bench_function("single_point_query", |b| {
        b.iter(|| {
            let point = *point_iter.next().unwrap();
            tree.find_intersecting(black_box(point))
        })
    });
}

criterion_group!(
    benches,
    bench_build_tree,
    bench_query_points,
    bench_single_query
);
criterion_main!(benches);
