//! Throughput-only benchmark for command parsing performance.
//!
//! This benchmark focuses exclusively on runtime throughput testing across
//! different command counts, without compile-time measurements. Designed for
//! quick performance validation and regression testing.

#[cfg(feature = "benchmarks")]
use std::time::Instant;
#[cfg(feature = "benchmarks")]
use unilang::prelude::*;

#[cfg(feature = "benchmarks")]
use clap::{Arg, Command as ClapCommand};
#[cfg(feature = "benchmarks")]
use pico_args::Arguments;

#[derive(Debug, Clone)]
#[cfg(feature = "benchmarks")]
struct ThroughputResult {
    framework: String,
    command_count: usize,
    init_time_us: f64,
    avg_lookup_ns: f64,
    p50_lookup_ns: u64,
    p95_lookup_ns: u64,
    p99_lookup_ns: u64,
    max_lookup_ns: u64,
    commands_per_second: f64,
    iterations_tested: usize,
}

#[cfg(feature = "benchmarks")]
fn benchmark_unilang_throughput(command_count: usize) -> ThroughputResult {
    println!("🦀 Throughput testing Unilang with {} commands", command_count);

    // Create command registry with N commands
    let init_start = Instant::now();
    let mut registry = CommandRegistry::new();
    
    // Add N commands to registry
    for i in 0..command_count {
        let cmd = CommandDefinition {
            name: format!("cmd_{}", i),
            namespace: ".perf".to_string(),
            description: format!("Performance test command {}", i),
            hint: "Performance test".to_string(),
            arguments: vec![
                ArgumentDefinition {
                    name: "input".to_string(),
                    description: "Input parameter".to_string(),
                    kind: Kind::String,
                    hint: "Input value".to_string(),
                    attributes: ArgumentAttributes::default(),
                    validation_rules: vec![],
                    aliases: vec!["i".to_string()],
                    tags: vec![],
                },
                ArgumentDefinition {
                    name: "verbose".to_string(),
                    description: "Enable verbose output".to_string(),
                    kind: Kind::Boolean,
                    hint: "Verbose flag".to_string(),
                    attributes: ArgumentAttributes {
                        optional: true,
                        default: Some("false".to_string()),
                        ..Default::default()
                    },
                    validation_rules: vec![],
                    aliases: vec!["v".to_string()],
                    tags: vec![],
                },
            ],
            routine_link: None,
            status: "stable".to_string(),
            version: "1.0.0".to_string(),
            tags: vec![],
            aliases: vec![],
            permissions: vec![],
            idempotent: true,
            deprecation_message: String::new(),
            http_method_hint: String::new(),
            examples: vec![],
        };
        
        registry.register(cmd);
    }
    
    let init_time = init_start.elapsed();
    let init_time_us = init_time.as_nanos() as f64 / 1000.0;

    // Create pipeline for command processing
    let pipeline = Pipeline::new(registry);
    
    // Generate test commands covering all registered commands
    let test_commands: Vec<String> = (0..command_count)
        .map(|i| format!(".perf.cmd_{} input::test_{} verbose::true", i, i))
        .collect();

    // Extended test set for better statistical sampling
    let iterations = (command_count * 10).max(1000).min(50000);
    let test_set: Vec<&String> = (0..iterations)
        .map(|i| &test_commands[i % test_commands.len()])
        .collect();

    // Warmup phase
    for cmd in test_set.iter().take(100.min(iterations / 10)) {
        let _ = pipeline.process_command_simple(cmd);
    }

    // Main throughput benchmark
    let mut lookup_times = Vec::with_capacity(iterations);
    let total_start = Instant::now();

    for cmd in &test_set {
        let lookup_start = Instant::now();
        let _ = pipeline.process_command_simple(cmd);
        let lookup_time = lookup_start.elapsed();
        lookup_times.push(lookup_time.as_nanos() as u64);
    }

    let total_time = total_start.elapsed();
    
    // Calculate statistical metrics
    lookup_times.sort_unstable();
    let avg_lookup_ns = lookup_times.iter().sum::<u64>() as f64 / lookup_times.len() as f64;
    let p50_lookup_ns = lookup_times[lookup_times.len() / 2];
    let p95_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.95) as usize];
    let p99_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.99) as usize];
    let max_lookup_ns = *lookup_times.last().unwrap();
    let commands_per_second = iterations as f64 / total_time.as_secs_f64();

    println!("  📊 Init: {:.1}μs, Avg: {:.0}ns, P99: {}ns, Throughput: {:.0}/s", 
             init_time_us, avg_lookup_ns, p99_lookup_ns, commands_per_second);

    ThroughputResult {
        framework: "unilang".to_string(),
        command_count,
        init_time_us,
        avg_lookup_ns,
        p50_lookup_ns,
        p95_lookup_ns,
        p99_lookup_ns,
        max_lookup_ns,
        commands_per_second,
        iterations_tested: iterations,
    }
}

