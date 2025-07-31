use std::time::Instant;

// Import both unilang and clap for comparison
use unilang::prelude::*;
use clap::{Arg, Command};

#[derive(Debug, Clone)]
struct FrameworkBenchmarkResult {
    framework: String,
    command_count: usize,
    init_time_us: f64,
    avg_lookup_ns: f64,
    p99_lookup_ns: u64,
    commands_per_second: f64,
}

fn benchmark_unilang_performance(command_count: usize) -> FrameworkBenchmarkResult {
    println!("🦀 Benchmarking unilang with {} commands", command_count);

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

    // Benchmark lookups using pipeline
    let pipeline = Pipeline::new(registry);
    let test_commands: Vec<String> = (0..1000)
        .map(|i| format!(".perf.cmd_{} input::test verbose::true", i % command_count))
        .collect();

    // Warmup
    for cmd in test_commands.iter().take(100) {
        let _ = pipeline.process_command_simple(cmd);
    }

    // Benchmark
    let mut lookup_times = Vec::new();
    let total_start = Instant::now();

    for cmd in &test_commands {
        let lookup_start = Instant::now();
        let _ = pipeline.process_command_simple(cmd);
        let lookup_time = lookup_start.elapsed();
        lookup_times.push(lookup_time.as_nanos() as u64);
    }

    let total_time = total_start.elapsed();
    
    // Calculate statistics
    lookup_times.sort_unstable();
    let avg_lookup_ns = lookup_times.iter().sum::<u64>() as f64 / lookup_times.len() as f64;
    let p99_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.99) as usize];
    let commands_per_second = test_commands.len() as f64 / total_time.as_secs_f64();

    println!("  ⏱️  Init: {:.2} μs, Lookup: {:.1} ns, Throughput: {:.0} cmd/sec", 
             init_time_us, avg_lookup_ns, commands_per_second);

    FrameworkBenchmarkResult {
        framework: "unilang".to_string(),
        command_count,
        init_time_us,
        avg_lookup_ns,
        p99_lookup_ns,
        commands_per_second,
    }
}

fn benchmark_clap_performance(command_count: usize) -> FrameworkBenchmarkResult {
    println!("🗡️  Benchmarking clap with {} commands", command_count);

    // Create clap app with N subcommands
    let init_start = Instant::now();
    let mut app = Command::new("benchmark")
        .version("1.0")
        .about("Clap benchmark application");

    for i in 0..command_count {
        let cmd_name = format!("cmd_{}", i);
        let cmd_desc = format!("Performance test command {}", i);
        let subcommand = Command::new(&cmd_name)
            .about(&cmd_desc)
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

    // Benchmark parsing
    let test_commands: Vec<Vec<String>> = (0..1000)
        .map(|i| {
            let cmd_idx = i % command_count;
            vec![
                "benchmark".to_string(),
                format!("cmd_{}", cmd_idx),
                "--input".to_string(),
                "test".to_string(),
                "--verbose".to_string(),
            ]
        })
        .collect();

    // Warmup
    for args in test_commands.iter().take(100) {
        let app_clone = app.clone();
        let _ = app_clone.try_get_matches_from(args);
    }

    // Benchmark
    let mut lookup_times = Vec::new();
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
    let p99_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.99) as usize];
    let commands_per_second = test_commands.len() as f64 / total_time.as_secs_f64();

    println!("  ⏱️  Init: {:.2} μs, Lookup: {:.1} ns, Throughput: {:.0} cmd/sec", 
             init_time_us, avg_lookup_ns, commands_per_second);

    FrameworkBenchmarkResult {
        framework: "clap".to_string(),
        command_count,
        init_time_us,
        avg_lookup_ns,
        p99_lookup_ns,
        commands_per_second,
    }
}

