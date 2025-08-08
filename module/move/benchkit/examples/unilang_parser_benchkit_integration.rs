//! Comprehensive benchkit integration with unilang_parser
//!
//! This demonstrates applying benchkit to parser performance analysis,
//! identifying parser-specific benchmarking needs and implementing solutions.

use benchkit::prelude::*;

// We'll simulate unilang_parser functionality since it's in a different workspace
// In real integration, you'd use: use unilang_parser::{Parser, UnilangParserOptions};

fn main() -> error_tools::Result<()>
{
  println!("🚀 Benchkit Integration with unilang_parser");
  println!("============================================");
  println!();

  // Phase 1: Parser-specific data generation
  test_parser_data_generation()?;
  
  // Phase 2: Parsing performance analysis
  test_parsing_performance_analysis()?;
  
  // Phase 3: Memory allocation in parsing pipeline  
  test_parser_memory_analysis()?;
  
  // Phase 4: Parser throughput and scaling
  test_parser_throughput_analysis()?;
  
  // Phase 5: Statistical validation of parser performance
  #[cfg(feature = "statistical_analysis")]
  test_parser_statistical_analysis()?;
  
  // Phase 6: Parser-specific reporting
  test_parser_comprehensive_reporting()?;

  println!("✅ unilang_parser benchkit integration completed!");
  println!();
  
  // Identify missing benchkit features for parsers
  identify_parser_specific_features();
  
  Ok(())
}

fn test_parser_data_generation() -> error_tools::Result<()>
{
  println!("1️⃣ Parser-Specific Data Generation");
  println!("---------------------------------");
  
  // Test command generation capabilities
  let command_generator = DataGenerator::new()
    .complexity(DataComplexity::Complex);
    
  let unilang_commands = command_generator.generate_unilang_commands(10);
  
  println!("  ✅ Generated {} unilang commands:", unilang_commands.len());
  for (i, cmd) in unilang_commands.iter().take(3).enumerate() 
  {
    println!("     {}. {}", i + 1, cmd);
  }
  
  // Test parser-specific patterns
  println!("\n  📊 Parser-specific pattern generation:");
  
  // Simple commands
  let simple_generator = DataGenerator::new()
    .pattern("command{}.action{}")
    .repetitions(5)
    .complexity(DataComplexity::Simple);
  let simple_commands = simple_generator.generate_string();
  println!("     Simple: {}", &simple_commands[..60.min(simple_commands.len())]);
  
  // Complex commands with arguments
  let complex_generator = DataGenerator::new()
    .pattern("namespace{}.cmd{} arg{}::value{} pos{}")
    .repetitions(3)
    .complexity(DataComplexity::Complex);
  let complex_commands = complex_generator.generate_string();
  println!("     Complex: {}", &complex_commands[..80.min(complex_commands.len())]);
  
  // Nested command structures
  let nested_data = generate_nested_parser_commands(3, 4);
  println!("     Nested: {} chars generated", nested_data.len());
  
  println!();
  Ok(())
}