#[cfg(feature = "benchmarks")]
fn benchmark_clap_throughput(command_count: usize) -> ThroughputResult {
    println!("🗡️  Throughput testing Clap with {} commands", command_count);

    // Create clap app with N subcommands
    let init_start = Instant::now();
    let mut app = ClapCommand::new("benchmark")
        .version("1.0")
        .about("Clap throughput benchmark");

    for i in 0..command_count {
        // Use simple static names for the first few, then fallback to generated ones
        let (cmd_name, cmd_desc) = match i {
            0 => ("cmd_0", "Performance test command 0"),
            1 => ("cmd_1", "Performance test command 1"),
            2 => ("cmd_2", "Performance test command 2"),
            3 => ("cmd_3", "Performance test command 3"),
            _ => ("cmd_dynamic", "Performance test command dynamic"),
        };
        
        let subcommand = ClapCommand::new(cmd_name)
            .about(cmd_desc)
            .arg(Arg::new("input")
                .short('i')
                .long("input")
                .help("Input parameter")
                .value_name("VALUE"))
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue));
        
        app = app.subcommand(subcommand);
    }
    
    let init_time = init_start.elapsed();
    let init_time_us = init_time.as_nanos() as f64 / 1000.0;

    // Generate test commands
    let iterations = (command_count * 10).max(1000).min(50000);
    let test_commands: Vec<Vec<String>> = (0..iterations)
        .map(|i| {
            let cmd_idx = i % command_count;
            vec![
                "benchmark".to_string(),
                format!("cmd_{}", cmd_idx),
                "--input".to_string(),
                format!("test_{}", i),
                "--verbose".to_string(),
            ]
        })
        .collect();

    // Warmup
    for args in test_commands.iter().take(100.min(iterations / 10)) {
        let app_clone = app.clone();
        let _ = app_clone.try_get_matches_from(args);
    }

    // Main benchmark
    let mut lookup_times = Vec::with_capacity(iterations);
    let total_start = Instant::now();

    for args in &test_commands {
        let lookup_start = Instant::now();
        let app_clone = app.clone();
        let _ = app_clone.try_get_matches_from(args);
        let lookup_time = lookup_start.elapsed();
        lookup_times.push(lookup_time.as_nanos() as u64);
    }

    let total_time = total_start.elapsed();
    
    // Calculate statistics
    lookup_times.sort_unstable();
    let avg_lookup_ns = lookup_times.iter().sum::<u64>() as f64 / lookup_times.len() as f64;
    let p50_lookup_ns = lookup_times[lookup_times.len() / 2];
    let p95_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.95) as usize];
    let p99_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.99) as usize];
    let max_lookup_ns = *lookup_times.last().unwrap();
    let commands_per_second = iterations as f64 / total_time.as_secs_f64();

    println!("  📊 Init: {:.1}μs, Avg: {:.0}ns, P99: {}ns, Throughput: {:.0}/s", 
             init_time_us, avg_lookup_ns, p99_lookup_ns, commands_per_second);

    ThroughputResult {
        framework: "clap".to_string(),
        command_count,
        init_time_us,
        avg_lookup_ns,
        p50_lookup_ns,
        p95_lookup_ns,
        p99_lookup_ns,
        max_lookup_ns,
        commands_per_second,
        iterations_tested: iterations,
    }
}

