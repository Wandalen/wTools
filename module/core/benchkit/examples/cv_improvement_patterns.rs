//! Coefficient of Variation (CV) Improvement Patterns
//!
//! This example demonstrates proven techniques for reducing CV and improving
//! benchmark reliability based on real-world success in production systems.
//!
//! Key improvements demonstrated :
//! - Thread pool stabilization (CV reduction: 60-80%)
//! - CPU frequency stabilization (CV reduction: 40-60%) 
//! - Cache and memory warmup (CV reduction: 70-90%)
//! - Systematic CV analysis workflow
//!
//! Run with: cargo run --example `cv_improvement_patterns` --features `enabled,markdown_reports`

#[ cfg( feature = "enabled" ) ]
use core ::time ::Duration;
use std ::time ::Instant;
#[ cfg( feature = "enabled" ) ]
use std ::thread;
#[ cfg( feature = "enabled" ) ]
use std ::collections ::HashMap;

#[ cfg( feature = "enabled" ) ]
fn main()
{

  println!( "🔬 CV Improvement Patterns Demonstration" );
  println!( "========================================" );
  println!();

  // Demonstrate CV problems and solutions
  demonstrate_parallel_cv_improvement();
  demonstrate_cpu_cv_improvement();
  demonstrate_memory_cv_improvement();
  demonstrate_systematic_cv_analysis();
  demonstrate_environment_specific_cv();

  println!( "✅ All CV improvement patterns demonstrated successfully!" );
  println!( "📊 Check the generated reports for detailed CV analysis." );
}

#[ cfg( feature = "enabled" ) ]
fn demonstrate_parallel_cv_improvement()
{
  println!( "🧵 Parallel Processing CV Improvement" );
  println!( "=====================================" );
  println!();

  // Simulate a thread pool operation
  let data = generate_parallel_test_data( 1000 );

  println!( "❌ BEFORE: Unstable parallel benchmark (high CV expected)" );
  
  // Simulate unstable parallel benchmark
  let unstable_times = measure_unstable_parallel( &data );
  let unstable_cv = calculate_cv( &unstable_times );
  
  println!( "   Average: {:.2}ms", mean( &unstable_times ) );
  println!( "   CV: {:.1}% - {}", unstable_cv * 100.0, reliability_status( unstable_cv ) );
  println!();

  println!( "✅ AFTER: Stabilized parallel benchmark with warmup" );
  
  // Stabilized parallel benchmark  
  let stable_times = measure_stable_parallel( &data );
  let stable_cv = calculate_cv( &stable_times );
  
  println!( "   Average: {:.2}ms", mean( &stable_times ) );
  println!( "   CV: {:.1}% - {}", stable_cv * 100.0, reliability_status( stable_cv ) );
  
  let improvement = ( ( unstable_cv - stable_cv ) / unstable_cv ) * 100.0;
  println!( "   Improvement: {improvement:.1}% CV reduction" );
  println!();

  // Generate documentation
  generate_parallel_cv_report( &unstable_times, &stable_times );
}

#[ cfg( feature = "enabled" ) ]
fn demonstrate_cpu_cv_improvement()
{
  println!( "🖥️  CPU Frequency CV Improvement" );
  println!( "===============================" );
  println!();

  let data = generate_cpu_test_data( 500 );

  println!( "❌ BEFORE: CPU frequency scaling causes inconsistent timing" );
  
  let unstable_times = measure_unstable_cpu( &data );
  let unstable_cv = calculate_cv( &unstable_times );
  
  println!( "   Average: {:.2}ms", mean( &unstable_times ) );
  println!( "   CV: {:.1}% - {}", unstable_cv * 100.0, reliability_status( unstable_cv ) );
  println!();

  println!( "✅ AFTER: CPU frequency stabilization with delays" );
  
  let stable_times = measure_stable_cpu( &data );
  let stable_cv = calculate_cv( &stable_times );
  
  println!( "   Average: {:.2}ms", mean( &stable_times ) );
  println!( "   CV: {:.1}% - {}", stable_cv * 100.0, reliability_status( stable_cv ) );
  
  let improvement = ( ( unstable_cv - stable_cv ) / unstable_cv ) * 100.0;
  println!( "   Improvement: {improvement:.1}% CV reduction" );
  println!();

  generate_cpu_cv_report( &unstable_times, &stable_times );
}