fn test_parsing_performance_analysis() -> error_tools::Result<()>
{
  println!("2️⃣ Parser Performance Analysis");
  println!("-----------------------------");
  
  // Generate realistic parser test data
  let simple_cmd = "system.status";
  let medium_cmd = "user.create name::alice email::alice@test.com active::true";
  let complex_cmd = "report.generate format::pdf output::\"/tmp/report.pdf\" compress::true metadata::\"Daily Report\" tags::[\"daily\",\"automated\"] priority::high";
  
  let simple_clone = simple_cmd.to_string();
  let medium_clone = medium_cmd.to_string();
  let complex_clone = complex_cmd.to_string();
  
  let mut parsing_comparison = ComparativeAnalysis::new("unilang_parsing_performance");
  
  parsing_comparison = parsing_comparison
    .algorithm("simple_command", move || {
      let result = simulate_parse_command(&simple_clone);
      std::hint::black_box(result);
    })
    .algorithm("medium_command", move || {
      let result = simulate_parse_command(&medium_clone);
      std::hint::black_box(result);
    })
    .algorithm("complex_command", move || {
      let result = simulate_parse_command(&complex_clone);
      std::hint::black_box(result);
    });

  let parsing_report = parsing_comparison.run();
  
  if let Some((fastest, result)) = parsing_report.fastest()
  {
    println!("  ✅ Parsing performance analysis:");
    println!("     - Fastest: {} ({:.0} parses/sec)", fastest, result.operations_per_second());
    println!("     - Reliability: CV = {:.1}%", result.coefficient_of_variation() * 100.0);
  }
  
  // Test batch parsing vs individual parsing
  println!("\n  📈 Batch vs Individual Parsing:");
  
  let commands = vec![
    "system.status",
    "user.list active::true",
    "log.rotate max_files::10",
    "cache.clear namespace::temp",
    "db.backup name::daily",
  ];
  
  let commands_clone = commands.clone();
  let commands_clone2 = commands.clone();
  
  let mut batch_comparison = ComparativeAnalysis::new("batch_vs_individual_parsing");
  
  batch_comparison = batch_comparison
    .algorithm("individual_parsing", move || {
      let mut total_parsed = 0;
      for cmd in &commands_clone {
        let _result = simulate_parse_command(cmd);
        total_parsed += 1;
      }
      std::hint::black_box(total_parsed);
    })
    .algorithm("batch_parsing", move || {
      let batch_input = commands_clone2.join(" ;; ");
      let result = simulate_batch_parse(&batch_input);
      std::hint::black_box(result);
    });

  let batch_report = batch_comparison.run();
  
  if let Some((fastest_batch, result)) = batch_report.fastest()
  {
    println!("     - Fastest approach: {} ({:.0} ops/sec)", fastest_batch, result.operations_per_second());
  }
  
  println!();
  Ok(())
}

fn test_parser_memory_analysis() -> error_tools::Result<()>
{
  println!("3️⃣ Parser Memory Analysis");
  println!("------------------------");
  
  let memory_benchmark = MemoryBenchmark::new("unilang_parser_memory");
  
  // Test memory usage patterns in parsing
  let complex_command = "system.process.management.service.restart name::web_server graceful::true timeout::30s force::false backup_config::true notify_admins::[\"admin1@test.com\",\"admin2@test.com\"] log_level::debug";
  
  let cmd_clone = complex_command.to_string();
  let cmd_clone2 = complex_command.to_string();
  
  let memory_comparison = memory_benchmark.compare_memory_usage(
    "string_based_parsing",
    move || {
      // Simulate string-heavy parsing (old approach)
      let parts = cmd_clone.split_whitespace().collect::<Vec<_>>();
      let tokens = parts.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
      std::hint::black_box(tokens.len());
    },
    "zero_copy_parsing", 
    move || {
      // Simulate zero-copy parsing (optimized approach)
      let parts = cmd_clone2.split_whitespace().collect::<Vec<_>>();
      std::hint::black_box(parts.len());
    },
    20,
  );
  
  let (efficient_name, efficient_stats) = memory_comparison.more_memory_efficient();
  let reduction = memory_comparison.memory_reduction_percentage();
  
  println!("  ✅ Parser memory analysis:");
  println!("     - More efficient: {} ({:.1}% reduction)", efficient_name, reduction);
  println!("     - Peak memory: {} bytes", efficient_stats.peak_usage);
  println!("     - Total allocations: {}", efficient_stats.allocation_count);
  
  // Test allocation patterns during parsing pipeline
  println!("\n  🧠 Parsing pipeline allocation analysis:");
  
  let mut profiler = MemoryProfiler::new();
  
  // Simulate parsing pipeline stages
  profiler.record_allocation(1024); // Tokenization
  profiler.record_allocation(512);  // AST construction  
  profiler.record_allocation(256);  // Argument processing
  profiler.record_deallocation(256); // Cleanup temporaries
  profiler.record_allocation(128);  // Final instruction building
  
  let pattern_analysis = profiler.analyze_patterns();
  
  println!("     - Total allocation events: {}", pattern_analysis.total_events);
  println!("     - Peak usage: {} bytes", pattern_analysis.peak_usage);
  println!("     - Memory leaks detected: {}", if pattern_analysis.has_potential_leaks() { "Yes" } else { "No" });
  
  if let Some(size_stats) = pattern_analysis.size_statistics()
  {
    println!("     - Allocation sizes: min={}, max={}, avg={:.1}", 
             size_stats.min, size_stats.max, size_stats.mean);
  }
  
  println!();
  Ok(())
}

