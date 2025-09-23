//! Comprehensive test of parser-specific benchkit features
//!
//! This example validates that the new parser analysis and data generation
//! modules work correctly with realistic parsing scenarios.

#![allow(clippy ::format_push_string)]
#![allow(clippy ::uninlined_format_args)]
#![allow(clippy ::std_instead_of_core)]
#![allow(clippy ::unnecessary_wraps)]
#![allow(clippy ::useless_format)]
#![allow(clippy ::redundant_closure_for_method_calls)]
#![allow(clippy ::cast_possible_truncation)]
#![allow(clippy ::cast_sign_loss)]
#![allow(clippy ::needless_borrows_for_generic_args)]
#![allow(clippy ::doc_markdown)]

use benchkit ::prelude :: *;

type Result< T > = std ::result ::Result< T, Box<dyn std ::error ::Error >>;

fn main() -> Result< () >
{
  println!("🧪 Testing Parser-Specific Benchkit Features");
  println!("==========================================");
  println!();

  // Test 1 : Parser command generation
  test_parser_command_generation()?;
  
  // Test 2 : Parser analysis capabilities
  test_parser_analysis()?;
  
  // Test 3 : Parser pipeline analysis
  test_parser_pipeline_analysis()?;
  
  // Test 4 : Parser workload generation and analysis
  test_parser_workload_analysis()?;
  
  // Test 5 : Parser throughput with real scenarios
  test_parser_throughput_scenarios()?;

  println!("✅ All parser-specific tests completed successfully!");
  println!();
  
  Ok(())
}

fn test_parser_command_generation() -> Result< () >
{
  println!("1️⃣ Parser Command Generation Test");
  println!("-------------------------------");

  // Test basic command generation
  let generator = ParserCommandGenerator ::new()
  .complexity(CommandComplexity ::Standard)
  .max_arguments(3);

  let commands = generator.generate_commands(5);
  println!("  ✅ Generated {} standard commands: ", commands.len());
  for (i, cmd) in commands.iter().enumerate() 
  {
  println!("     {}. {}", i + 1, cmd);
 }

  // Test complexity variations
  let simple_gen = ParserCommandGenerator ::new().complexity(CommandComplexity ::Simple);
  let complex_gen = ParserCommandGenerator ::new().complexity(CommandComplexity ::Complex);
  
  let simple_cmd = simple_gen.generate_command(0);
  let complex_cmd = complex_gen.generate_command(0);
  
  println!("  📊 Complexity comparison: ");
  println!("     - Simple: {} ({} chars)", simple_cmd, simple_cmd.len());
  println!("     - Complex: {} ({} chars)", complex_cmd, complex_cmd.len());

  // Test error case generation
  let error_cases = generator.generate_error_cases(3);
  println!("  ⚠️  Error cases generated: ");
  for (i, err_case) in error_cases.iter().enumerate() 
  {
  println!("     {}. {}", i + 1, err_case);
 }

  // Test workload generation with statistics
  let mut workload = generator.generate_workload(50);
  workload.calculate_statistics();
  
  println!("  📈 Workload statistics: ");
  println!("     - Total commands: {}", workload.commands.len());
  println!("     - Average length: {:.1} chars", workload.average_command_length);
  println!("     - Error cases: {}", workload.error_case_count);

  println!();
  Ok(())
}

