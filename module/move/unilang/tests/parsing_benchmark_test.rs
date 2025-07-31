//! Additional parsing benchmark tests for measuring command parsing performance
//!
//! This module complements the existing stress test by focusing specifically on
//! command parsing runtime delay measurements.

use unilang_parser::*;
use std::time::{Duration, Instant};

#[test]
fn benchmark_1000_command_parsing_delay()
{
    println!("=== UNILANG COMMAND PARSING BENCHMARK ===");
    
    // Create parser instance
    let parser = Parser::new(UnilangParserOptions::default());
    
    // Test commands that cover various parsing scenarios (all valid unilang syntax)
    let test_commands = vec![
        ".run_file file::./examples/rust_learning.yaml",
        ".namespace.command arg1 arg2",
        ".simple_command",
        ".tool.build config::debug target::x86_64",
        ".test.run file::./tests/unit_tests.rs verbose::true",
        ".git.commit message::\"Initial commit\" author::user",
        ".cargo.build release::true target::x86_64-unknown-linux-gnu",
        ".fs.copy src::./file.txt dest::./backup/file.txt",
        ".network.request url::https://api.example.com method::GET",
        ".docker.run image::ubuntu:20.04 port::8080 volume::./data:/app/data",
        ".deep.nested.namespace.command.path arg::value",
        ".file.operations move::./src/file.rs dest::./backup/file.rs force::true",
        ".database.query table::users where::\"id > 100\" limit::50",
        ".system.monitor cpu::true memory::true disk::false",
        ".config.set theme::dark font_size::14 auto_save::true",
    ];
    
    let iterations = 1000;
    let mut parse_times = Vec::with_capacity(iterations);
    let mut successful_parses = 0;
    let mut failed_parses = 0;
    
    println!("Testing {} parsing iterations...", iterations);
    
    // Warm-up phase
    for _ in 0..10 {
        for command in &test_commands {
            let _ = parser.parse_single_instruction(command);
        }
    }
    
    // Main benchmark
    let total_start = Instant::now();
    
    for i in 0..iterations {
        let command = &test_commands[i % test_commands.len()];
        
        let parse_start = Instant::now();
        let result = parser.parse_single_instruction(command);
        let parse_time = parse_start.elapsed();
        
        parse_times.push(parse_time);
        
        match result {
            Ok(_) => successful_parses += 1,
            Err(e) => {
                failed_parses += 1;
                eprintln!("Parse error for '{}': {:?}", command, e);
            }
        }
    }
    
    let total_time = total_start.elapsed();
    
    // Calculate statistics
    parse_times.sort();
    let min_time = parse_times[0];
    let max_time = parse_times[iterations - 1];
    let median_time = parse_times[iterations / 2];
    let p95_time = parse_times[(iterations as f64 * 0.95) as usize];
    let p99_time = parse_times[(iterations as f64 * 0.99) as usize];
    
    let total_parse_time: Duration = parse_times.iter().sum();
    let avg_parse_time = total_parse_time / iterations as u32;
    
    // Results
    println!("\n=== PARSING PERFORMANCE RESULTS ===");
    println!("Total iterations: {}", iterations);
    println!("Successful parses: {}", successful_parses);
    println!("Failed parses: {}", failed_parses);
    println!("Total wall time: {:?}", total_time);
    println!("Total parsing time: {:?}", total_parse_time);
    println!();
    println!("=== LATENCY STATISTICS ===");
    println!("Minimum parse time: {:?} ({} ns)", min_time, min_time.as_nanos());
    println!("Average parse time: {:?} ({} ns)", avg_parse_time, avg_parse_time.as_nanos());
    println!("Median parse time: {:?} ({} ns)", median_time, median_time.as_nanos());
    println!("95th percentile: {:?} ({} ns)", p95_time, p95_time.as_nanos());
    println!("99th percentile: {:?} ({} ns)", p99_time, p99_time.as_nanos());
    println!("Maximum parse time: {:?} ({} ns)", max_time, max_time.as_nanos());
    println!();
    println!("=== THROUGHPUT METRICS ===");
    println!("Commands per second: {:.2}", iterations as f64 / total_parse_time.as_secs_f64());
    println!("Nanoseconds per command: {:.2}", avg_parse_time.as_nanos());
    println!("Microseconds per command: {:.2}", avg_parse_time.as_nanos() as f64 / 1000.0);
    
    // Performance requirements check
    let performance_target_ns = 100_000; // 100 microseconds in nanoseconds
    let meets_requirement = avg_parse_time.as_nanos() < performance_target_ns as u128;
    
    println!("\n=== PERFORMANCE ANALYSIS ===");
    println!("Target: < 100 μs per command");
    println!("Actual: {:.2} μs per command", avg_parse_time.as_nanos() as f64 / 1000.0);
    
    if meets_requirement {
        println!("✅ PERFORMANCE TARGET ACHIEVED");
    } else {
        println!("❌ PERFORMANCE TARGET MISSED");
    }
    
    // Command complexity breakdown
    println!("\n=== COMMAND COMPLEXITY BREAKDOWN ===");
    for (i, command) in test_commands.iter().take(5).enumerate() {
        let iterations_for_cmd = (iterations + test_commands.len() - 1 - i) / test_commands.len();
        println!("  {}: '{}' (parsed {} times)", i + 1, command, iterations_for_cmd);
    }
    println!("  ... and {} more command types", test_commands.len() - 5);
    
    // Memory estimation
    let est_memory_per_parse = 1000; // Rough estimate in bytes
    let total_est_memory = est_memory_per_parse * iterations;
    println!("\n=== MEMORY USAGE ESTIMATE ===");
    println!("Est. memory per parse: {} bytes", est_memory_per_parse);
    println!("Est. total memory used: {} bytes ({:.2} KB)", total_est_memory, total_est_memory as f64 / 1024.0);
    
    // Assertions
    assert_eq!(failed_parses, 0, "All commands should parse successfully");
    assert!(successful_parses > 0, "Should have successful parses");
    assert!(avg_parse_time.as_nanos() < 1_000_000, "Average parse time should be under 1ms");
    assert!(p99_time.as_nanos() < 10_000_000, "P99 parse time should be under 10ms");
    
    println!("\n✅ BENCHMARK COMPLETED SUCCESSFULLY");
}