fn test_parser_throughput_analysis() -> error_tools::Result<()>
{
  println!("4️⃣ Parser Throughput Analysis");  
  println!("----------------------------");
  
  // Generate realistic parser workload
  let parser_workload = generate_parser_workload(1000);
  println!("  📊 Generated parser workload: {} commands, {} total chars", 
           parser_workload.len(), 
           parser_workload.iter().map(|s| s.len()).sum::<usize>());
  
  let total_chars = parser_workload.iter().map(|s| s.len()).sum::<usize>();
  let throughput_analyzer = ThroughputAnalyzer::new("parser_throughput", total_chars as u64)
    .with_items(parser_workload.len() as u64);
  
  // Simulate different parser implementations
  let mut parser_results = std::collections::HashMap::new();
  
  // Fast parser (optimized)
  let fast_times = vec![std::time::Duration::from_micros(50); 15];
  parser_results.insert("optimized_parser".to_string(), 
                       BenchmarkResult::new("optimized", fast_times));
  
  // Standard parser
  let standard_times = vec![std::time::Duration::from_micros(150); 15];
  parser_results.insert("standard_parser".to_string(),
                       BenchmarkResult::new("standard", standard_times));
  
  // Naive parser (baseline)
  let naive_times = vec![std::time::Duration::from_micros(400); 15];
  parser_results.insert("naive_parser".to_string(),
                       BenchmarkResult::new("naive", naive_times));
  
  let throughput_comparison = throughput_analyzer.compare_throughput(&parser_results);
  
  if let Some((fastest_name, fastest_metrics)) = throughput_comparison.fastest_throughput()
  {
    println!("  ✅ Parser throughput analysis:");
    println!("     - Fastest parser: {} ({})", fastest_name, fastest_metrics.throughput_description());
    
    if let Some(items_desc) = fastest_metrics.items_description()
    {
      println!("     - Command parsing rate: {}", items_desc);
    }
  }
  
  if let Some(speedups) = throughput_comparison.calculate_speedups("naive_parser")
  {
    println!("     - Performance improvements:");
    for (name, speedup) in speedups
    {
      if name != "naive_parser"
      {
        println!("       * {}: {:.1}x faster than baseline", name, speedup);
      }
    }
  }
  
  // Parser-specific throughput metrics
  println!("\n  📈 Parser-specific metrics:");
  
  if let Some(fastest_metrics) = throughput_comparison.fastest_throughput().map(|(_, m)| m)
  {
    let chars_per_sec = (total_chars as f64 / fastest_metrics.processing_time.as_secs_f64()) as u64;
    let commands_per_sec = (parser_workload.len() as f64 / fastest_metrics.processing_time.as_secs_f64()) as u64;
    
    println!("     - Characters processed: {}/sec", format_throughput_number(chars_per_sec));
    println!("     - Commands parsed: {}/sec", format_throughput_number(commands_per_sec));
    println!("     - Average command size: {} chars", total_chars / parser_workload.len());
  }
  
  println!();
  Ok(())
}

