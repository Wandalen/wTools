#![allow(clippy::all, warnings, missing_docs)]
//! Simple baseline benchmark using rust compiler timing
//!
//! This uses a practical approach to measure macro expansion performance by timing
//! actual rustc compilation of different complexity former-derived structs.

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::fs;
#[allow(unused_imports)]
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("🎯 Simple Former Macro Baseline Benchmark");
    println!("==========================================");
    println!();

    // Create test files
    create_test_files()?;
    
    // Measure rustc compilation time directly
    let baseline_results = measure_rustc_performance()?;
    
    // Generate baseline report
    generate_baseline_report(&baseline_results)?;
    
    println!("✅ Baseline benchmark completed!");
    Ok(())
}

fn create_test_files() -> Result<()> {
    println!("1️⃣ Creating Test Files");
    println!("--------------------");
    
    fs::create_dir_all("target/baseline_bench")?;
    
    // Simple test file - 2 fields
    let simple_test = r#"//! Simple 2-field struct test
#![allow(dead_code)]

use former::Former;

#[derive(Former, Debug, Clone)]
pub struct SimpleTest {
    pub name: String,
    pub value: i32,
}

fn main() {
    let _s = SimpleTest::former()
        .name("test".to_string())
        .value(42)
        .form();
}
"#;

    // Medium test file - 8 fields  
    let medium_test = r#"//! Medium 8-field struct test
#![allow(dead_code)]

use former::Former;

#[derive(Former, Debug, Clone)]
pub struct MediumTest {
    pub name: String,
    pub description: String,
    pub value: i32,
    pub count: usize,
    pub enabled: bool,
    pub tags: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub config: Option<String>,
}

fn main() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("key".to_string(), "value".to_string());
    
    let _m = MediumTest::former()
        .name("test".to_string())
        .description("test desc".to_string())
        .value(42)
        .count(10)
        .enabled(true)
        .tags(vec!["tag1".to_string()])
        .metadata(metadata)
        .config(Some("config".to_string()))
        .form();
}
"#;

    // Complex test file - 18 fields
    let complex_test = r#"//! Complex 18-field struct test
#![allow(dead_code)]

use former::Former;

#[derive(Former, Debug, Clone)]
pub struct ComplexTest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub repository: String,
    pub documentation: String,
    pub keywords: Vec<String>,
    pub dependencies: std::collections::HashMap<String, String>,
    pub dev_dependencies: std::collections::HashMap<String, String>,
    pub features: std::collections::HashMap<String, Vec<String>>,
    pub targets: Vec<String>,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
    pub publish: bool,
    pub edition: String,
    pub rust_version: Option<String>,
}

fn main() {
    let mut deps = std::collections::HashMap::new();
    deps.insert("serde".to_string(), "1.0".to_string());
    
    let _c = ComplexTest::former()
        .name("test".to_string())
        .description("A test".to_string())
        .category("testing".to_string())
        .version("0.1.0".to_string())
        .author("Test".to_string())
        .license("MIT".to_string())
        .repository("https://test.com".to_string())
        .documentation("https://docs.test.com".to_string())
        .keywords(vec!["test".to_string()])
        .dependencies(deps)
        .dev_dependencies(std::collections::HashMap::new())
        .features(std::collections::HashMap::new())
        .targets(vec!["lib".to_string()])
        .exclude(vec!["target".to_string()])
        .include(vec!["src".to_string()])
        .publish(true)
        .edition("2021".to_string())
        .rust_version(Some("1.70".to_string()))
        .form();
}
"#;

    // Create Cargo.toml for test project
    let cargo_toml = r#"[package]
name = "baseline-bench"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
former = { path = ".." }

[[bin]]
name = "simple_test"
path = "simple_test.rs"

[[bin]]
name = "medium_test"
path = "medium_test.rs"

[[bin]]
name = "complex_test"
path = "complex_test.rs"
"#;

    // Write test files
    fs::write("target/baseline_bench/simple_test.rs", simple_test)?;
    fs::write("target/baseline_bench/medium_test.rs", medium_test)?;
    fs::write("target/baseline_bench/complex_test.rs", complex_test)?;
    fs::write("target/baseline_bench/Cargo.toml", cargo_toml)?;
    
    println!("  ✅ Test files created");
    println!("     - simple_test.rs: 2 fields");
    println!("     - medium_test.rs: 8 fields");
    println!("     - complex_test.rs: 18 fields");
    println!();
    
    Ok(())
}

