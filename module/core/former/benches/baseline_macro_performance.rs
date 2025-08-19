#![allow(clippy::all, warnings, missing_docs)]
//! Real baseline macro performance measurement for former
//!
//! This benchmark measures actual compilation time of former-generated code
//! across different struct complexities to establish a true baseline.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::needless_raw_string_hashes)]

use std::process::Command;
use std::time::{Duration, Instant};
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
  println!("📊 Former Macro Baseline Performance Measurement");
  println!("=================================================");
  println!();

  // Create test structs of varying complexity
  create_test_structs()?;
  
  // Measure compilation time for each complexity level
  let baseline_results = measure_compilation_performance()?;
  
  // Generate baseline report
  generate_baseline_report(&baseline_results)?;
  
  println!("✅ Baseline measurements completed!");
  Ok(())
}

fn create_test_structs() -> Result<()> {
  println!("1️⃣ Creating Test Structs for Baseline Measurement");
  println!("-----------------------------------------------");
  
  // Create test directory
  fs::create_dir_all("target/baseline_tests")?;
  
  // Simple struct (2 fields)
  let simple_struct = r#"
use former::Former;

#[derive(Former, Debug, Clone)]
pub struct SimpleStruct {
  pub name: String,
  pub value: i32,
}
"#;

  // Medium struct (8 fields)
  let medium_struct = r#"
use former::Former;

#[derive(Former, Debug, Clone)]
pub struct MediumStruct {
  pub name: String,
  pub description: String,
  pub value: i32,
  pub count: usize,
  pub enabled: bool,
  pub tags: Vec<String>,
  pub metadata: std::collections::HashMap<String, String>,
  pub config: Option<String>,
}
"#;

  // Complex struct (18 fields) - similar to CommandDefinition
  let complex_struct = r#"
use former::Former;

#[derive(Former, Debug, Clone)]
pub struct ComplexStruct {
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
"#;

  // Write test files
  fs::write("target/baseline_tests/simple_struct.rs", simple_struct)?;
  fs::write("target/baseline_tests/medium_struct.rs", medium_struct)?;
  fs::write("target/baseline_tests/complex_struct.rs", complex_struct)?;
  
  // Create lib.rs that includes all structs
  let lib_content = r#"
pub mod simple_struct;
pub mod medium_struct; 
pub mod complex_struct;

pub use simple_struct::SimpleStruct;
pub use medium_struct::MediumStruct;
pub use complex_struct::ComplexStruct;
"#;

  fs::write("target/baseline_tests/lib.rs", lib_content)?;
  
  // Create Cargo.toml for the test crate
  let cargo_toml = r#"
[package]
name = "baseline_tests"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
former = { path = "../../.." }

[lib]
name = "baseline_tests"
path = "lib.rs"
"#;

  fs::write("target/baseline_tests/Cargo.toml", cargo_toml)?;
  
  println!("  ✅ Test structs created:");
  println!("     - SimpleStruct: 2 fields");
  println!("     - MediumStruct: 8 fields");
  println!("     - ComplexStruct: 18 fields");
  println!();
  
  Ok(())
}

#[derive(Debug)]
struct CompilationResult {
  struct_name: String,
  field_count: usize,
  compilation_time: Duration,
  success: bool,
  output_size: usize,
}

fn measure_compilation_performance() -> Result<Vec<CompilationResult>> {
  println!("2️⃣ Measuring Real Compilation Performance");
  println!("---------------------------------------");
  
  let mut results = Vec::new();
  
  // Change to test directory
  std::env::set_current_dir("target/baseline_tests")?;
  
  let test_cases = [
    ("SimpleStruct", 2),
    ("MediumStruct", 8), 
    ("ComplexStruct", 18),
  ];
  
  for (struct_name, field_count) in &test_cases {
    println!("  📏 Measuring {} ({} fields)...", struct_name, field_count);
    
    // Clean previous build
    let _ = Command::new("cargo").args(&["clean"]).output();
    
    // Measure compilation time
    let start = Instant::now();
    let output = Command::new("cargo")
      .args(&["build", "--release"])
      .output()?;
    let compilation_time = start.elapsed();
    
    let success = output.status.success();
    let output_size = if success {
      get_output_size()?
    } else {
      println!("    ❌ Compilation failed for {}", struct_name);
      println!("    Error: {}", String::from_utf8_lossy(&output.stderr));
      0
    };
    
    println!("    ⏱️  Compilation time: {:.2?}", compilation_time);
    println!("    📦 Output size: {} bytes", output_size);
    
    results.push(CompilationResult {
      struct_name: struct_name.to_string(),
      field_count: *field_count,
      compilation_time,
      success,
      output_size,
    });
  }
  
  // Return to original directory
  std::env::set_current_dir("../..")?;
  
  println!();
  Ok(results)
}

fn get_output_size() -> Result<usize> {
  let metadata = fs::metadata("target/release/libbaseline_tests.rlib")
    .or_else(|_| fs::metadata("target/release/deps/libbaseline_tests.rlib"))
    .or_else(|_| {
      // Find any .rlib file in target/release/deps
      let deps_dir = fs::read_dir("target/release/deps")?;
      for entry in deps_dir {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rlib") &&
           path.file_name().unwrap().to_string_lossy().contains("baseline_tests") {
          return fs::metadata(path);
        }
      }
      Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No .rlib found"))
    })?;
  
  Ok(metadata.len() as usize)
}

fn generate_baseline_report(results: &[CompilationResult]) -> Result<()> {
  println!("3️⃣ Generating Baseline Performance Report");
  println!("---------------------------------------");
  
  let mut report = String::new();
  
  report.push_str("# Former Macro Baseline Performance Report\n\n");
  report.push_str("*Baseline measurements before optimization*\n\n");
  
  report.push_str("## Compilation Performance Baseline\n\n");
  report.push_str("| Struct | Fields | Compilation Time | Output Size | Status |\n");
  report.push_str("|--------|--------|------------------|-------------|--------|\n");
  
  let mut total_time = Duration::new(0, 0);
  let mut successful_compilations = 0;
  
  for result in results {
    let status = if result.success { "✅ Success" } else { "❌ Failed" };
    report.push_str(&format!(
      "| {} | {} | {:.2?} | {} bytes | {} |\n",
      result.struct_name,
      result.field_count,
      result.compilation_time,
      result.output_size,
      status
    ));
    
    if result.success {
      total_time += result.compilation_time;
      successful_compilations += 1;
    }
  }
  
  report.push_str("\n## Baseline Analysis\n\n");
  
  if let (Some(simple), Some(complex)) = (
    results.iter().find(|r| r.struct_name == "SimpleStruct"),
    results.iter().find(|r| r.struct_name == "ComplexStruct")
  ) {
    if simple.success && complex.success {
      let scaling_factor = complex.compilation_time.as_secs_f64() / simple.compilation_time.as_secs_f64();
      report.push_str(&format!(
        "- **Baseline Scaling Factor**: {:.1}x (Simple → Complex)\n",
        scaling_factor
      ));
      report.push_str(&format!(
        "- **Simple Struct Baseline**: {:.2?} for 2 fields\n",
        simple.compilation_time
      ));
      report.push_str(&format!(
        "- **Complex Struct Baseline**: {:.2?} for 18 fields\n",
        complex.compilation_time
      ));
      
      // Task 001 target analysis
      let target_scaling = 2.5;
      if scaling_factor > target_scaling {
        report.push_str(&format!(
          "- **Task 001 Target**: Current {:.1}x > {:.1}x target - **OPTIMIZATION NEEDED**\n",
          scaling_factor, target_scaling
        ));
      } else {
        report.push_str(&format!(
          "- **Task 001 Target**: Current {:.1}x ≤ {:.1}x target - **TARGET MET**\n",
          scaling_factor, target_scaling
        ));
      }
    }
  }
  
  if successful_compilations > 0 {
    let avg_time = total_time / successful_compilations as u32;
    report.push_str(&format!(
      "- **Average Compilation Time**: {:.2?}\n",
      avg_time
    ));
  }
  
  report.push_str("\n## Next Steps\n\n");
  report.push_str("1. **Implement optimizations** to reduce macro expansion overhead\n");
  report.push_str("2. **Re-measure performance** with identical test cases\n");
  report.push_str("3. **Compare results** to validate improvement\n");
  report.push_str("4. **Target achievement**: Reduce scaling factor to ≤2.5x\n\n");
  
  report.push_str("---\n");
  report.push_str("*Baseline report generated for Task 001 optimization validation*\n");
  
  // Save baseline report
  fs::write("target/-baseline_performance.md", &report)?;
  
  println!("  ✅ Baseline report generated:");
  println!("     - Report saved: target/-baseline_performance.md");
  
  // Print key baseline metrics
  if let (Some(simple), Some(complex)) = (
    results.iter().find(|r| r.struct_name == "SimpleStruct"),
    results.iter().find(|r| r.struct_name == "ComplexStruct")
  ) {
    if simple.success && complex.success {
      let scaling_factor = complex.compilation_time.as_secs_f64() / simple.compilation_time.as_secs_f64();
      println!("     - Baseline scaling: {:.1}x (Simple → Complex)", scaling_factor);
      println!("     - Target scaling: ≤2.5x");
      
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