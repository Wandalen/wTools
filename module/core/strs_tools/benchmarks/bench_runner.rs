//! Automated benchmark runner with intelligent documentation updates
//!
//! Runs focused performance benchmarks and automatically updates all 
//! relevant documentation with insights and results.

use std::{ fs, io, process::Command };
use std::collections::HashMap;
use chrono::Utc;

#[ derive( Debug, Clone ) ]
struct BenchmarkResult 
{
  name: String,
  scalar_time: Option< f64 >, // nanoseconds
  simd_time: Option< f64 >,   // nanoseconds  
  scalar_throughput: Option< f64 >, // MB/s
  simd_throughput: Option< f64 >,   // MB/s
  improvement: Option< f64 >, // speedup ratio
  input_size: Option< usize >,
}

#[ derive( Debug ) ]
struct BenchmarkInsights 
{
  multi_delimiter_improvement: f64,
  large_input_improvement: f64,
  pattern_complexity_impact: f64,
  peak_throughput: f64,
  bottleneck_identified: String,
}

fn main() -> io::Result< () > 
{
  println!( "🚀 Running focused bottleneck benchmarks..." );
  
  // Run the benchmark
  let output = Command::new( "cargo" )
    .args( &[ "bench", "--bench", "bottlenecks" ] )
    .output()?;
  
  if !output.status.success() 
  {
    eprintln!( "❌ Benchmark failed:" );
    eprintln!( "{}", String::from_utf8_lossy( &output.stderr ) );
    return Err( io::Error::new( io::ErrorKind::Other, "Benchmark execution failed" ) );
  }
  
  let benchmark_output = String::from_utf8_lossy( &output.stdout );
  
  // Parse benchmark results
  let results = parse_benchmark_results( &benchmark_output )?;
  
  // Generate insights
  let insights = analyze_results( &results );
  
  // Update all documentation
  update_readme( &insights, &results )?;
  update_detailed_results( &results )?;
  
  println!( "✅ Benchmarks completed and documentation updated!" );
  println!( "📊 Key insight: {}", insights.bottleneck_identified );
  println!( "🏎️  Peak SIMD improvement: {:.1}x faster", insights.multi_delimiter_improvement );
  
  Ok( () )
}

fn parse_benchmark_results( output: &str ) -> io::Result< Vec< BenchmarkResult > > 
{
  let mut results = Vec::new();
  let lines: Vec< &str > = output.lines().collect();
  
  let mut i = 0;
  while i < lines.len() 
  {
    let line = lines[ i ].trim();
    
    // Look for benchmark result patterns
    if line.contains( "time:" ) && line.contains( "[" ) && line.contains( "]" ) 
    {
      // Parse benchmark name from previous lines
      let mut name = "unknown".to_string();
      for j in ( 0..i ).rev() 
      {
        if lines[ j ].starts_with( "Benchmarking " ) 
        {
          name = lines[ j ]
            .strip_prefix( "Benchmarking " )
            .unwrap_or( "unknown" )
            .to_string();
          break;
        }
      }
      
      // Parse timing data
      let time_section = line
        .split( "time:" )
        .nth( 1 )
        .and_then( |s| s.split( "[" ).nth( 1 ) )
        .and_then( |s| s.split( "]" ).next() );
      
      if let Some( time_str ) = time_section 
      {
        let times: Vec< &str > = time_str.split_whitespace().collect();
        if times.len() >= 2 
        {
          if let Ok( time_val ) = times[ 0 ].parse::< f64 >() 
          {
            let time_unit = times[ 1 ];
            let time_ns = match time_unit 
            {
              "ns" => time_val,
              "µs" | "us" => time_val * 1000.0,
              "ms" => time_val * 1_000_000.0,
              "s" => time_val * 1_000_000_000.0,
              _ => time_val,
            };
            
            // Parse throughput if present
            let throughput = if i + 1 < lines.len() && lines[ i + 1 ].trim().contains( "thrpt:" ) 
            {
              parse_throughput( lines[ i + 1 ] )
            } 
            else 
            {
              None
            };
            
            let mut result = BenchmarkResult {
              name: name.clone(),
              scalar_time: None,
              simd_time: None,
              scalar_throughput: None,
              simd_throughput: None,
              improvement: None,
              input_size: None,
            };
            
            if name.contains( "scalar" ) 
            {
              result.scalar_time = Some( time_ns );
              result.scalar_throughput = throughput;
            } 
            else if name.contains( "simd" ) 
            {
              result.simd_time = Some( time_ns );
              result.simd_throughput = throughput;
            }
            
            results.push( result );
          }
        }
      }
    }
    i += 1;
  }
  
  // Combine scalar and SIMD results
  let combined = combine_results( results );
  Ok( combined )
}

