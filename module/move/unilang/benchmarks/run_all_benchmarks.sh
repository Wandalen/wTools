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
echo "🏁 Starting comprehensive benchmark suite..."
echo "This will run ALL benchmarks and update documentation"
echo ""

cargo test run_all_benchmarks --release --features benchmarks -- --nocapture --ignored

echo ""
echo "✅ All benchmarks completed successfully!"
echo "📊 All results and documentation updated!"
echo ""
echo "Key output files:"
echo "  - target/comprehensive_framework_comparison/ (3-way comparison with build metrics)"
echo "  - target/throughput_benchmark/ (fast runtime-only testing)"
echo "  - benchmark/readme.md (updated with latest results)"