#[cfg(feature = "benchmarks")]
fn benchmark_pico_args_throughput(command_count: usize) -> ThroughputResult {
    println!("⚡ Throughput testing Pico-Args with {} commands", command_count);

    let init_start = Instant::now();
    // pico-args doesn't have complex initialization, so we just track timing
    let _arg_keys: Vec<String> = (0..command_count)
        .map(|i| format!("cmd-{}", i))
        .collect();
    let init_time = init_start.elapsed();
    let init_time_us = init_time.as_nanos() as f64 / 1000.0;

    // Generate test arguments
    let iterations = (command_count * 10).max(1000).min(50000);
    let test_args: Vec<Vec<String>> = (0..iterations)
        .map(|i| {
            let cmd_idx = i % command_count;
            vec![
                "benchmark".to_string(),
                format!("--cmd-{}", cmd_idx),
                format!("test_{}", i),
            ]
        })
        .collect();

    // Warmup
    for args_vec in test_args.iter().take(100.min(iterations / 10)) {
        let args = Arguments::from_vec(args_vec.iter().map(|s| s.into()).collect());
        let _ = args.finish();
    }

    // Main benchmark
    let mut lookup_times = Vec::with_capacity(iterations);
    let total_start = Instant::now();

    for args_vec in &test_args {
        let lookup_start = Instant::now();
        let args = Arguments::from_vec(args_vec.iter().map(|s| s.into()).collect());
        let _ = args.finish();
        let lookup_time = lookup_start.elapsed();
        lookup_times.push(lookup_time.as_nanos() as u64);
    }

    let total_time = total_start.elapsed();
    
    // Calculate statistics
    lookup_times.sort_unstable();
    let avg_lookup_ns = lookup_times.iter().sum::<u64>() as f64 / lookup_times.len() as f64;
    let p50_lookup_ns = lookup_times[lookup_times.len() / 2];
    let p95_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.95) as usize];
    let p99_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.99) as usize];
    let max_lookup_ns = *lookup_times.last().unwrap();
    let commands_per_second = iterations as f64 / total_time.as_secs_f64();

    println!("  📊 Init: {:.1}μs, Avg: {:.0}ns, P99: {}ns, Throughput: {:.0}/s", 
             init_time_us, avg_lookup_ns, p99_lookup_ns, commands_per_second);

    ThroughputResult {
        framework: "pico-args".to_string(),
        command_count,
        init_time_us,
        avg_lookup_ns,
        p50_lookup_ns,
        p95_lookup_ns,
        p99_lookup_ns,
        max_lookup_ns,
        commands_per_second,
        iterations_tested: iterations,
    }
}

