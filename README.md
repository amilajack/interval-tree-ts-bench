# Interval Tree Benchmark: TypeScript vs Rust

A performance comparison between TypeScript and Rust implementations of an interval tree data structure.

## Overview

This project benchmarks interval tree operations in both TypeScript and Rust, demonstrating the performance characteristics of each language for this specific use case. The benchmark creates 1000 random intervals and performs 10,000 point queries to find overlapping intervals.

## Results

### Performance Summary

| Language | Avg Search Time | Avg Time per Point | Points per Second |
|----------|----------------|-------------------|-------------------|
| **TypeScript** | ~2.935ms | 0.0003ms | ~3,459,171 |
| **Rust** | ~0.977ms | 0.0001ms | ~10,278,414 |

**Rust is approximately 3x faster than TypeScript** for interval tree operations in this benchmark.

### Detailed Results (5 runs each)

#### TypeScript Performance
- Run 1: 2.850ms (3,508,618 points/sec)
- Run 2: 2.971ms (3,365,634 points/sec)
- Run 3: 3.006ms (3,326,680 points/sec)
- Run 4: 2.768ms (3,612,390 points/sec)
- Run 5: 3.080ms (3,246,534 points/sec)

#### Rust Performance
- Run 1: 0.986ms (10,139,849 points/sec)
- Run 2: 0.957ms (10,447,498 points/sec)
- Run 3: 0.964ms (10,371,195 points/sec)
- Run 4: 0.954ms (10,487,226 points/sec)
- Run 5: 1.026ms (9,745,401 points/sec)

## Project Structure

```
interval-ts-bench/
├── interval-tree-benchmark.ts  # TypeScript implementation
├── src/
│   └── main.rs                 # Rust implementation
├── run-benchmarks.sh           # Benchmark runner script
├── package.json                # Node.js dependencies
└── Cargo.toml                  # Rust dependencies
```

## Implementation Details

Both implementations use the same algorithm:
- **Data Structure**: Augmented interval tree
- **Test Data**: 1000 random intervals with bounds between 0-10000
- **Queries**: 10,000 random point queries
- **Metrics**: Total search time, intersections found, and throughput

## Requirements

- Node.js (v14+)
- Rust (1.70+)
- npm or yarn

## Running the Benchmarks

```bash
# Install dependencies
npm install
cargo build --release

# Run benchmarks
./run-benchmarks.sh
```

## Key Takeaways

1. **Rust's Performance Advantage**: The compiled nature and zero-cost abstractions of Rust provide significant performance benefits for compute-intensive operations like interval tree searches.

2. **TypeScript's Accessibility**: While slower, the TypeScript implementation is still performant enough for many use cases and offers easier integration with web applications.

3. **Memory Efficiency**: Rust's ownership model and lack of garbage collection contribute to more predictable performance characteristics.

## Use Cases

This benchmark is relevant for applications that require:
- Geometric algorithms
- Calendar scheduling systems
- Database query optimization
- Collision detection in games
- Time-series data analysis

## License

MIT