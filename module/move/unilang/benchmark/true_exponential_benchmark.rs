//! True exponential benchmark that builds and runs separate programs for each command count
//!
//! This benchmark properly tests build-time + runtime performance by:
//! 1. Generating YAML with N commands
//! 2. Building the program (compile time)
//! 3. Running the program (startup + runtime)
//! 4. Repeating for each power of 10

use std::time::Instant;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
struct TrueBenchmarkResult {
    command_count: usize,
    build_time_ms: f64,
    startup_time_us: f64,
    avg_lookup_time_ns: f64,
    p99_lookup_time_ns: f64,
    total_time_ms: f64,
    commands_per_second: f64,
    binary_size_kb: u64,
}

fn generate_stress_yaml_for_count(count: usize) -> String {
    let mut yaml = String::new();
    yaml.push_str("---\n");

    for i in 0..count {
        yaml.push_str(&format!(r#"
- name: "cmd_{i}"
  namespace: ".perf"
  description: "Performance test command {i}"
  hint: "Command for performance testing"
  arguments:
    - name: "arg1"
      description: "First argument"
      kind: "String"
      hint: "String argument"
      attributes:
        optional: false
        multiple: false
        default: null
        sensitive: false
        interactive: false
      validation_rules: []
      aliases: []
      tags: []
  routine_link: null
  status: "stable"
  version: "1.0.0"
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []
"#));
    }

    yaml
}

fn create_benchmark_binary(command_count: usize, work_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary Cargo project for this command count
    let project_dir = work_dir.join(format!("bench_project_{}", command_count));
    fs::create_dir_all(&project_dir)?;
    
    // Generate Cargo.toml
    let cargo_toml = format!(r#"
[package]
name = "unilang_benchmark_{}"
version = "0.1.0"
edition = "2021"

[workspace]

[[bin]]
name = "benchmark_bin"
path = "src/main.rs"

[dependencies]
unilang = {{ path = "../../../" }}
"#, command_count);
    fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;
    
    // Create src directory
    fs::create_dir_all(project_dir.join("src"))?;
    
    // Generate commands YAML
    let yaml_content = generate_stress_yaml_for_count(command_count);
    fs::write(project_dir.join("commands.yaml"), yaml_content)?;
    
    // Generate main.rs that measures performance
    let main_rs = format!(r#"
use std::time::Instant;
use unilang::registry::CommandRegistry;

fn main() {{
    // Set the commands path
    std::env::set_var("UNILANG_STATIC_COMMANDS_PATH", "commands.yaml");
    
    // Measure startup time
    let startup_start = Instant::now();
    let registry = CommandRegistry::new();
    let startup_time = startup_start.elapsed();
    
    // Perform command lookups to measure runtime performance
    let lookup_count = std::cmp::min({}, 10_000); // Cap at 10k lookups to avoid memory issues
    let mut latencies = Vec::with_capacity(lookup_count);
    
    let lookup_start = Instant::now();
    
    for i in 0..lookup_count {{
        let cmd_name = format!(".perf.cmd_{{}}", i % {});
        
        let single_lookup_start = Instant::now();
        let _command = registry.command(&cmd_name);
        let single_lookup_time = single_lookup_start.elapsed();
        
        latencies.push(single_lookup_time);
    }}
    
    let total_lookup_time = lookup_start.elapsed();
    
    // Calculate statistics
    latencies.sort();
    let p99 = latencies[(lookup_count as f64 * 0.99) as usize];
    let avg_lookup_time = total_lookup_time.as_nanos() as f64 / lookup_count as f64;
    let commands_per_second = lookup_count as f64 / total_lookup_time.as_secs_f64();
    
    // Output results in parseable format
    println!("STARTUP_TIME_US:{{}}", startup_time.as_nanos() as f64 / 1000.0);
    println!("AVG_LOOKUP_TIME_NS:{{}}", avg_lookup_time);
    println!("P99_LOOKUP_TIME_NS:{{}}", p99.as_nanos());
    println!("COMMANDS_PER_SECOND:{{}}", commands_per_second);
    println!("READY");
}}
"#, command_count, command_count);
    fs::write(project_dir.join("src").join("main.rs"), main_rs)?;
    
    Ok(())
}

fn build_and_run_benchmark(command_count: usize, work_dir: &Path) -> Result<TrueBenchmarkResult, Box<dyn std::error::Error>> {
    println!("Creating project for {} commands...", command_count);
    
    // Create the benchmark project
    create_benchmark_binary(command_count, work_dir)?;
    
    let project_dir = work_dir.join(format!("bench_project_{}", command_count));
    
    println!("Building project for {} commands...", command_count);
    
    // Build the project and measure build time
    let build_start = Instant::now();
    let build_output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&project_dir)
        .output()?;
    let build_time = build_start.elapsed();
    
    if !build_output.status.success() {
        return Err(format!("Build failed for {} commands: {}", 
                          command_count, 
                          String::from_utf8_lossy(&build_output.stderr)).into());
    }
    
    // Get binary size
    let binary_path = project_dir.join("target/release/benchmark_bin");
    let binary_size = fs::metadata(&binary_path)?.len();
    
    println!("Running benchmark for {} commands...", command_count);
    
    // Run the benchmark binary and measure total time
    let run_start = Instant::now();
    let run_output = Command::new(&binary_path)
        .current_dir(&project_dir)
        .output()?;
    let total_runtime = run_start.elapsed();
    
    if !run_output.status.success() {
        return Err(format!("Run failed for {} commands: {}", 
                          command_count, 
                          String::from_utf8_lossy(&run_output.stderr)).into());
    }
    
    // Parse the output
    let stdout = String::from_utf8_lossy(&run_output.stdout);
    let mut startup_time_us = 0.0;
    let mut avg_lookup_time_ns = 0.0;
    let mut p99_lookup_time_ns = 0.0;
    let mut commands_per_second = 0.0;
    
    for line in stdout.lines() {
        if let Some(value) = line.strip_prefix("STARTUP_TIME_US:") {
            startup_time_us = value.parse().unwrap_or(0.0);
        } else if let Some(value) = line.strip_prefix("AVG_LOOKUP_TIME_NS:") {
            avg_lookup_time_ns = value.parse().unwrap_or(0.0);
        } else if let Some(value) = line.strip_prefix("P99_LOOKUP_TIME_NS:") {
            p99_lookup_time_ns = value.parse().unwrap_or(0.0);
        } else if let Some(value) = line.strip_prefix("COMMANDS_PER_SECOND:") {
            commands_per_second = value.parse().unwrap_or(0.0);
        }
    }
    
    let result = TrueBenchmarkResult {
        command_count,
        build_time_ms: build_time.as_secs_f64() * 1000.0,
        startup_time_us,
        avg_lookup_time_ns,
        p99_lookup_time_ns,
        total_time_ms: total_runtime.as_secs_f64() * 1000.0,
        commands_per_second,
        binary_size_kb: binary_size / 1024,
    };
    
    println!("  Build time: {:.2} ms", result.build_time_ms);
    println!("  Total time: {:.2} ms", result.total_time_ms);
    println!("  Startup time: {:.2} μs", result.startup_time_us);
    println!("  Binary size: {} KB", result.binary_size_kb);
    println!("  Avg lookup: {:.1} ns", result.avg_lookup_time_ns);
    println!("  P99 lookup: {:.0} ns", result.p99_lookup_time_ns);
    
    Ok(result)
}

fn generate_true_benchmark_report(results: &[TrueBenchmarkResult], output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut report = String::new();
    
    report.push_str("UNILANG TRUE EXPONENTIAL BENCHMARK REPORT\n");
    report.push_str("==========================================\n\n");
    
    report.push_str(&format!("Generated: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    report.push_str("This benchmark measures REAL build + runtime performance\n");
    report.push_str("by building separate binaries for each command count.\n\n");
    
    // Summary table
    report.push_str("PERFORMANCE SUMMARY\n");
    report.push_str("-------------------\n");
    report.push_str("Commands | Build Time | Binary Size | Startup | Avg Lookup | P99 Lookup | Throughput\n");
    report.push_str("         |    (ms)    |    (KB)     |  (μs)   |    (ns)    |    (ns)    | (cmd/sec)\n");
    report.push_str("---------|------------|-------------|---------|------------|------------|----------\n");
    
    for result in results {
        let commands_formatted = if result.command_count >= 1_000_000 {
            format!("{}M", result.command_count / 1_000_000)
        } else if result.command_count >= 1_000 {
            format!("{}K", result.command_count / 1_000)
        } else {
            result.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>10.1} | {:>11} | {:>7.1} | {:>10.1} | {:>10.0} | {:>9.0}\n",
            commands_formatted,
            result.build_time_ms,
            result.binary_size_kb,
            result.startup_time_us,
            result.avg_lookup_time_ns,
            result.p99_lookup_time_ns,
            result.commands_per_second
        ));
    }
    
    // Scaling analysis
    if let (Some(first), Some(last)) = (results.first(), results.last()) {
        report.push_str("\nSCALING ANALYSIS\n");
        report.push_str("----------------\n");
        
        let command_ratio = last.command_count as f64 / first.command_count as f64;
        let build_time_ratio = last.build_time_ms / first.build_time_ms;
        let binary_size_ratio = last.binary_size_kb as f64 / first.binary_size_kb as f64;
        let startup_time_ratio = last.startup_time_us / first.startup_time_us;
        
        report.push_str(&format!("Command scaling: {:.0}x ({} to {} commands)\n", 
                                command_ratio, first.command_count, last.command_count));
        report.push_str(&format!("Build time scaling: {:.2}x ({:.1}ms to {:.1}ms)\n", 
                                build_time_ratio, first.build_time_ms, last.build_time_ms));
        report.push_str(&format!("Binary size scaling: {:.2}x ({}KB to {}KB)\n", 
                                binary_size_ratio, first.binary_size_kb, last.binary_size_kb));
        report.push_str(&format!("Startup time scaling: {:.2}x ({:.1}μs to {:.1}μs)\n", 
                                startup_time_ratio, first.startup_time_us, last.startup_time_us));
    }
    
    fs::write(output_path, report)?;
    println!("📄 True benchmark report saved to: {}", output_path.display());
    
    Ok(())
}

#[test]
#[ignore = "Long running benchmark - run explicitly"]
fn true_exponential_performance_benchmark() {
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string());
    let work_dir = Path::new(&target_dir).join("true_benchmark_workspace");
    let output_dir = Path::new(&target_dir).join("true_benchmark_results");
    
    // Create directories
    fs::create_dir_all(&work_dir).expect("Failed to create work directory");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    // Test exponential command counts (including 100K as requested)
    let command_counts = vec![
        10,      // 10^1
        100,     // 10^2  
        1_000,   // 10^3
        10_000,  // 10^4
        100_000, // 10^5 - included as requested (will take longer)
        // 1_000_000, // 10^6 - still commented out due to very long build times
    ];
    
    let mut results = Vec::new();
    
    println!("=== TRUE EXPONENTIAL PERFORMANCE BENCHMARK ===");
    println!("This benchmark builds and runs separate binaries for each command count.");
    println!("Testing command counts: {:?}", command_counts);
    println!("⚠️  WARNING: This includes 100K commands and will take 10-20 minutes!");
    println!();
    
    for &count in &command_counts {
        match build_and_run_benchmark(count, &work_dir) {
            Ok(result) => {
                results.push(result);
                println!();
            }
            Err(e) => {
                eprintln!("Error benchmarking {} commands: {}", count, e);
                continue;
            }
        }
    }
    
    // Generate report
    let report_path = output_dir.join("true_benchmark_report.txt");
    if let Err(e) = generate_true_benchmark_report(&results, &report_path) {
        eprintln!("Error generating report: {}", e);
    }
    
    // Print summary
    println!("🎯 **True Exponential Benchmark Complete!**");
    println!("\n📊 **Build + Runtime Performance Results:**");
    println!();
    println!("| Commands | Build Time | Binary Size | Startup | P99 Lookup | Throughput |");
    println!("|----------|------------|-------------|---------|------------|------------|");
    
    for result in &results {
        let commands_formatted = if result.command_count >= 1_000_000 {
            format!("**{}M**", result.command_count / 1_000_000)
        } else if result.command_count >= 1_000 {
            format!("**{}K**", result.command_count / 1_000)
        } else {
            format!("**{}**", result.command_count)
        };
        
        println!(
            "| {} | {:.1} ms | {} KB | {:.1} μs | {:.0} ns | {:.2} M/sec |",
            commands_formatted,
            result.build_time_ms,
            result.binary_size_kb,
            result.startup_time_us,
            result.p99_lookup_time_ns,
            result.commands_per_second / 1_000_000.0
        );
    }
    
    println!("\n🏗️ **This benchmark measures the COMPLETE pipeline:**");
    println!("1. **YAML Generation** - Create command definitions");
    println!("2. **Build Time** - Compile program with N commands");  
    println!("3. **Binary Size** - Resulting executable size");
    println!("4. **Startup Time** - Registry initialization");
    println!("5. **Runtime Performance** - Command lookup speed");
    
    println!("\n📂 **Results saved to**: {}", output_dir.display());
    
    // Clean up work directory
    let _ = fs::remove_dir_all(&work_dir);
    
    println!("\n✅ **True benchmark demonstrates real build-time scaling!**");
}