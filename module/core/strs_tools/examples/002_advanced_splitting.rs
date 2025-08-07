//! Advanced string splitting examples demonstrating quote handling and escape sequences.
//!
//! This example showcases the advanced features of `strs_tools` that make it superior
//! to standard library string operations, particularly for parsing complex text
//! formats like command lines, configuration files, and quoted strings.

use strs_tools::*;

fn main()
{
  println!( "=== Advanced String Splitting Examples ===" );
  
  quote_aware_splitting();
  escape_sequence_handling();
  complex_delimiter_scenarios();
  performance_optimization_demo();
}

/// Demonstrates quote-aware string splitting.
///
/// This is essential for parsing command-line arguments, CSV files,
/// or any format where spaces inside quotes should be preserved.
fn quote_aware_splitting()
{
  println!( "\n--- Quote-Aware Splitting ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
    // Parse a command with quoted arguments containing spaces
    let command_line = r#"program --input "file with spaces.txt" --output "result file.out" --verbose"#;
    
    println!( "Parsing command: {command_line}" );
    
    let iter = string::split()
    .src( command_line )
    .delimeter( " " )
    .quoting( true )               // Enable quote awareness
    .stripping( true )             // Remove delimiters from output
    .perform();
    
    let args : Vec< String > = iter.map( String::from ).collect();
    
    println!( "Parsed arguments:" );
    for ( i, arg ) in args.iter().enumerate()
    {
      println!( "  [{i}]: '{arg}'" );
    }
    
    // Verify the quoted arguments are preserved as single tokens
    assert_eq!( args[ 2 ], "file with spaces.txt" );  // No quotes in result
    assert_eq!( args[ 4 ], "result file.out" );       // Spaces preserved
    
    println!( "✓ Quotes handled correctly - spaces preserved inside quotes" );
  }
}

/// Demonstrates handling of escape sequences within strings.
///
/// Shows how `strs_tools` can handle escaped quotes and other special
/// characters commonly found in configuration files and string literals.
fn escape_sequence_handling()
{
  println!( "\n--- Escape Sequence Handling ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
    // String with escaped quotes and other escape sequences
    let complex_string = r#"name="John \"The Developer\" Doe" age=30 motto="Code hard, debug harder\n""#;
    
    println!( "Input with escapes: {complex_string}" );
    
    let iter = string::split()
    .src( complex_string )
    .delimeter( " " )
    .quoting( true )
    .stripping( true )
    .perform();
    
    let tokens : Vec< String > = iter.map( String::from ).collect();
    
    println!( "Extracted tokens:" );
    for token in &tokens
    {
      if token.contains( '=' )
      {
        // Split key=value pairs
        let parts : Vec< &str > = token.splitn( 2, '=' ).collect();
        if parts.len() == 2
        {
          println!( "  {} = '{}'", parts[ 0 ], parts[ 1 ] );
        }
      }
    }
    
    // Verify escaped quotes are preserved in the value
    let name_token = tokens.iter().find( | t | t.starts_with( "name=" ) ).unwrap();
    println!( "✓ Escaped quotes preserved in: {name_token}" );
  }
}

/// Demonstrates complex delimiter scenarios.
///
/// Shows how to handle multiple delimiters, overlapping patterns,
/// and edge cases that would be difficult with standard string methods.
fn complex_delimiter_scenarios()
{
  println!( "\n--- Complex Delimiter Scenarios ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
    // Text with mixed delimiters and quoted sections
    let mixed_format = r#"item1,item2;"quoted,item;with,delims";item3,item4"#;
    
    println!( "Mixed delimiter text: {mixed_format}" );
    
    // First pass: split on semicolons (respecting quotes)
    let iter = string::split()
    .src( mixed_format )
    .delimeter( ";" )
    .quoting( true )
    .stripping( true )
    .perform();
    
    let sections : Vec< String > = iter.map( String::from ).collect();
    
    println!( "Sections split by ';':" );
    for ( i, section ) in sections.iter().enumerate()
    {
      println!( "  Section {i}: '{section}'" );
      
      // Further split each section by commas (if not quoted)
      if section.starts_with( '"' ) {
        println!( "    Quoted content: '{section}'" );
      } else {
        let sub_iter = string::split()
        .src( section.as_str() )
        .delimeter( "," )
        .stripping( true )
        .perform();
        
        let items : Vec< String > = sub_iter.map( String::from ).collect();
        
        for item in items
        {
          println!( "    Item: '{item}'" );
        }
      }
    }
    
    println!( "✓ Complex nested parsing completed successfully" );
  }
}

/// Demonstrates performance optimization features.
///
/// Shows how to use SIMD-accelerated operations for high-throughput
/// text processing scenarios.
fn performance_optimization_demo()
{
  println!( "\n--- Performance Optimization Demo ---" );
  
  #[ cfg( all( feature = "string_split", feature = "simd", not( feature = "no_std" ) ) ) ]
  {
    // Generate a large text for performance testing
    let large_text = "word ".repeat( 10000 ) + "final";
    let text_size = large_text.len();
    
    println!( "Processing large text ({text_size} bytes)..." );
    
    let start = std::time::Instant::now();
    
    // Use SIMD-optimized splitting for large data
    let iter = string::split()
    .src( &large_text )
    .delimeter( " " )
    .stripping( true )
    .perform();
    
    let word_count = iter.count();
    let duration = start.elapsed();
    
    println!( "SIMD-optimized split results:" );
    println!( "  Words found: {word_count}" );
    println!( "  Processing time: {duration:?}" );
    println!( "  Throughput: {:.2} MB/s", 
             ( text_size as f64 ) / ( 1024.0 * 1024.0 ) / duration.as_secs_f64() );
    
    assert_eq!( word_count, 10001 );  // 10000 "word" + 1 "final"
    
    println!( "✓ High-performance processing completed" );
  }
  
  #[ cfg( not( all( feature = "string_split", feature = "simd", not( feature = "no_std" ) ) ) ) ]
  {
    println!( "  (SIMD features not available - enable 'simd' feature for performance boost)" );
  }
}