#[derive(Debug)]
struct CompileResult {
    test_name: String,
    field_count: usize,
    compile_time: Duration,
    success: bool,
    #[allow(dead_code)]
    stderr_size: usize,
}

fn measure_rustc_performance() -> Result<Vec<CompileResult>> {
    println!("2️⃣ Measuring Rustc Compilation Performance");
    println!("----------------------------------------");
    
    let mut results = Vec::new();
    
    // Change to test directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir("target/baseline_bench")?;
    
    let test_cases = [
        ("simple", 2),
        ("medium", 8),
        ("complex", 18),
    ];
    
    for (test_name, field_count) in &test_cases {
        println!("  📏 Measuring {} test ({} fields)...", test_name, field_count);
        
        // Clean previous build
        let _ = Command::new("cargo").args(&["clean"]).output();
        
        // Measure compilation time
        let start = Instant::now();
        let output = Command::new("cargo")
            .args(&["build", "--bin", &format!("{}_test", test_name), "--release"])
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .output()?;
        let compile_time = start.elapsed();
        
        let success = output.status.success();
        let stderr_size = output.stderr.len();
        
        if !success {
            println!("    ❌ Compilation failed");
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("    Error preview: {}", stderr.lines().take(3).collect::<Vec<_>>().join("\n"));
        } else {
            println!("    ✅ Compilation successful");
        }
        
        println!("    ⏱️  Compile time: {:.2?}", compile_time);
        
        results.push(CompileResult {
            test_name: test_name.to_string(),
            field_count: *field_count,
            compile_time,
            success,
            stderr_size,
        });
    }
    
    // Return to original directory
    std::env::set_current_dir(original_dir)?;
    
    println!();
    Ok(results)
}

