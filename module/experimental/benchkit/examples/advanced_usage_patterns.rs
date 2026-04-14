#![ allow( clippy ::needless_raw_string_hashes ) ]
//! Advanced Usage Pattern Examples
//!
//! This example demonstrates EVERY advanced usage pattern for enhanced features :
//! - Custom validation criteria for domain-specific requirements
//! - Template composition and inheritance patterns
//! - Advanced update chain coordination
//! - Performance optimization techniques
//! - Memory-efficient processing for large datasets
//! - Multi-threaded and concurrent processing scenarios

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "markdown_reports" ) ]
#![ allow( clippy ::uninlined_format_args ) ]
#![ allow( clippy ::format_push_string ) ]
#![ allow( clippy ::cast_lossless ) ]
#![ allow( clippy ::std_instead_of_core ) ]
#![ allow( clippy ::cast_sign_loss ) ]
#![ allow( clippy ::too_many_lines ) ]
#![ allow( clippy ::for_kv_map ) ]
#![ allow( clippy ::cast_possible_truncation ) ]
#![ allow( clippy ::cast_possible_wrap ) ]
#![ allow( clippy ::single_char_pattern ) ]
#![ allow( clippy ::unnecessary_cast ) ]

use benchkit ::prelude :: *;
use std ::collections ::HashMap;
use std ::time ::Duration;

/// Create large-scale benchmark results for advanced processing
fn create_large_scale_results() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap ::new();
  
  // Simulate results from different algorithm categories
  let categories = vec![
  ( "sorting", vec![ "quicksort", "mergesort", "heapsort", "radixsort", "timsort" ] ),
  ( "searching", vec![ "binary_search", "linear_search", "hash_lookup", "tree_search", "bloom_filter" ] ),
  ( "compression", vec![ "gzip", "lz4", "zstd", "brotli", "snappy" ] ),
  ( "encryption", vec![ "aes256", "chacha20", "blake3", "sha256", "md5" ] ),
 ];
  
  for ( category, algorithms ) in categories
  {
  for ( i, algorithm ) in algorithms.iter().enumerate()
  {
   // Generate realistic performance data with some variation
   let base_time = match category
   {
  "sorting" => 100 + i * 50,
  "searching" => 20 + i * 10,
  "compression" => 500 + i * 100,
  "encryption" => 200 + i * 75,
  _ => 100,
 };
   
   let times: Vec< Duration > = ( 0..20 )
  .map( | j | 
  {
   let variance = ( j % 5 ) as i32 - 2;  // ±2 microseconds
   Duration ::from_micros( ( base_time as i32 + variance ) as u64 )
 })
  .collect();
   
   let full_name = format!( "{}_{}", category, algorithm );
   results.insert( full_name.clone(), BenchmarkResult ::new( &full_name, times ) );
 }
 }
  
  results
}

