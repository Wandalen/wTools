//! Automated benchmarking with change documentation
//! 
//! Usage: cargo run --bin benchmark_runner -- [OPTIONS]
//! 
//! This binary runs strs_tools benchmarks and optionally auto-appends results 
//! to benchmark/changes.md with structured performance data.

use std::{ env, fs, io, process };
use std::path::PathBuf;

/// Command line arguments
#[derive( Debug, Default )]
struct Args
{
  baseline : bool,
  simd : bool,
  compare : bool,
  append_changes : bool,
  change_type : Option< String >,
  description : Option< String >,
  help : bool,
  quick : bool,
}

/// Performance metrics extracted from benchmark output
#[derive( Debug, Default )]
struct PerformanceData
{
  single_delimiter_1kb : Option< f64 >,
  single_delimiter_10kb : Option< f64 >,
  multi_delimiter_1kb : Option< f64 >,
  substring_search_1kb : Option< f64 >,
  character_counting_1kb : Option< f64 >,
  single_delimiter_time : Option< f64 >,
}

/// Benchmark runner
struct BenchmarkRunner
{
  script_dir : PathBuf,
  changes_file : PathBuf,
}

impl BenchmarkRunner
{
  fn new() -> io::Result< Self >
  {
    let script_dir = env::current_dir()?;
    let changes_file = script_dir.join( "benchmark" ).join( "changes.md" );
    
    Ok( Self { script_dir, changes_file } )
  }
  
  fn log_info( &self, msg : &str )
  {
    println!( "\x1b[0;34m[INFO]\x1b[0m {}", msg );
  }
  
  fn log_success( &self, msg : &str )
  {
    println!( "\x1b[0;32m[SUCCESS]\x1b[0m {}", msg );
  }
  
  #[allow(dead_code)]
  fn log_warning( &self, msg : &str )
  {
    println!( "\x1b[1;33m[WARNING]\x1b[0m {}", msg );
  }
  
  fn log_error( &self, msg : &str )
  {
    eprintln!( "\x1b[0;31m[ERROR]\x1b[0m {}", msg );
  }
  
  fn check_prerequisites( &self ) -> bool
  {
    self.log_info( "Checking prerequisites..." );
    
    if process::Command::new( "cargo" ).arg( "--version" ).output().is_err()
    {
      self.log_error( "cargo not found. Please install Rust." );
      return false;
    }
    
    if process::Command::new( "rustc" ).arg( "--version" ).output().is_err()
    {
      self.log_error( "rustc not found. Please install Rust." );
      return false;
    }
    
    self.log_success( "Prerequisites check passed" );
    true
  }
  
  fn extract_performance_data( &self, benchmark_output : &str ) -> PerformanceData
  {
    let mut data = PerformanceData::default();
    
    // Parse throughput data (MiB/s)
    let throughput_patterns = [
      ( "single_delimiter_split/size_1000", &mut data.single_delimiter_1kb ),
      ( "single_delimiter_split/size_10000", &mut data.single_delimiter_10kb ),
      ( "multi_delimiter_split/size_1000", &mut data.multi_delimiter_1kb ),
      ( "substring_search/size_1000", &mut data.substring_search_1kb ),
      ( "character_counting/size_1000", &mut data.character_counting_1kb ),
    ];
    
    for ( pattern, field ) in throughput_patterns
    {
      if let Some( line ) = benchmark_output.lines().find( |line| line.contains( pattern ) )
      {
        // Look for thrpt: [x.x MiB/s y.y MiB/s z.z MiB/s] pattern
        if let Some( thrpt_start ) = line.find( "thrpt:" )
        {
          let thrpt_section = &line[ thrpt_start.. ];
          if let Some( start ) = thrpt_section.find( '[' )
          {
            if let Some( end ) = thrpt_section.find( ']' )
            {
              let values_section = &thrpt_section[ start + 1..end ];
              // Extract middle value (typically the mean)
              let values : Vec< &str > = values_section.split_whitespace().collect();
              if values.len() >= 4 // Should be "x.x MiB/s y.y MiB/s"
              {
                if let Ok( value ) = values[ 2 ].parse::< f64 >()
                {
                  *field = Some( value );
                }
              }
            }
          }
        }
      }
    }
    
    // Parse timing data (microseconds)
    if let Some( line ) = benchmark_output.lines().find( |line| 
      line.contains( "single_delimiter_split/size_1000" ) && line.contains( "time:" ) )
    {
      if let Some( time_start ) = line.find( "time:" )
      {
        let time_section = &line[ time_start.. ];
        if let Some( start ) = time_section.find( '[' )
        {
          if let Some( end ) = time_section.find( ']' )
          {
            let values_section = &time_section[ start + 1..end ];
            let values : Vec< &str > = values_section.split_whitespace().collect();
            if values.len() >= 4 // Should be "x.x µs y.y µs"
            {
              if let Ok( value ) = values[ 2 ].parse::< f64 >()
              {
                data.single_delimiter_time = Some( value );
              }
            }
          }
        }
      }
    }
    
    data
  }
  
