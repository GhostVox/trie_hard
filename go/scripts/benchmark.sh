#!/bin/bash

# Go Benchmark Script - Similar to Criterion functionality
# Usage: ./benchmark.sh [iterations] [measurement_time] [sample_size]

set -e  # Exit on error

# Configuration (matching your Rust Criterion settings)
ITERATIONS=${1:-5}
MEASUREMENT_TIME=${2:-10}
SAMPLE_SIZE=${3:-20}
CONFIDENCE_LEVEL=0.95

# Ensure results directory exists
mkdir -p benchmark_results

echo "========================================"
echo "Go Benchmark Script"
echo "========================================"
echo "Iterations: $ITERATIONS"
echo "Measurement time: ${MEASUREMENT_TIME}s"
echo "Sample size per iteration: $SAMPLE_SIZE"
echo "CPU cores: 8"
echo "Working directory: $(pwd)"
echo "========================================"

# Check if benchmark files exist
echo "Checking for benchmark files..."
if ! find . -name "*_test.go" -exec grep -l "func Benchmark" {} \; | grep -q .; then
    echo "⚠️  Warning: No benchmark functions found in *_test.go files"
    echo "Make sure you have functions starting with 'func Benchmark' in your test files"

    # Still try to run, but create a fallback result
    echo "No benchmarks found" > benchmark_results/error.txt
    echo "Generated on: $(date)" >> benchmark_results/error.txt
fi

# Verify go test works
echo "Verifying go test works..."
if ! go test -list=. > /dev/null 2>&1; then
    echo "❌ Error: 'go test' failed. Check your Go code for compilation errors."
    echo "go test error" > benchmark_results/error.txt
    exit 1
fi

# Run a quick benchmark test to verify benchmarks exist
echo "Testing benchmark discovery..."
if go test -bench=. -run=^$ -benchtime=1ns > benchmark_results/test_discovery.txt 2>&1; then
    echo "✅ Benchmarks found and can run"
else
    echo "⚠️  Warning: Benchmark test failed, but continuing..."
    echo "Benchmark discovery failed" >> benchmark_results/error.txt
fi

# Run benchmarks multiple times for statistical significance
success=0
for i in $(seq 1 $ITERATIONS); do
    echo "Running benchmark iteration $i/$ITERATIONS..."
    if go test -bench=. -benchtime=${MEASUREMENT_TIME}s -count=${SAMPLE_SIZE} -cpu=8 -benchmem > benchmark_results/run$i.txt 2>&1; then
        echo "✅ Iteration $i complete"
        ((success++))
    else
        echo "❌ Iteration $i failed"
        echo "Iteration $i failed" >> benchmark_results/error.txt
    fi
done

if [ $success -eq 0 ]; then
    echo "❌ All benchmark iterations failed!"
    echo "All iterations failed" > benchmark_results/all_results.txt
    echo "Check individual run files for details" >> benchmark_results/all_results.txt
    exit 1
fi

echo "✅ $success/$ITERATIONS iterations succeeded"

# Combine all successful results
echo "Analyzing results with benchstat..."
cat benchmark_results/run*.txt > benchmark_results/all_results.txt

# Check if we have valid benchmark data
if grep -q "^Benchmark" benchmark_results/all_results.txt; then
    echo "✅ Valid benchmark data found"

    # Run statistical analysis
    if command -v benchstat &> /dev/null; then
        benchstat benchmark_results/all_results.txt > benchmark_results/stats_summary.txt
        echo "✅ Statistical analysis complete"

        # Generate HTML report if possible
        if benchstat -html benchmark_results/all_results.txt > benchmark_results/report.html 2>/dev/null; then
            echo "✅ HTML report generated: benchmark_results/report.html"
        else
            echo "⚠️  HTML generation failed"
        fi
    else
        echo "❌ benchstat not found. Install with: go install golang.org/x/perf/cmd/benchstat@latest"
        echo "Raw benchmark data available in all_results.txt"
    fi
else
    echo "❌ No valid benchmark data found in results"
    echo "No valid benchmark data found" > benchmark_results/stats_summary.txt
fi

# Add metadata (especially useful for CI)
echo "Generated on: $(date)" > benchmark_results/metadata.txt
echo "Go version: $(go version)" >> benchmark_results/metadata.txt
echo "Script iterations: $ITERATIONS" >> benchmark_results/metadata.txt
echo "Measurement time: ${MEASUREMENT_TIME}s" >> benchmark_results/metadata.txt
echo "Sample size per iteration: $SAMPLE_SIZE" >> benchmark_results/metadata.txt
echo "Successful iterations: $success/$ITERATIONS" >> benchmark_results/metadata.txt

# Add Git/CI metadata if available
if [ -n "$GITHUB_SHA" ]; then
    echo "Commit: $GITHUB_SHA" >> benchmark_results/metadata.txt
fi
if [ -n "$GITHUB_REF_NAME" ]; then
    echo "Branch: $GITHUB_REF_NAME" >> benchmark_results/metadata.txt
fi
if [ -n "$GITHUB_WORKFLOW" ]; then
    echo "Workflow: $GITHUB_WORKFLOW" >> benchmark_results/metadata.txt
fi

# Generate comparison reports (if you have previous results)
if [ -f benchmark_results/previous_results.txt ]; then
    echo "Generating comparison with previous results..."
    if command -v benchstat &> /dev/null; then
        benchstat benchmark_results/previous_results.txt benchmark_results/all_results.txt > benchmark_results/comparison.txt
        echo "✅ Comparison report generated: benchmark_results/comparison.txt"
    fi
fi

# Display summary
echo ""
echo "============================================"
echo "Benchmark Results Summary"
echo "============================================"
if [ -f benchmark_results/stats_summary.txt ]; then
    cat benchmark_results/stats_summary.txt
else
    echo "❌ No statistical summary available"
    echo "Check benchmark_results/error.txt for details"
fi

echo ""
echo "============================================"
echo "Files Generated"
echo "============================================"
ls -la benchmark_results/

echo ""
echo "============================================"
echo "Quick Instructions"
echo "============================================"
echo "View detailed results: cat benchmark_results/all_results.txt"
if [ -f benchmark_results/report.html ]; then
    echo "View HTML report: open benchmark_results/report.html"
fi
echo "Compare with previous: benchstat old_results.txt benchmark_results/all_results.txt"
echo "============================================"