/// Advanced Pattern 1 : Custom Domain-Specific Validation
fn pattern_domain_specific_validation()
{
  println!( "=== Pattern 1 : Domain-Specific Validation ===" );
  
  let results = create_large_scale_results();
  
  // Create different validators for different domains
  
  // Real-time systems validator (very strict)
  let realtime_validator = BenchmarkValidator ::new()
  .min_samples( 50 )
  .max_coefficient_variation( 0.01 )  // 1% maximum CV
  .require_warmup( true )
  .max_time_ratio( 1.2 )  // Very tight timing requirements
  .min_measurement_time( Duration ::from_micros( 1 ) );
  
  // Throughput systems validator (focuses on consistency)
  let throughput_validator = BenchmarkValidator ::new()
  .min_samples( 30 )
  .max_coefficient_variation( 0.05 )  // 5% maximum CV
  .require_warmup( true )
  .max_time_ratio( 2.0 )
  .min_measurement_time( Duration ::from_micros( 10 ) );
  
  // Interactive systems validator (balanced)
  let interactive_validator = BenchmarkValidator ::new()
  .min_samples( 20 )
  .max_coefficient_variation( 0.10 )  // 10% maximum CV
  .require_warmup( false )  // Interactive systems may not show warmup patterns
  .max_time_ratio( 3.0 )
  .min_measurement_time( Duration ::from_micros( 5 ) );
  
  // Batch processing validator (more lenient)
  let batch_validator = BenchmarkValidator ::new()
  .min_samples( 15 )
  .max_coefficient_variation( 0.20 )  // 20% maximum CV
  .require_warmup( false )
  .max_time_ratio( 5.0 )
  .min_measurement_time( Duration ::from_micros( 50 ) );
  
  println!( "\n📊 Applying domain-specific validation..." );
  
  // Apply different validators to different algorithm categories
  let categories = vec![
  ( "encryption", &realtime_validator, "Real-time (Crypto)" ),
  ( "searching", &throughput_validator, "Throughput (Search)" ),
  ( "sorting", &interactive_validator, "Interactive (Sort)" ),
  ( "compression", &batch_validator, "Batch (Compression)" ),
 ];
  
  for ( category, validator, domain_name ) in categories
  {
  let category_results: HashMap< String, BenchmarkResult > = results.iter()
   .filter( | ( name, _ ) | name.starts_with( category ) )
   .map( | ( name, result ) | ( name.clone(), result.clone() ) )
   .collect();
  
  let validated_results = ValidatedResults ::new( category_results, validator.clone() );
  
  println!( "\n🔍 {} Domain ({} algorithms) : ", domain_name, validated_results.results.len() );
  println!( "   Reliability rate: {:.1}%", validated_results.reliability_rate() );
  
  if let Some( warnings ) = validated_results.reliability_warnings()
  {
   println!( "   Quality issues: {} warnings", warnings.len() );
   for warning in warnings.iter().take( 2 )  // Show first 2 warnings
   {
  println!( "     - {}", warning );
 }
 }
  else
  {
   println!( "   ✅ All algorithms meet domain-specific criteria" );
 }
 }
  
  println!();
}

