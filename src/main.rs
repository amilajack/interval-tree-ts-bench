use rand::Rng;
use std::cmp;
use std::time::Instant;

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

fn benchmark() {
    println!("Rust Interval Tree Benchmark");
    println!("=============================");

    let interval_count = 1000;
    let point_count = 10000;
    let max_value = 100000;

    println!("Generating {} random intervals...", interval_count);
    let intervals = generate_random_intervals(interval_count, max_value);

    println!("Generating {} random points...", point_count);
    let points = generate_random_points(point_count, max_value);

    println!("Building interval tree...");
    let build_start = Instant::now();
    let mut tree = IntervalTree::new();
    for interval in &intervals {
        tree.insert(*interval);
    }
    let build_duration = build_start.elapsed();
    println!(
        "Tree build time: {:.3}ms",
        build_duration.as_secs_f64() * 1000.0
    );

    println!("\nRunning benchmark...");
    let mut total_intersections = 0;

    let search_start = Instant::now();
    for point in &points {
        let intersecting = tree.find_intersecting(*point);
        total_intersections += intersecting.len();
    }
    let search_duration = search_start.elapsed();

    let search_time_ms = search_duration.as_secs_f64() * 1000.0;
    let avg_time_per_point = search_time_ms / point_count as f64;
    let points_per_second = point_count as f64 / search_duration.as_secs_f64();

    println!("\nResults:");
    println!("Total search time: {:.3}ms", search_time_ms);
    println!("Average time per point: {:.4}ms", avg_time_per_point);
    println!("Total intersections found: {}", total_intersections);
    println!("Points per second: {:.0}", points_per_second);
}

fn main() {
    benchmark();
}