#[cfg(feature = "benchmarks")]
fn generate_throughput_report(results: &[Vec<ThroughputResult>]) {
    use std::fs;
    
    println!("📊 Generating throughput performance report...");
    
    // Ensure output directory exists
    let output_dir = "target/throughput_benchmark";
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Generate detailed report
    let mut report = String::new();
    report.push_str("THROUGHPUT-ONLY BENCHMARK REPORT\n");
    report.push_str("================================\n\n");
    
    let now = chrono::Utc::now();
    report.push_str(&format!("Generated: {} UTC\n", now.format("%Y-%m-%d %H:%M:%S")));
    report.push_str("Test Type: Runtime throughput only (no compilation testing)\n");
    report.push_str("Frameworks: Unilang vs Clap vs Pico-Args\n");
    report.push_str("Statistical Method: Single run with extended sampling per command count\n\n");

    // Throughput comparison table
    report.push_str("THROUGHPUT COMPARISON (commands/second)\n");
    report.push_str("=======================================\n");
    report.push_str("Commands | Unilang    | Clap       | Pico-Args  | Winner     | Ratio\n");
    report.push_str("---------|------------|------------|------------|------------|-------\n");
    
    for result_set in results {
        let unilang = result_set.iter().find(|r| r.framework == "unilang").unwrap();
        let clap = result_set.iter().find(|r| r.framework == "clap").unwrap();
        let pico_args = result_set.iter().find(|r| r.framework == "pico-args").unwrap();
        
        let max_throughput = unilang.commands_per_second.max(clap.commands_per_second.max(pico_args.commands_per_second));
        let (winner, ratio) = if (unilang.commands_per_second - max_throughput).abs() < 1000.0 {
            ("🦀 Unilang", max_throughput / clap.commands_per_second.min(pico_args.commands_per_second))
        } else if (clap.commands_per_second - max_throughput).abs() < 1000.0 {
            ("🗡️ Clap", max_throughput / unilang.commands_per_second.min(pico_args.commands_per_second))
        } else {
            ("⚡ Pico-Args", max_throughput / unilang.commands_per_second.min(clap.commands_per_second))
        };
        
        let cmd_display = if unilang.command_count >= 1000 {
            format!("{}K", unilang.command_count / 1000)
        } else {
            unilang.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>10.0} | {:>10.0} | {:>10.0} | {:>10} | {:>5.1}x\n",
            cmd_display,
            unilang.commands_per_second,
            clap.commands_per_second,
            pico_args.commands_per_second,
            winner,
            ratio
        ));
    }

    // Latency comparison table
    report.push_str("\nLATENCY COMPARISON (P99 in nanoseconds)\n");
    report.push_str("======================================\n");
    report.push_str("Commands | Unilang    | Clap       | Pico-Args  | Winner\n");
    report.push_str("---------|------------|------------|------------|--------\n");
    
    for result_set in results {
        let unilang = result_set.iter().find(|r| r.framework == "unilang").unwrap();
        let clap = result_set.iter().find(|r| r.framework == "clap").unwrap();
        let pico_args = result_set.iter().find(|r| r.framework == "pico-args").unwrap();
        
        let min_p99 = unilang.p99_lookup_ns.min(clap.p99_lookup_ns.min(pico_args.p99_lookup_ns));
        let winner = if unilang.p99_lookup_ns == min_p99 { "🦀 Unilang" }
                    else if clap.p99_lookup_ns == min_p99 { "🗡️ Clap" }
                    else { "⚡ Pico-Args" };
        
        let cmd_display = if unilang.command_count >= 1000 {
            format!("{}K", unilang.command_count / 1000)
        } else {
            unilang.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>10} | {:>10} | {:>10} | {}\n",
            cmd_display, unilang.p99_lookup_ns, clap.p99_lookup_ns, pico_args.p99_lookup_ns, winner
        ));
    }

    // Summary and recommendations
    report.push_str("\nPERFORMANCE INSIGHTS\n");
    report.push_str("===================\n\n");
    report.push_str("**Quick Throughput Test Benefits:**\n");
    report.push_str("- ⚡ Fast execution (seconds vs minutes)\n");
    report.push_str("- 🎯 Focus on runtime performance only\n");
    report.push_str("- 📊 Extended statistical sampling\n");
    report.push_str("- 🔄 Ideal for regression testing\n\n");
    
    report.push_str("**When to use this benchmark:**\n");
    report.push_str("- Daily performance validation\n");
    report.push_str("- CI/CD pipeline integration\n");
    report.push_str("- Quick performance regression detection\n");
    report.push_str("- Runtime optimization validation\n\n");
    
    report.push_str("**For comprehensive analysis, also run:**\n");
    report.push_str("- Full benchmark: `cargo test run_all_benchmarks --release --features benchmarks -- --ignored`\n");
    report.push_str("- Compilation metrics: `cargo run --release --bin comprehensive_benchmark --features benchmarks`\n");

    fs::write("target/throughput_benchmark/throughput_report.txt", &report)
        .expect("Failed to write throughput report");

    // Generate CSV data
    let mut csv_content = format!("# Throughput-Only Benchmark Results\n");
    csv_content.push_str(&format!("# Generated: {} UTC\n", now.format("%Y-%m-%d %H:%M:%S")));
    csv_content.push_str("# Test Type: Runtime throughput only\n");
    csv_content.push_str("#\n");
    csv_content.push_str("framework,command_count,init_time_us,avg_lookup_ns,p50_lookup_ns,p95_lookup_ns,p99_lookup_ns,max_lookup_ns,commands_per_second,iterations_tested\n");
    
    for result_set in results {
        for result in result_set {
            csv_content.push_str(&format!(
                "{},{},{:.2},{:.2},{},{},{},{},{:.0},{}\n",
                result.framework,
                result.command_count,
                result.init_time_us,
                result.avg_lookup_ns,
                result.p50_lookup_ns,
                result.p95_lookup_ns,
                result.p99_lookup_ns,
                result.max_lookup_ns,
                result.commands_per_second,
                result.iterations_tested
            ));
        }
    }
    
    fs::write("target/throughput_benchmark/throughput_results.csv", &csv_content)
        .expect("Failed to write CSV results");

    println!("✅ Throughput benchmark results saved to:");
    println!("   - target/throughput_benchmark/throughput_report.txt");
    println!("   - target/throughput_benchmark/throughput_results.csv");
}

