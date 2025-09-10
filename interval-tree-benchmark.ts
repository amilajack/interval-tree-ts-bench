interface Interval {
  start: number;
  end: number;
}

class IntervalTreeNode {
  interval: Interval;
  max: number;
  left: IntervalTreeNode | null = null;
  right: IntervalTreeNode | null = null;

  constructor(interval: Interval) {
    this.interval = interval;
    this.max = interval.end;
  }
}

class IntervalTree {
  root: IntervalTreeNode | null = null;

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

    if (node.right && point >= node.interval.start) {
      this.searchNode(node.right, point, results);
    }
  }
}

function generateRandomIntervals(count: number, maxValue: number = 100000): Interval[] {
  const intervals: Interval[] = [];
  for (let i = 0; i < count; i++) {
    const start = Math.floor(Math.random() * maxValue);
    const length = Math.floor(Math.random() * 1000) + 1;
    intervals.push({ start, end: start + length });
  }
  return intervals;
}

function generateRandomPoints(count: number, maxValue: number = 100000): number[] {
  const points: number[] = [];
  for (let i = 0; i < count; i++) {
    points.push(Math.floor(Math.random() * maxValue));
  }
  return points;
}

function benchmark(): void {
  console.log("TypeScript Interval Tree Benchmark");
  console.log("===================================");
  
  const intervalCount = 1000;
  const pointCount = 10000;
  
  console.log(`Generating ${intervalCount} random intervals...`);
  const intervals = generateRandomIntervals(intervalCount);
  
  console.log(`Generating ${pointCount} random points...`);
  const points = generateRandomPoints(pointCount);
  
  console.log("Building interval tree...");
  const buildStart = performance.now();
  const tree = new IntervalTree();
  for (const interval of intervals) {
    tree.insert(interval);
  }
  const buildEnd = performance.now();
  console.log(`Tree build time: ${(buildEnd - buildStart).toFixed(3)}ms`);
  
  console.log("\nRunning benchmark...");
  let totalIntersections = 0;
  
  const searchStart = performance.now();
  for (const point of points) {
    const intersecting = tree.findIntersecting(point);
    totalIntersections += intersecting.length;
  }
  const searchEnd = performance.now();
  
  const searchTime = searchEnd - searchStart;
  const avgTimePerPoint = searchTime / pointCount;
  
  console.log("\nResults:");
  console.log(`Total search time: ${searchTime.toFixed(3)}ms`);
  console.log(`Average time per point: ${avgTimePerPoint.toFixed(4)}ms`);
  console.log(`Total intersections found: ${totalIntersections}`);
  console.log(`Points per second: ${(pointCount / (searchTime / 1000)).toFixed(0)}`);
}

benchmark();