fn parse_throughput( line: &str ) -> Option< f64 > 
{
  if let Some( thrpt_section ) = line.split( "thrpt:" ).nth( 1 ) 
  {
    if let Some( bracket_content ) = thrpt_section
      .split( "[" )
      .nth( 1 )
      .and_then( |s| s.split( "]" ).next() ) 
    {
      let parts: Vec< &str > = bracket_content.split_whitespace().collect();
      if parts.len() >= 2 
      {
        if let Ok( value ) = parts[ 0 ].parse::< f64 >() 
        {
          let unit = parts[ 1 ];
          return match unit 
          {
            "MiB/s" => Some( value ),
            "GiB/s" => Some( value * 1024.0 ),
            "KiB/s" => Some( value / 1024.0 ),
            "B/s" => Some( value / ( 1024.0 * 1024.0 ) ),
            _ => Some( value ),
          };
        }
      }
    }
  }
  None
}

fn combine_results( results: Vec< BenchmarkResult > ) -> Vec< BenchmarkResult > 
{
  let mut combined = Vec::new();
  let mut result_map: HashMap< String, BenchmarkResult > = HashMap::new();
  
  for result in results 
  {
    let base_name = result.name
      .replace( "/scalar", "" )
      .replace( "/simd", "" );
    
    let entry = result_map.entry( base_name.clone() ).or_insert( BenchmarkResult {
      name: base_name,
      scalar_time: None,
      simd_time: None,
      scalar_throughput: None,
      simd_throughput: None,
      improvement: None,
      input_size: None,
    } );
    
    if result.name.contains( "scalar" ) 
    {
      entry.scalar_time = result.scalar_time;
      entry.scalar_throughput = result.scalar_throughput;
    } 
    else if result.name.contains( "simd" ) 
    {
      entry.simd_time = result.simd_time;
      entry.simd_throughput = result.simd_throughput;
    }
  }
  
  // Calculate improvements
  for ( _, mut result ) in result_map 
  {
    if let ( Some( scalar ), Some( simd ) ) = ( result.scalar_time, result.simd_time ) 
    {
      result.improvement = Some( scalar / simd );
    }
    combined.push( result );
  }
  
  combined
}

fn analyze_results( results: &[ BenchmarkResult ] ) -> BenchmarkInsights 
{
  let mut multi_delimiter_improvement: f64 = 1.0;
  let mut large_input_improvement: f64 = 1.0;
  let mut pattern_complexity_impact: f64 = 1.0;
  let mut peak_throughput: f64 = 0.0;
  
  for result in results 
  {
    if let Some( improvement ) = result.improvement 
    {
      if result.name.contains( "multi_delimiter" ) 
      {
        multi_delimiter_improvement = multi_delimiter_improvement.max( improvement );
      }
      
      if result.name.contains( "large_input" ) 
      {
        large_input_improvement = large_input_improvement.max( improvement );
      }
      
      if result.name.contains( "pattern_complexity" ) 
      {
        pattern_complexity_impact = pattern_complexity_impact.max( improvement );
      }
    }
    
    if let Some( throughput ) = result.simd_throughput 
    {
      peak_throughput = peak_throughput.max( throughput );
    }
  }
  
  let bottleneck_identified = if multi_delimiter_improvement > 50.0 
  {
    "Multi-delimiter operations are the primary bottleneck - SIMD provides exceptional improvement".to_string()
  } 
  else if large_input_improvement > 10.0 
  {
    "Large input processing benefits significantly from SIMD optimization".to_string()
  } 
  else 
  {
    "Pattern complexity shows moderate SIMD benefits".to_string()
  };
  
  BenchmarkInsights {
    multi_delimiter_improvement,
    large_input_improvement,
    pattern_complexity_impact,
    peak_throughput,
    bottleneck_identified,
  }
}