  fn format_performance_impact( &self, perf_data : &PerformanceData, _benchmark_type : &str ) -> String
  {
    let mut impact_lines = Vec::new();
    
    if let Some( value ) = perf_data.single_delimiter_1kb
    {
      impact_lines.push( format!( "- Single delimiter split (1KB): {:.1} MiB/s", value ) );
    }
    
    if let Some( value ) = perf_data.single_delimiter_10kb
    {
      impact_lines.push( format!( "- Single delimiter split (10KB): {:.1} MiB/s", value ) );
    }
    
    if let Some( value ) = perf_data.multi_delimiter_1kb
    {
      impact_lines.push( format!( "- Multi delimiter split (1KB): {:.1} MiB/s", value ) );
    }
    
    if let Some( value ) = perf_data.substring_search_1kb
    {
      impact_lines.push( format!( "- Substring search (1KB): {:.1} MiB/s", value ) );
    }
    
    if let Some( value ) = perf_data.character_counting_1kb
    {
      impact_lines.push( format!( "- Character counting (1KB): {:.1} MiB/s", value ) );
    }
    
    if let Some( value ) = perf_data.single_delimiter_time
    {
      impact_lines.push( format!( "- Single delimiter split timing: {:.2} µs", value ) );
    }
    
    if impact_lines.is_empty()
    {
      "- Performance metrics extracted from benchmark run".to_string()
    }
    else
    {
      impact_lines.join( "\n" )
    }
  }
  
  fn run_benchmark_command( &self, args : &[ &str ] ) -> io::Result< ( bool, String ) >
  {
    self.log_info( &format!( "Command: cargo {}", args.join( " " ) ) );
    
    let output = process::Command::new( "cargo" )
      .args( args )
      .current_dir( &self.script_dir )
      .output()?;
    
    let stdout = String::from_utf8_lossy( &output.stdout );
    let stderr = String::from_utf8_lossy( &output.stderr );
    let combined_output = format!( "{}\n{}", stdout, stderr );
    
    Ok( ( output.status.success(), combined_output ) )
  }
  
  fn run_baseline_benchmarks( &self, quick : bool ) -> io::Result< ( bool, String ) >
  {
    self.log_info( "Running baseline (scalar) benchmarks..." );
    
    let benchmark_name = if quick { "quick_test" } else { "string_operations" };
    let args = [
      "bench", "--bench", benchmark_name, 
      "--", "--sample-size", "10", "--measurement-time", "1"
    ];
    
    let ( success, output ) = self.run_benchmark_command( &args )?;
    
    if success
    {
      self.log_success( "Baseline benchmarks completed" );
    }
    else
    {
      self.log_error( "Baseline benchmarks failed" );
    }
    
    Ok( ( success, output ) )
  }
  
  fn run_simd_benchmarks( &self ) -> io::Result< ( bool, String ) >
  {
    self.log_info( "Running SIMD benchmarks..." );
    
    let args = [
      "bench", "--features", "simd", "--bench", "string_operations",
      "--", "--sample-size", "10", "--measurement-time", "1"
    ];
    
    let ( success, output ) = self.run_benchmark_command( &args )?;
    
    if success
    {
      self.log_success( "SIMD benchmarks completed" );
    }
    else
    {
      self.log_error( "SIMD benchmarks failed" );
    }
    
    Ok( ( success, output ) )
  }
  
