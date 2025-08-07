#!/bin/bash
# Quick demo script to show benchmark functionality
# This runs a minimal benchmark to verify everything works

set -e

echo "🎯 Unilang Benchmark Demo"
echo "========================"
echo "This is a quick demo to verify benchmark functionality"
echo ""

cd "$(dirname "$0")/.."

echo "🔍 Checking existing benchmark results..."
if [ -d "target/comprehensive_framework_comparison" ]; then
    echo "✅ Found existing results:"
    ls -la target/comprehensive_framework_comparison/
    echo ""
    echo "📊 Latest CSV results (first 5 lines):"
    head -5 target/comprehensive_framework_comparison/comprehensive_results.csv
    echo ""
    echo "📋 Report summary:"
    head -20 target/comprehensive_framework_comparison/comprehensive_report.txt
else
    echo "❌ No existing results found"
fi

echo ""
echo "🚀 To run full benchmarks:"
echo "  ./benchmarks/run_comprehensive_benchmark.sh    # 3-way comparison (8-10 min)"
echo "  ./benchmarks/run_all_benchmarks.sh             # All benchmarks (30+ min)"
echo ""
echo "📂 Results will be generated in:"
echo "  - target/comprehensive_framework_comparison/comprehensive_results.csv"
echo "  - target/comprehensive_framework_comparison/comprehensive_report.txt"
echo "  - benchmarks/readme.md (updated tables)"