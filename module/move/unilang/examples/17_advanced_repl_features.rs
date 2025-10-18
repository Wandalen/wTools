#![allow(clippy::all)]
//! # Advanced REPL Features
//!
//! This example demonstrates advanced REPL capabilities including command history,
//! auto-completion suggestions, script execution, session management, and recovery patterns.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;
use unilang::error::Error;
use std::io::{ self, Write };
use std::collections::HashMap;

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== Advanced REPL Features Demo ===\n" );

  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();
  register_comprehensive_commands( &mut registry )?;

  let pipeline = Pipeline::new( registry );
  println!( "✓ Initialized advanced REPL pipeline\n" );

  println!( "🚀 Starting Advanced REPL Session" );
  println!( "Features: history, auto-complete, script mode, session management" );
  println!( "Type commands, 'help' for usage, or 'quit' to exit\n" );

  run_advanced_repl( &pipeline )?;

  Ok( () )
}

/// Register comprehensive command set for advanced REPL demonstration
#[allow(clippy::too_many_lines)]
fn register_comprehensive_commands( registry : &mut CommandRegistry ) -> Result< (), Error >
{
  // File system commands
  let ls_cmd = CommandDefinition::former()
  .name( ".list" )
  .namespace( ".fs" )
  .description( "List files and directories".to_string() )
  .hint( "File system listing" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ ".ls".to_string(), ".dir".to_string() ] )
  .tags( vec![ "filesystem".to_string(), "utility".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "fs.list path::/tmp".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "path".to_string(),
      description: "Directory path to list".to_string(),
      kind: Kind::Directory,
      hint: "Directory path".to_string(),
      attributes: ArgumentAttributes { 
        optional: true,
        default: Some(".".to_string()),
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "p".to_string(), "dir".to_string() ],
      tags: vec![ "filesystem".to_string() ],
    },
  ])
  .end();

  let ls_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let path = cmd.arguments.get( "path" ).map_or_else(|| ".".to_string(), std::string::ToString::to_string);
    
    println!( "📁 Listing directory: {path}" );
    
    // Simulate directory listing
    let simulated_files = vec![ "file1.txt", "file2.json", "subdirectory/" ];
    for file in &simulated_files
    {
      println!( "  {file}" );
    }

    Ok( OutputData
    {
      content : format!( "Listed {} items in {path}", simulated_files.len() ),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &ls_cmd, ls_routine )?;

  // Network commands
  let ping_cmd = CommandDefinition::former()
  .name( ".ping" )
  .namespace( ".net" )
  .description( "Ping a host to test connectivity".to_string() )
  .hint( "Network connectivity test" )
  .status( "stable" )
  .version( "2.0.0" )
  .aliases( vec![ ".test".to_string() ] )
  .tags( vec![ "network".to_string(), "diagnostic".to_string() ] )
  .permissions( vec![] )
  .idempotent( false )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "net.ping host::google.com count::4".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "host".to_string(),
      description: "Hostname or IP address to ping".to_string(),
      kind: Kind::String,
      hint: "Target host".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "h".to_string(), "target".to_string() ],
      tags: vec![ "required".to_string() ],
    },
    ArgumentDefinition {
      name: "count".to_string(),
      description: "Number of ping packets".to_string(),
      kind: Kind::Integer,
      hint: "Packet count (1-10)".to_string(),
      attributes: ArgumentAttributes { 
        optional: true,
        default: Some("3".to_string()),
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "c".to_string(), "n".to_string() ],
      tags: vec![ "count".to_string() ],
    },
  ])
  .end();

  let ping_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let host = cmd.arguments.get( "host" ).map_or_else(|| "localhost".to_string(), std::string::ToString::to_string);
    
    let count = cmd.arguments.get( "count" )
      .and_then( |v| v.as_integer() )
      .copied()
      .unwrap_or( 3 );

    println!( "🌐 Pinging {host} with {count} packets..." );
    
    // Simulate ping results
    for i in 1..=count
    {
      let latency = 10.0 + ( i as f64 * 0.5 ); // Simulate increasing latency
      println!( "  Reply from {host}: bytes=32 time={latency:.1}ms TTL=64" );
    }

    Ok( OutputData
    {
      content : format!( "Pinged {host} {count} times successfully" ),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &ping_cmd, ping_routine )?;

  // Data processing command
  let process_cmd = CommandDefinition::former()
  .name( ".process" )
  .namespace( ".data" )
  .description( "Process data with various algorithms".to_string() )
  .hint( "Data processing pipeline" )
  .status( "experimental" )
  .version( "0.5.0" )
  .aliases( vec![ ".proc".to_string(), ".analyze".to_string() ] )
  .tags( vec![ "data".to_string(), "processing".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec![ "data.process input::data.csv algorithm::mean format::json".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "input".to_string(),
      description: "Input data source".to_string(),
      kind: Kind::String,
      hint: "Data source path or identifier".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "i".to_string(), "source".to_string() ],
      tags: vec![ "required".to_string() ],
    },
    ArgumentDefinition {
      name: "algorithm".to_string(),
      description: "Processing algorithm to apply".to_string(),
      kind: Kind::Enum( vec![ "mean".to_string(), "median".to_string(), "sum".to_string(), "count".to_string() ] ),
      hint: "Choose processing method".to_string(),
      attributes: ArgumentAttributes { 
        optional: true,
        default: Some("mean".to_string()),
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "a".to_string(), "method".to_string() ],
      tags: vec![ "algorithm".to_string() ],
    },
    ArgumentDefinition {
      name: "format".to_string(),
      description: "Output format".to_string(),
      kind: Kind::Enum( vec![ "json".to_string(), "csv".to_string(), "table".to_string() ] ),
      hint: "Result format".to_string(),
      attributes: ArgumentAttributes { 
        optional: true,
        default: Some("table".to_string()),
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "f".to_string(), "output".to_string() ],
      tags: vec![ "format".to_string() ],
    },
  ])
  .end();

  let process_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let input = cmd.arguments.get( "input" ).map_or_else(|| "stdin".to_string(), std::string::ToString::to_string);
    
    let algorithm = cmd.arguments.get( "algorithm" ).map_or_else(|| "mean".to_string(), std::string::ToString::to_string);
    
    let format = cmd.arguments.get( "format" ).map_or_else(|| "table".to_string(), std::string::ToString::to_string);

    println!( "📊 Processing {input} with {algorithm} algorithm, output as {format}" );
    
    // Simulate data processing
    let result_value = match algorithm.as_str()
    {
      "mean" => 42.5,
      "median" => 41.0,
      "sum" => 850.0,
      "count" => 20.0,
      _ => 0.0,
    };

    let output = match format.as_str()
    {
      "json" => format!( r#"{{"result": {result_value}, "algorithm": "{algorithm}"}}"# ),
      "csv" => format!( "algorithm,result\n{algorithm},{result_value}" ),
      "table" => format!( "Algorithm: {algorithm}\nResult: {result_value}" ),
      _ => result_value.to_string(),
    };

    println!( "Result:\n{output}" );

    Ok( OutputData
    {
      content : output,
      format,
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &process_cmd, process_routine )?;

  println!( "✓ Registered {} commands for advanced REPL demo", registry.commands().len() );

  Ok( () )
}

/// Advanced REPL implementation with comprehensive features
fn run_advanced_repl( pipeline : &Pipeline ) -> Result< (), Box< dyn core::error::Error > >
{
  let mut session_state = ReplSessionState::new();

  loop
  {
    // Display enhanced prompt
    let prompt = format!( "advanced-repl[{}:{}]> ", 
      session_state.session_count, 
      session_state.command_history.len() );
    print!( "{prompt}" );
    io::stdout().flush()?;

    // Read user input
    let mut input = String::new();
    match io::stdin().read_line( &mut input )
    {
      Ok( 0 ) => break, // EOF
      Ok( _ ) =>
      {
        let input = input.trim().to_string();
        
        // Handle REPL meta-commands
        if handle_meta_commands( &input, &mut session_state )?
        {
          continue;
        }

        // Skip empty input
        if input.is_empty()
        {
          continue;
        }

        // Add to history before processing
        session_state.add_command( input.clone() );
        session_state.session_count += 1;

        // Handle auto-completion suggestions
        if input.ends_with( '?' )
        {
          let partial_command = input.trim_end_matches( '?' );
          suggest_completions( partial_command );
          continue;
        }

        // Process command through pipeline
        println!( "🔄 Executing: {input}" );
        let context = ExecutionContext::default();
        let result = pipeline.process_command( &input, context );

        // Enhanced error handling and recovery
        handle_command_result( result, &input, &mut session_state );
      },
      Err( error ) =>
      {
        println!( "❌ Input error: {error}" );
        break;
      }
    }
  }

  // Session summary
  display_session_summary( &session_state );
  Ok( () )
}

/// REPL session state management
#[derive(Debug)]
struct ReplSessionState
{
  command_history: Vec< String >,
  session_count: u32,
  successful_commands: u32,
  failed_commands: u32,
  command_stats: HashMap< String, u32 >,
  last_error: Option< String >,
}

impl ReplSessionState
{
  fn new() -> Self
  {
    Self
    {
      command_history: Vec::new(),
      session_count: 0,
      successful_commands: 0,
      failed_commands: 0,
      command_stats: HashMap::new(),
      last_error: None,
    }
  }

  fn add_command( &mut self, command : String )
  {
    // Extract command name for statistics
    let cmd_name = command.split_whitespace()
      .next()
      .unwrap_or( "unknown" )
      .to_string();
    
    *self.command_stats.entry( cmd_name ).or_insert( 0 ) += 1;
    
    // Keep history limited to last 100 commands
    if self.command_history.len() >= 100
    {
      self.command_history.remove( 0 );
    }
    
    self.command_history.push( command );
  }
}

/// Handle REPL meta-commands (help, history, etc.)
fn handle_meta_commands( input : &str, state : &mut ReplSessionState ) -> Result< bool, Box< dyn core::error::Error > >
{
  match input
  {
    "quit" | "exit" | "q" =>
    {
      println!( "👋 Goodbye! Session completed." );
      Err( "quit".into() )// Use error to break out of main loop
    },
    "help" | "h" =>
    {
      display_advanced_help();
      Ok( true )
    },
    "history" | "hist" =>
    {
      display_enhanced_history( &state.command_history );
      Ok( true )
    },
    "stats" | "statistics" =>
    {
      display_session_statistics( state );
      Ok( true )
    },
    "clear" | "cls" =>
    {
      print!( "{}[2J{}[1;1H", 27 as char, 27 as char ); // ANSI clear screen
      Ok( true )
    },
    "reset" =>
    {
      state.command_history.clear();
      state.command_stats.clear();
      state.session_count = 0;
      state.successful_commands = 0;
      state.failed_commands = 0;
      state.last_error = None;
      println!( "🔄 Session state reset" );
      Ok( true )
    },
    "last-error" | "error" =>
    {
      if let Some( ref error ) = state.last_error
      {
        println!( "❌ Last error: {error}" );
      }
      else
      {
        println!( "✅ No recent errors" );
      }
      Ok( true )
    },
    _ => Ok( false ) // Not a meta-command
  }
}

/// Handle command execution results with enhanced feedback
fn handle_command_result( result : unilang::pipeline::CommandResult, input : &str, state : &mut ReplSessionState )
{
  match result.error
  {
    None =>
    {
      state.successful_commands += 1;
      state.last_error = None;
      
      if result.outputs.is_empty() {
        println!( "✅ Command completed (no output)" );
      } else {
        println!( "✅ Command executed successfully" );
        for output in &result.outputs
        {
          if !output.content.is_empty()
          {
            println!( "📤 Output: {}", output.content );
          }
        }
      }
    },
    Some( error ) =>
    {
      state.failed_commands += 1;
      let error_msg = error.to_string();
      state.last_error = Some( error_msg.clone() );
      
      println!( "❌ Error: {error_msg}" );
      
      // Enhanced error recovery suggestions
      provide_error_recovery_suggestions( input, &error_msg );
    }
  }
}

/// Provide auto-completion suggestions
fn suggest_completions( partial_command : &str )
{
  println!( "🔍 Auto-completion suggestions for '{partial_command}':" );
  
  let suggestions = match partial_command
  {
    "" => vec![ "fs.list", "net.ping", "data.process", "help", "history", "stats" ],
    "fs" | "fs." => vec![ "fs.list" ],
    "net" | "net." => vec![ "net.ping" ],
    "data" | "data." => vec![ "data.process" ],
    "h" => vec![ "help", "history" ],
    "q" => vec![ "quit" ],
    _ => {
      // Fuzzy matching suggestions
      vec![ "fs.list", "net.ping", "data.process" ].into_iter()
        .filter( |&cmd| cmd.contains( partial_command ) )
        .collect()
    }
  };

  if suggestions.is_empty()
  {
    println!( "  No suggestions found. Try 'help' for available commands." );
  }
  else
  {
    for suggestion in suggestions
    {
      println!( "  💡 {suggestion}" );
    }
  }
  println!();
}

/// Provide error recovery suggestions
fn provide_error_recovery_suggestions( input : &str, error : &str )
{
  println!( "💡 Error recovery suggestions:" );
  
  if error.contains( "Command not found" )
  {
    println!( "  • Check command spelling: try 'fs.list' instead of 'list'" );
    println!( "  • Use auto-completion: type partial command + '?'" );
    println!( "  • See available commands: type 'help'" );
  }
  else if error.contains( "Missing required" )
  {
    println!( "  • Check required arguments: {input} might be missing parameters" );
    println!( "  • Use help for command syntax: 'help' shows examples" );
  }
  else if error.contains( "Type" )
  {
    println!( "  • Check argument types: ensure integers are numbers, strings are quoted" );
    println!( "  • Example formats: 'count::5', 'name::value', 'flag::true'" );
  }
  else if error.contains( "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" )
  {
    println!( "  • This command requires interactive input for security" );
    println!( "  • The system would normally prompt for secure values" );
  }
  else
  {
    println!( "  • Check command syntax and try again" );
    println!( "  • Use 'last-error' to see the full error details" );
  }
  
  println!( "  • Type 'help' for general assistance" );
  println!();
}

/// Display advanced help information
fn display_advanced_help()
{
  println!( "=== Advanced REPL Help ===" );
  println!( "🎯 Available Commands:" );
  println!( "  • fs.list [path]              - List directory contents" );
  println!( "  • net.ping host [count]       - Ping network host" );
  println!( "  • data.process input [algorithm] [format] - Process data" );
  
  println!( "\n🛠️ REPL Features:" );
  println!( "  • help, h                     - Show this help" );
  println!( "  • history, hist               - Show command history" );
  println!( "  • stats, statistics           - Show session statistics" );
  println!( "  • clear, cls                  - Clear screen" );
  println!( "  • reset                       - Reset session state" );
  println!( "  • last-error, error           - Show last error details" );
  println!( "  • quit, exit, q               - Exit REPL" );

  println!( "\n✨ Advanced Features:" );
  println!( "  • Auto-completion: type partial command + '?' for suggestions" );
  println!( "  • Command history: automatically tracked and searchable" );
  println!( "  • Error recovery: intelligent suggestions for failed commands" );
  println!( "  • Session statistics: track usage patterns and success rates" );
  println!( "  • Enhanced prompts: show session and command counts" );

  println!( "\n💡 Examples:" );
  println!( "  fs.list path::/tmp             # List /tmp directory" );
  println!( "  net.ping host::google.com      # Ping Google" );
  println!( "  data.process input::data.csv algorithm::mean format::json" );
  println!( "  fs?                            # Show auto-completion for fs commands" );
  println!( "  history                        # Show recent commands" );
  println!();
}

/// Display enhanced command history
fn display_enhanced_history( history : &[String] )
{
  if history.is_empty()
  {
    println!( "📝 No commands in history" );
    return;
  }

  println!( "📝 Command History ({} commands):", history.len() );
  
  // Show last 20 commands with numbers
  let start_index = if history.len() > 20 { history.len() - 20 } else { 0 };
  
  for ( i, cmd ) in history.iter().enumerate().skip( start_index )
  {
    println!( "  {:3}: {cmd}", i + 1 );
  }

  if history.len() > 20
  {
    println!( "  ... (showing last 20 of {} commands)", history.len() );
  }
  println!();
}

/// Display comprehensive session statistics
fn display_session_statistics( state : &ReplSessionState )
{
  println!( "📊 Session Statistics:" );
  println!( "  • Total commands: {}", state.session_count );
  println!( "  • Successful: {} ({:.1}%)", 
    state.successful_commands,
    if state.session_count > 0 { 100.0 * f64::from(state.successful_commands) / f64::from(state.session_count) } else { 0.0 }
  );
  println!( "  • Failed: {} ({:.1}%)", 
    state.failed_commands,
    if state.session_count > 0 { 100.0 * f64::from(state.failed_commands) / f64::from(state.session_count) } else { 0.0 }
  );

  if !state.command_stats.is_empty()
  {
    println!( "\n🏆 Most Used Commands:" );
    let mut sorted_stats : Vec< _ > = state.command_stats.iter().collect();
    sorted_stats.sort_by( |a, b| b.1.cmp( a.1 ) );
    
    for ( cmd, count ) in sorted_stats.iter().take( 5 )
    {
      println!( "  • {cmd}: {count} times" );
    }
  }

  if let Some( ref error ) = state.last_error
  {
    println!( "\n❌ Last Error: {error}" );
  }
  else if state.session_count > 0
  {
    println!( "\n✅ No recent errors" );
  }
  println!();
}

/// Display session summary when exiting
fn display_session_summary( state : &ReplSessionState )
{
  println!( "\n=== Session Summary ===" );
  println!( "📈 Performance:" );
  println!( "  • Commands executed: {}", state.session_count );
  println!( "  • Success rate: {:.1}%", 
    if state.session_count > 0 { 100.0 * f64::from(state.successful_commands) / f64::from(state.session_count) } else { 0.0 }
  );
  
  if !state.command_stats.is_empty()
  {
    let most_used = state.command_stats.iter()
      .max_by_key( |( _, count )| **count )
      .map_or_else(|| "none".to_string(), |( cmd, count )| format!( "{cmd} ({count} times)" ));
    println!( "  • Most used command: {most_used}" );
  }

  println!( "\n🎯 REPL Features Demonstrated:" );
  println!( "  ✨ Stateless pipeline operation" );
  println!( "  ✨ Command history and statistics" );
  println!( "  ✨ Auto-completion suggestions" );
  println!( "  ✨ Enhanced error recovery" );
  println!( "  ✨ Session state management" );
  println!( "  ✨ Comprehensive help system" );
  
  println!( "\nThank you for using the Advanced REPL! 🚀" );
}