#[cfg(feature = "benchmarks")]
fn run_throughput_benchmark() {
    println!("🚀 Starting Throughput-Only Performance Benchmark");
    println!("=================================================");
    println!("Testing Unilang vs Clap vs Pico-Args runtime performance");
    println!("Focus: Command parsing throughput (no compilation testing)");
    println!("Duration: ~30-60 seconds\n");

    let command_counts = vec![10, 100, 1000, 10000];
    let mut all_results = Vec::new();

    for &count in &command_counts {
        println!("╔════════════════════════════════════════════╗");
        println!("║ 🎯 TESTING {} COMMANDS (throughput only)   ║", 
                 if count >= 1000 { format!("{}K", count/1000) } else { count.to_string() });
        println!("╚════════════════════════════════════════════╝");

        let unilang_result = benchmark_unilang_throughput(count);
        let clap_result = benchmark_clap_throughput(count);
        let pico_args_result = benchmark_pico_args_throughput(count);

        // Quick comparison
        let max_throughput = unilang_result.commands_per_second
            .max(clap_result.commands_per_second.max(pico_args_result.commands_per_second));
        let winner = if (unilang_result.commands_per_second - max_throughput).abs() < 1000.0 { "🦀 Unilang" }
                    else if (clap_result.commands_per_second - max_throughput).abs() < 1000.0 { "🗡️ Clap" }
                    else { "⚡ Pico-Args" };

        println!("🏆 Winner for {} commands: {} ({:.0} cmd/sec)", count, winner, max_throughput);
        println!();

        all_results.push(vec![unilang_result, clap_result, pico_args_result]);
    }

    // Generate comprehensive report
    generate_throughput_report(&all_results);

    println!("🎉 Throughput benchmark completed!");
    println!("\n📊 **Quick Summary:**");
    println!();
    println!("| Commands | Winner | Throughput | P99 Latency |");
    println!("|----------|--------|------------|-------------|");
    
    for (i, result_set) in all_results.iter().enumerate() {
        let unilang = &result_set[0];
        let clap = &result_set[1];
        let pico_args = &result_set[2];
        
        let cmd_display = if command_counts[i] >= 1000 {
            format!("{}K", command_counts[i] / 1000)
        } else {
            command_counts[i].to_string()
        };
        
        let max_throughput = unilang.commands_per_second.max(clap.commands_per_second.max(pico_args.commands_per_second));
        let min_p99 = unilang.p99_lookup_ns.min(clap.p99_lookup_ns.min(pico_args.p99_lookup_ns));
        
        let throughput_winner = if (unilang.commands_per_second - max_throughput).abs() < 1000.0 { "🦀 Unilang" }
                               else if (clap.commands_per_second - max_throughput).abs() < 1000.0 { "🗡️ Clap" }
                               else { "⚡ Pico-Args" };
        
        println!("| {:>8} | {:>10} | {:>8.0}/s | {:>9}ns |",
                 cmd_display, throughput_winner, max_throughput, min_p99);
    }

    println!("\n✨ For detailed analysis, see: target/throughput_benchmark/throughput_report.txt");
}

fn main() {
    #[cfg(feature = "benchmarks")]
    {
        run_throughput_benchmark();
    }
    
    #[cfg(not(feature = "benchmarks"))]
    {
        eprintln!("Error: Benchmarks not enabled!");
        eprintln!("Run with: cargo run --release --bin throughput_benchmark --features benchmarks");
        std::process::exit(1); 
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "benchmarks")]
    use super::*;

    #[cfg(feature = "benchmarks")]
    #[test]
    #[ignore = "Throughput benchmark - run explicitly"]
    fn throughput_performance_benchmark() {
        run_throughput_benchmark();
    }
}