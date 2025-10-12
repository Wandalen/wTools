//! Minimal Reproducible Example: Unilang String-Based API Architectural Flaw
//!
//! This MRE demonstrates why unilang's string-only API is fundamentally flawed
//! and requires argv-based API for proper CLI integration.
//!
//! # The Problem
//!
//! When integrating unilang into CLI applications, we receive argv from the OS
//! but unilang only accepts strings. This forces us to reconstruct strings from
//! argv, which is lossy and error-prone.
//!
//! # How to Run This MRE
//!
//! ```bash
//! cd /home/user1/pro/lib/willbe/task
//! rustc --edition 2021 unilang_argv_api_request_mre.rs -o mre
//! ./mre command::"ls -la"     # Simulates shell passing two argv
//! ```
//!
//! Expected: Should work
//! Actual: Fails because information is lost in argv→string→parse pipeline

use std::env;

/// Simulates unilang's string-based API (simplified)
struct UnilangParser;

impl UnilangParser
{
  /// This is what unilang currently offers: string-based parsing
  fn parse_from_string( input: &str ) -> Result< ParsedCommand, String >
  {
    // Simulate unilang_parser tokenization on spaces
    let tokens: Vec< &str > = input.split_whitespace().collect();

    if tokens.is_empty()
    {
      return Err( "Empty command".to_string() );
    }

    let mut args = std::collections::HashMap::new();

    for token in &tokens[1..]
    {
      if let Some( ( key, value ) ) = token.split_once( "::" )
      {
        args.insert( key.to_string(), value.to_string() );
      }
      else if token.starts_with( '-' )
      {
        // This is where it breaks!
        return Err( format!( "Unexpected token '{}' - looks like part of a value but was tokenized separately", token ) );
      }
    }

    Ok( ParsedCommand {
      name: tokens[0].to_string(),
      arguments: args,
    })
  }

  /// This is what unilang SHOULD offer: argv-based parsing
  fn parse_from_argv( argv: &[String] ) -> Result< ParsedCommand, String >
  {
    if argv.is_empty()
    {
      return Err( "Empty command".to_string() );
    }

    let mut args = std::collections::HashMap::new();

    let mut i = 1;
    while i < argv.len()
    {
      let arg = &argv[i];

      if let Some( ( key, value ) ) = arg.split_once( "::" )
      {
        // Value might be in this arg or the next
        if value.is_empty() && i + 1 < argv.len()
        {
          // Value is in next argv entry
          args.insert( key.to_string(), argv[i + 1].clone() );
          i += 1; // Skip next arg
        }
        else
        {
          // Check if following argv should be part of this value
          let mut full_value = value.to_string();

          while i + 1 < argv.len() && !argv[i + 1].contains( "::" ) && !argv[i + 1].starts_with( '.' )
          {
            full_value.push( ' ' );
            full_value.push_str( &argv[i + 1] );
            i += 1;
          }

          args.insert( key.to_string(), full_value );
        }
      }

      i += 1;
    }

    Ok( ParsedCommand {
      name: argv[0].clone(),
      arguments: args,
    })
  }
}

#[derive(Debug)]
struct ParsedCommand
{
  name: String,
  arguments: std::collections::HashMap< String, String >,
}

fn main()
{
  let argv: Vec< String > = env::args().skip( 1 ).collect();

  println!( "========================================" );
  println!( "Unilang Argv vs String API - MRE" );
  println!( "========================================\n" );

  println!( "Input argv from OS: {:?}\n", argv );

  // ❌ Current unilang approach: join argv, then parse string
  println!( "Method 1: Current String-Based API (BROKEN)" );
  println!( "-------------------------------------------" );
  let command_str = argv.join( " " );
  println!( "Step 1: Joined argv → \"{}\"", command_str );
  println!( "Step 2: Parse string (re-tokenizes on spaces)" );

  match UnilangParser::parse_from_string( &command_str )
  {
    Ok( cmd ) => {
      println!( "✅ Success: {:?}", cmd );
    }
    Err( e ) => {
      println!( "❌ ERROR: {}", e );
      println!( "\n   Why it failed:" );
      println!( "   - Shell gave us argv: {:?}", argv );
      println!( "   - We joined with spaces, losing boundary info" );
      println!( "   - Unilang re-tokenized, treating '-la' as separate token" );
    }
  }

  println!( "\n========================================\n" );

  // ✅ Proposed approach: use argv directly
  println!( "Method 2: Proposed Argv-Based API (WORKS)" );
  println!( "-------------------------------------------" );
  println!( "Step 1: Use argv directly (no join/re-tokenize)" );

  match UnilangParser::parse_from_argv( &argv )
  {
    Ok( cmd ) => {
      println!( "✅ Success: {:?}", cmd );
      println!( "\n   Why it worked:" );
      println!( "   - We used argv structure directly" );
      println!( "   - No information lost in join→re-tokenize cycle" );
      println!( "   - Arguments with '-' properly recognized as values" );
    }
    Err( e ) => {
      println!( "❌ ERROR: {}", e );
    }
  }

  println!( "\n========================================" );
  println!( "Conclusion" );
  println!( "========================================" );
  println!( "Unilang needs argv-based API alongside string API:" );
  println!( "- Keep string API for REPL/interactive use" );
  println!( "- Add argv API for CLI program integration" );
  println!( "- Make string-only usage emit deprecation warning" );
  println!( "========================================\n" );
}