fn generate_baseline_report(results: &[CompileResult]) -> Result<()> {
    println!("3️⃣ Generating Baseline Report");
    println!("---------------------------");
    
    let mut report = String::new();
    
    report.push_str("# Former Macro Baseline Performance Report\n\n");
    report.push_str(&format!("*Generated: {}*\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    
    report.push_str("## Baseline Compilation Results\n\n");
    report.push_str("| Test | Fields | Compile Time | Status |\n");
    report.push_str("|------|--------|--------------|--------|\n");
    
    let mut total_time = Duration::new(0, 0);
    let mut successful_compiles = 0;
    
    for result in results {
        let status = if result.success { "✅ Success" } else { "❌ Failed" };
        report.push_str(&format!(
            "| {} | {} | {:.2?} | {} |\n",
            result.test_name,
            result.field_count,
            result.compile_time,
            status
        ));
        
        if result.success {
            total_time += result.compile_time;
            successful_compiles += 1;
        }
    }
    
    report.push_str("\n## Baseline Analysis\n\n");
    
    // Calculate scaling factor
    if let (Some(simple), Some(complex)) = (
        results.iter().find(|r| r.test_name == "simple"),
        results.iter().find(|r| r.test_name == "complex")
    ) {
        if simple.success && complex.success {
            let scaling_factor = complex.compile_time.as_secs_f64() / simple.compile_time.as_secs_f64();
            
            report.push_str(&format!(
                "### Scaling Factor Analysis\n\n"
            ));
            report.push_str(&format!(
                "- **Simple → Complex scaling**: {:.1}x\n",
                scaling_factor
            ));
            report.push_str(&format!(
                "- **Simple baseline**: {:.2?} for 2 fields\n",
                simple.compile_time
            ));
            report.push_str(&format!(
                "- **Complex baseline**: {:.2?} for 18 fields\n",
                complex.compile_time
            ));
            
            // Task 001 evaluation
            let target_scaling = 2.5;
            report.push_str(&format!(
                "- **Task 001 target**: ≤{:.1}x scaling factor\n",
                target_scaling
            ));
            
            if scaling_factor > target_scaling {
                report.push_str(&format!(
                    "- **Baseline status**: 🔴 **OPTIMIZATION NEEDED** ({:.1}x > {:.1}x)\n",
                    scaling_factor, target_scaling
                ));
            } else {
                report.push_str(&format!(
                    "- **Baseline status**: 🟢 **TARGET ALREADY MET** ({:.1}x ≤ {:.1}x)\n",
                    scaling_factor, target_scaling
                ));
            }
        }
    }
    
    if successful_compiles > 0 {
        let avg_time = total_time / successful_compiles as u32;
        report.push_str(&format!(
            "\n### Overall Statistics\n\n"
        ));
        report.push_str(&format!(
            "- **Average compile time**: {:.2?}\n",
            avg_time
        ));
        report.push_str(&format!(
            "- **Successful compilations**: {}/{}\n",
            successful_compiles, results.len()
        ));
    }
    
    report.push_str("\n## Optimization Strategy\n\n");
    report.push_str("Based on baseline measurements:\n\n");
    
    if let (Some(simple), Some(complex)) = (
        results.iter().find(|r| r.test_name == "simple"),
        results.iter().find(|r| r.test_name == "complex")
    ) {
        if simple.success && complex.success {
            let scaling_factor = complex.compile_time.as_secs_f64() / simple.compile_time.as_secs_f64();
            
            if scaling_factor > 2.5 {
                report.push_str("1. **Primary focus**: Reduce macro expansion complexity\n");
                report.push_str("2. **Helper functions**: Extract common code generation patterns\n");
                report.push_str("3. **Template optimization**: Streamline generated code\n");
                report.push_str("4. **Const evaluation**: Move computation to compile time\n");
            } else {
                report.push_str("1. **Target already met**: Focus on maintaining performance\n");
                report.push_str("2. **Code quality**: Ensure optimizations don't break functionality\n");
                report.push_str("3. **Future proofing**: Prepare for more complex use cases\n");
            }
        }
    }
    
    report.push_str("\n## Next Steps\n\n");
    report.push_str("1. **Apply optimizations** to former_meta code generation\n");
    report.push_str("2. **Re-run benchmark** with identical test cases\n");
    report.push_str("3. **Calculate improvement** percentage\n");
    report.push_str("4. **Validate Task 001** completion criteria\n\n");
    
    report.push_str("---\n");
    report.push_str("*Baseline established using direct rustc compilation timing*\n");
    
    // Save baseline report
    fs::write("target/-simple_baseline_report.md", &report)?;
    
    println!("  ✅ Baseline report saved: target/-simple_baseline_report.md");
    
    // Print summary
    if let (Some(simple), Some(complex)) = (
        results.iter().find(|r| r.test_name == "simple"),
        results.iter().find(|r| r.test_name == "complex")
    ) {
        if simple.success && complex.success {
            let scaling_factor = complex.compile_time.as_secs_f64() / simple.compile_time.as_secs_f64();
            println!("  📊 Baseline Summary:");
            println!("     - Scaling factor: {:.1}x", scaling_factor);
            println!("     - Target: ≤2.5x");
            
            if scaling_factor > 2.5 {
                println!("     - Status: 🔴 Optimization needed");
            } else {
                println!("     - Status: 🟢 Target already met");
            }
        }
    }
    
    println!();
    Ok(())
}

// Add chrono for timestamps
mod chrono {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub struct Utc;
    
    impl Utc {
        pub fn now() -> UtcDateTime {
            UtcDateTime { timestamp: SystemTime::now() }
        }
    }
    
    pub struct UtcDateTime {
        timestamp: SystemTime,
    }
    
    impl UtcDateTime {
        pub fn format(&self, _format: &str) -> String {
            match self.timestamp.duration_since(UNIX_EPOCH) {
                Ok(duration) => {
                    let secs = duration.as_secs();
                    let days = secs / 86400;
                    let hours = (secs % 86400) / 3600;
                    let minutes = (secs % 3600) / 60;
                    let seconds = secs % 60;
                    
                    // Simple date formatting (approximate)
                    format!("2024-01-{:02} {:02}:{:02}:{:02} UTC", 
                           (days % 31) + 1, hours, minutes, seconds)
                }
                Err(_) => "2024-01-01 00:00:00 UTC".to_string(),
            }
        }
    }
}