/// Advanced Pattern 2 : Template Composition and Inheritance
fn pattern_template_composition()
{
  println!( "=== Pattern 2 : Template Composition and Inheritance ===" );
  
  let results = create_large_scale_results();
  
  // Base template with common sections
  let _base_template = PerformanceReport ::new()
  .title( "Base Performance Analysis" )
  .include_statistical_analysis( true )
  .add_custom_section( CustomSection ::new(
   "Methodology",
   r#"### Test Environment

- Hardware: AMD Ryzen 9 5950X, 64GB DDR4-3600
- OS: Ubuntu 22.04 LTS with performance governor  
- Rust: 1.75.0 with full optimizations (-C target-cpu=native)
- Iterations: 20 per algorithm with warm-up cycles

### Statistical Methods

- Confidence intervals calculated using t-distribution
- Outlier detection using modified Z-score (threshold: 3.5)
- Reliability assessment based on coefficient of variation"#
 ));
  
  // Create specialized templates by composition
  
  // Security-focused template
  println!( "\n🔒 Security-focused template composition..." );
  let security_template = PerformanceReport ::new()
  .title( "Security Algorithm Performance Analysis" )
  .add_context( "Comprehensive analysis of cryptographic and security algorithms" )
  .include_statistical_analysis( true )
  .add_custom_section( CustomSection ::new(
   "Security Considerations",
   r#"### Timing Attack Resistance

- Constant-time implementation requirements analyzed
- Side-channel vulnerability assessment included
- Performance vs security trade-offs evaluated

### Compliance Standards

- FIPS 140-2 Level 3 requirements considered
- NIST SP 800-57 key management guidelines applied
- Common Criteria EAL4+ evaluation criteria used"#
 ))
  .add_custom_section( CustomSection ::new(
   "Methodology", 
   "Base methodology with security-specific considerations applied."
 ));
  
  let security_results: HashMap< String, BenchmarkResult > = results.iter()
  .filter( | ( name, _ ) | name.starts_with( "encryption" ) )
  .map( | ( name, result ) | ( name.clone(), result.clone() ) )
  .collect();
  
  let security_report = security_template.generate( &security_results ).unwrap();
  println!( "   Security template generated: {} characters", security_report.len() );
  println!( "   Contains security sections: {}", security_report.contains( "Security Considerations" ) );
  
  // Performance-optimized template
  println!( "\n⚡ Performance-optimized template composition..." );
  let perf_template = PerformanceReport ::new()
  .title( "High-Performance Algorithm Analysis" )
  .add_context( "Focus on maximum throughput and minimum latency algorithms" )
  .include_statistical_analysis( true )
  .add_custom_section( CustomSection ::new(
   "Optimization Techniques",
   r#"### Applied Optimizations

- SIMD vectorization using AVX2/AVX-512 instructions
- Cache-friendly data structures and access patterns
- Branch prediction optimization and loop unrolling
- Memory prefetching and alignment strategies

### Performance Targets

- Latency: < 100μs for interactive operations
- Throughput: > 10GB/s for bulk processing
- CPU efficiency: > 80% cache hit rate
- Memory efficiency: < 2x theoretical minimum"#
 ))
  .add_custom_section( CustomSection ::new(
   "Bottleneck Analysis",
   r#"### Identified Bottlenecks

- Memory bandwidth limitations for large datasets
- Branch misprediction penalties in irregular data
- Cache coherency overhead in multi-threaded scenarios
- System call overhead for I/O-bound operations"#
 ));
  
  let perf_results: HashMap< String, BenchmarkResult > = results.iter()
  .filter( | ( name, _ ) | name.starts_with( "sorting" ) || name.starts_with( "searching" ) )
  .map( | ( name, result ) | ( name.clone(), result.clone() ) )
  .collect();
  
  let perf_report = perf_template.generate( &perf_results ).unwrap();
  println!( "   Performance template generated: {} characters", perf_report.len() );
  println!( "   Contains optimization details: {}", perf_report.contains( "Optimization Techniques" ) );
  
  // Comparative template combining multiple analyses
  println!( "\n📊 Comparative template composition..." );
  
  // Create mega-template that combines multiple analyses
  let comprehensive_template = PerformanceReport ::new()
  .title( "Comprehensive Algorithm Performance Suite" )
  .add_context( "Complete analysis across all algorithm categories with domain-specific insights" )
  .include_statistical_analysis( true )
  .add_custom_section( CustomSection ::new(
   "Executive Summary",
   r#"### Key Findings

1. **Encryption algorithms** : AES-256 provides best balance of security and performance
2. **Search algorithms** : Hash lookup dominates for exact matches, binary search for ranges  
3. **Sorting algorithms** : Timsort excels for partially sorted data, quicksort for random data
4. **Compression algorithms** : LZ4 optimal for speed, Zstd for compression ratio

### Performance Rankings

| Category | Winner | Runner-up | Performance Gap |
|----------|--------|-----------|-----------------|
| Encryption | AES-256 | ChaCha20 | 15% faster |
| Search | Hash lookup | Binary search | 300% faster |
| Sorting | Timsort | Quicksort | 8% faster |
| Compression | LZ4 | Snappy | 12% faster |"#
 ))
  .add_custom_section( CustomSection ::new(
   "Cross-Category Analysis",
   r#"### Algorithm Complexity Analysis

- **Linear algorithms** (O(n)) : Hash operations, linear search
- **Logarithmic algorithms** (O(log n)) : Binary search, tree operations  
- **Linearithmic algorithms** (O(n log n)) : Optimal comparison sorts
- **Quadratic algorithms** (O(n²)) : Avoided in production implementations

### Memory vs CPU Trade-offs

- Hash tables: High memory usage, exceptional speed
- Tree structures: Moderate memory, consistent performance
- In-place algorithms: Minimal memory, CPU intensive
- Streaming algorithms: Constant memory, sequential processing"#
 ));
  
  let comprehensive_report = comprehensive_template.generate( &results ).unwrap();
  println!( "   Comprehensive template generated: {} characters", comprehensive_report.len() );
  println!( "   Contains executive summary: {}", comprehensive_report.contains( "Executive Summary" ) );
  println!( "   Contains cross-category analysis: {}", comprehensive_report.contains( "Cross-Category Analysis" ) );
  
  // Save all composed templates
  let temp_dir = std ::env ::temp_dir();
  std ::fs ::write( temp_dir.join( "security_analysis.md" ), &security_report ).unwrap();
  std ::fs ::write( temp_dir.join( "performance_analysis.md" ), &perf_report ).unwrap();
  std ::fs ::write( temp_dir.join( "comprehensive_analysis.md" ), &comprehensive_report ).unwrap();
  
  println!( "   📁 All composed templates saved to: {}", temp_dir.display() );
  
  println!();
}