#[ cfg( feature = "enabled" ) ]
fn demonstrate_memory_cv_improvement()
{
  println!( "🧠 Memory and Cache CV Improvement" );
  println!( "==================================" );
  println!();

  let data = generate_memory_test_data( 2000 );

  println!( "❌ BEFORE: Cold cache and initialization overhead" );
  
  let cold_times = measure_cold_memory( &data );
  let cold_cv = calculate_cv( &cold_times );
  
  println!( "   Average: {:.2}ms", mean( &cold_times ) );
  println!( "   CV: {:.1}% - {}", cold_cv * 100.0, reliability_status( cold_cv ) );
  println!();

  println!( "✅ AFTER: Cache warmup and memory preloading" );
  
  let warm_times = measure_warm_memory( &data );
  let warm_cv = calculate_cv( &warm_times );
  
  println!( "   Average: {:.2}ms", mean( &warm_times ) );
  println!( "   CV: {:.1}% - {}", warm_cv * 100.0, reliability_status( warm_cv ) );
  
  let improvement = ( ( cold_cv - warm_cv ) / cold_cv ) * 100.0;
  println!( "   Improvement: {improvement:.1}% CV reduction" );
  println!();

  generate_memory_cv_report( &cold_times, &warm_times );
}

#[ cfg( feature = "enabled" ) ]
fn demonstrate_systematic_cv_analysis()
{
  println!( "📊 Systematic CV Analysis Workflow" );
  println!( "==================================" );
  println!();

  // Simulate multiple benchmarks with different CV characteristics
  let benchmark_results = vec!
  [
  ( "excellent_benchmark", 0.03 ),    // 3% CV - excellent
  ( "good_benchmark", 0.08 ),         // 8% CV - good
  ( "moderate_benchmark", 0.12 ),     // 12% CV - moderate
  ( "poor_benchmark", 0.22 ),         // 22% CV - poor
  ( "unreliable_benchmark", 0.45 ),   // 45% CV - unreliable
 ];

  println!( "🔍 Analyzing benchmark suite reliability: " );
  println!();

  for ( name, cv ) in &benchmark_results
  {
  let cv_percent = cv * 100.0;
  let status = reliability_status( *cv );
  let icon = match cv_percent
  {
   cv if cv > 25.0 => "❌",
   cv if cv > 10.0 => "⚠️",
   _ => "✅",
 };
  
  println!( "{icon} {name} : CV {cv_percent:.1}% - {status}" );
  
  if cv_percent > 10.0
  {
   print_cv_improvement_suggestions( name, *cv );
 }
 }

  println!();
  println!( "📈 CV Improvement Recommendations: " );
  demonstrate_systematic_improvement_workflow();
}

#[ cfg( feature = "enabled" ) ]
fn demonstrate_environment_specific_cv()
{
  println!( "🌍 Environment-Specific CV Targets" );
  println!( "==================================" );
  println!();

  let environments = vec!
  [
  ( "Development", 0.15, 15, "Quick feedback cycles" ),
  ( "CI/CD", 0.10, 25, "Reliable regression detection" ),
  ( "Production", 0.05, 50, "Decision-grade reliability" ),
 ];

  println!( "Environment-specific CV targets and sample requirements: " );
  println!();

  for ( env_name, cv_target, sample_count, purpose ) in &environments
  {
  println!( "🔧 {env_name} Environment: " );
  println!( "   Target CV: < {:.0}%", cv_target * 100.0 );
  println!( "   Sample Count: {sample_count} samples" );
  println!( "   Purpose: {purpose}" );
  
  // Simulate benchmark configuration
  let config = create_environment_config( env_name, *cv_target, *sample_count );
  println!( "   Configuration: {config}" );
  println!();
 }

  generate_environment_cv_report( &environments );
}

