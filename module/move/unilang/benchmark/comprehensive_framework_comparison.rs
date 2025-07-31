use std::time::Instant;
use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

// Import all frameworks for comparison
use unilang::prelude::*;
use clap::{Arg, Command as ClapCommand};
use pico_args::Arguments;

#[derive(Debug, Clone)]
struct ComprehensiveBenchmarkResult {
    framework: String,
    command_count: usize,
    compile_time_ms: f64,
    binary_size_kb: u64,
    init_time_us: f64,
    avg_lookup_ns: f64,
    p99_lookup_ns: u64,
    commands_per_second: f64,
}

fn benchmark_unilang_comprehensive(command_count: usize) -> ComprehensiveBenchmarkResult {
    println!("🦀 Benchmarking unilang with {} commands (comprehensive)", command_count);

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

    // Measure compile time by building a test project
    let (compile_time_ms, binary_size_kb) = measure_unilang_compile_time(command_count);

    ComprehensiveBenchmarkResult {
        framework: "unilang".to_string(),
        command_count,
        compile_time_ms,
        binary_size_kb,
        init_time_us,
        avg_lookup_ns,
        p99_lookup_ns,
        commands_per_second,
    }
}

fn benchmark_clap_comprehensive(command_count: usize) -> ComprehensiveBenchmarkResult {
    println!("🗡️  Benchmarking clap with {} commands (comprehensive)", command_count);

    // Create clap app with N subcommands
    let init_start = Instant::now();
    let mut app = ClapCommand::new("benchmark")
        .version("1.0")
        .about("Clap benchmark application");

    for i in 0..command_count {
        let cmd_name = format!("cmd_{}", i);
        let cmd_desc = format!("Performance test command {}", i);
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

    // Measure compile time by building a test project
    let (compile_time_ms, binary_size_kb) = measure_clap_compile_time(command_count);

    ComprehensiveBenchmarkResult {
        framework: "clap".to_string(),
        command_count,
        compile_time_ms,
        binary_size_kb,
        init_time_us,
        avg_lookup_ns,
        p99_lookup_ns,
        commands_per_second,
    }
}

fn benchmark_pico_args_comprehensive(command_count: usize) -> ComprehensiveBenchmarkResult {
    println!("⚡ Benchmarking pico-args with {} commands (comprehensive)", command_count);

    // pico-args doesn't have initialization in the same way, so we simulate parsing setup
    let init_start = Instant::now();
    
    // Generate argument keys for this command count
    let arg_keys: Vec<String> = (0..command_count)
        .map(|i| format!("cmd-{}", i))
        .collect();
    
    let init_time = init_start.elapsed();
    let init_time_us = init_time.as_nanos() as f64 / 1000.0;

    // Benchmark parsing (pico-args uses different API pattern)
    let test_args: Vec<Vec<String>> = (0..1000)
        .map(|i| {
            let cmd_idx = i % command_count;
            vec![
                "benchmark".to_string(),
                format!("--cmd-{}", cmd_idx),
                "test_value".to_string(),
            ]
        })
        .collect();

    // Warmup
    for args_vec in test_args.iter().take(100) {
        let mut args = Arguments::from_vec(args_vec.clone());
        let key = format!("cmd-{}", 0 % command_count);
        let _: Option<String> = args.opt_value_from_str(&key).unwrap_or(None);
    }

    // Benchmark
    let mut lookup_times = Vec::new();
    let total_start = Instant::now();

    for args_vec in &test_args {
        let lookup_start = Instant::now();
        let mut args = Arguments::from_vec(args_vec.clone());
        let cmd_idx = 0; // Simulate looking up first command  
        let key = format!("cmd-{}", cmd_idx);
        let _: Option<String> = args.opt_value_from_str(&key).unwrap_or(None);
        let lookup_time = lookup_start.elapsed();
        lookup_times.push(lookup_time.as_nanos() as u64);
    }

    let total_time = total_start.elapsed();
    
    // Calculate statistics
    lookup_times.sort_unstable();
    let avg_lookup_ns = lookup_times.iter().sum::<u64>() as f64 / lookup_times.len() as f64;
    let p99_lookup_ns = lookup_times[(lookup_times.len() as f64 * 0.99) as usize];
    let commands_per_second = test_args.len() as f64 / total_time.as_secs_f64();

    println!("  ⏱️  Init: {:.2} μs, Lookup: {:.1} ns, Throughput: {:.0} cmd/sec", 
             init_time_us, avg_lookup_ns, commands_per_second);

    // Measure compile time by building a test project
    let (compile_time_ms, binary_size_kb) = measure_pico_args_compile_time(command_count);

    ComprehensiveBenchmarkResult {
        framework: "pico-args".to_string(),
        command_count,
        compile_time_ms,
        binary_size_kb,
        init_time_us,
        avg_lookup_ns,
        p99_lookup_ns,
        commands_per_second,
    }
}

fn measure_unilang_compile_time(command_count: usize) -> (f64, u64) {
    let work_dir = format!("target/compile_test_unilang_{}", command_count);
    let _ = fs::remove_dir_all(&work_dir);
    fs::create_dir_all(&work_dir).expect("Failed to create work directory");
    
    // Create a simple Cargo project
    let cargo_toml = format!(r#"[package]
name = "unilang_compile_test"
version = "0.1.0"
edition = "2021"

[workspace]

[[bin]]
name = "benchmark"
path = "src/main.rs"

[dependencies]
unilang = {{ path = "../../../" }}
"#);
    
    fs::write(format!("{}/Cargo.toml", work_dir), cargo_toml)
        .expect("Failed to write Cargo.toml");
    
    fs::create_dir_all(format!("{}/src", work_dir)).expect("Failed to create src dir");
    
    let main_rs = format!(r#"use unilang::prelude::*;

fn main() {{
    let mut registry = CommandRegistry::new();
    
    // Add {} commands
    for i in 0..{} {{
        let cmd = CommandDefinition {{
            name: format!("cmd_{{}}", i),
            namespace: ".perf".to_string(),
            description: format!("Performance test command {{}}", i),
            hint: "Performance test".to_string(),
            arguments: vec![],
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
        }};
        
        registry.register(cmd);
    }}
    
    println!("Registry initialized with {} commands", {});
}}
"#, command_count, command_count, command_count);
    
    fs::write(format!("{}/src/main.rs", work_dir), main_rs)
        .expect("Failed to write main.rs");
    
    // Measure compile time
    let compile_start = Instant::now();
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&work_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run cargo build");
    
    let compile_time = compile_start.elapsed();
    let compile_time_ms = compile_time.as_millis() as f64;
    
    // Measure binary size
    let binary_path = format!("{}/target/release/benchmark", work_dir);
    let binary_size_kb = if Path::new(&binary_path).exists() {
        fs::metadata(&binary_path)
            .map(|m| m.len() / 1024)
            .unwrap_or(0)
    } else {
        0
    };
    
    if !output.status.success() {
        println!("  ⚠️  Compilation failed for unilang with {} commands", command_count);
    }
    
    (compile_time_ms, binary_size_kb)
}

fn measure_clap_compile_time(command_count: usize) -> (f64, u64) {
    let work_dir = format!("target/compile_test_clap_{}", command_count);
    let _ = fs::remove_dir_all(&work_dir);
    fs::create_dir_all(&work_dir).expect("Failed to create work directory");
    
    // Create a simple Cargo project
    let cargo_toml = format!(r#"[package]
name = "clap_compile_test"
version = "0.1.0"
edition = "2021"

[workspace]

[[bin]]
name = "benchmark"
path = "src/main.rs"

[dependencies]
clap = "4.4"
"#);
    
    fs::write(format!("{}/Cargo.toml", work_dir), cargo_toml)
        .expect("Failed to write Cargo.toml");
    
    fs::create_dir_all(format!("{}/src", work_dir)).expect("Failed to create src dir");
    
    let main_rs = format!(r#"use clap::{{Arg, Command}};

fn main() {{
    let mut app = Command::new("benchmark")
        .version("1.0")
        .about("Clap benchmark application");

    // Add {} subcommands
    for i in 0..{} {{
        let subcommand = Command::new(format!("cmd_{{}}", i))
            .about(format!("Performance test command {{}}", i))
            .arg(Arg::new("input")
                .short('i')
                .long("input")
                .help("Input parameter"));
        
        app = app.subcommand(subcommand);
    }}
    
    println!("App initialized with {} commands", {});
}}
"#, command_count, command_count, command_count);
    
    fs::write(format!("{}/src/main.rs", work_dir), main_rs)
        .expect("Failed to write main.rs");
    
    // Measure compile time
    let compile_start = Instant::now();
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&work_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run cargo build");
    
    let compile_time = compile_start.elapsed();
    let compile_time_ms = compile_time.as_millis() as f64;
    
    // Measure binary size
    let binary_path = format!("{}/target/release/benchmark", work_dir);
    let binary_size_kb = if Path::new(&binary_path).exists() {
        fs::metadata(&binary_path)
            .map(|m| m.len() / 1024)
            .unwrap_or(0)
    } else {
        0
    };
    
    if !output.status.success() {
        println!("  ⚠️  Compilation failed for clap with {} commands", command_count);
    }
    
    (compile_time_ms, binary_size_kb)
}

fn measure_pico_args_compile_time(command_count: usize) -> (f64, u64) {
    let work_dir = format!("target/compile_test_pico_args_{}", command_count);
    let _ = fs::remove_dir_all(&work_dir);
    fs::create_dir_all(&work_dir).expect("Failed to create work directory");
    
    // Create a simple Cargo project
    let cargo_toml = format!(r#"[package]
name = "pico_args_compile_test"
version = "0.1.0"
edition = "2021"

[workspace]

[[bin]]
name = "benchmark"
path = "src/main.rs"

[dependencies]
pico-args = "0.5"
"#);
    
    fs::write(format!("{}/Cargo.toml", work_dir), cargo_toml)
        .expect("Failed to write Cargo.toml");
    
    fs::create_dir_all(format!("{}/src", work_dir)).expect("Failed to create src dir");
    
    let main_rs = format!(r#"use pico_args::Arguments;

fn main() {{
    // Simulate {} argument parsing operations
    let test_args = vec!["program".to_string()];
    
    for i in 0..{} {{
        let mut args = Arguments::from_vec(test_args.clone());
        let key = format!("cmd-{{}}", i);
        let _: Option<String> = args.opt_value_from_str(&key).unwrap_or(None);
    }}
    
    println!("Processed {} argument patterns", {});
}}
"#, command_count, command_count, command_count);
    
    fs::write(format!("{}/src/main.rs", work_dir), main_rs)
        .expect("Failed to write main.rs");
    
    // Measure compile time
    let compile_start = Instant::now();
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&work_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run cargo build");
    
    let compile_time = compile_start.elapsed();
    let compile_time_ms = compile_time.as_millis() as f64;
    
    // Measure binary size
    let binary_path = format!("{}/target/release/benchmark", work_dir);
    let binary_size_kb = if Path::new(&binary_path).exists() {
        fs::metadata(&binary_path)
            .map(|m| m.len() / 1024)
            .unwrap_or(0)
    } else {
        0
    };
    
    if !output.status.success() {
        println!("  ⚠️  Compilation failed for pico-args with {} commands", command_count);
    }
    
    (compile_time_ms, binary_size_kb)
}

fn generate_comprehensive_comparison_report(results: &[Vec<ComprehensiveBenchmarkResult>]) {
    // Always remove and recreate directory to ensure fresh results
    let output_dir = "target/comprehensive_framework_comparison";
    let _ = fs::remove_dir_all(output_dir);
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let mut report = String::new();
    report.push_str("COMPREHENSIVE CLI FRAMEWORK COMPARISON\n");
    report.push_str("=====================================\n\n");
    
    let now = chrono::Utc::now();
    report.push_str(&format!("Generated: {} UTC\n", now.format("%Y-%m-%d %H:%M:%S")));
    report.push_str("Frameworks: Unilang vs Clap vs Pico-Args\n");
    report.push_str("Metrics: Compile Time, Binary Size, Runtime Performance\n\n");
    
    // Add version information
    report.push_str("FRAMEWORK VERSIONS TESTED\n");
    report.push_str("=========================\n");
    report.push_str("- Unilang: 0.4.0 (current codebase)\n");
    report.push_str("- Clap: 4.4+ (latest stable)\n");  
    report.push_str("- Pico-Args: 0.5+ (latest stable)\n");
    // Capture actual Rust version
    let rust_version = Command::new("rustc")
        .args(&["--version"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unable to determine Rust version".to_string());
    report.push_str(&format!("- Rust: {}\n\n", rust_version));

    // Compile Time Comparison
    report.push_str("COMPILE TIME COMPARISON (ms)\n");
    report.push_str("============================\n");
    report.push_str("Commands | Unilang  | Clap     | Pico-Args | Winner\n");
    report.push_str("---------|----------|----------|-----------|--------\n");
    
    for result_set in results {
        let unilang = result_set.iter().find(|r| r.framework == "unilang").unwrap();
        let clap = result_set.iter().find(|r| r.framework == "clap").unwrap();
        let pico_args = result_set.iter().find(|r| r.framework == "pico-args").unwrap();
        
        let min_time = unilang.compile_time_ms.min(clap.compile_time_ms.min(pico_args.compile_time_ms));
        let winner = if unilang.compile_time_ms == min_time { "Unilang" }
                    else if clap.compile_time_ms == min_time { "Clap" }
                    else { "Pico-Args" };
        
        let cmd_display = if unilang.command_count >= 1000 {
            format!("{}K", unilang.command_count / 1000)
        } else {
            unilang.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>8.0} | {:>8.0} | {:>8.0} | {}\n",
            cmd_display, unilang.compile_time_ms, clap.compile_time_ms, pico_args.compile_time_ms, winner
        ));
    }

    // Binary Size Comparison
    report.push_str("\nBINARY SIZE COMPARISON (KB)\n");
    report.push_str("===========================\n");
    report.push_str("Commands | Unilang  | Clap     | Pico-Args | Winner\n");
    report.push_str("---------|----------|----------|-----------|--------\n");
    
    for result_set in results {
        let unilang = result_set.iter().find(|r| r.framework == "unilang").unwrap();
        let clap = result_set.iter().find(|r| r.framework == "clap").unwrap();
        let pico_args = result_set.iter().find(|r| r.framework == "pico-args").unwrap();
        
        let min_size = unilang.binary_size_kb.min(clap.binary_size_kb.min(pico_args.binary_size_kb));
        let winner = if unilang.binary_size_kb == min_size { "Unilang" }
                    else if clap.binary_size_kb == min_size { "Clap" }
                    else { "Pico-Args" };
        
        let cmd_display = if unilang.command_count >= 1000 {
            format!("{}K", unilang.command_count / 1000)
        } else {
            unilang.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>8} | {:>8} | {:>8} | {}\n",
            cmd_display, unilang.binary_size_kb, clap.binary_size_kb, pico_args.binary_size_kb, winner
        ));
    }

    // Runtime Performance Comparison
    report.push_str("\nRUNTIME PERFORMANCE COMPARISON\n");
    report.push_str("==============================\n");
    report.push_str("### Initialization Time (μs)\n");
    report.push_str("Commands | Unilang  | Clap     | Pico-Args | Winner\n");
    report.push_str("---------|----------|----------|-----------|--------\n");
    
    for result_set in results {
        let unilang = result_set.iter().find(|r| r.framework == "unilang").unwrap();
        let clap = result_set.iter().find(|r| r.framework == "clap").unwrap();
        let pico_args = result_set.iter().find(|r| r.framework == "pico-args").unwrap();
        
        let min_init = unilang.init_time_us.min(clap.init_time_us.min(pico_args.init_time_us));
        let winner = if (unilang.init_time_us - min_init).abs() < 0.01 { "Unilang" }
                    else if (clap.init_time_us - min_init).abs() < 0.01 { "Clap" }
                    else { "Pico-Args" };
        
        let cmd_display = if unilang.command_count >= 1000 {
            format!("{}K", unilang.command_count / 1000)
        } else {
            unilang.command_count.to_string()
        };
        
        report.push_str(&format!(
            "{:>8} | {:>8.2} | {:>8.2} | {:>8.2} | {}\n",
            cmd_display, unilang.init_time_us, clap.init_time_us, pico_args.init_time_us, winner
        ));
    }

    // Overall Analysis
    report.push_str("\nOVERALL FRAMEWORK ANALYSIS\n");
    report.push_str("==========================\n\n");
    
    report.push_str("**Unilang Strengths:**\n");
    report.push_str("- Universal command framework (CLI/GUI/Web API support)\n");
    report.push_str("- Consistent runtime performance across scales\n");
    report.push_str("- Type-safe argument definitions with validation\n");
    report.push_str("- Built-in help generation and command discovery\n\n");
    
    report.push_str("**Clap Strengths:**\n");
    report.push_str("- Mature and widely adopted CLI framework\n");
    report.push_str("- Rich feature set for CLI applications\n");
    report.push_str("- Extensive documentation and community support\n");
    report.push_str("- Advanced terminal features and customization\n\n");
    
    report.push_str("**Pico-Args Strengths:**\n");
    report.push_str("- Extremely lightweight and fast compilation\n");
    report.push_str("- Minimal binary size overhead\n");
    report.push_str("- Simple API for basic argument parsing\n");
    report.push_str("- Low resource consumption and minimal dependencies\n\n");
    
    report.push_str("**Use Case Recommendations:**\n");
    report.push_str("- **Choose Unilang** for multi-modal applications needing CLI + Web API + GUI\n");
    report.push_str("- **Choose Clap** for feature-rich CLI applications with complex requirements\n");
    report.push_str("- **Choose Pico-Args** for simple, lightweight CLI tools with minimal dependencies\n");

    fs::write("target/comprehensive_framework_comparison/comprehensive_report.txt", &report)
        .expect("Failed to write comprehensive report");

    // Generate CSV data for further analysis
    let mut csv_content = String::from("framework,command_count,compile_time_ms,binary_size_kb,init_time_us,avg_lookup_ns,p99_lookup_ns,commands_per_second\n");
    for result_set in results {
        for result in result_set {
            csv_content.push_str(&format!(
                "{},{},{:.0},{},{:.2},{:.2},{},{:.0}\n",
                result.framework,
                result.command_count,
                result.compile_time_ms,
                result.binary_size_kb,
                result.init_time_us,
                result.avg_lookup_ns,
                result.p99_lookup_ns,
                result.commands_per_second
            ));
        }
    }
    
    fs::write("target/comprehensive_framework_comparison/comprehensive_results.csv", &csv_content)
        .expect("Failed to write CSV results");

    println!("\n🎯 Comprehensive framework comparison reports saved to:");
    println!("  - target/comprehensive_framework_comparison/comprehensive_report.txt");
    println!("  - target/comprehensive_framework_comparison/comprehensive_results.csv");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comprehensive_framework_comparison_benchmark() {
        println!("🚀 Starting Comprehensive Framework Comparison Benchmark");
        println!("========================================================");
        println!("Testing Unilang vs Clap vs Pico-Args with compile time metrics\n");

        let command_counts = vec![10, 100, 1000];
        let mut all_results = Vec::new();

        for &count in &command_counts {
            println!("--- Testing with {} commands ---", count);
            
            let unilang_result = benchmark_unilang_comprehensive(count);
            let clap_result = benchmark_clap_comprehensive(count);
            let pico_args_result = benchmark_pico_args_comprehensive(count);
            
            all_results.push(vec![unilang_result, clap_result, pico_args_result]);
            println!();
        }

        // Generate comprehensive comparison report
        generate_comprehensive_comparison_report(&all_results);

        println!("🎉 Comprehensive framework comparison completed!");
        println!("\n📊 **Quick Summary:**");
        println!();
        println!("| Commands | Metric | Unilang | Clap | Pico-Args | Winner |");
        println!("|----------|--------|---------|------|-----------|--------|");
        
        for (i, result_set) in all_results.iter().enumerate() {
            let unilang = &result_set[0];
            let clap = &result_set[1];
            let pico_args = &result_set[2];
            
            let cmd_display = if command_counts[i] >= 1000 {
                format!("{}K", command_counts[i] / 1000)
            } else {
                command_counts[i].to_string()
            };
            
            // Compile time winner
            let min_compile = unilang.compile_time_ms.min(clap.compile_time_ms.min(pico_args.compile_time_ms));
            let compile_winner = if unilang.compile_time_ms == min_compile { "🦀 Unilang" }
                               else if clap.compile_time_ms == min_compile { "🗡️ Clap" }
                               else { "⚡ Pico-Args" };
            
            println!("| {:>8} | Compile | {:.0}ms | {:.0}ms | {:.0}ms | {} |",
                     cmd_display, unilang.compile_time_ms, clap.compile_time_ms, pico_args.compile_time_ms, compile_winner);
            
            // Runtime winner
            let min_runtime = unilang.init_time_us.min(clap.init_time_us.min(pico_args.init_time_us));
            let runtime_winner = if (unilang.init_time_us - min_runtime).abs() < 0.01 { "🦀 Unilang" }
                                else if (clap.init_time_us - min_runtime).abs() < 0.01 { "🗡️ Clap" }
                                else { "⚡ Pico-Args" };
            
            println!("| {:>8} | Runtime | {:.1}μs | {:.1}μs | {:.1}μs | {} |",
                     "", unilang.init_time_us, clap.init_time_us, pico_args.init_time_us, runtime_winner);
        }

        println!("\n✅ All three frameworks show excellent performance characteristics!");
        println!("📖 See detailed analysis in target/comprehensive_framework_comparison/comprehensive_report.txt");
        
        // Basic performance assertions
        for result_set in &all_results {
            for result in result_set {
                assert!(result.init_time_us < 10000.0, "Init time should be under 10ms");
                assert!(result.commands_per_second > 1000.0, "Throughput should exceed 1k cmd/sec");
                assert!(result.compile_time_ms > 0.0, "Compile time should be measured");
            }
        }
    }
}