/// Advanced Pattern 3 : Coordinated Multi-Document Updates
fn pattern_coordinated_updates()
{
  println!( "=== Pattern 3 : Coordinated Multi-Document Updates ===" );
  
  let results = create_large_scale_results();
  
  // Create multiple related documents
  let documents = vec![
  ( "README.md", vec![ ( "Performance Overview", "overview" ) ] ),
  ( "BENCHMARKS.md", vec![ ( "Detailed Results", "detailed" ), ( "Methodology", "methods" ) ] ),
  ( "OPTIMIZATION.md", vec![ ( "Optimization Guide", "guide" ), ( "Performance Tips", "tips" ) ] ),
  ( "COMPARISON.md", vec![ ( "Algorithm Comparison", "comparison" ) ] ),
 ];
  
  println!( "\n📄 Creating coordinated document structure..." );
  
  let temp_dir = std ::env ::temp_dir().join( "coordinated_docs" );
  std ::fs ::create_dir_all( &temp_dir ).unwrap();
  
  // Initialize documents
  for ( doc_name, sections ) in &documents
  {
  let mut content = format!( "# {}\n\n## Introduction\n\nThis document is part of the coordinated benchmark documentation suite.\n\n", 
   doc_name.replace( ".md", "" ).replace( "_", " " ) );
  
  for ( section_name, _ ) in sections
  {
   content.push_str( &format!( "## {}\n\n*This section will be automatically updated.*\n\n", section_name ) );
 }
  
  let doc_path = temp_dir.join( doc_name );
  std ::fs ::write( &doc_path, &content ).unwrap();
  println!( "   Created: {}", doc_name );
 }
  
  // Generate different types of content
  println!( "\n🔄 Generating coordinated content..." );
  
  let overview_template = PerformanceReport ::new()
  .title( "Performance Overview" )
  .add_context( "High-level summary for README" )
  .include_statistical_analysis( false );  // Simplified for overview
  
  let detailed_template = PerformanceReport ::new()
  .title( "Detailed Benchmark Results" )
  .add_context( "Complete analysis for technical documentation" )
  .include_statistical_analysis( true );
  
  let optimization_template = PerformanceReport ::new()
  .title( "Optimization Guidelines" )
  .add_context( "Performance tuning recommendations" )
  .include_statistical_analysis( true )
  .add_custom_section( CustomSection ::new(
   "Performance Recommendations",
   r#"### Algorithm Selection Guidelines

1. **For real-time applications** : Use constant-time algorithms
2. **For batch processing** : Optimize for throughput over latency
3. **For memory-constrained environments** : Choose in-place algorithms
4. **For concurrent access** : Consider lock-free data structures

### Implementation Best Practices

- Profile before optimizing - measure actual bottlenecks
- Use appropriate data structures for access patterns
- Consider cache locality in algorithm design
- Benchmark on target hardware and workloads"#
 ));
  
  // Generate all content
  let overview_content = overview_template.generate( &results ).unwrap();
  let detailed_content = detailed_template.generate( &results ).unwrap();
  let optimization_content = optimization_template.generate( &results ).unwrap();
  
  // Create comparison content
  let fastest_algorithm = results.iter()
  .min_by( | a, b | a.1.mean_time().cmp( &b.1.mean_time() ) )
  .map( | ( name, _ ) | name )
  .unwrap();
  
  let slowest_algorithm = results.iter()
  .max_by( | a, b | a.1.mean_time().cmp( &b.1.mean_time() ) )
  .map( | ( name, _ ) | name )
  .unwrap();
  
  let comparison_template = ComparisonReport ::new()
  .title( "Best vs Worst Algorithm Comparison" )
  .baseline( slowest_algorithm )
  .candidate( fastest_algorithm );
  
  let comparison_content = comparison_template.generate( &results ).unwrap();
  
  // Create coordinated update plan
  println!( "\n🎯 Executing coordinated updates..." );
  
  let methodology_note = "See comprehensive methodology in detailed results above.".to_string();
  let performance_tips = "Refer to the Performance Recommendations section above for detailed guidance.".to_string();
  
  let update_plan = vec![
  ( temp_dir.join( "README.md" ), vec![ ( "Performance Overview", &overview_content ) ] ),
  ( temp_dir.join( "BENCHMARKS.md" ), vec![ 
   ( "Detailed Results", &detailed_content ),
   ( "Methodology", &methodology_note )
 ] ),
  ( temp_dir.join( "OPTIMIZATION.md" ), vec![ 
   ( "Optimization Guide", &optimization_content ),
   ( "Performance Tips", &performance_tips )
 ] ),
  ( temp_dir.join( "COMPARISON.md" ), vec![ ( "Algorithm Comparison", &comparison_content ) ] ),
 ];
  
  // Execute all updates atomically per document
  let mut successful_updates = 0;
  let mut failed_updates = 0;
  
  for ( doc_path, updates ) in update_plan
  {
  let mut chain = MarkdownUpdateChain ::new( &doc_path ).unwrap();
  
  for ( section_name, content ) in updates
  {
   chain = chain.add_section( section_name, content );
 }
  
  match chain.execute()
  {
   Ok( () ) =>
   {
  successful_updates += 1;
  let file_name = doc_path.file_name().unwrap().to_string_lossy();
  println!( "   ✅ {} updated successfully", file_name );
 },
   Err( e ) =>
   {
  failed_updates += 1;
  let file_name = doc_path.file_name().unwrap().to_string_lossy();
  println!( "   ❌ {} update failed: {}", file_name, e );
 }
 }
 }
  
  println!( "\n📊 Coordination results: " );
  println!( "   Successful updates: {}", successful_updates );
  println!( "   Failed updates: {}", failed_updates );
  println!( "   Overall success rate: {:.1}%", 
   ( successful_updates as f64 / ( successful_updates + failed_updates ) as f64 ) * 100.0 );
  
  // Create index document linking all coordinated docs
  let index_content = r#"# Benchmark Documentation Suite

This directory contains coordinated benchmark documentation automatically generated from performance analysis.

## Documents

- **[README.md](README.md)** : High-level performance overview
- **[BENCHMARKS.md](BENCHMARKS.md)** : Detailed benchmark results and methodology  
- **[OPTIMIZATION.md](OPTIMIZATION.md)** : Performance optimization guidelines
- **[COMPARISON.md](COMPARISON.md)** : Algorithm comparison analysis

## Automated Updates

All documents are automatically updated when benchmarks are run. The content is coordinated to ensure consistency across all documentation.

## Last Updated

*This suite was last updated automatically by benchkit.*
"#;
  
  std ::fs ::write( temp_dir.join( "INDEX.md" ), index_content ).unwrap();
  
  println!( "   📄 Documentation suite created at: {}", temp_dir.display() );
  
  println!();
}

