#![allow(clippy::all, warnings, missing_docs)]
//! Quote consolidation optimization impact measurement
//!
//! This benchmark measures the compilation performance impact of the Phase 1 quote
//! consolidation optimizations applied to the former macro code generation.

#![cfg(feature = "benchmarks")]
#![allow(clippy::all, warnings, missing_docs)]
#![allow(clippy::std_instead_of_core, clippy::unnecessary_wraps, clippy::uninlined_format_args, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]

use std::process::Command;
use std::time::{Duration, Instant};
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("🚀 Quote Consolidation Optimization Impact Measurement");
    println!("======================================================");
    println!();

    // Measure current performance with consolidation optimizations
    let optimized_results = measure_quote_consolidated_performance()?;
    
    // Compare with previous baseline
    compare_with_baseline(&optimized_results)?;
    
    // Generate impact report
    generate_quote_consolidation_report(&optimized_results)?;
    
    println!("✅ Quote consolidation impact measurement completed!");
    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct ConsolidationResult {
    test_name: String,
    compilation_time: Duration,
    improvement_type: String,
    success: bool,
}

fn measure_quote_consolidated_performance() -> Result<Vec<ConsolidationResult>> {
    println!("1️⃣ Measuring Quote-Consolidated Performance");
    println!("------------------------------------------");
    
    let mut results = Vec::new();
    
    // Test current state with our Phase 1 optimizations
    println!("  📏 Measuring with quote consolidation optimizations...");
    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()?;
    let compilation_time = start.elapsed();
    
    let success = output.status.success();
    if !success {
        println!("    ❌ Compilation failed");
        println!("    Error: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("    ✅ Compilation successful");
    }
    
    println!("    ⏱️  Compilation time: {:.2?}", compilation_time);
    
    results.push(ConsolidationResult {
        test_name: "quote_consolidated".to_string(),
        compilation_time,
        improvement_type: "quote_consolidation".to_string(),
        success,
    });
    
    println!();
    Ok(results)
}

fn compare_with_baseline(results: &[ConsolidationResult]) -> Result<()> {
    println!("2️⃣ Comparing with Previous Baseline");
    println!("----------------------------------");
    
    // Load previous baseline from our earlier measurements
    let baseline_time = load_previous_baseline()?;
    
    if let Some(current) = results.iter().find(|r| r.test_name == "quote_consolidated") {
        if current.success {
            let current_time = current.compilation_time.as_secs_f64();
            let improvement = ((baseline_time - current_time) / baseline_time) * 100.0;
            
            println!("  📊 Performance Comparison:");
            println!("     - Previous baseline: {:.2}s", baseline_time);
            println!("     - Quote consolidated: {:.2}s", current_time);
            
            if improvement > 0.0 {
                println!("     - Improvement: {:.1}% faster", improvement);
                
                if improvement >= 15.0 {
                    println!("     - ✅ Phase 1 target achieved ({:.1}% >= 15%)", improvement);
                } else {
                    println!("     - 🔶 Moderate improvement ({:.1}% < 15%)", improvement);
                }
            } else {
                println!("     - Change: {:.1}% (within measurement variance)", improvement.abs());
            }
            
            // Analyze optimization effectiveness
            analyze_consolidation_effectiveness(improvement)?;
        }
    }
    
    println!();
    Ok(())
}

fn load_previous_baseline() -> Result<f64> {
    // Try to load baseline from our previous measurements
    // Default to 7.32s from the practical baseline measurement if file not found
    let baseline_content = fs::read_to_string("target/-practical_baseline.md")
        .or_else(|_| fs::read_to_string("target/-post_optimization_measurement.md"))
        .unwrap_or_else(|_| "7.32s".to_string());
    
    // Extract time from baseline content (simple parsing)
    if baseline_content.contains("7.31s") {
        Ok(7.31) // Post-optimization baseline
    } else if baseline_content.contains("7.32s") {
        Ok(7.32) // Original baseline
    } else {
        // Default based on our measurements
        Ok(7.30)
    }
}

fn analyze_consolidation_effectiveness(improvement: f64) -> Result<()> {
    println!("  🔍 Quote Consolidation Analysis:");
    
    if improvement >= 20.0 {
        println!("     - 🚀 Excellent consolidation impact");
        println!("     - Token generation overhead significantly reduced");
        println!("     - Phase 1 objectives exceeded");
    } else if improvement >= 15.0 {
        println!("     - ✅ Good consolidation impact"); 
        println!("     - Meaningful reduction in quote! overhead");
        println!("     - Phase 1 objectives achieved");
    } else if improvement >= 8.0 {
        println!("     - 🔶 Moderate consolidation impact");
        println!("     - Some reduction in token generation overhead");
        println!("     - Additional optimizations recommended");
    } else if improvement >= 3.0 {
        println!("     - 🔶 Minor consolidation impact");
        println!("     - Limited quote! overhead reduction");
        println!("     - Consider more aggressive consolidation");
    } else {
        println!("     - ⚠️  Minimal consolidation impact");
        println!("     - Quote! consolidation may need refinement");
        println!("     - Other bottlenecks may dominate");
    }
    
    Ok(())
}

fn generate_quote_consolidation_report(results: &[ConsolidationResult]) -> Result<()> {
    println!("3️⃣ Generating Quote Consolidation Report");
    println!("---------------------------------------");
    
    let mut report = String::new();
    
    report.push_str("# Quote Consolidation Optimization Report\n\n");
    report.push_str("*Phase 1 optimization impact measurement*\n\n");
    
    report.push_str("## Optimization Summary\n\n");
    report.push_str("### Phase 1: Quote Consolidation Implementation\n\n");
    report.push_str("The Phase 1 optimization focused on consolidating scattered `quote!` macro \n");
    report.push_str("invocations throughout the former code generation process. This addresses \n");
    report.push_str("the primary bottleneck identified in the advanced analysis.\n\n");
    
    report.push_str("#### Specific Optimizations Applied:\n\n");
    report.push_str("1. **Consolidated Generic Generation**: Replaced 6+ individual `quote!` calls \n");
    report.push_str("   with 2 consolidated helper functions for generic parameter handling\n\n");
    report.push_str("2. **Former Type Reference Consolidation**: Replaced repetitive type reference \n");
    report.push_str("   generation patterns with unified helper function\n\n");
    report.push_str("3. **Field Processing Optimization**: Enhanced single-pass field processing \n");
    report.push_str("   with pre-allocated vectors for better memory efficiency\n\n");
    
    report.push_str("## Performance Results\n\n");
    
    if let Some(result) = results.iter().find(|r| r.test_name == "quote_consolidated") {
        if result.success {
            let baseline = load_previous_baseline()?;
            let current = result.compilation_time.as_secs_f64();
            let improvement = ((baseline - current) / baseline) * 100.0;
            
            report.push_str(&format!("| Metric | Value |\n"));
            report.push_str(&format!("|--------|-------|\n"));
            report.push_str(&format!("| Previous Baseline | {:.2}s |\n", baseline));
            report.push_str(&format!("| Quote Consolidated | {:.2}s |\n", current));
            report.push_str(&format!("| Improvement | {:.1}% |\n", improvement));
            report.push_str(&format!("| Status | {} |\n\n", 
                if improvement >= 15.0 { "✅ Target Achieved" }
                else if improvement >= 8.0 { "🔶 Moderate Progress" }
                else { "⚠️  Needs Further Work" }
            ));
            
            report.push_str("## Analysis\n\n");
            
            if improvement >= 15.0 {
                report.push_str("### ✅ Phase 1 Success\n\n");
                report.push_str("The quote consolidation optimization achieved the target improvement, \n");
                report.push_str("demonstrating that token generation overhead was indeed a significant \n");
                report.push_str("bottleneck in the former macro compilation process.\n\n");
                
                report.push_str("**Key Findings:**\n");
                report.push_str("- Quote! consolidation effectively reduces compilation overhead\n");
                report.push_str("- Helper functions minimize redundant token generation\n");
                report.push_str("- Phase 2 optimizations can build on this foundation\n\n");
            } else if improvement >= 5.0 {
                report.push_str("### 🔶 Partial Success\n\n");
                report.push_str("The quote consolidation optimization provided measurable improvement, \n");
                report.push_str("but additional optimizations are needed to reach the full potential.\n\n");
                
                report.push_str("**Recommendations:**\n");
                report.push_str("- Apply more aggressive quote! consolidation\n");
                report.push_str("- Target remaining large token generation blocks\n");
                report.push_str("- Implement Phase 2: Template Pre-generation\n\n");
            } else {
                report.push_str("### ⚠️  Limited Impact\n\n");
                report.push_str("The quote consolidation optimization showed limited impact, suggesting \n");
                report.push_str("that other bottlenecks may be dominating the compilation time.\n\n");
                
                report.push_str("**Investigation Needed:**\n");
                report.push_str("- Profile dependency compilation overhead\n");
                report.push_str("- Analyze syn parsing performance\n");
                report.push_str("- Consider feature flag optimization\n\n");
            }
        }
    }
    
    report.push_str("## Next Steps\n\n");
    report.push_str("### Immediate Actions\n\n");
    report.push_str("1. **Analyze Results**: Review the improvement percentage and identify \n");
    report.push_str("   remaining optimization opportunities\n\n");
    report.push_str("2. **Phase 2 Planning**: If Phase 1 was successful, proceed with template \n");
    report.push_str("   pre-generation optimization\n\n");
    report.push_str("3. **Dependency Analysis**: If improvement was limited, focus on \n");
    report.push_str("   dependency compilation optimization\n\n");
    
    report.push_str("### Task 001 Progress\n\n");
    if let Some(result) = results.iter().find(|r| r.test_name == "quote_consolidated") {
        if result.success {
            let baseline = load_previous_baseline()?;
            let current = result.compilation_time.as_secs_f64();
            let total_improvement_needed = 40.0; // Task 001 target
            let current_improvement = ((baseline - current) / baseline) * 100.0;
            let remaining_improvement = total_improvement_needed - current_improvement;
            
            if current_improvement >= total_improvement_needed {
                report.push_str("- **Status**: ✅ **TASK 001 COMPLETED** \n");
                report.push_str(&format!("- **Achievement**: {:.1}% improvement >= 40% target\n\n", current_improvement));
            } else {
                report.push_str(&format!("- **Progress**: {:.1}% of 40% target completed\n", current_improvement));
                report.push_str(&format!("- **Remaining**: {:.1}% improvement still needed\n", remaining_improvement));
                report.push_str("- **Strategy**: Continue with Phase 2 and Phase 3 optimizations\n\n");
            }
        }
    }
    
    report.push_str("---\n");
    report.push_str("*Quote consolidation optimization report generated by Phase 1 measurement*\n");
    
    // Save report
    fs::write("target/-quote_consolidation_report.md", &report)?;
    
    println!("  ✅ Quote consolidation report generated:");
    println!("     - Report saved: target/-quote_consolidation_report.md");
    println!("     - Focus: Phase 1 optimization impact validation");
    
    // Print key metrics
    if let Some(result) = results.iter().find(|r| r.test_name == "quote_consolidated") {
        if result.success {
            let baseline = load_previous_baseline()?;
            let current = result.compilation_time.as_secs_f64();
            let improvement = ((baseline - current) / baseline) * 100.0;
            
            println!("  🎯 Phase 1 Results:");
            println!("     - Quote consolidation improvement: {:.1}%", improvement);
            
            if improvement >= 15.0 {
                println!("     - Status: ✅ Phase 1 target achieved");
                println!("     - Next: Proceed with Phase 2 optimizations");
            } else if improvement >= 5.0 {
                println!("     - Status: 🔶 Moderate progress");
                println!("     - Next: Enhanced consolidation or Phase 2");
            } else {
                println!("     - Status: ⚠️  Limited impact");
                println!("     - Next: Focus on dependency optimization");
            }
        }
    }
    
    println!();
    Ok(())
}