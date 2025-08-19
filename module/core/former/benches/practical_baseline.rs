#![allow(clippy::all, warnings, missing_docs)]
//! Practical baseline measurement using actual former compilation
//!
#![allow(clippy::std_instead_of_core, clippy::unnecessary_wraps, clippy::uninlined_format_args, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
//! This measures compilation time of former itself with different feature sets
//! to establish a real performance baseline.

use std::process::Command;
use std::time::{Duration, Instant};
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("📊 Practical Former Baseline Measurement");
    println!("=========================================");
    println!();

    // Measure compilation times for different configurations
    let baseline_results = measure_former_compilation()?;
    
    // Analyze existing test files for complexity
    let test_analysis = analyze_test_complexity()?;
    
    // Generate practical baseline report
    generate_practical_report(&baseline_results, &test_analysis)?;
    
    println!("✅ Practical baseline completed!");
    Ok(())
}

#[derive(Debug)]
struct CompilationMeasurement {
    config_name: String,
    compile_time: Duration,
    success: bool,
    features: String,
}

fn measure_former_compilation() -> Result<Vec<CompilationMeasurement>> {
    println!("1️⃣ Measuring Former Compilation Performance");
    println!("------------------------------------------");
    
    let mut results = Vec::new();
    
    // Test different compilation scenarios
    let test_configs = [
        ("minimal", ""),
        ("default", "default"),
        ("full", "full"),
        ("benchmarks", "benchmarks"),
    ];
    
    for (config_name, features) in &test_configs {
        println!("  📏 Measuring {} configuration...", config_name);
        
        // Clean to ensure fresh build
        let _ = Command::new("cargo").args(&["clean"]).output();
        
        // Measure compilation time
        let start = Instant::now();
        let output = if features.is_empty() {
            Command::new("cargo")
                .args(&["build", "--release", "--no-default-features"])
                .output()?
        } else {
            Command::new("cargo")
                .args(&["build", "--release", "--features", features])
                .output()?
        };
        let compile_time = start.elapsed();
        
        let success = output.status.success();
        
        if success {
            println!("    ✅ Compilation successful: {:.2?}", compile_time);
        } else {
            println!("    ❌ Compilation failed: {:.2?}", compile_time);
        }
        
        results.push(CompilationMeasurement {
            config_name: config_name.to_string(),
            compile_time,
            success,
            features: features.to_string(),
        });
    }
    
    println!();
    Ok(results)
}

#[derive(Debug)]
struct TestComplexity {
    test_file: String,
    struct_count: usize,
    field_count: usize,
    former_usage: usize,
}

fn analyze_test_complexity() -> Result<Vec<TestComplexity>> {
    println!("2️⃣ Analyzing Test File Complexity");
    println!("--------------------------------");
    
    let mut complexities = Vec::new();
    
    // Find test files that use Former
    let test_dirs = ["tests", "examples"];
    
    for test_dir in &test_dirs {
        if let Ok(entries) = fs::read_dir(test_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let analysis = analyze_file_content(&content);
                        if analysis.former_usage > 0 {
                            complexities.push(TestComplexity {
                                test_file: path.to_string_lossy().to_string(),
                                struct_count: analysis.struct_count,
                                field_count: analysis.field_count,
                                former_usage: analysis.former_usage,
                            });
                        }
                    }
                }
            }
        }
    }
    
    // Sort by complexity (field count)
    complexities.sort_by_key(|c| c.field_count);
    
    println!("  ✅ Analyzed {} test files with Former usage", complexities.len());
    for complexity in &complexities {
        println!("     - {}: {} structs, {} fields", 
                 complexity.test_file.split('/').last().unwrap_or("unknown"),
                 complexity.struct_count, 
                 complexity.field_count);
    }
    
    println!();
    Ok(complexities)
}

#[derive(Default)]
struct FileAnalysis {
    struct_count: usize,
    field_count: usize,
    former_usage: usize,
}

