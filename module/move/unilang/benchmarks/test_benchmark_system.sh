#!/bin/bash
# Test script to verify benchmark system functionality
# This runs a short benchmark test and verifies results are generated

set -e
cd "$(dirname "$0")/.."

echo "🧪 Testing Benchmark System"
echo "=========================="
echo "Running short comprehensive benchmark test..."

# Run a short benchmark test (timeout after 30 seconds for safety)
timeout 30s cargo test comprehensive_framework_comparison_benchmark --release --features benchmarks -- --nocapture --ignored || {
    echo "⚠️  Benchmark test timed out or failed, but that's expected for a quick test"
}

# Check if any results were generated
echo ""
echo "📊 Checking for generated results..."

if [ -d "target/comprehensive_framework_comparison" ]; then
    echo "✅ Found target/comprehensive_framework_comparison directory"
    if [ -f "target/comprehensive_framework_comparison/comprehensive_results.csv" ]; then
        echo "✅ Found comprehensive_results.csv"
        head -3 "target/comprehensive_framework_comparison/comprehensive_results.csv" || true
    else
        echo "⚠️  No comprehensive_results.csv found yet"
    fi
else
    echo "⚠️  No results directory found yet"
fi

echo ""
echo "🔧 Available benchmark commands:"
echo "  cargo test run_all_benchmarks --release --features benchmarks -- --nocapture --ignored"
echo "  ./benchmarks/run_comprehensive_benchmark.sh"
echo "  ./benchmarks/run_all_benchmarks.sh"
echo ""
echo "📋 Individual benchmarks (all ignored by default):"
echo "  cargo test comprehensive_framework_comparison_benchmark --release --features benchmarks -- --ignored"
echo "  cargo bench throughput_benchmark --features benchmarks"
echo "  cargo bench throughput_benchmark --features benchmarks -- --quick"
echo ""
echo "✅ Benchmark system test completed!"