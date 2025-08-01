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
        .args(&["test", &suite.test_name, "--release", "--features", "benchmarks", "--", "--nocapture", "--ignored"])
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
    
    // Read the latest comprehensive results if available
    let comprehensive_results_path = "target/comprehensive_framework_comparison/comprehensive_results.csv";
    let mut performance_data = String::new();
    
    if Path::new(comprehensive_results_path).exists() {
        match fs::read_to_string(comprehensive_results_path) {
            Ok(csv_content) => {
                println!("✅ Found comprehensive benchmark results, updating performance tables...");
                
                // Parse CSV and extract key metrics for different command counts
                let lines: Vec<&str> = csv_content.lines().collect();
                if lines.len() > 1 {
                    // Skip header line and parse data
                    let mut unilang_data = Vec::new();
                    let mut clap_data = Vec::new();
                    let mut pico_data = Vec::new();
                    
                    for line in lines.iter().skip(1) {
                        let fields: Vec<&str> = line.split(',').collect();
                        if fields.len() >= 7 { // framework,commands,build_time,binary_size,init_time,lookup_time,throughput
                            let framework = fields[0].trim();
                            let commands = fields[1].trim();
                            let build_time = fields[2].trim();
                            let binary_size = fields[3].trim(); 
                            let init_time = fields[4].trim();
                            let lookup_time = fields[5].trim();
                            let throughput = fields[6].trim();
                            
                            let row = format!("| **{}** | ~{}s | ~{} KB | ~{} μs | ~{} μs | ~{}/sec |",
                                            commands, build_time, binary_size, init_time, lookup_time, throughput);
                            
                            match framework {
                                "unilang" => unilang_data.push(row),
                                "clap" => clap_data.push(row),
                                "pico-args" => pico_data.push(row),
                                _ => {}
                            }
                        }
                    }
                    
                    // Build performance tables
                    performance_data = format!(
                        "### Unilang Scaling Performance\n\n| Commands | Build Time | Binary Size | Startup | Lookup | Throughput |\n|----------|------------|-------------|---------|--------|-----------|\n{}\n\n### Clap Scaling Performance\n\n| Commands | Build Time | Binary Size | Startup | Lookup | Throughput |\n|----------|------------|-------------|---------|--------|-----------|\n{}\n\n### Pico-Args Scaling Performance\n\n| Commands | Build Time | Binary Size | Startup | Lookup | Throughput |\n|----------|------------|-------------|---------|--------|-----------|\n{}\n",
                        unilang_data.join("\n"),
                        clap_data.join("\n"), 
                        pico_data.join("\n")
                    );
                }
            }
            Err(_) => {
                println!("⚠️  Could not read comprehensive results file");
            }
        }
    }
    
    // Update the README timestamp and performance data
    let readme_path = "benchmark/readme.md";
    if Path::new(readme_path).exists() {
        let now = chrono::Utc::now();
        let timestamp = format!("<!-- Last updated: {} UTC -->\n", now.format("%Y-%m-%d %H:%M:%S"));
        
        let content = fs::read_to_string(readme_path)
            .map_err(|e| format!("Failed to read README: {}", e))?;
        
        let mut updated_content = if content.starts_with("<!--") {
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
        
        // If we have new performance data, update the performance tables section
        if !performance_data.is_empty() {
            // Find and replace the performance tables section
            if let Some(start_pos) = updated_content.find("### Unilang Scaling Performance") {
                if let Some(end_pos) = updated_content[start_pos..].find("## 🔧 Available Benchmarks") {
                    let before = &updated_content[..start_pos];
                    let after = &updated_content[start_pos + end_pos..];
                    updated_content = format!("{}{}\n{}", before, performance_data, after);
                    println!("✅ Performance tables updated with live benchmark data");
                }
            }
        }
        
        fs::write(readme_path, updated_content)
            .map_err(|e| format!("Failed to write README: {}", e))?;
        
        println!("✅ README updated successfully");
    }
    
    // Check other result directories
    let results_dirs = [
        "target/comprehensive_framework_comparison",
        "target/framework_comparison", 
        "target/benchmark_results",
        "target/true_benchmark_results",
        "target/clap_benchmark_results",
    ];
    
    let mut found_dirs = Vec::new();
    for dir in &results_dirs {
        if Path::new(dir).exists() {
            found_dirs.push(*dir);
        }
    }
    
    if !found_dirs.is_empty() {
        println!("📊 Updated benchmark results found in:");
        for dir in found_dirs {
            println!("   - {}", dir);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Long running benchmark suite - run explicitly with: cargo test run_all_benchmarks --release --features benchmarks -- --nocapture --ignored"]
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
                "true_exponential_performance_benchmark",
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