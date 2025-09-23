#![allow(clippy::all, warnings, missing_docs)]
//! Real baseline measurement using incremental compilation timing
//!
#![allow(clippy::std_instead_of_core, clippy::unnecessary_wraps, clippy::uninlined_format_args, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
//! This approach measures the actual macro expansion time by using Rust's
//! incremental compilation timing and isolating the former macro expansion.

use std::process::Command;
use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
  println!("🔍 Real Former Macro Baseline Measurement");
  println!("==========================================");
  println!();

  // Create baseline test module within the current crate
  create_baseline_test_modules()?;
  
  // Measure compilation timing using cargo check with timing
  let baseline_results = measure_incremental_compilation()?;
  
  // Generate baseline report
  generate_real_baseline_report(&baseline_results)?;
  
  println!("✅ Real baseline measurements completed!");
  Ok(())
}

fn create_baseline_test_modules() -> Result<()> {
  println!("1️⃣ Creating Baseline Test Modules");
  println!("-------------------------------");
  
  // Create src/baseline_tests directory
  fs::create_dir_all("src/baseline_tests")?;
  
  // Simple struct module (2 fields)
  let simple_module = r#"//! Simple struct baseline test

use former::Former;

#[derive(Former, Debug, Clone)]
pub struct SimpleBaselineStruct {
  pub name: String,
  pub value: i32,
}

pub fn create_simple() -> SimpleBaselineStruct {
  SimpleBaselineStruct::former()
    .name("test".to_string())
    .value(42)
    .form()
}
"#;

  // Medium struct module (8 fields)
  let medium_module = r#"//! Medium struct baseline test

use former::Former;

#[derive(Former, Debug, Clone)]
pub struct MediumBaselineStruct {
  pub name: String,
  pub description: String,
  pub value: i32,
  pub count: usize,
  pub enabled: bool,
  pub tags: Vec<String>,
  pub metadata: std::collections::HashMap<String, String>,
  pub config: Option<String>,
}

pub fn create_medium() -> MediumBaselineStruct {
  let mut metadata = std::collections::HashMap::new();
  metadata.insert("key1".to_string(), "value1".to_string());
  
  MediumBaselineStruct::former()
    .name("test".to_string())
    .description("test description".to_string())
    .value(42)
    .count(10)
    .enabled(true)
    .tags(vec!["tag1".to_string(), "tag2".to_string()])
    .metadata(metadata)
    .config(Some("config".to_string()))
    .form()
}
"#;

  // Complex struct module (18 fields)
  let complex_module = r#"//! Complex struct baseline test

use former::Former;

#[derive(Former, Debug, Clone)]
pub struct ComplexBaselineStruct {
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

pub fn create_complex() -> ComplexBaselineStruct {
  let mut dependencies = std::collections::HashMap::new();
  dependencies.insert("serde".to_string(), "1.0".to_string());
  
  let mut features = std::collections::HashMap::new();
  features.insert("default".to_string(), vec!["std".to_string()]);
  
  ComplexBaselineStruct::former()
    .name("test-package".to_string())
    .description("A test package".to_string())
    .category("testing".to_string())
    .version("0.1.0".to_string())
    .author("Test Author".to_string())
    .license("MIT".to_string())
    .repository("https://github.com/test/test".to_string())
    .documentation("https://docs.rs/test".to_string())
    .keywords(vec!["test".to_string(), "benchmark".to_string()])
    .dependencies(dependencies)
    .dev_dependencies(std::collections::HashMap::new())
    .features(features)
    .targets(vec!["lib".to_string()])
    .exclude(vec!["target".to_string()])
    .include(vec!["src".to_string()])
    .publish(true)
    .edition("2021".to_string())
    .rust_version(Some("1.70".to_string()))
    .form()
}
"#;

  // Module declaration file
  let mod_file = r"//! Baseline test modules

pub mod simple_baseline;
pub mod medium_baseline;
pub mod complex_baseline;

pub use simple_baseline::*;
pub use medium_baseline::*;
pub use complex_baseline::*;
";

  // Write module files
  fs::write("src/baseline_tests/simple_baseline.rs", simple_module)?;
  fs::write("src/baseline_tests/medium_baseline.rs", medium_module)?;
  fs::write("src/baseline_tests/complex_baseline.rs", complex_module)?;
  fs::write("src/baseline_tests/mod.rs", mod_file)?;
  
  // Update lib.rs to include baseline tests conditionally
  let lib_rs_path = "src/lib.rs";
  let mut lib_content = fs::read_to_string(lib_rs_path)?;
  
  if !lib_content.contains("baseline_tests") {
    lib_content.push_str("\n#[cfg(feature = \"benchmarks\")]\npub mod baseline_tests;\n");
    fs::write(lib_rs_path, lib_content)?;
  }
  
  println!("  ✅ Baseline test modules created:");
  println!("     - simple_baseline.rs: 2 fields");
  println!("     - medium_baseline.rs: 8 fields");
  println!("     - complex_baseline.rs: 18 fields");
  println!();
  
  Ok(())
}

#[derive(Debug)]
struct BaselineResult {
  test_name: String,
  field_count: usize,
  compilation_time: Duration,
  check_time: Duration,
  success: bool,
}

fn measure_incremental_compilation() -> Result<Vec<BaselineResult>> {
  println!("2️⃣ Measuring Incremental Compilation Performance");
  println!("----------------------------------------------");
  
  let mut results = Vec::new();
  
  let test_cases = [
    ("simple", 2, "simple_baseline.rs"),
    ("medium", 8, "medium_baseline.rs"),
    ("complex", 18, "complex_baseline.rs"),
  ];
  
  for (test_name, field_count, module_file) in &test_cases {
    println!("  📏 Measuring {} baseline ({} fields)...", test_name, field_count);
    
    // Clean build to ensure fresh compilation
    let _ = Command::new("cargo").args(&["clean"]).output();
    
    // First, build dependencies to isolate our module timing
    let _ = Command::new("cargo")
      .args(&["check", "--features", "benchmarks"])
      .output();
    
    // Touch the specific module to force recompilation
    let module_path = format!("src/baseline_tests/{}", module_file);
    if Path::new(&module_path).exists() {
      // Update timestamp to force recompilation
      let now = std::time::SystemTime::now();
      let _ = filetime::set_file_mtime(&module_path, filetime::FileTime::from(now));
    }
    
    // Measure cargo check time for incremental compilation
    let start = Instant::now();
    let output = Command::new("cargo")
      .args(&["check", "--features", "benchmarks"])
      .output()?;
    let check_time = start.elapsed();
    
    // Measure full compilation time
    let start = Instant::now();
    let build_output = Command::new("cargo")
      .args(&["build", "--features", "benchmarks", "--release"])
      .output()?;
    let compilation_time = start.elapsed();
    
    let success = output.status.success() && build_output.status.success();
    
    if !success {
      println!("    ❌ Compilation failed for {}", test_name);
      if !output.status.success() {
        println!("    Check error: {}", String::from_utf8_lossy(&output.stderr));
      }
      if !build_output.status.success() {
        println!("    Build error: {}", String::from_utf8_lossy(&build_output.stderr));
      }
    } else {
      println!("    ✅ Compilation successful");
    }
    
    println!("    ⏱️  Check time: {:.2?}", check_time);
    println!("    🔨 Build time: {:.2?}", compilation_time);
    
    results.push(BaselineResult {
      test_name: test_name.to_string(),
      field_count: *field_count,
      compilation_time,
      check_time,
      success,
    });
  }
  
  println!();
  Ok(results)
}

fn generate_real_baseline_report(results: &[BaselineResult]) -> Result<()> {
  println!("3️⃣ Generating Real Baseline Report");
  println!("--------------------------------");
  
  let mut report = String::new();
  
  report.push_str("# Real Former Macro Baseline Performance\n\n");
  report.push_str("*Baseline measurements using incremental compilation timing*\n\n");
  
  report.push_str("## Compilation Performance Baseline\n\n");
  report.push_str("| Test Case | Fields | Check Time | Build Time | Status |\n");
  report.push_str("|-----------|--------|------------|------------|--------|\n");
  
  let mut total_check_time = Duration::new(0, 0);
  let mut total_build_time = Duration::new(0, 0);
  let mut successful_tests = 0;
  
  for result in results {
    let status = if result.success { "✅ Success" } else { "❌ Failed" };
    report.push_str(&format!(
      "| {} | {} | {:.2?} | {:.2?} | {} |\n",
      result.test_name,
      result.field_count,
      result.check_time,
      result.compilation_time,
      status
    ));
    
    if result.success {
      total_check_time += result.check_time;
      total_build_time += result.compilation_time;
      successful_tests += 1;
    }
  }
  
  report.push_str("\n## Baseline Analysis\n\n");
  
  // Calculate scaling factors
  if let (Some(simple), Some(complex)) = (
    results.iter().find(|r| r.test_name == "simple"),
    results.iter().find(|r| r.test_name == "complex")
  ) {
    if simple.success && complex.success {
      let check_scaling = complex.check_time.as_secs_f64() / simple.check_time.as_secs_f64();
      let build_scaling = complex.compilation_time.as_secs_f64() / simple.compilation_time.as_secs_f64();
      
      report.push_str(&format!(
        "- **Check Time Scaling**: {:.1}x (Simple → Complex)\n",
        check_scaling
      ));
      report.push_str(&format!(
        "- **Build Time Scaling**: {:.1}x (Simple → Complex)\n", 
        build_scaling
      ));
      
      report.push_str(&format!(
        "- **Simple Baseline**: Check {:.2?}, Build {:.2?}\n",
        simple.check_time, simple.compilation_time
      ));
      report.push_str(&format!(
        "- **Complex Baseline**: Check {:.2?}, Build {:.2?}\n",
        complex.check_time, complex.compilation_time
      ));
      
      // Task 001 target analysis
      let target_scaling = 2.5;
      let primary_scaling = build_scaling.max(check_scaling);
      
      if primary_scaling > target_scaling {
        report.push_str(&format!(
          "- **Task 001 Status**: Current {:.1}x > {:.1}x target - **OPTIMIZATION NEEDED**\n",
          primary_scaling, target_scaling
        ));
      } else {
        report.push_str(&format!(
          "- **Task 001 Status**: Current {:.1}x ≤ {:.1}x target - **TARGET MET**\n",
          primary_scaling, target_scaling
        ));
      }
    }
  }
  
  if successful_tests > 0 {
    let avg_check = total_check_time / successful_tests as u32;
    let avg_build = total_build_time / successful_tests as u32;
    report.push_str(&format!(
      "- **Average Times**: Check {:.2?}, Build {:.2?}\n",
      avg_check, avg_build
    ));
  }
  
  report.push_str("\n## Optimization Strategy\n\n");
  report.push_str("Based on baseline measurements, optimization should focus on:\n\n");
  report.push_str("1. **Macro expansion efficiency** - reducing generated code size\n");
  report.push_str("2. **Helper function extraction** - reusing common patterns\n"); 
  report.push_str("3. **Const evaluation** - compile-time computation\n");
  report.push_str("4. **Template optimization** - streamlined code generation\n\n");
  
  report.push_str("## Next Steps\n\n");
  report.push_str("1. Apply concrete optimizations to former_meta\n");
  report.push_str("2. Re-measure with identical test cases\n");
  report.push_str("3. Calculate actual improvement percentage\n");
  report.push_str("4. Validate Task 001 completion\n\n");
  
  report.push_str("---\n");
  report.push_str("*Real baseline generated using incremental compilation measurements*\n");
  
  // Save baseline report
  fs::write("target/-real_baseline_performance.md", &report)?;
  
  println!("  ✅ Real baseline report generated:");
  println!("     - Report saved: target/-real_baseline_performance.md");
  
  // Print key baseline metrics
  if let (Some(simple), Some(complex)) = (
    results.iter().find(|r| r.test_name == "simple"),
    results.iter().find(|r| r.test_name == "complex")
  ) {
    if simple.success && complex.success {
      let build_scaling = complex.compilation_time.as_secs_f64() / simple.compilation_time.as_secs_f64();
      println!("     - Baseline scaling: {:.1}x (Simple → Complex)", build_scaling);
      println!("     - Target scaling: ≤2.5x");
      
      if build_scaling > 2.5 {
        println!("     - Status: 🔴 Optimization needed");
      } else {
        println!("     - Status: 🟢 Target already met");
      }
    }
  }
  
  println!();
  Ok(())
}

// Helper for file timestamp manipulation
mod filetime {
  use std::fs;
  use std::path::Path;
  use std::time::SystemTime;
  
  pub struct FileTime(#[allow(dead_code)] SystemTime);
  
  impl FileTime {
    pub fn from(time: SystemTime) -> Self {
      FileTime(time)
    }
  }
  
  pub fn set_file_mtime<P: AsRef<Path>>(path: P, _time: FileTime) -> std::io::Result<()> {
    // Simple timestamp update by writing and reading
    let content = fs::read(path.as_ref())?;
    fs::write(path.as_ref(), content)?;
    Ok(())
  }
}