#[cfg(feature = "statistical_analysis")]
fn test_parser_statistical_analysis() -> error_tools::Result<()>
{
  println!("5️⃣ Parser Statistical Analysis");
  println!("-----------------------------");
  
  // Create parser performance data with different characteristics
  let consistent_parser_times: Vec<_> = (0..25)
    .map(|i| std::time::Duration::from_micros(100 + i * 2))
    .collect();
  let consistent_result = BenchmarkResult::new("consistent_parser", consistent_parser_times);
  
  let variable_parser_times: Vec<_> = (0..25)
    .map(|i| std::time::Duration::from_micros(100 + (i * i) % 50))
    .collect();  
  let variable_result = BenchmarkResult::new("variable_parser", variable_parser_times);
  
  // Analyze statistical properties
  let consistent_analysis = StatisticalAnalysis::analyze(&consistent_result, SignificanceLevel::Standard)?;
  let variable_analysis = StatisticalAnalysis::analyze(&variable_result, SignificanceLevel::Standard)?;
  
  println!("  ✅ Parser statistical analysis:");
  println!("     - Consistent parser:");
  println!("       * CV: {:.1}% ({})", 
           consistent_analysis.coefficient_of_variation * 100.0,
           if consistent_analysis.is_reliable() { "✅ Reliable" } else { "⚠️ Questionable" });
  println!("       * 95% CI: [{:.1}, {:.1}] μs",
           consistent_analysis.mean_confidence_interval.lower_bound.as_micros(),
           consistent_analysis.mean_confidence_interval.upper_bound.as_micros());
  
  println!("     - Variable parser:");
  println!("       * CV: {:.1}% ({})",
           variable_analysis.coefficient_of_variation * 100.0,
           if variable_analysis.is_reliable() { "✅ Reliable" } else { "⚠️ Questionable" });
  println!("       * 95% CI: [{:.1}, {:.1}] μs",
           variable_analysis.mean_confidence_interval.lower_bound.as_micros(),
           variable_analysis.mean_confidence_interval.upper_bound.as_micros());
  
  // Statistical comparison
  let comparison = StatisticalAnalysis::compare(
    &consistent_result,
    &variable_result,
    SignificanceLevel::Standard
  )?;
  
  println!("  ✅ Statistical comparison:");
  println!("     - Effect size: {:.3} ({})", 
           comparison.effect_size,
           comparison.effect_size_interpretation());
  println!("     - Statistically significant: {}", 
           if comparison.is_significant { "✅ Yes" } else { "❌ No" });
  println!("     - P-value: {:.6}", comparison.p_value);
  
  // Parser performance reliability assessment
  println!("\n  📊 Parser reliability assessment:");
  
  let reliability_threshold = 10.0; // 10% CV threshold for parsers
  let consistent_reliable = consistent_analysis.coefficient_of_variation * 100.0 < reliability_threshold;
  let variable_reliable = variable_analysis.coefficient_of_variation * 100.0 < reliability_threshold;
  
  println!("     - Reliability threshold: {}% CV", reliability_threshold);
  println!("     - Consistent parser meets standard: {}", if consistent_reliable { "✅" } else { "❌" });
  println!("     - Variable parser meets standard: {}", if variable_reliable { "✅" } else { "❌" });
  
  println!();
  Ok(())
}