fn generate_comparison_report(unilang_results: &[FrameworkBenchmarkResult], clap_results: &[FrameworkBenchmarkResult]) {
    use std::fs;

    // Create output directory
    fs::create_dir_all("target/framework_comparison").expect("Failed to create output directory");

    let mut report = String::new();
    report.push_str("UNILANG vs CLAP FRAMEWORK COMPARISON\n");
    report.push_str("====================================\n\n");
    
    let now = chrono::Utc::now();
    report.push_str(&format!("Generated: {} UTC\n", now.format("%Y-%m-%d %H:%M:%S")));
    report.push_str(&format!("Test Range: {} to {} commands\n\n", 
        unilang_results.first().unwrap().command_count,
        unilang_results.last().unwrap().command_count));

    // Side-by-side comparison table
    report.push_str("PERFORMANCE COMPARISON\n");
    report.push_str("======================\n\n");
    report.push_str("### Initialization Time (μs)\n");
    report.push_str("Commands | Unilang | Clap   | Winner   | Speedup\n");
    report.push_str("---------|---------|--------|----------|--------\n");
    
    for (unilang, clap) in unilang_results.iter().zip(clap_results.iter()) {
        let winner = if unilang.init_time_us < clap.init_time_us { "Unilang" } else { "Clap" };
        let speedup = if unilang.init_time_us < clap.init_time_us {
            clap.init_time_us / unilang.init_time_us
        } else {
            unilang.init_time_us / clap.init_time_us
        };
        
        let cmd_display = if unilang.command_count >= 1000 {
            format!("{}K", unilang.command_count / 1000)
        } else {
            unilang.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>7.2} | {:>6.2} | {:>8} | {:>6.1}x\n",
            cmd_display, unilang.init_time_us, clap.init_time_us, winner, speedup
        ));
    }

    report.push_str("\n### Lookup Performance (ns)\n");
    report.push_str("Commands | Unilang | Clap    | Winner   | Speedup\n");
    report.push_str("---------|---------|---------|----------|--------\n");
    
    for (unilang, clap) in unilang_results.iter().zip(clap_results.iter()) {
        let winner = if unilang.avg_lookup_ns < clap.avg_lookup_ns { "Unilang" } else { "Clap" };
        let speedup = if unilang.avg_lookup_ns < clap.avg_lookup_ns {
            clap.avg_lookup_ns / unilang.avg_lookup_ns
        } else {
            unilang.avg_lookup_ns / clap.avg_lookup_ns
        };
        
        let cmd_display = if unilang.command_count >= 1000 {
            format!("{}K", unilang.command_count / 1000)
        } else {
            unilang.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>7.1} | {:>7.1} | {:>8} | {:>6.1}x\n",
            cmd_display, unilang.avg_lookup_ns, clap.avg_lookup_ns, winner, speedup
        ));
    }

    report.push_str("\n### Throughput (commands/second)\n");
    report.push_str("Commands | Unilang  | Clap     | Winner   | Speedup\n");
    report.push_str("---------|----------|----------|----------|--------\n"); 
    
    for (unilang, clap) in unilang_results.iter().zip(clap_results.iter()) {
        let winner = if unilang.commands_per_second > clap.commands_per_second { "Unilang" } else { "Clap" };
        let speedup = if unilang.commands_per_second > clap.commands_per_second {
            unilang.commands_per_second / clap.commands_per_second
        } else {
            clap.commands_per_second / unilang.commands_per_second
        };
        
        let cmd_display = if unilang.command_count >= 1000 {
            format!("{}K", unilang.command_count / 1000)
        } else {
            unilang.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>8.0} | {:>8.0} | {:>8} | {:>6.1}x\n",
            cmd_display, unilang.commands_per_second, clap.commands_per_second, winner, speedup
        ));
    }

    // Overall analysis
    report.push_str("\nOVERALL ANALYSIS\n");
    report.push_str("================\n\n");
    
    let mut unilang_wins = 0;
    let mut clap_wins = 0;
    
    for (unilang, clap) in unilang_results.iter().zip(clap_results.iter()) {
        let unilang_score = (if unilang.init_time_us < clap.init_time_us { 1 } else { 0 }) +
                           (if unilang.avg_lookup_ns < clap.avg_lookup_ns { 1 } else { 0 }) +
                           (if unilang.commands_per_second > clap.commands_per_second { 1 } else { 0 });
        
        if unilang_score >= 2 {
            unilang_wins += 1;
        } else {
            clap_wins += 1;
        }
    }
    
    report.push_str(&format!("Test scenarios: {}\n", unilang_results.len()));
    report.push_str(&format!("Unilang wins: {} scenarios\n", unilang_wins));
    report.push_str(&format!("Clap wins: {} scenarios\n", clap_wins));
    
    let overall_winner = if unilang_wins > clap_wins { "UNILANG" } else { "CLAP" };
    report.push_str(&format!("\n🏆 OVERALL WINNER: {}\n", overall_winner));

    // Key insights
    report.push_str("\nKEY INSIGHTS\n");
    report.push_str("============\n\n");
    
    report.push_str("**Unilang Strengths:**\n");
    report.push_str("- Universal command framework (CLI/GUI/Web API support)\n");
    report.push_str("- Consistent performance across scales\n");
    report.push_str("- Type-safe argument definitions\n");
    report.push_str("- Built-in validation and help generation\n\n");
    
    report.push_str("**Clap Strengths:**\n");
    report.push_str("- Mature and widely adopted CLI framework\n");
    report.push_str("- Rich feature set for CLI applications\n");
    report.push_str("- Excellent documentation and community\n");
    report.push_str("- Optimized specifically for command-line interfaces\n\n");
    
    report.push_str("**Use Case Recommendations:**\n");
    report.push_str("- **Choose Unilang** when you need multi-modal interfaces (CLI + Web API + GUI)\n");
    report.push_str("- **Choose Clap** for pure CLI applications with extensive terminal features\n");
    report.push_str("- **Choose Unilang** for enterprise applications requiring consistent APIs\n");
    report.push_str("- **Choose Clap** for traditional Unix-style command line tools\n");

    fs::write("target/framework_comparison/comparison_report.txt", &report)
        .expect("Failed to write comparison report");

    println!("\n📊 Framework comparison report saved to:");
    println!("  - target/framework_comparison/comparison_report.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framework_comparison_benchmark() {
        println!("🚀 Starting Framework Comparison Benchmark");
        println!("===========================================");
        println!("Comparing unilang vs clap performance across different scales\n");

        let command_counts = vec![10, 100, 1000, 10000];
        let mut unilang_results = Vec::new();
        let mut clap_results = Vec::new();

        for &count in &command_counts {
            println!("--- Testing with {} commands ---", count);
            
            let unilang_result = benchmark_unilang_performance(count);
            let clap_result = benchmark_clap_performance(count);
            
            unilang_results.push(unilang_result);
            clap_results.push(clap_result);
            println!();
        }

        // Generate comparison report
        generate_comparison_report(&unilang_results, &clap_results);

        println!("🎉 Framework comparison completed!");
        println!("\n📊 **Quick Comparison Summary:**");
        println!();
        println!("| Commands | Framework | Init (μs) | Lookup (ns) | Throughput (K/sec) |");
        println!("|----------|-----------|-----------|-------------|-------------------|");
        
        for (unilang, clap) in unilang_results.iter().zip(clap_results.iter()) {
            let cmd_display = if unilang.command_count >= 1000 {
                format!("{}K", unilang.command_count / 1000)
            } else {
                unilang.command_count.to_string()
            };
            
            println!("| {:>8} | Unilang   | {:>9.2} | {:>11.1} | {:>17.0} |",
                     cmd_display, unilang.init_time_us, unilang.avg_lookup_ns, unilang.commands_per_second / 1000.0);
            println!("| {:>8} | Clap      | {:>9.2} | {:>11.1} | {:>17.0} |",
                     "", clap.init_time_us, clap.avg_lookup_ns, clap.commands_per_second / 1000.0);
        }

        println!("\n✅ Both frameworks show excellent performance characteristics!");
        println!("📖 See detailed analysis in target/framework_comparison/comparison_report.txt");
    }
}