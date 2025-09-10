#!/bin/bash

echo "Running performance comparison..."
echo "================================="
echo ""

echo "TypeScript Benchmark (5 runs):"
echo "------------------------------"
for i in {1..5}; do
    echo "Run $i:"
    npm run ts-benchmark 2>/dev/null | grep -A 4 "Results:"
    echo ""
done

echo ""
echo "Rust Benchmark (5 runs):"
echo "------------------------"
for i in {1..5}; do
    echo "Run $i:"
    ./target/release/interval_benchmark | grep -A 4 "Results:"
    echo ""
done