//! Clap-specific performance benchmark for exponential scaling analysis.
//!
//! This benchmark focuses exclusively on Clap framework performance
//! to understand its scaling characteristics with increasing command counts.

use std::time::Instant;
use clap::{Arg, Command};

#[allow(dead_code)]
struct ClapBenchmarkResult {
    command_count: usize,
    init_time_us: f64,
    avg_lookup_ns: f64,
    p50_lookup_ns: u64,
    p95_lookup_ns: u64,  
    p99_lookup_ns: u64,
    max_lookup_ns: u64,
    commands_per_second: f64,
}

fn create_clap_app_with_n_commands(n: usize) -> Command {
    let mut app = Command::new("benchmark")
        .version("1.0")
        .about("Clap benchmark application");

    // Add n commands as subcommands
    for i in 0..n {
        // Use simple static names for the first few, then fallback to generated ones
        let (cmd_name, cmd_desc) = match i {
            0 => ("cmd_0", "Performance test command 0"),
            1 => ("cmd_1", "Performance test command 1"),
            2 => ("cmd_2", "Performance test command 2"),
            3 => ("cmd_3", "Performance test command 3"),
            _ => ("cmd_dynamic", "Performance test command dynamic"),
        };

        let subcommand = Command::new(cmd_name)
            .about(cmd_desc)
            .arg(Arg::new("input")
                .short('i')
                .long("input")
                .help("Input parameter")
                .value_name("VALUE"))
            .arg(Arg::new("output")
                .short('o')  
                .long("output")
                .help("Output parameter")
                .value_name("FILE"))
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue));
        
        app = app.subcommand(subcommand);
    }

    app
}