fn test_parser_comprehensive_reporting() -> error_tools::Result<()>
{
  println!("6️⃣ Parser Comprehensive Reporting");
  println!("--------------------------------");
  
  // Generate comprehensive parser benchmark suite
  let parser_workload = generate_parser_workload(500);
  
  let workload_clone = parser_workload.clone();
  let workload_clone2 = parser_workload.clone();
  let workload_clone3 = parser_workload.clone();
  let workload_clone4 = parser_workload.clone();
  
  let mut parser_suite = BenchmarkSuite::new("unilang_parser_comprehensive");
  
  // Add parser-specific benchmarks
  parser_suite.benchmark("tokenization", move || {
    let mut token_count = 0;
    for cmd in &workload_clone {
      token_count += cmd.split_whitespace().count();
    }
    std::hint::black_box(token_count);
  });
  
  parser_suite.benchmark("command_path_parsing", move || {
    let mut command_count = 0;
    for cmd in &workload_clone2 {
      // Simulate command path extraction
      if let Some(first_part) = cmd.split_whitespace().next() {
        command_count += first_part.split('.').count();
      }
    }
    std::hint::black_box(command_count);
  });
  
  parser_suite.benchmark("argument_parsing", move || {
    let mut arg_count = 0;
    for cmd in &workload_clone3 {
      // Simulate argument parsing
      arg_count += cmd.matches("::").count();
      arg_count += cmd.split_whitespace().count().saturating_sub(1);
    }
    std::hint::black_box(arg_count);
  });
  
  parser_suite.benchmark("full_parsing", move || {
    let mut parsed_count = 0;
    for cmd in &workload_clone4 {
      let _result = simulate_parse_command(cmd);
      parsed_count += 1;
    }
    std::hint::black_box(parsed_count);
  });

  let parser_results = parser_suite.run_analysis();
  let parser_report = parser_results.generate_markdown_report();
  
  // Generate parser-specific comprehensive report
  let comprehensive_report = generate_parser_report(&parser_workload, &parser_results);
  
  // Save parser report (temporary file with hyphen prefix)
  let report_path = "target/-unilang_parser_benchkit_report.md";
  std::fs::write(report_path, comprehensive_report)?;
  
  println!("  ✅ Parser comprehensive reporting:");
  println!("     - Report saved: {}", report_path);
  println!("     - Parser benchmarks: {} analyzed", parser_results.results.len());
  
  // Show parser-specific insights
  if let Some((fastest_stage, result)) = parser_results.results.iter()
    .max_by(|a, b| a.1.operations_per_second().partial_cmp(&b.1.operations_per_second()).unwrap()) 
  {
    println!("     - Fastest parsing stage: {} ({:.0} ops/sec)", fastest_stage, result.operations_per_second());
  }
  
  // Parser quality assessment
  let mut reliable_stages = 0;
  let total_stages = parser_results.results.len();
  
  for (stage, result) in &parser_results.results {
    let is_reliable = result.is_reliable();
    if is_reliable { reliable_stages += 1; }
    
    let cv = result.coefficient_of_variation() * 100.0;
    let status = if is_reliable { "✅" } else { "⚠️" };
    
    println!("     - {}: {} (CV: {:.1}%)", stage, status, cv);
  }
  
  println!("     - Parser reliability: {}/{} stages meet standards", reliable_stages, total_stages);
  
  println!();
  Ok(())
}

fn identify_parser_specific_features()
{
  println!("🔍 Parser-Specific Features Identified for benchkit");
  println!("===================================================");
  println!();
  
  println!("💡 Missing Features Needed for Parser Benchmarking:");
  println!();
  
  println!("1️⃣ **Parser Data Generation**");
  println!("   - Command syntax generators with realistic patterns");
  println!("   - Argument structure generation (positional, named, quoted)");
  println!("   - Nested command hierarchies");
  println!("   - Error case generation for parser robustness testing");
  println!("   - Batch command generation with separators");
  println!();
  
  println!("2️⃣ **Parser Performance Metrics**");
  println!("   - Commands per second (cmd/s) calculations");
  println!("   - Tokens per second processing rates");
  println!("   - Parse tree construction throughput");
  println!("   - Error handling performance impact");
  println!("   - Memory allocation per parse operation");
  println!();
  
  println!("3️⃣ **Parser-Specific Analysis**");
  println!("   - Tokenization vs parsing vs AST construction breakdown");
  println!("   - Command complexity impact analysis");
  println!("   - Argument count scaling characteristics");
  println!("   - Quoting/escaping performance overhead");
  println!("   - Batch vs individual parsing efficiency");
  println!();
  
  println!("4️⃣ **Parser Quality Metrics**");
  println!("   - Parse success rate tracking");
  println!("   - Error recovery performance");
  println!("   - Parser reliability under load");  
  println!("   - Memory leak detection in parsing pipeline");
  println!("   - Zero-copy optimization validation");
  println!();
  
  println!("5️⃣ **Parser Reporting Enhancements**");
  println!("   - Command pattern performance matrices");
  println!("   - Parser stage bottleneck identification");
  println!("   - Parsing throughput vs accuracy tradeoffs");
  println!("   - Comparative parser implementation analysis");
  println!("   - Real-world command distribution impact");
  println!();
  
  println!("6️⃣ **Integration Capabilities**");
  println!("   - AST validation benchmarks");
  println!("   - Parser configuration impact testing");
  println!("   - Error message generation performance");
  println!("   - Multi-threaded parsing coordination");
  println!("   - Stream parsing vs batch parsing analysis");
  println!();
  
  println!("🎯 **Implementation Priority:**");
  println!("   Phase 1: Parser data generation and command syntax generators");
  println!("   Phase 2: Parser-specific throughput metrics (cmd/s, tokens/s)");
  println!("   Phase 3: Parsing pipeline stage analysis and bottleneck detection");
  println!("   Phase 4: Parser reliability and quality metrics");
  println!("   Phase 5: Advanced parser reporting and comparative analysis");
  println!();
}