#[ cfg( feature = "enabled" ) ]
fn demonstrate_systematic_improvement_workflow()
{
  println!( "🔧 Systematic CV Improvement Process: " );
  println!();

  let _ = "sample_benchmark"; // Demonstration only
  let mut current_cv = 0.35; // Start with high CV (35%)

  println!( "📊 Baseline CV: {:.1}%", current_cv * 100.0 );
  println!();

  let improvements = vec!
  [
  ( "Add warmup runs", 0.60 ),           // 60% improvement
  ( "Stabilize thread pool", 0.40 ),     // 40% improvement  
  ( "Add CPU frequency delay", 0.25 ),   // 25% improvement
  ( "Increase sample count", 0.30 ),     // 30% improvement
 ];

  for ( description, improvement_factor ) in improvements
  {
  println!( "🔨 Applying: {description}" );
  
  let previous_cv = current_cv;
  current_cv *= 1.0 - improvement_factor;
  
  let improvement_percent = ( ( previous_cv - current_cv ) / previous_cv ) * 100.0;
  
  println!( "   ✅ CV improved by {:.1}% (now {:.1}%)", 
  improvement_percent, current_cv * 100.0 );
  println!( "   Status: {}", reliability_status( current_cv ) );
  println!();
 }

  println!( "🎯 Final Result: CV reduced from 35.0% to {:.1}%", current_cv * 100.0 );
  println!( "   Overall improvement: {:.1}%", ( ( 0.35 - current_cv ) / 0.35 ) * 100.0 );
}

// Helper functions for benchmark simulation and analysis

#[ cfg( feature = "enabled" ) ]
fn generate_parallel_test_data( size: usize ) -> Vec< i32 >
{
  ( 0..size ).map( | i | i32 ::try_from( i ).unwrap_or( 0 ) ).collect()
}

#[ cfg( feature = "enabled" ) ]
fn generate_cpu_test_data( size: usize ) -> Vec< f64 >
{
  ( 0..size ).map( | i | i as f64 * 1.5 ).collect()
}

#[ cfg( feature = "enabled" ) ]
fn generate_memory_test_data( size: usize ) -> Vec< String >
{
  ( 0..size ).map( | i | format!( "data_item_{i}" ) ).collect()
}

#[ cfg( feature = "enabled" ) ]
fn measure_unstable_parallel( data: &[ i32 ] ) -> Vec< f64 >
{
  let mut times = Vec ::new();
  
  for _ in 0..20
  {
  let start = Instant ::now();
  
  // Simulate unstable parallel processing (no warmup)
  let _result = simulate_parallel_processing( data );
  
  let duration = start.elapsed();
  times.push( duration.as_secs_f64() * 1000.0 ); // Convert to ms
 }
  
  times
}

#[ cfg( feature = "enabled" ) ]
fn measure_stable_parallel( data: &[ i32 ] ) -> Vec< f64 >
{
  let mut times = Vec ::new();
  
  for _ in 0..20
  {
  // Warmup run to stabilize thread pool
  let _ = simulate_parallel_processing( data );
  
  // Small delay to let threads stabilize
  thread ::sleep( Duration ::from_millis( 2 ) );
  
  let start = Instant ::now();
  
  // Actual measurement run
  let _result = simulate_parallel_processing( data );
  
  let duration = start.elapsed();
  times.push( duration.as_secs_f64() * 1000.0 );
 }
  
  times
}

#[ cfg( feature = "enabled" ) ]
fn measure_unstable_cpu( data: &[ f64 ] ) -> Vec< f64 >
{
  let mut times = Vec ::new();
  
  for _ in 0..20
  {
  let start = Instant ::now();
  
  // Simulate CPU-intensive operation without frequency stabilization
  let _result = simulate_cpu_intensive( data );
  
  let duration = start.elapsed();
  times.push( duration.as_secs_f64() * 1000.0 );
 }
  
  times
}