fn test_parser_analysis() -> Result< () >
{
  println!("2️⃣ Parser Analysis Test");
  println!("---------------------");

  // Create parser analyzer
  let analyzer = ParserAnalyzer ::new("test_parser", 1000, 25000)
  .with_complexity(2.5);

  // Simulate benchmark results
  let fast_times = vec![Duration ::from_micros(100); 10];
  let fast_result = BenchmarkResult ::new("fast_parser", fast_times);

  let slow_times = vec![Duration ::from_micros(300); 10];
  let slow_result = BenchmarkResult ::new("slow_parser", slow_times);

  // Analyze individual parser
  let metrics = analyzer.analyze(&fast_result);
  
  println!("  ✅ Parser metrics analysis: ");
  println!("     - Commands/sec: {}", metrics.commands_description());
  println!("     - Tokens/sec: {}", metrics.tokens_description());
  println!("     - Throughput: {}", metrics.throughput_description());

  // Compare multiple parsers
  let mut results = std ::collections ::HashMap ::new();
  results.insert("fast_implementation".to_string(), fast_result);
  results.insert("slow_implementation".to_string(), slow_result);

  let comparison = analyzer.compare_parsers(&results);
  
  if let Some((fastest_name, fastest_metrics)) = comparison.fastest_parser() 
  {
  println!("  🚀 Comparison results: ");
  println!("     - Fastest: {} ({})", fastest_name, fastest_metrics.commands_description());
 }

  if let Some(speedups) = comparison.calculate_speedups("slow_implementation") 
  {
  for (name, speedup) in speedups 
  {
   if name != "slow_implementation" 
   {
  println!("     - {} : {:.1}x faster", name, speedup);
 }
 }
 }

  println!();
  Ok(())
}

fn test_parser_pipeline_analysis() -> Result< () >
{
  println!("3️⃣ Parser Pipeline Analysis Test");
  println!("------------------------------");

  // Create pipeline analyzer
  let mut pipeline = ParserPipelineAnalyzer ::new();

  // Add realistic parser stages
  let tokenization_times = vec![Duration ::from_micros(50); 8];
  let parsing_times = vec![Duration ::from_micros(120); 8];
  let ast_times = vec![Duration ::from_micros(80); 8];
  let validation_times = vec![Duration ::from_micros(30); 8];

  pipeline
  .add_stage("tokenization", BenchmarkResult ::new("tokenization", tokenization_times))
  .add_stage("command_parsing", BenchmarkResult ::new("parsing", parsing_times))
  .add_stage("ast_construction", BenchmarkResult ::new("ast", ast_times))
  .add_stage("validation", BenchmarkResult ::new("validation", validation_times));

  // Analyze bottlenecks
  let analysis = pipeline.analyze_bottlenecks();

  println!("  ✅ Pipeline analysis results: ");
  println!("     - Total stages: {}", analysis.stage_count);
  println!("     - Total time: {:.2?}", analysis.total_time);

  if let Some((bottleneck_name, bottleneck_time)) = &analysis.bottleneck 
  {
  println!("     - Bottleneck: {} ({:.2?})", bottleneck_name, bottleneck_time);
  
  if let Some(percentage) = analysis.stage_percentages.get(bottleneck_name) 
  {
   println!("     - Impact: {:.1}% of total time", percentage);
 }
 }

  // Show stage breakdown
  println!("  📊 Stage breakdown: ");
  for (stage, time) in &analysis.stage_times 
  {
  if let Some(percentage) = analysis.stage_percentages.get(stage) 
  {
   println!("     - {} : {:.2?} ({:.1}%)", stage, time, percentage);
 }
 }

  println!();
  Ok(())
}

fn test_parser_workload_analysis() -> Result< () >
{
  println!("4️⃣ Parser Workload Analysis Test");
  println!("------------------------------");

  // Generate realistic parser workload
  let generator = ParserCommandGenerator ::new()
  .complexity(CommandComplexity ::Standard)
  .with_pattern(ArgumentPattern ::Named)
  .with_pattern(ArgumentPattern ::Quoted)
  .with_pattern(ArgumentPattern ::Array);

  let mut workload = generator.generate_workload(200);
  workload.calculate_statistics();

  println!("  ✅ Workload generation: ");
  println!("     - Commands: {}", workload.commands.len());
  println!("     - Characters: {}", workload.total_characters);
  println!("     - Avg length: {:.1} chars/cmd", workload.average_command_length);

  // Show complexity distribution
  println!("  📈 Complexity distribution: ");
  for (complexity, count) in &workload.complexity_distribution 
  {
  let percentage = *count as f64 / (workload.commands.len() - workload.error_case_count) as f64 * 100.0;
  println!("     - {:?} : {} ({:.1}%)", complexity, count, percentage);
 }

  // Show sample commands
  println!("  📝 Sample commands: ");
  let samples = workload.sample_commands(3);
  for (i, cmd) in samples.iter().enumerate() 
  {
  println!("     {}. {}", i + 1, cmd);
 }

  println!();
  Ok(())
}

