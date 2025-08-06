#!/bin/bash
# Run comprehensive framework comparison benchmark
# This script runs the fixed throughput measurement benchmark

set -e

echo "🚀 Running Comprehensive Framework Comparison Benchmark"
echo "========================================================"
echo "This will take approximately 8-10 minutes"
echo ""

cd "$(dirname "$0")/.."

# Run the comprehensive benchmark directly  
cargo test comprehensive_benchmark_test --release --features benchmarks -- --ignored --nocapture

echo ""
echo "✅ Benchmark completed successfully!"
echo "📊 Results saved to:"
echo "  - target/comprehensive_framework_comparison/comprehensive_results.csv"  
echo "  - target/comprehensive_framework_comparison/comprehensive_report.txt"
echo "  - benchmark/readme.md (updated tables)"