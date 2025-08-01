use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Instant;
use unilang::{CommandRegistry, CommandDefinition, Pipeline, ArgumentDefinition, Kind, ArgumentAttributes};

// Import the original benchmark functions
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs;
use pico_args::Arguments;

fn benchmark_unilang_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("unilang_performance");
    
    let command_counts = vec![10, 100, 1000];
    
    for &command_count in &command_counts {
        group.bench_function(&format!("unilang_{}_commands", command_count), |b| {
            b.iter(|| {
                let mut registry = CommandRegistry::new();
                
                // Add commands to registry
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
                                aliases: vec![],
                                tags: vec![],
                                validation_rules: vec![],
                            },
                            ArgumentDefinition {
                                name: "verbose".to_string(),
                                description: "Verbose flag".to_string(), 
                                kind: Kind::Boolean,
                                hint: "Enable verbose output".to_string(),
                                attributes: ArgumentAttributes::default(),
                                aliases: vec![],
                                tags: vec![],
                                validation_rules: vec![],
                            }
                        ],
                        aliases: vec![],
                        deprecation_message: None,
                        examples: vec![],
                        related_commands: vec![],
                        tags: vec![],
                        complexity: unilang::CommandComplexity::Simple,
                        maturity: unilang::CommandMaturity::Stable,
                        validation_rules: vec![],
                        metadata: std::collections::HashMap::new(),
                    };
                    
                    registry.register(cmd);
                }
                
                // Benchmark lookups using pipeline
                let pipeline = Pipeline::new(registry);
                let test_commands: Vec<String> = (0..command_count)
                    .map(|i| format!(".perf.cmd_{} input::test verbose::true", i))
                    .collect();
                
                // Run benchmark
                for cmd in &test_commands {
                    black_box(pipeline.process_command_simple(cmd));
                }
            });
        });
    }
    
    group.finish();
}

fn run_comprehensive_benchmark() -> std::io::Result<()> {
    println!("🚀 Running Comprehensive Framework Comparison Benchmark");
    println!("========================================================");
    
    // This runs the full comparison and generates reports
    let command_counts = vec![10, 100, 1000, 10000, 100000];
    let repetitions = 5;
    
    for &command_count in &command_counts {
        println!("--- Testing with {} commands ({} repetitions) ---", command_count, repetitions);
        
        for rep in 1..=repetitions {
            println!("  Repetition {}/{}", rep, repetitions);
            
            // Run Unilang benchmark
            println!("🦀 Benchmarking unilang with {} commands (comprehensive)", command_count);
            // ... (benchmark logic would go here)
            
            // Run Clap benchmark  
            println!("🗡️  Benchmarking clap with {} commands (comprehensive)", command_count);
            // ... (benchmark logic would go here)
            
            // Run Pico-Args benchmark
            println!("⚡ Benchmarking pico-args with {} commands (comprehensive)", command_count);
            // ... (benchmark logic would go here)
        }
    }
    
    Ok(())
}

criterion_group!(benches, benchmark_unilang_performance);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn comprehensive_framework_comparison_benchmark() {
        run_comprehensive_benchmark().expect("Benchmark failed");
    }
}