fn analyze_file_content(content: &str) -> FileAnalysis {
    let mut analysis = FileAnalysis::default();
    
    // Count Former derives
    analysis.former_usage = content.matches("#[derive(").filter(|derive_line| {
        derive_line.contains("Former")
    }).count();
    
    // Count struct definitions with Former
    let lines: Vec<&str> = content.lines().collect();
    let mut in_former_struct = false;
    
    for (i, line) in lines.iter().enumerate() {
        // Check if this is a Former struct
        if line.contains("#[derive(") && line.contains("Former") {
            in_former_struct = true;
            continue;
        }
        
        if in_former_struct && line.trim().starts_with("pub struct") {
            analysis.struct_count += 1;
            
            // Count fields in this struct
            let mut field_count = 0;
            for j in (i + 1)..lines.len() {
                let field_line = lines[j].trim();
                if field_line.starts_with('}') {
                    break;
                }
                if field_line.contains(':') && !field_line.starts_with("//") {
                    field_count += 1;
                }
            }
            analysis.field_count += field_count;
            in_former_struct = false;
        }
    }
    
    analysis
}

fn generate_practical_report(measurements: &[CompilationMeasurement], test_analysis: &[TestComplexity]) -> Result<()> {
    println!("3️⃣ Generating Practical Baseline Report");
    println!("-------------------------------------");
    
    let mut report = String::new();
    
    report.push_str("# Practical Former Baseline Performance\n\n");
    report.push_str("*Real-world baseline using actual former compilation and test analysis*\n\n");
    
    // Compilation performance section
    report.push_str("## Compilation Performance Baseline\n\n");
    report.push_str("| Configuration | Features | Compile Time | Status |\n");
    report.push_str("|---------------|----------|--------------|--------|\n");
    
    let mut successful_measurements = Vec::new();
    for measurement in measurements {
        let status = if measurement.success { "✅ Success" } else { "❌ Failed" };
        let features_display = if measurement.features.is_empty() { "none" } else { &measurement.features };
        
        report.push_str(&format!(
            "| {} | {} | {:.2?} | {} |\n",
            measurement.config_name,
            features_display,
            measurement.compile_time,
            status
        ));
        
        if measurement.success {
            successful_measurements.push(measurement);
        }
    }
    
    // Calculate baseline metrics
    if successful_measurements.len() >= 2 {
        let minimal = successful_measurements.iter().min_by_key(|m| m.compile_time).unwrap();
        let maximal = successful_measurements.iter().max_by_key(|m| m.compile_time).unwrap();
        
        let scaling_factor = maximal.compile_time.as_secs_f64() / minimal.compile_time.as_secs_f64();
        
        report.push_str("\n### Compilation Scaling Analysis\n\n");
        report.push_str(&format!(
            "- **Fastest configuration**: {} ({:.2?})\n",
            minimal.config_name, minimal.compile_time
        ));
        report.push_str(&format!(
            "- **Slowest configuration**: {} ({:.2?})\n",
            maximal.config_name, maximal.compile_time
        ));
        report.push_str(&format!(
            "- **Configuration scaling**: {:.1}x\n",
            scaling_factor
        ));
    }
    
    // Test complexity analysis
    report.push_str("\n## Test File Complexity Analysis\n\n");
    report.push_str("| Test File | Structs | Total Fields | Former Usage |\n");
    report.push_str("|-----------|---------|--------------|-------------|\n");
    
    for test in test_analysis {
        let filename = test.test_file.split('/').last().unwrap_or("unknown");
        report.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            filename, test.struct_count, test.field_count, test.former_usage
        ));
    }
    
    if !test_analysis.is_empty() {
        let total_structs: usize = test_analysis.iter().map(|t| t.struct_count).sum();
        let total_fields: usize = test_analysis.iter().map(|t| t.field_count).sum();
        let total_usage: usize = test_analysis.iter().map(|t| t.former_usage).sum();
        
        report.push_str("\n### Test Complexity Summary\n\n");
        report.push_str(&format!("- **Total Former structs**: {}\n", total_structs));
        report.push_str(&format!("- **Total fields**: {}\n", total_fields));
        report.push_str(&format!("- **Total Former usage**: {}\n", total_usage));
        
        if total_structs > 0 {
            let avg_fields = total_fields as f64 / total_structs as f64;
            report.push_str(&format!("- **Average fields per struct**: {:.1}\n", avg_fields));
            
            // Find complexity range
            let min_fields = test_analysis.iter().map(|t| t.field_count).min().unwrap_or(0);
            let max_fields = test_analysis.iter().map(|t| t.field_count).max().unwrap_or(0);
            report.push_str(&format!("- **Complexity range**: {} - {} fields\n", min_fields, max_fields));
        }
    }
    
    // Performance implications
    report.push_str("\n## Performance Baseline Insights\n\n");
    
    if let Some(default_measurement) = measurements.iter().find(|m| m.config_name == "default" && m.success) {
        report.push_str(&format!(
            "- **Current former compilation**: {:.2?}\n",
            default_measurement.compile_time
        ));
        
        // Estimate macro overhead
        if let Some(minimal_measurement) = measurements.iter().find(|m| m.config_name == "minimal" && m.success) {
            let macro_overhead = default_measurement.compile_time.as_secs_f64() - minimal_measurement.compile_time.as_secs_f64();
            report.push_str(&format!(
                "- **Estimated macro overhead**: {:.2?}\n",
                Duration::from_secs_f64(macro_overhead.max(0.0))
            ));
        }
    }
    
    // Calculate realistic optimization targets
    if !test_analysis.is_empty() {
        let complex_tests = test_analysis.iter().filter(|t| t.field_count >= 10).count();
        let simple_tests = test_analysis.iter().filter(|t| t.field_count <= 5).count();
        
        report.push_str(&format!(
            "- **Complex structs (10+ fields)**: {} test files\n",
            complex_tests
        ));
        report.push_str(&format!(
            "- **Simple structs (≤5 fields)**: {} test files\n",
            simple_tests
        ));
        
        if complex_tests > 0 && simple_tests > 0 {
            report.push_str("- **Optimization focus**: Complex struct compilation efficiency\n");
        }
    }
    
    // Task 001 status evaluation
    report.push_str("\n## Task 001 Baseline Assessment\n\n");
    report.push_str("Based on practical measurements:\n\n");
    
    if let Some(default_time) = measurements.iter().find(|m| m.config_name == "default" && m.success).map(|m| m.compile_time) {
        if default_time < Duration::from_secs(5) {
            report.push_str("- **Current performance**: Compilation time is reasonable for optimization\n");
        } else {
            report.push_str("- **Current performance**: Compilation time indicates optimization opportunities\n");
        }
        
        let target_time = Duration::from_secs_f64(default_time.as_secs_f64() * 0.6); // 40% improvement
        report.push_str(&format!(
            "- **Optimization target**: Reduce to {:.2?} (40% improvement)\n",
            target_time
        ));
    }
    
    report.push_str("- **Next step**: Apply concrete optimizations and re-measure\n");
    
    report.push_str("\n---\n");
    report.push_str("*Practical baseline using real former compilation and test analysis*\n");
    
    // Save report
    fs::write("target/-practical_baseline.md", &report)?;
    
    println!("  ✅ Practical baseline report saved: target/-practical_baseline.md");
    
    // Print key findings
    if let Some(default_measurement) = measurements.iter().find(|m| m.config_name == "default" && m.success) {
        println!("  📊 Key Baseline Metrics:");
        println!("     - Former compilation time: {:.2?}", default_measurement.compile_time);
        println!("     - Test files analyzed: {}", test_analysis.len());
        
        if !test_analysis.is_empty() {
            let total_fields: usize = test_analysis.iter().map(|t| t.field_count).sum();
            let total_structs: usize = test_analysis.iter().map(|t| t.struct_count).sum();
            if total_structs > 0 {
                let avg_fields = total_fields as f64 / total_structs as f64;
                println!("     - Average complexity: {:.1} fields per struct", avg_fields);
            }
        }
    }
    
    println!();
    Ok(())
}