  fn run_comparison_benchmarks( &self ) -> io::Result< ( bool, String ) >
  {
    self.log_info( "Running comparison benchmarks..." );
    
    // Step 1: Baseline
    self.log_info( "Step 1/2: Running baseline measurements..." );
    let baseline_args = [
      "bench", "--bench", "string_operations",
      "--", "--sample-size", "10", "--save-baseline", "scalar_baseline"
    ];
    
    let ( success, _ ) = self.run_benchmark_command( &baseline_args )?;
    if !success
    {
      return Ok( ( false, "Baseline measurement failed".to_string() ) );
    }
    
    // Step 2: SIMD comparison
    self.log_info( "Step 2/2: Running SIMD comparison..." );
    let simd_args = [
      "bench", "--features", "simd", "--bench", "string_operations",
      "--", "--sample-size", "10", "--load-baseline", "scalar_baseline"
    ];
    
    let ( success, output ) = self.run_benchmark_command( &simd_args )?;
    
    if success
    {
      self.log_success( "Comparison benchmarks completed" );
    }
    else
    {
      self.log_error( "Comparison benchmarks failed" );
    }
    
    Ok( ( success, output ) )
  }
  
  fn append_to_changes( 
    &self, 
    change_type : &str, 
    description : &str, 
    benchmark_type : &str,
    performance_data : &PerformanceData, 
    full_output : &str 
  ) -> io::Result< () >
  {
    self.log_info( &format!( "Appending results to {}...", self.changes_file.display() ) );
    
    // Backup changes.md
    if self.changes_file.exists()
    {
      let backup_file = self.changes_file.with_extension( "md.backup" );
      fs::copy( &self.changes_file, &backup_file )?;
      self.log_info( &format!( "Backup created: {}", backup_file.display() ) );
    }
    
    // Get environment info
    let rust_version = process::Command::new( "rustc" )
      .arg( "--version" )
      .output()
      .map( |o| String::from_utf8_lossy( &o.stdout ).trim().to_string() )
      .unwrap_or_else( |_| "Unknown".to_string() );
    
    let platform_info = format!( "{} {}", 
      env::consts::OS, 
      env::consts::ARCH 
    );
    
    let date_str = chrono::Utc::now().format( "%Y-%m-%d" ).to_string();
    let datetime_str = chrono::Utc::now().format( "%Y-%m-%d %H:%M:%S UTC" ).to_string();
    let entry_title = format!( "{} - {}", date_str, description );
    
    // Format performance impact
    let performance_impact = self.format_performance_impact( performance_data, benchmark_type );
    
    // Truncate full output for readability
    let lines : Vec< &str > = full_output.lines().collect();
    let truncated_output = if lines.len() > 50
    {
      format!( "{}\n[Output truncated - see full logs for complete results]", 
        lines[ ..50 ].join( "\n" ) )
    }
    else
    {
      full_output.to_string()
    };
    
    // Create new entry
    let new_entry = format!( r#"

## {}

**Change Type**: {}  
**Description**: {}

**Performance Impact**:
{}

**Benchmark Evidence**:
```
{}
```

**Environment**:
- Platform: {}
- Rust: {}
- Date: {}
- Test conditions: criterion.rs, 10 samples, 1s measurement time  
- Benchmark type: {}

**Root Cause Analysis**: Performance change due to {} implementation

**Related Files**:
- benches/string_operations.rs - Main benchmark suite
- src/string/split/ - String splitting implementation

**Validation**: Automated benchmark run with consistent measurement methodology
"#, 
      entry_title, change_type, description, performance_impact, 
      truncated_output, platform_info, rust_version, datetime_str, 
      benchmark_type, change_type.to_lowercase()
    );
    
    // Read current content and append new entry
    let current_content = if self.changes_file.exists()
    {
      fs::read_to_string( &self.changes_file )?
    }
    else
    {
      "# Performance Changes History\n\n**Documentation**: See `benchmark/readme.md` for guidelines and templates.\n\n---\n".to_string()
    };
    
    // Append new entry to the end
    let updated_content = format!( "{}{}", current_content, new_entry );
    fs::write( &self.changes_file, updated_content )?;
    
    self.log_success( "Results appended to benchmark/changes.md" );
    Ok( () )
  }
}