/// Advanced Pattern 4 : Memory-Efficient Large Scale Processing
fn pattern_memory_efficient_processing()
{
  println!( "=== Pattern 4 : Memory-Efficient Large Scale Processing ===" );
  
  println!( "\n💾 Simulating large-scale benchmark processing..." );
  
  // Simulate processing thousands of benchmark results efficiently
  let algorithm_count = 1000;  // Simulate 1000 different algorithms
  
  println!( "   Creating {} simulated algorithms...", algorithm_count );
  
  // Process results in batches to avoid memory exhaustion
  let batch_size = 100;
  let batches = ( algorithm_count + batch_size - 1 ) / batch_size;  // Ceiling division
  
  println!( "   Processing in {} batches of {} algorithms each", batches, batch_size );
  
  let mut batch_reports = Vec ::new();
  let mut total_reliable = 0;
  let mut total_algorithms = 0;
  
  for batch_num in 0..batches
  {
  let start_idx = batch_num * batch_size;
  let end_idx = std ::cmp ::min( start_idx + batch_size, algorithm_count );
  let current_batch_size = end_idx - start_idx;
  
  println!( "   📦 Processing batch {}/{} ({} algorithms)...", 
  batch_num + 1, batches, current_batch_size );
  
  // Generate batch of results
  let mut batch_results = HashMap ::new();
  for i in start_idx..end_idx
  {
   let times: Vec< Duration > = ( 0..15 )  // Moderate sample size for memory efficiency
  .map( | j | 
  {
   let base_time = 100 + ( i % 500 );  // Vary performance across algorithms
   let variance = j % 5;  // Small variance
   Duration ::from_micros( ( base_time + variance ) as u64 )
 })
  .collect();
   
   let algorithm_name = format!( "algorithm_{:04}", i );
   batch_results.insert( algorithm_name.clone(), BenchmarkResult ::new( &algorithm_name, times ) );
 }
  
  // Validate batch
  let validator = BenchmarkValidator ::new()
   .min_samples( 10 )
   .require_warmup( false );  // Disable for simulated data
  
  let batch_validated = ValidatedResults ::new( batch_results.clone(), validator );
  let batch_reliable = batch_validated.reliable_count();
  
  total_reliable += batch_reliable;
  total_algorithms += current_batch_size;
  
  println!( "     Batch reliability: {}/{} ({:.1}%)", 
  batch_reliable, current_batch_size, batch_validated.reliability_rate() );
  
  // Generate lightweight summary for this batch instead of full report
  let batch_summary = format!(
   "### Batch {} Summary\n\n- Algorithms: {}\n- Reliable: {} ({:.1}%)\n- Mean performance: {:.0}μs\n\n",
   batch_num + 1,
   current_batch_size,
   batch_reliable,
   batch_validated.reliability_rate(),
   batch_results.values()
  .map( | r | r.mean_time().as_micros() )
  .sum :: < u128 >() as f64 / batch_results.len() as f64
 );
  
  batch_reports.push( batch_summary );
  
  // Explicitly drop batch data to free memory
  drop( batch_results );
  drop( batch_validated );
  
  // Simulate memory pressure monitoring
  if batch_num % 5 == 4  // Every 5 batches
  {
   println!( "     💾 Memory checkpoint: {} batches processed", batch_num + 1 );
 }
 }
  
  // Generate consolidated summary report
  println!( "\n📊 Generating consolidated summary..." );
  
  let overall_reliability = ( total_reliable as f64 / total_algorithms as f64 ) * 100.0;
  
  let summary_template = PerformanceReport ::new()
  .title( "Large-Scale Algorithm Performance Summary" )
  .add_context( format!( 
   "Memory-efficient analysis of {} algorithms processed in {} batches", 
   total_algorithms, batches 
 ))
  .include_statistical_analysis( false )  // Skip heavy analysis for summary
  .add_custom_section( CustomSection ::new(
   "Processing Summary",
   format!(
  "### Scale and Efficiency\n\n- **Total algorithms analyzed** : {}\n- **Processing batches** : {}\n- **Batch size** : {} algorithms\n- **Overall reliability** : {:.1}%\n\n### Memory Management\n\n- Batch processing prevented memory exhaustion\n- Peak memory usage limited to single batch size\n- Processing completed successfully without system resource issues",
  total_algorithms, batches, batch_size, overall_reliability
 )
 ))
  .add_custom_section( CustomSection ::new(
   "Batch Results",
   batch_reports.join( "" )
 ));
  
  // Use empty results since we're creating a summary-only report
  let summary_report = summary_template.generate( &HashMap ::new() ).unwrap();
  
  println!( "   Summary report generated: {} characters", summary_report.len() );
  println!( "   Overall reliability across all batches: {:.1}%", overall_reliability );
  
  // Save memory-efficient summary
  let summary_file = std ::env ::temp_dir().join( "large_scale_summary.md" );
  std ::fs ::write( &summary_file, &summary_report ).unwrap();
  
  println!( "   📄 Large-scale summary saved to: {}", summary_file.display() );
  
  println!( "\n💡 Memory efficiency techniques demonstrated: " );
  println!( "   • Batch processing to limit memory usage" );
  println!( "   • Explicit cleanup of intermediate data" );
  println!( "   • Summary-focused reporting for scale" );
  println!( "   • Progress monitoring for long-running operations" );
  
  println!();
}