// Helper functions for parser simulation and data generation

fn simulate_parse_command(command: &str) -> usize
{
  // Simulate parsing by counting tokens and operations
  let tokens = command.split_whitespace().count();
  let named_args = command.matches("::").count();
  let quoted_parts = command.matches('"').count() / 2;
  
  // Simulate parsing work
  std::thread::sleep(std::time::Duration::from_nanos(tokens as u64 * 100 + named_args as u64 * 200));
  
  tokens + named_args + quoted_parts
}

fn simulate_batch_parse(batch_input: &str) -> usize
{
  let commands = batch_input.split(" ;; ");
  let mut total_operations = 0;
  
  for cmd in commands {
    total_operations += simulate_parse_command(cmd);
  }
  
  // Batch parsing has some efficiency benefits
  std::thread::sleep(std::time::Duration::from_nanos(total_operations as u64 * 80));
  
  total_operations
}

fn generate_nested_parser_commands(depth: usize, width: usize) -> String
{
  let mut commands = Vec::new();
  
  for i in 0..depth {
    for j in 0..width {
      let command = format!(
        "level{}.section{}.action{} param{}::value{} flag{}::true",
        i, j, (i + j) % 5, j, i + j, (i * j) % 3
      );
      commands.push(command);
    }
  }
  
  commands.join(" ;; ")
}

fn generate_parser_workload(count: usize) -> Vec<String>
{
  let patterns = [
    "simple.command",
    "user.create name::test email::test@example.com",
    "system.process.restart service::web graceful::true timeout::30",
    "report.generate format::pdf output::\"/tmp/report.pdf\" compress::true",
    "backup.database name::production exclude::[\"logs\",\"temp\"] compress::gzip",
    "notify.admin message::\"System maintenance\" priority::high channels::[\"email\",\"slack\"]",
    "log.rotate path::\"/var/log/app.log\" max_size::100MB keep::7 compress::true",
    "security.scan target::\"web_app\" depth::full report::detailed exclude::[\"assets\"]",
  ];
  
  (0..count)
    .map(|i| {
      let base_pattern = patterns[i % patterns.len()];
      format!("{} seq::{}", base_pattern, i)
    })
    .collect()
}

fn format_throughput_number(num: u64) -> String
{
  if num >= 1_000_000 {
    format!("{:.1}M", num as f64 / 1_000_000.0)
  } else if num >= 1_000 {
    format!("{:.1}K", num as f64 / 1_000.0)
  } else {
    format!("{}", num)
  }
}

