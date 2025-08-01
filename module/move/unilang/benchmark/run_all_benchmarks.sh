#!/bin/bash
# Run all benchmarks and update documentation
# This script runs all benchmark suites and regenerates reports

set -e

echo "🏁 Running ALL Benchmarks and Updating Documentation"
echo "===================================================="
echo "This will take approximately 30+ minutes"
echo ""

cd "$(dirname "$0")/.."

# Run all benchmarks via the run_all_benchmarks test
cargo test run_all_benchmarks --release -- --nocapture

echo ""
echo "✅ All benchmarks completed successfully!"
echo "📊 All results and documentation updated!"
echo ""
echo "Key output files:"
echo "  - target/comprehensive_framework_comparison/ (3-way comparison)"
echo "  - target/framework_comparison/ (2-way comparison)"
echo "  - target/benchmark_results/ (fast benchmarks)"
echo "  - target/true_benchmark_results/ (build+runtime)"
echo "  - benchmark/readme.md (updated with latest results)"