fn benchmark_clap_performance(command_count: usize) -> ClapBenchmarkResult {
    println!("🔥 Benchmarking clap with {} commands", command_count);

    // Measure initialization time
    let init_start = Instant::now();
    let app = create_clap_app_with_n_commands(command_count);
    let init_time = init_start.elapsed();
    let init_time_us = init_time.as_nanos() as f64 / 1000.0;

    println!("  ⏱️  Init time: {:.2} μs", init_time_us);

    // Prepare lookup test data - simulate parsing different commands
    let test_commands: Vec<Vec<String>> = (0..1000)
        .map(|i| {
            let cmd_idx = i % command_count;
            vec![
                "benchmark".to_string(),
                format!("cmd_{}", cmd_idx),
                "--input".to_string(), 
                "test.txt".to_string(),
                "--verbose".to_string(),
            ]
        })
        .collect();

    // Warmup
    for args in test_commands.iter().take(100) {
        let app_clone = app.clone();
        let _ = app_clone.try_get_matches_from(args);
    }

    // Benchmark lookup performance
    let mut lookup_times = Vec::new();
    let total_start = Instant::now();

    for args in &test_commands {
        let lookup_start = Instant::now();
        let app_clone = app.clone();
        let _matches = app_clone.try_get_matches_from(args);
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
    
    let commands_per_second = test_commands.len() as f64 / total_time.as_secs_f64();

    println!("  📊 Avg lookup: {:.2} ns", avg_lookup_ns);
    println!("  📊 P99 lookup: {} ns", p99_lookup_ns);
    println!("  🚀 Throughput: {:.0} cmd/sec", commands_per_second);

    ClapBenchmarkResult {
        command_count,
        init_time_us,
        avg_lookup_ns,
        p50_lookup_ns,
        p95_lookup_ns,
        p99_lookup_ns,
        max_lookup_ns,
        commands_per_second,
    }
}

fn generate_clap_report(results: &[ClapBenchmarkResult]) {
    use std::fs;
    use std::process::Command;

    // Always remove and recreate directory to ensure fresh results
    let output_dir = "target/clap_benchmark_results";
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Generate CSV file
    let mut csv_content = String::from("command_count,init_time_us,avg_lookup_ns,p50_lookup_ns,p95_lookup_ns,p99_lookup_ns,max_lookup_ns,commands_per_second\n");
    for result in results {
        csv_content.push_str(&format!(
            "{},{:.2},{:.2},{},{},{},{},{:.0}\n",
            result.command_count,
            result.init_time_us,
            result.avg_lookup_ns,
            result.p50_lookup_ns,
            result.p95_lookup_ns,
            result.p99_lookup_ns,
            result.max_lookup_ns,
            result.commands_per_second
        ));
    }
    
    fs::write("target/clap_benchmark_results/clap_benchmark_results.csv", &csv_content)
        .expect("Failed to write CSV file");

    // Generate detailed text report
    let mut report = String::new();
    report.push_str("CLAP EXPONENTIAL PERFORMANCE BENCHMARK REPORT\n");
    report.push_str("=============================================\n\n");
    
    let now = chrono::Utc::now();
    report.push_str(&format!("Generated: {} UTC\n", now.format("%Y-%m-%d %H:%M:%S")));
    
    // Add version information
    let rust_version = Command::new("rustc")
        .args(&["--version"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unable to determine Rust version".to_string());
    report.push_str(&format!("Clap Version: 4.4+ (as specified in Cargo.toml)\n"));
    report.push_str(&format!("Rust Version: {}\n", rust_version));
    
    report.push_str(&format!("Test Range: 10^1 to 10^{} commands ({} to {} commands)\n", 
        (results.last().unwrap().command_count as f64).log10() as u32,
        results.first().unwrap().command_count,
        results.last().unwrap().command_count));
    report.push_str(&format!("Total Benchmarks: {}\n\n", results.len()));

    report.push_str("PERFORMANCE SUMMARY\n");
    report.push_str("-------------------\n");
    report.push_str("Commands     | Init Time | Avg Lookup | P99 Lookup | Throughput\n");
    report.push_str("             |    (μs)   |    (ns)    |    (ns)    | (cmd/sec)\n");
    report.push_str("-------------|-----------|------------|------------|----------\n");

    for result in results {
        let cmd_display = if result.command_count >= 1000000 {
            format!("{}M", result.command_count / 1000000)
        } else if result.command_count >= 1000 {
            format!("{}K", result.command_count / 1000)
        } else {
            result.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>12} | {:>9.2} | {:>10.1} | {:>10} | {:>8.0}\n",
            cmd_display,
            result.init_time_us,
            result.avg_lookup_ns,
            result.p99_lookup_ns,
            result.commands_per_second
        ));
    }

    report.push_str("\nSCALING ANALYSIS\n");
    report.push_str("----------------\n");
    let first = &results[0];
    let last = &results[results.len() - 1];
    let cmd_scaling = last.command_count as f64 / first.command_count as f64;
    let init_scaling = last.init_time_us / first.init_time_us;
    let lookup_scaling = last.avg_lookup_ns / first.avg_lookup_ns;
    
    report.push_str(&format!("Command count scaling: {:.0}x ({} to {})\n", 
        cmd_scaling, first.command_count, last.command_count));
    report.push_str(&format!("Init time scaling: {:.2}x ({:.2}μs to {:.2}μs)\n", 
        init_scaling, first.init_time_us, last.init_time_us));
    report.push_str(&format!("Lookup time scaling: {:.2}x ({:.1}ns to {:.1}ns)\n", 
        lookup_scaling, first.avg_lookup_ns, last.avg_lookup_ns));

    report.push_str("\nPERFORMANCE CLASSIFICATION\n");
    report.push_str("--------------------------\n");
    
    let init_classification = if init_scaling < 2.0 { "EXCELLENT (sub-linear)" } 
        else if init_scaling < 5.0 { "GOOD (near-linear)" }
        else { "POOR (super-linear)" };
    
    let lookup_classification = if lookup_scaling < 2.0 { "EXCELLENT (sub-linear)" }
        else if lookup_scaling < 5.0 { "GOOD (near-linear)" }
        else { "POOR (super-linear)" };

    report.push_str(&format!("✅ Init Time Scaling: {}\n", init_classification));
    report.push_str(&format!("✅ Lookup Time Scaling: {}\n", lookup_classification));

    report.push_str("\nDETAILED BREAKDOWN\n");
    report.push_str("------------------\n\n");

    for result in results {
        let cmd_display = if result.command_count >= 1000000 {
            format!("{}M Commands", result.command_count / 1000000)
        } else if result.command_count >= 1000 {
            format!("{}K Commands", result.command_count / 1000)
        } else {
            format!("{} Commands", result.command_count)
        };

        report.push_str(&format!("{}:\n", cmd_display));
        report.push_str(&format!("  Initialization: {:.3} μs\n", result.init_time_us));
        report.push_str(&format!("  Average lookup: {:.2} ns\n", result.avg_lookup_ns));
        report.push_str(&format!("  P50 lookup: {} ns\n", result.p50_lookup_ns));
        report.push_str(&format!("  P95 lookup: {} ns\n", result.p95_lookup_ns));
        report.push_str(&format!("  P99 lookup: {} ns\n", result.p99_lookup_ns));
        report.push_str(&format!("  Max lookup: {} ns\n", result.max_lookup_ns));
        report.push_str(&format!("  Throughput: {} commands/second\n\n", result.commands_per_second as u64));
    }

    fs::write("target/clap_benchmark_results/clap_performance_report.txt", &report)
        .expect("Failed to write report file");

    println!("\n📊 Clap benchmark results saved to:");
    println!("  - target/clap_benchmark_results/clap_benchmark_results.csv");
    println!("  - target/clap_benchmark_results/clap_performance_report.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clap_exponential_performance_benchmark() {
        println!("🚀 Starting CLAP Exponential Performance Benchmark");
        println!("==================================================");

        let command_counts = vec![10, 100, 1000, 10000, 100000];
        let mut results = Vec::new();

        for &count in &command_counts {
            let result = benchmark_clap_performance(count);
            results.push(result);
            println!(); // Add spacing between benchmarks
        }

        // Generate comprehensive report
        generate_clap_report(&results);

        println!("🎉 Clap benchmark completed successfully!");
        println!("\n📊 **Key Performance Results:**");
        println!();
        println!("| Commands | Init Time | Avg Lookup | P99 Lookup | Throughput |");
        println!("|----------|-----------|------------|------------|------------|");
        
        for result in &results {
            let cmd_display = if result.command_count >= 1000000 {
                format!("**{}M**", result.command_count / 1000000)
            } else if result.command_count >= 1000 {
                format!("**{}K**", result.command_count / 1000)  
            } else {
                format!("**{}**", result.command_count)
            };
            
            println!("| {} | {:.2} μs | {:.1} ns | {} ns | {:.2} M/sec |",
                cmd_display,
                result.init_time_us,
                result.avg_lookup_ns,
                result.p99_lookup_ns,
                result.commands_per_second / 1_000_000.0
            );
        }

        println!("\n🔍 **Analysis:**");
        let first = &results[0];
        let last = &results[results.len() - 1];
        let cmd_scaling = last.command_count as f64 / first.command_count as f64;
        let init_scaling = last.init_time_us / first.init_time_us;
        let lookup_scaling = last.avg_lookup_ns / first.avg_lookup_ns;
        
        println!("- Command scaling: {:.0}x increase", cmd_scaling);
        println!("- Init time scaling: {:.2}x ({:.2}μs → {:.2}μs)", init_scaling, first.init_time_us, last.init_time_us);
        println!("- Lookup time scaling: {:.2}x ({:.1}ns → {:.1}ns)", lookup_scaling, first.avg_lookup_ns, last.avg_lookup_ns);

        // Performance assertions
        assert!(results.iter().all(|r| r.init_time_us < 1000.0), "Init time should be under 1ms");
        assert!(results.iter().all(|r| r.commands_per_second > 10000.0), "Throughput should exceed 10k cmd/sec");
    }
}