fn generate_parser_report(workload: &[String], results: &SuiteResults) -> String
{
  let mut report = String::new();
  
  report.push_str("# unilang_parser Benchkit Integration Report\n\n");
  report.push_str("*Generated with benchkit parser-specific analysis*\n\n");
  
  report.push_str("## Executive Summary\n\n");
  report.push_str("This report demonstrates comprehensive benchkit integration with unilang_parser, ");
  report.push_str("showcasing parser-specific performance analysis capabilities and identifying ");
  report.push_str("additional features needed for parser benchmarking.\n\n");
  
  report.push_str(&format!("**Parser Workload Configuration:**\n"));
  report.push_str(&format!("- Commands tested: {}\n", workload.len()));
  report.push_str(&format!("- Total characters: {}\n", workload.iter().map(|s| s.len()).sum::<usize>()));
  report.push_str(&format!("- Average command length: {:.1} chars\n", 
                           workload.iter().map(|s| s.len()).sum::<usize>() as f64 / workload.len() as f64));
  report.push_str(&format!("- Parsing stages analyzed: {}\n\n", results.results.len()));
  
  report.push_str("## Parser Performance Results\n\n");
  let base_report = results.generate_markdown_report();
  report.push_str(&base_report.generate());
  
  report.push_str("## Parser-Specific Analysis\n\n");
  
  // Analyze parser stage performance
  if let Some((fastest_stage, fastest_result)) = results.results.iter()
    .max_by(|a, b| a.1.operations_per_second().partial_cmp(&b.1.operations_per_second()).unwrap())
  {
    report.push_str(&format!("**Fastest Parsing Stage**: {} ({:.0} ops/sec)\n\n", 
                             fastest_stage, fastest_result.operations_per_second()));
  }
  
  // Parser reliability assessment
  let mut reliable_stages = 0;
  let total_stages = results.results.len();
  
  for (stage, result) in &results.results {
    let is_reliable = result.is_reliable();
    if is_reliable { reliable_stages += 1; }
    
    let cv = result.coefficient_of_variation() * 100.0;
    let status = if is_reliable { "✅ Reliable" } else { "⚠️ Needs improvement" };
    
    report.push_str(&format!("- **{}**: {} (CV: {:.1}%, samples: {})\n",
                             stage, status, cv, result.times.len()));
  }
  
  report.push_str(&format!("\n**Parser Reliability**: {}/{} stages meet reliability standards\n\n",
                           reliable_stages, total_stages));
  
  report.push_str("## Parser-Specific Features Identified\n\n");
  report.push_str("### Missing benchkit Capabilities for Parsers\n\n");
  report.push_str("1. **Parser Data Generation**: Command syntax generators, argument patterns, error cases\n");
  report.push_str("2. **Parser Metrics**: Commands/sec, tokens/sec, parse tree throughput\n");
  report.push_str("3. **Pipeline Analysis**: Stage-by-stage performance breakdown\n");
  report.push_str("4. **Quality Metrics**: Success rates, error recovery, memory leak detection\n");
  report.push_str("5. **Parser Reporting**: Pattern matrices, bottleneck identification\n\n");
  
  report.push_str("## Integration Success\n\n");
  report.push_str("✅ **Parser benchmarking successfully integrated with benchkit**\n\n");
  report.push_str("**Key Achievements:**\n");
  report.push_str("- Comprehensive parser performance analysis\n");
  report.push_str("- Memory allocation tracking in parsing pipeline\n");
  report.push_str("- Statistical validation of parser performance\n");
  report.push_str("- Throughput analysis for parsing operations\n");
  report.push_str("- Professional parser benchmark reporting\n\n");
  
  report.push_str("**Recommendations:**\n");
  report.push_str("1. **Implement parser-specific data generators** for realistic command patterns\n");
  report.push_str("2. **Add parsing throughput metrics** (cmd/s, tokens/s) to benchkit\n");
  report.push_str("3. **Develop parser pipeline analysis** for bottleneck identification\n");
  report.push_str("4. **Integrate parser quality metrics** for reliability assessment\n");
  report.push_str("5. **Enhanced parser reporting** with command pattern analysis\n\n");
  
  report.push_str("---\n");
  report.push_str("*Report generated by benchkit parser integration analysis*\n");
  
  report
}