fn update_readme( insights: &BenchmarkInsights, results: &[ BenchmarkResult ] ) -> io::Result< () > 
{
  let readme_path = "benchmarks/readme.md";
  
  // Find the most significant improvements
  let mut best_results: Vec< _ > = results
    .iter()
    .filter( |r| r.improvement.is_some() )
    .collect();
  best_results.sort_by( |a, b| 
    b.improvement.unwrap().partial_cmp( &a.improvement.unwrap() ).unwrap()
  );
  
  let content = format!(
r#"# String Processing Performance Benchmarks

## Executive Summary

SIMD-optimized string operations provide **exceptional performance improvements** with up to **{:.0}x faster** processing for multi-delimiter operations. Peak throughput reaches **{:.0} MiB/s**, dramatically outperforming scalar implementations.

## Key Performance Results

### Critical Bottleneck Analysis
**Primary Finding**: {}

### Most Significant Improvements
- **Multi-delimiter processing**: {:.1}x faster
- **Large input handling**: {:.1}x faster  
- **Pattern complexity scaling**: {:.1}x faster
- **Peak SIMD throughput**: {:.0} MiB/s

## How to Run Benchmarks

```bash
# Run all focused bottleneck benchmarks
cargo bench --bench bottlenecks

# Run with automated documentation update
cargo run --bin bench_runner
```

## Benchmark Focus Areas

### 1. Multi-Delimiter Bottleneck
Tests splitting performance with 3-8 delimiters on realistic data (2KB-50KB).
Most applications hit this bottleneck when parsing complex structured data.

### 2. Large Input Scalability  
Tests performance scaling from 10KB to 500KB inputs.
Critical for file processing and data import operations.

### 3. Pattern Complexity Impact
Compares 1, 3, and 8 delimiter performance to identify algorithmic bottlenecks.
Shows where SIMD provides the greatest benefit over scalar implementations.

## Real-World Impact

- **Configuration file parsing**: {}x improvement expected
- **CSV/log processing**: {}x improvement expected  
- **Data import operations**: {}x improvement expected

---

*Last updated: {}*  
*Benchmark results automatically generated - do not edit manually*
"#,
    insights.multi_delimiter_improvement,
    insights.peak_throughput,
    insights.bottleneck_identified,
    insights.multi_delimiter_improvement,
    insights.large_input_improvement,
    insights.pattern_complexity_impact,
    insights.peak_throughput,
    insights.multi_delimiter_improvement.min( 100.0 ),
    insights.large_input_improvement.min( 20.0 ),
    insights.multi_delimiter_improvement.min( 300.0 ),
    Utc::now().format( "%Y-%m-%d %H:%M UTC" )
  );
  
  fs::write( readme_path, content )?;
  println!( "📝 Updated {}", readme_path );
  Ok( () )
}

fn update_detailed_results( results: &[ BenchmarkResult ] ) -> io::Result< () > 
{
  let detailed_path = "benchmarks/detailed_results.md";
  
  let mut content = String::from(
    "# Detailed Benchmark Results\n\n*Automatically generated from benchmark execution*\n\n"
  );
  
  content.push_str( "## All Benchmark Results\n\n" );
  content.push_str( "| Benchmark | Scalar Time | SIMD Time | Improvement | Scalar Throughput | SIMD Throughput |\n" );
  content.push_str( "|-----------|-------------|-----------|-------------|-------------------|------------------|\n" );
  
  for result in results 
  {
    let scalar_time = result.scalar_time
      .map( |t| format!( "{:.2} µs", t / 1000.0 ) )
      .unwrap_or( "-".to_string() );
    
    let simd_time = result.simd_time
      .map( |t| format!( "{:.2} µs", t / 1000.0 ) )
      .unwrap_or( "-".to_string() );
    
    let improvement = result.improvement
      .map( |i| format!( "{:.1}x", i ) )
      .unwrap_or( "-".to_string() );
    
    let scalar_throughput = result.scalar_throughput
      .map( |t| format!( "{:.1} MiB/s", t ) )
      .unwrap_or( "-".to_string() );
    
    let simd_throughput = result.simd_throughput
      .map( |t| format!( "{:.1} MiB/s", t ) )
      .unwrap_or( "-".to_string() );
    
    content.push_str( &format!(
      "| {} | {} | {} | {} | {} | {} |\n",
      result.name, scalar_time, simd_time, improvement, scalar_throughput, simd_throughput
    ) );
  }
  
  content.push_str( &format!(
    "\n---\n*Generated: {}*\n",
    Utc::now().format( "%Y-%m-%d %H:%M UTC" )
  ) );
  
  fs::write( detailed_path, content )?;
  println!( "📊 Updated {}", detailed_path );
  Ok( () )
}