fn test_parser_throughput_scenarios() -> Result< () >
{
  println!("5️⃣ Parser Throughput Scenarios Test");
  println!("----------------------------------");

  // Generate different command types for throughput testing
  let simple_commands = ParserCommandGenerator ::new()
  .complexity(CommandComplexity ::Simple)
  .generate_commands(100);

  let complex_commands = ParserCommandGenerator ::new()
  .complexity(CommandComplexity ::Complex)
  .generate_commands(100);

  // Calculate workload characteristics
  let simple_chars: usize = simple_commands.iter().map(|s| s.len()).sum();
  let complex_chars: usize = complex_commands.iter().map(|s| s.len()).sum();

  println!("  📊 Workload characteristics: ");
  println!("     - Simple commands: {} chars total, {:.1} avg", 
   simple_chars, simple_chars as f64 / simple_commands.len() as f64);
  println!("     - Complex commands: {} chars total, {:.1} avg", 
   complex_chars, complex_chars as f64 / complex_commands.len() as f64);

  // Simulate throughput analysis for different scenarios
  let simple_analyzer = ThroughputAnalyzer ::new("simple_parser", simple_chars as u64)
  .with_items(simple_commands.len() as u64);

  let complex_analyzer = ThroughputAnalyzer ::new("complex_parser", complex_chars as u64)
  .with_items(complex_commands.len() as u64);

  // Create mock results for different parser performance scenarios
  let mut simple_results = std ::collections ::HashMap ::new();
  simple_results.insert("optimized".to_string(), 
   BenchmarkResult ::new("opt", vec![Duration ::from_micros(200); 5]));
  simple_results.insert("standard".to_string(),
   BenchmarkResult ::new("std", vec![Duration ::from_micros(500); 5]));

  let mut complex_results = std ::collections ::HashMap ::new();
  complex_results.insert("optimized".to_string(),
  BenchmarkResult ::new("opt", vec![Duration ::from_micros(800); 5]));
  complex_results.insert("standard".to_string(),
  BenchmarkResult ::new("std", vec![Duration ::from_micros(1500); 5]));

  // Analyze throughput
  let simple_comparison = simple_analyzer.compare_throughput(&simple_results);
  let complex_comparison = complex_analyzer.compare_throughput(&complex_results);

  println!("  ⚡ Throughput analysis results: ");

  if let Some((name, metrics)) = simple_comparison.fastest_throughput() 
  {
  println!("     - Simple commands fastest: {} ({})", name, metrics.throughput_description());
  if let Some(items_desc) = metrics.items_description() 
  {
   println!("       Command rate: {}", items_desc);
 }
 }

  if let Some((name, metrics)) = complex_comparison.fastest_throughput() 
  {
  println!("     - Complex commands fastest: {} ({})", name, metrics.throughput_description());
  if let Some(items_desc) = metrics.items_description() 
  {
   println!("       Command rate: {}", items_desc);
 }
 }

  // Calculate speedups
  if let Some(simple_speedups) = simple_comparison.calculate_speedups("standard") 
  {
  if let Some(speedup) = simple_speedups.get("optimized") 
  {
   println!("     - Simple command speedup: {:.1}x", speedup);
 }
 }

  if let Some(complex_speedups) = complex_comparison.calculate_speedups("standard") 
  {
  if let Some(speedup) = complex_speedups.get("optimized") 
  {
   println!("     - Complex command speedup: {:.1}x", speedup);
 }
 }

  println!();
  Ok(())
}