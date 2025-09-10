import Benchmark from 'benchmark';

class Interval {
  constructor(public start: number, public end: number) {}
}

class IntervalTreeNode {
  max: number;
  left: IntervalTreeNode | null = null;
  right: IntervalTreeNode | null = null;

  constructor(public interval: Interval) {
    this.max = interval.end;
  }
}

class IntervalTree {
  private root: IntervalTreeNode | null = null;

  insert(interval: Interval): void {
    this.root = this.insertNode(this.root, interval);
  }

  private insertNode(node: IntervalTreeNode | null, interval: Interval): IntervalTreeNode {
    if (!node) {
      return new IntervalTreeNode(interval);
    }

    if (interval.start < node.interval.start) {
      node.left = this.insertNode(node.left, interval);
    } else {
      node.right = this.insertNode(node.right, interval);
    }

    node.max = Math.max(node.max, interval.end);
    return node;
  }

  findIntersecting(point: number): Interval[] {
    const results: Interval[] = [];
    this.searchNode(this.root, point, results);
    return results;
  }

  private searchNode(node: IntervalTreeNode | null, point: number, results: Interval[]): void {
    if (!node) return;

    if (point >= node.interval.start && point <= node.interval.end) {
      results.push(node.interval);
    }

    if (node.left && node.left.max >= point) {
      this.searchNode(node.left, point, results);
    }

    if (point >= node.interval.start) {
      this.searchNode(node.right, point, results);
    }
  }
}

function generateRandomIntervals(count: number, maxValue: number): Interval[] {
  const intervals: Interval[] = [];
  for (let i = 0; i < count; i++) {
    const start = Math.floor(Math.random() * maxValue);
    const length = Math.floor(Math.random() * 1000) + 1;
    intervals.push(new Interval(start, start + length));
  }
  return intervals;
}

function generateRandomPoints(count: number, maxValue: number): number[] {
  const points: number[] = [];
  for (let i = 0; i < count; i++) {
    points.push(Math.floor(Math.random() * maxValue));
  }
  return points;
}

// Setup
const intervals = generateRandomIntervals(1000, 100000);
const points100 = generateRandomPoints(100, 100000);
const points1000 = generateRandomPoints(1000, 100000);
const points10000 = generateRandomPoints(10000, 100000);

// Build tree once for query benchmarks
const tree = new IntervalTree();
for (const interval of intervals) {
  tree.insert(interval);
}

// Create benchmark suite
const suite = new Benchmark.Suite();

// Add benchmarks
suite
  .add('Build tree (1000 intervals)', function() {
    const localTree = new IntervalTree();
    for (const interval of intervals) {
      localTree.insert(interval);
    }
  })
  .add('Query 100 points', function() {
    let total = 0;
    for (const point of points100) {
      const results = tree.findIntersecting(point);
      total += results.length;
    }
    return total;
  })
  .add('Query 1000 points', function() {
    let total = 0;
    for (const point of points1000) {
      const results = tree.findIntersecting(point);
      total += results.length;
    }
    return total;
  })
  .add('Query 10000 points', function() {
    let total = 0;
    for (const point of points10000) {
      const results = tree.findIntersecting(point);
      total += results.length;
    }
    return total;
  })
  .add('Single point query', function() {
    const point = points10000[Math.floor(Math.random() * points10000.length)];
    return tree.findIntersecting(point);
  })
  // Add listeners
  .on('cycle', function(event: any) {
    console.log(String(event.target));
  })
  .on('complete', function(this: any) {
    console.log('\nFastest is ' + this.filter('fastest').map('name'));
    
    // Calculate and display additional metrics
    console.log('\nDetailed Results:');
    this.forEach((bench: any) => {
      const hz = bench.hz; // Operations per second
      const period = 1 / hz * 1000; // Time per operation in ms
      const rme = bench.stats.rme; // Relative margin of error
      
      console.log(`\n${bench.name}:`);
      console.log(`  Ops/sec: ${hz.toFixed(2)} (±${rme.toFixed(2)}%)`);
      console.log(`  Mean time: ${period.toFixed(4)}ms`);
      
      // Calculate points per second for query benchmarks
      if (bench.name.includes('Query')) {
        const match = bench.name.match(/(\d+) points/);
        if (match) {
          const numPoints = parseInt(match[1]);
          const pointsPerSec = hz * numPoints;
          console.log(`  Points/sec: ${pointsPerSec.toFixed(0)}`);
        }
      }
    });
  })
  // Run the benchmarks
  .run({ 'async': false });