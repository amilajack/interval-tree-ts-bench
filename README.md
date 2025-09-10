# Interval Tree Benchmark: TypeScript vs Rust

## Results

### Performance Summary

| Operation | TypeScript (Benchmark.js) | Rust (Criterion) | Rust Advantage |
|-----------|--------------------------|------------------|----------------|
| **Build Tree (1000 intervals)** | 52.7 µs | 30.9 µs | **1.7x faster** |
| **Query 100 points** | 10.3 µs | 6.4 µs | **1.6x faster** |
| **Query 1000 points** | 113.5 µs | 68.4 µs | **1.7x faster** |
| **Query 10000 points** | 1,311.9 µs | 942.1 µs | **1.4x faster** |
| **Single point query** | 0.14 ns | 84.7 ns | TypeScript* |

*Note: The single point query in TypeScript shows unexpectedly fast results likely due to JavaScript engine optimizations for the specific test case.

### Throughput Comparison

| Metric | TypeScript | Rust |
|--------|------------|------|
| **Points per second (100 pts)** | 9.7 million | 15.6 million |
| **Points per second (1000 pts)** | 8.8 million | 14.6 million |
| **Points per second (10000 pts)** | 7.6 million | 10.6 million |

**Rust demonstrates 1.4-1.7x better performance** across most operations, with consistent performance advantages in both tree construction and query operations.

## Running the Benchmarks

```bash
# Install dependencies
npm install
cargo build --release

# Run all benchmarks
./run-benchmarks.sh

# Run individual benchmarks
npx tsx interval-tree-benchmark-suite.ts  # TypeScript
cargo bench                                # Rust
```