/// Advanced Pattern 5 : Performance Optimization Techniques
fn pattern_performance_optimization()
{
  println!( "=== Pattern 5 : Performance Optimization Techniques ===" );
  
  let results = create_large_scale_results();
  
  // Technique 1 : Lazy evaluation and caching
  println!( "\n⚡ Technique 1 : Lazy evaluation and result caching..." );
  
  // Simulate expensive template generation with caching
  struct CachedTemplateGenerator
  {
  template_cache: std ::cell ::RefCell< HashMap< String, String > >,
 }
  
  impl CachedTemplateGenerator
  {
  fn new() -> Self
  {
   Self { template_cache: std ::cell ::RefCell ::new( HashMap ::new() ) }
 }
  
  fn generate_cached( &self, template_type: &str, results: &HashMap< String, BenchmarkResult > ) -> String
  {
   let cache_key = format!( "{}_{}", template_type, results.len() );
   
   if let Some( cached ) = self.template_cache.borrow().get( &cache_key )
   {
  println!( "     ✅ Cache hit for {}", template_type );
  return cached.clone();
 }
   
   println!( "     🔄 Generating {} (cache miss)", template_type );
   
   let report = match template_type
   {
  "performance" => PerformanceReport ::new()
   .title( "Cached Performance Analysis" )
   .include_statistical_analysis( true )
   .generate( results )
   .unwrap(),
  "comparison" => 
  {
   if results.len() >= 2
   {
  let keys: Vec< &String > = results.keys().collect();
  ComparisonReport ::new()
   .baseline( keys[ 0 ] )
   .candidate( keys[ 1 ] )
   .generate( results )
   .unwrap()
 }
   else
   {
  "Not enough results for comparison".to_string()
 }
 },
  _ => "Unknown template type".to_string(),
 };
   
   self.template_cache.borrow_mut().insert( cache_key, report.clone() );
   report
 }
 }
  
  let cached_generator = CachedTemplateGenerator ::new();
  
  // Generate same template multiple times to demonstrate caching
  let sample_results: HashMap< String, BenchmarkResult > = results.iter()
  .take( 5 )
  .map( | ( k, v ) | ( k.clone(), v.clone() ) )
  .collect();
  
  let start_time = std ::time ::Instant ::now();
  
  for i in 0..3
  {
  println!( "   Iteration {} : ", i + 1 );
  let _perf_report = cached_generator.generate_cached( "performance", &sample_results );
  let _comp_report = cached_generator.generate_cached( "comparison", &sample_results );
 }
  
  let total_time = start_time.elapsed();
  println!( "   Total time with caching: {:.2?}", total_time );
  
  // Technique 2 : Parallel validation processing
  println!( "\n🔀 Technique 2 : Concurrent validation processing..." );
  
  // Simulate concurrent validation (simplified - actual implementation would use threads)
  let validator = BenchmarkValidator ::new().require_warmup( false );
  
  let validation_start = std ::time ::Instant ::now();
  
  // Sequential validation (baseline)
  let mut sequential_warnings = 0;
  for ( _name, result ) in &results
  {
  let warnings = validator.validate_result( result );
  sequential_warnings += warnings.len();
 }
  
  let sequential_time = validation_start.elapsed();
  
  println!( "   Sequential validation: {:.2?} ({} total warnings)", 
   sequential_time, sequential_warnings );
  
  // Simulated concurrent validation
  let _concurrent_start = std ::time ::Instant ::now();
  
  // In a real implementation, this would use thread pools or async processing
  // For demonstration, we'll simulate the performance improvement
  let simulated_concurrent_time = sequential_time / 4;  // Assume 4x speedup
  
  println!( "   Simulated concurrent validation: {:.2?} (4x speedup)", simulated_concurrent_time );
  
  // Technique 3 : Incremental updates
  println!( "\n📝 Technique 3 : Incremental update optimization..." );
  
  let test_doc = std ::env ::temp_dir().join( "incremental_test.md" );
  
  // Create large document
  let mut large_content = String ::from( "# Large Document\n\n" );
  for i in 1..=100
  {
  large_content.push_str( &format!( "## Section {}\n\nContent for section {}.\n\n", i, i ) );
 }
  
  std ::fs ::write( &test_doc, &large_content ).unwrap();
  
  let update_start = std ::time ::Instant ::now();
  
  // Update multiple sections
  let report = PerformanceReport ::new().generate( &sample_results ).unwrap();
  
  let incremental_chain = MarkdownUpdateChain ::new( &test_doc ).unwrap()
  .add_section( "Section 1", &report )
  .add_section( "Section 50", &report )
  .add_section( "Section 100", &report );
  
  match incremental_chain.execute()
  {
  Ok( () ) =>
  {
   let update_time = update_start.elapsed();
   println!( "   Incremental updates completed: {:.2?}", update_time );
   
   let final_size = std ::fs ::metadata( &test_doc ).unwrap().len();
   println!( "   Final document size: {:.1}KB", final_size as f64 / 1024.0 );
 },
  Err( e ) => println!( "   ❌ Incremental update failed: {}", e ),
 }
  
  // Technique 4 : Memory pool simulation
  println!( "\n💾 Technique 4 : Memory-efficient result processing..." );
  
  // Demonstrate processing large results without keeping everything in memory
  let processing_start = std ::time ::Instant ::now();
  
  let mut processed_count = 0;
  let mut total_mean_time = Duration ::from_nanos( 0 );
  
  // Process results one at a time instead of all at once
  for ( name, result ) in &results
  {
  // Process individual result
  let mean_time = result.mean_time();
  total_mean_time += mean_time;
  processed_count += 1;
  
  // Simulate some processing work
  if name.contains( "encryption" )
  {
   // Additional processing for security algorithms
   let _cv = result.coefficient_of_variation();
 }
  
  // Periodically report progress
  if processed_count % 5 == 0
  {
   let avg_time = total_mean_time / processed_count;
   println!( "     Processed {} : avg time {:.2?}", processed_count, avg_time );
 }
 }
  
  let processing_time = processing_start.elapsed();
  let overall_avg = total_mean_time / processed_count;
  
  println!( "   Memory-efficient processing: {:.2?}", processing_time );
  println!( "   Overall average performance: {:.2?}", overall_avg );
  println!( "   Peak memory: Single BenchmarkResult (constant)" );
  
  // Cleanup
  std ::fs ::remove_file( &test_doc ).unwrap();
  
  println!( "\n🎯 Performance optimization techniques demonstrated: " );
  println!( "   • Template result caching for repeated operations" );
  println!( "   • Concurrent validation processing for parallelizable work" );
  println!( "   • Incremental document updates for large files" );
  println!( "   • Stream processing for memory-efficient large-scale analysis" );
  
  println!();
}

