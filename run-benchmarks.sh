#!/bin/bash

echo "========================================="
echo "Interval Tree Performance Benchmarks"
echo "========================================="
echo ""

echo "Building Rust benchmark..."
cargo build --release

echo ""
echo "TypeScript Benchmark (Benchmark.js):"
echo "-------------------------------------"
npx tsx interval-tree-benchmark-suite.ts

echo ""
echo ""
echo "Rust Benchmark (Criterion):"
echo "----------------------------"
cargo bench --quiet

echo ""
echo "Note: Criterion results are saved in target/criterion/"
echo "You can view detailed HTML reports there."