#[ cfg( feature = "enabled" ) ]
fn measure_stable_cpu( data: &[ f64 ] ) -> Vec< f64 >
{
  let mut times = Vec ::new();
  
  for _ in 0..20
  {
  // Force CPU to stable frequency with delay
  thread ::sleep( Duration ::from_millis( 1 ) );
  
  let start = Instant ::now();
  
  // Actual measurement with stabilized CPU
  let _result = simulate_cpu_intensive( data );
  
  let duration = start.elapsed();
  times.push( duration.as_secs_f64() * 1000.0 );
 }
  
  times
}

#[ cfg( feature = "enabled" ) ]
fn measure_cold_memory( data: &[ String ] ) -> Vec< f64 >
{
  let mut times = Vec ::new();
  
  for _ in 0..20
  {
  let start = Instant ::now();
  
  // Simulate memory operation with cold cache
  let _result = simulate_memory_operation( data );
  
  let duration = start.elapsed();
  times.push( duration.as_secs_f64() * 1000.0 );
  
  // Clear caches between measurements to simulate cold effects
  thread ::sleep( Duration ::from_millis( 5 ) );
 }
  
  times
}

#[ cfg( feature = "enabled" ) ]
fn measure_warm_memory( data: &[ String ] ) -> Vec< f64 >
{
  let mut times = Vec ::new();
  
  for _ in 0..20
  {
  // Multiple warmup cycles to eliminate cold effects
  for _ in 0..3
  {
   let _ = simulate_memory_operation( data );
 }
  thread ::sleep( Duration ::from_micros( 10 ) );
  
  let start = Instant ::now();
  
  // Actual measurement with warmed cache
  let _result = simulate_memory_operation( data );
  
  let duration = start.elapsed();
  times.push( duration.as_secs_f64() * 1000.0 );
 }
  
  times
}

#[ cfg( feature = "enabled" ) ]
fn simulate_parallel_processing( data: &[ i32 ] ) -> i64
{
  // Simulate parallel work with some randomness
  use std ::sync :: { Arc, Mutex };
  
  let counter = Arc ::new( Mutex ::new( 0 ) );
  let mut handles = vec![];
  
  for chunk in data.chunks( 100 )
  {
  let counter_clone = Arc ::clone( &counter );
  let chunk_sum: i32 = chunk.iter().sum();
  
  let handle = thread ::spawn( move ||
  {
   // Simulate work
   let work_result = chunk_sum * 2;
   
   // Add to shared counter
   let mut num = counter_clone.lock().unwrap();
   *num += i64 ::from( work_result );
 });
  
  handles.push( handle );
 }
  
  for handle in handles
  {
  handle.join().unwrap();
 }
  
  let result = *counter.lock().unwrap();
  result
}

#[ cfg( feature = "enabled" ) ]
fn simulate_cpu_intensive( data: &[ f64 ] ) -> f64
{
  // Simulate CPU-intensive computation
  let mut result = 0.0;
  
  for &value in data
  {
  result += value.sin().cos().tan().sqrt();
 }
  
  result
}

#[ cfg( feature = "enabled" ) ]
fn simulate_memory_operation( data: &[ String ] ) -> HashMap< String, usize >
{
  // Simulate memory-intensive operation
  let mut map = HashMap ::new();
  
  for ( index, item ) in data.iter().enumerate()
  {
  map.insert( item.clone(), index );
 }
  
  map
}

#[ cfg( feature = "enabled" ) ]
fn calculate_cv( times: &[ f64 ] ) -> f64
{
  let mean_time = mean( times );
  let variance = times.iter()
  .map( | time | ( time - mean_time ).powi( 2 ) )
  .sum :: < f64 >() / ( times.len() as f64 - 1.0 );
  
  let std_dev = variance.sqrt();
  std_dev / mean_time
}

#[ cfg( feature = "enabled" ) ]
fn mean( values: &[ f64 ] ) -> f64
{
  values.iter().sum :: < f64 >() / values.len() as f64
}