fn parse_args() -> Args
{
  let args : Vec< String > = env::args().collect();
  let mut parsed = Args::default();
  let mut i = 1;
  
  while i < args.len()
  {
    match args[ i ].as_str()
    {
      "--baseline" => parsed.baseline = true,
      "--simd" => parsed.simd = true,
      "--compare" => parsed.compare = true,
      "--append-changes" => parsed.append_changes = true,
      "--change-type" =>
      {
        if i + 1 < args.len()
        {
          parsed.change_type = Some( args[ i + 1 ].clone() );
          i += 1;
        }
      }
      "--description" =>
      {
        if i + 1 < args.len()
        {
          parsed.description = Some( args[ i + 1 ].clone() );
          i += 1;
        }
      }
      "--quick" => parsed.quick = true,
      "--help" => parsed.help = true,
      _ =>
      {
        eprintln!( "Unknown option: {}", args[ i ] );
        parsed.help = true;
      }
    }
    i += 1;
  }
  
  // Validation
  if parsed.append_changes
  {
    if parsed.change_type.is_none()
    {
      eprintln!( "Error: --append-changes requires --change-type" );
      parsed.help = true;
    }
    if parsed.description.is_none()
    {
      eprintln!( "Error: --append-changes requires --description" );
      parsed.help = true;
    }
  }
  
  // Default to baseline if nothing specified
  if !parsed.baseline && !parsed.simd && !parsed.compare && !parsed.help
  {
    parsed.baseline = true;
    println!( "No benchmark type specified, defaulting to --baseline" );
  }
  
  parsed
}

fn show_help()
{
  println!( r#"
benchmark_runner - Automated benchmarking with change documentation

USAGE:
    cargo run --bin benchmark_runner -- [OPTIONS]

OPTIONS:
    --baseline                  Run baseline (scalar) benchmarks only
    --simd                     Run SIMD benchmarks only  
    --compare                  Run comparison benchmarks
    --quick                    Use quick test benchmark (faster for testing)
    --append-changes           Auto-append results to benchmark/changes.md
    --change-type TYPE         Change type: Infrastructure/Feature/Optimization/Bug Fix/Refactor/Regression
    --description "DESC"       Description of the change being benchmarked
    --help                     Show this help message

EXAMPLES:
    # Run baseline benchmarks (full suite)
    cargo run --bin benchmark_runner -- --baseline

    # Run quick test (faster for development)
    cargo run --bin benchmark_runner -- --baseline --quick

    # Run SIMD benchmarks and document the optimization
    cargo run --bin benchmark_runner -- --simd --append-changes --change-type "Optimization" --description "SIMD implementation using aho-corasick"

    # Compare baseline vs SIMD and auto-document
    cargo run --bin benchmark_runner -- --compare --append-changes --change-type "Optimization" --description "SIMD vs scalar performance comparison"
"# );
}

fn main() -> io::Result< () >
{
  let args = parse_args();
  
  if args.help
  {
    show_help();
    return Ok( () );
  }
  
  let runner = BenchmarkRunner::new()?;
  
  if !runner.check_prerequisites()
  {
    process::exit( 1 );
  }
  
  runner.log_info( &format!( "strs_tools Benchmark Runner - {}", chrono::Utc::now().format( "%Y-%m-%d %H:%M:%S UTC" ) ) );
  
  // Run benchmarks
  let mut success = true;
  let mut output = String::new();
  let mut benchmark_type = String::new();
  
  if args.baseline
  {
    let ( bench_success, bench_output ) = runner.run_baseline_benchmarks( args.quick )?;
    success = bench_success;
    output = bench_output;
    benchmark_type = "Baseline".to_string();
  }
  
  if args.simd && success
  {
    let ( bench_success, bench_output ) = runner.run_simd_benchmarks()?;
    success = bench_success;
    output = bench_output;
    benchmark_type = "SIMD".to_string();
  }
  
  if args.compare && success
  {
    let ( bench_success, bench_output ) = runner.run_comparison_benchmarks()?;
    success = bench_success;
    output = bench_output;
    benchmark_type = "Comparison".to_string();
  }
  
  if !success
  {
    runner.log_error( "Benchmark run failed" );
    process::exit( 1 );
  }
  
  // Auto-append to changes.md if requested
  if args.append_changes
  {
    let performance_data = runner.extract_performance_data( &output );
    runner.append_to_changes( 
      &args.change_type.unwrap(), 
      &args.description.unwrap(), 
      &benchmark_type,
      &performance_data, 
      &output 
    )?;
  }
  
  runner.log_success( "Benchmark run completed successfully!" );
  
  if args.append_changes
  {
    runner.log_info( "Performance data has been automatically documented in benchmark/changes.md" );
  }
  
  Ok( () )
}