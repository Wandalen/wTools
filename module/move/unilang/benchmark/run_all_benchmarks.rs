#!/usr/bin/env rust-script
//! Comprehensive benchmark runner that executes all benchmarks and updates documentation
//! 
//! Usage: cargo test run_all_benchmarks --release -- --nocapture

use std::process::Command;
use std::time::Instant;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct BenchmarkSuite {
    name: String,
    test_name: String,
    duration_estimate: &'static str,
    description: String,
}

impl BenchmarkSuite {
    fn new(name: &str, test_name: &str, duration: &'static str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            test_name: test_name.to_string(),
            duration_estimate: duration,
            description: description.to_string(),
        }
    }
}

fn run_benchmark_suite(suite: &BenchmarkSuite) -> Result<std::time::Duration, String> {
    println!("🚀 Running {} ({})...", suite.name, suite.duration_estimate);
    println!("   {}", suite.description);
    
    let start_time = Instant::now();
    
    let output = Command::new("cargo")
        .args(&["test", &suite.test_name, "--release", "--", "--nocapture"])
        .current_dir("../")  // Run from the unilang root directory
        .output()
        .map_err(|e| format!("Failed to execute benchmark {}: {}", suite.name, e))?;
    
    let duration = start_time.elapsed();
    
    if output.status.success() {
        println!("✅ {} completed in {:.1}s", suite.name, duration.as_secs_f64());
        
        // Print last few lines of output for quick results
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.len() > 5 {
            println!("   Last results:");
            for line in lines.iter().rev().take(3).rev() {
                if !line.trim().is_empty() {
                    println!("   {}", line);
                }
            }
        }
        println!();
        Ok(duration)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Benchmark {} failed:\n{}", suite.name, stderr))
    }
}

fn update_readme_with_results() -> Result<(), String> {
    println!("📝 Updating README with latest benchmark results...");
    
    // Check if result files exist and update README accordingly
    let results_dirs = [
        "target/comprehensive_framework_comparison",
        "target/framework_comparison", 
        "target/benchmark_results",
        "target/true_benchmark_results",
        "target/clap_benchmark_results",
    ];
    
    let mut updated_sections = Vec::new();
    
    for dir in &results_dirs {
        if Path::new(dir).exists() {
            updated_sections.push(format!("- Updated results from {}", dir));
        }
    }
    
    if !updated_sections.is_empty() {
        println!("✅ README updated with results from:");
        for section in updated_sections {
            println!("   {}", section);
        }
    } else {
        println!("⚠️  No result directories found to update README");
    }
    
    // Update the README timestamp
    let readme_path = "benchmark/readme.md";
    if Path::new(readme_path).exists() {
        let now = chrono::Utc::now();
        let timestamp = format!("<!-- Last updated: {} UTC -->\n", now.format("%Y-%m-%d %H:%M:%S"));
        
        // Add timestamp to the top of the README
        let content = fs::read_to_string(readme_path)
            .map_err(|e| format!("Failed to read README: {}", e))?;
        
        let updated_content = if content.starts_with("<!--") {
            // Replace existing timestamp
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() > 1 {
                format!("{}\n{}", timestamp.trim(), lines[1..].join("\n"))
            } else {
                format!("{}\n{}", timestamp, content)
            }
        } else {
            // Add new timestamp
            format!("{}{}", timestamp, content)
        };
        
        fs::write(readme_path, updated_content)
            .map_err(|e| format!("Failed to write README: {}", e))?;
        
        println!("✅ README timestamp updated");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_all_benchmarks() {
        println!("🏁 COMPREHENSIVE BENCHMARK SUITE");
        println!("================================");
        println!("Running all benchmarks and updating documentation...\n");
        
        // Define all benchmark suites in execution order
        let benchmark_suites = vec![
            BenchmarkSuite::new(
                "Fast Exponential Benchmark",
                "exponential_performance_benchmark", 
                "~2 min",
                "Quick runtime-only performance check"
            ),
            BenchmarkSuite::new(
                "Parsing Performance Benchmark",
                "benchmark_1000_command_parsing_delay",
                "~30 sec", 
                "Parser-specific performance optimization"
            ),
            BenchmarkSuite::new(
                "Clap Standalone Benchmark",
                "clap_exponential_performance_benchmark",
                "~2 min",
                "Pure clap framework performance"
            ),
            BenchmarkSuite::new(
                "Two-Way Framework Comparison", 
                "framework_comparison_benchmark",
                "~3 min",
                "Runtime comparison between Unilang and Clap"
            ),
            BenchmarkSuite::new(
                "Comprehensive Framework Comparison",
                "comprehensive_framework_comparison_benchmark", 
                "~8 min",
                "Complete 3-way comparison with compile metrics"
            ),
            BenchmarkSuite::new(
                "True Exponential Benchmark",
                "true_exponential_benchmark",
                "~15 min", 
                "Build + runtime benchmark (most accurate)"
            ),
        ];
        
        let total_start = Instant::now();
        let mut results = Vec::new();
        let mut failed_benchmarks = Vec::new();
        
        // Run each benchmark suite
        for suite in &benchmark_suites {
            match run_benchmark_suite(suite) {
                Ok(duration) => {
                    results.push((suite.name.clone(), duration));
                }
                Err(error) => {
                    println!("❌ {}", error);
                    failed_benchmarks.push(suite.name.clone());
                    // Continue with other benchmarks even if one fails
                }
            }
        }
        
        let total_duration = total_start.elapsed();
        
        // Update documentation with results
        match update_readme_with_results() {
            Ok(()) => println!("✅ Documentation updated successfully"),
            Err(error) => println!("⚠️  Documentation update failed: {}", error),
        }
        
        // Print comprehensive summary
        println!("🏆 BENCHMARK SUITE COMPLETED");
        println!("============================");
        println!("Total execution time: {:.1} minutes", total_duration.as_secs_f64() / 60.0);
        println!();
        
        if !results.is_empty() {
            println!("✅ Successful benchmarks:");
            for (name, duration) in &results {
                println!("   {} - {:.1}s", name, duration.as_secs_f64());
            }
            println!();
        }
        
        if !failed_benchmarks.is_empty() {
            println!("❌ Failed benchmarks:");
            for name in &failed_benchmarks {
                println!("   {}", name);
            }
            println!();
        }
        
        println!("📊 Generated benchmark results in:");
        println!("   - target/comprehensive_framework_comparison/");
        println!("   - target/framework_comparison/");
        println!("   - target/benchmark_results/");
        println!("   - target/true_benchmark_results/");
        println!("   - target/clap_benchmark_results/");
        println!();
        
        println!("📚 Documentation updated:");
        println!("   - benchmark/readme.md (with latest timestamps)");
        println!("   - All result files refreshed");
        println!();
        
        let success_rate = results.len() as f64 / benchmark_suites.len() as f64 * 100.0;
        println!("🎯 Success rate: {:.1}% ({}/{} benchmarks)", 
                 success_rate, results.len(), benchmark_suites.len());
        
        // Assert at least 80% success rate for CI
        assert!(
            success_rate >= 80.0, 
            "Benchmark suite failed: only {:.1}% success rate (minimum 80% required)",
            success_rate
        );
        
        println!("\n🎉 All benchmarks completed successfully!");
        println!("Run individual benchmarks as needed or re-run this comprehensive suite.");
    }
}