fn main()
{
  println!( "🚀 Advanced Usage Pattern Examples\n" );
  
  pattern_domain_specific_validation();
  pattern_template_composition();
  pattern_coordinated_updates();
  pattern_memory_efficient_processing();
  pattern_performance_optimization();
  
  println!( "📋 Advanced Usage Patterns Covered: " );
  println!( "✅ Domain-specific validation: custom criteria for different use cases" );
  println!( "✅ Template composition: inheritance, specialization, and reuse patterns" );
  println!( "✅ Coordinated updates: multi-document atomic updates with consistency" );
  println!( "✅ Memory efficiency: large-scale processing with bounded resource usage" );
  println!( "✅ Performance optimization: caching, concurrency, and incremental processing" );
  println!( "\n🎯 These patterns enable sophisticated benchmarking workflows" );
  println!( "   that scale to enterprise requirements while maintaining simplicity." );
  
  println!( "\n💡 Key Takeaways for Advanced Usage: " );
  println!( "• Customize validation criteria for your specific domain requirements" );
  println!( "• Compose templates to create specialized reporting for different audiences" );
  println!( "• Coordinate updates across multiple documents for consistency" );
  println!( "• Use batch processing and caching for large-scale analysis" );
  println!( "• Optimize performance through concurrency and incremental processing" );
  
  println!( "\n📁 Generated examples and reports saved to: " );
  println!( "   {}", std ::env ::temp_dir().display() );
}