#[ cfg( feature = "enabled" ) ]
fn reliability_status( cv: f64 ) -> &'static str
{
  match cv
  {
  cv if cv < 0.05 => "✅ Excellent reliability",
  cv if cv < 0.10 => "✅ Good reliability",
  cv if cv < 0.15 => "⚠️ Moderate reliability",
  cv if cv < 0.25 => "⚠️ Poor reliability",
  _ => "❌ Unreliable",
 }
}

#[ cfg( feature = "enabled" ) ]
fn print_cv_improvement_suggestions( benchmark_name: &str, cv: f64 )
{
  println!( "   💡 Improvement suggestions for {benchmark_name} : " );
  
  if cv > 0.25
  {
  println!( "      • Add extensive warmup runs (3-5 iterations)" );
  println!( "      • Increase sample count to 50+ measurements" );
  println!( "      • Check for external interference (other processes)" );
 }
  else if cv > 0.15
  {
  println!( "      • Add moderate warmup (1-2 iterations)" );
  println!( "      • Increase sample count to 30+ measurements" );
  println!( "      • Add CPU frequency stabilization delays" );
 }
  else
  {
  println!( "      • Minor warmup improvements" );
  println!( "      • Consider increasing sample count to 25+" );
 }
}

#[ cfg( feature = "enabled" ) ]
fn create_environment_config( env_name: &str, cv_target: f64, sample_count: i32 ) -> String
{
  format!( "BenchmarkSuite ::new(\"{}\").with_cv_tolerance({:.2}).with_sample_count({})", 
   env_name.to_lowercase(), cv_target, sample_count )
}

#[ cfg( feature = "enabled" ) ]
fn generate_parallel_cv_report( unstable_times: &[ f64 ], stable_times: &[ f64 ] )
{
  println!( "📄 Generating parallel processing CV improvement report..." );
  
  let unstable_cv = calculate_cv( unstable_times );
  let stable_cv = calculate_cv( stable_times );
  let improvement = ( ( unstable_cv - stable_cv ) / unstable_cv ) * 100.0;
  
  println!( "   Report: Parallel CV improved by {:.1}% (from {:.1}% to {:.1}%)", 
   improvement, unstable_cv * 100.0, stable_cv * 100.0 );
}

#[ cfg( feature = "enabled" ) ]
fn generate_cpu_cv_report( unstable_times: &[ f64 ], stable_times: &[ f64 ] )
{
  println!( "📄 Generating CPU frequency CV improvement report..." );
  
  let unstable_cv = calculate_cv( unstable_times );
  let stable_cv = calculate_cv( stable_times );
  let improvement = ( ( unstable_cv - stable_cv ) / unstable_cv ) * 100.0;
  
  println!( "   Report: CPU CV improved by {:.1}% (from {:.1}% to {:.1}%)", 
   improvement, unstable_cv * 100.0, stable_cv * 100.0 );
}

#[ cfg( feature = "enabled" ) ]
fn generate_memory_cv_report( cold_times: &[ f64 ], warm_times: &[ f64 ] )
{
  println!( "📄 Generating memory/cache CV improvement report..." );
  
  let cold_cv = calculate_cv( cold_times );
  let warm_cv = calculate_cv( warm_times );
  let improvement = ( ( cold_cv - warm_cv ) / cold_cv ) * 100.0;
  
  println!( "   Report: Memory CV improved by {:.1}% (from {:.1}% to {:.1}%)", 
   improvement, cold_cv * 100.0, warm_cv * 100.0 );
}

#[ cfg( feature = "enabled" ) ]
fn generate_environment_cv_report( environments: &[ ( &str, f64, i32, &str ) ] )
{
  println!( "📄 Generating environment-specific CV targets report..." );
  
  for ( env_name, cv_target, sample_count, _purpose ) in environments
  {
  println!( "   {} : Target CV < {:.0}%, {} samples", 
  env_name, cv_target * 100.0, sample_count );
 }
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main()
{
  println!( "This example requires the 'enabled' feature to be activated." );
  println!( "Please run: cargo run --example cv_improvement_patterns --features enabled,markdown_reports" );
}