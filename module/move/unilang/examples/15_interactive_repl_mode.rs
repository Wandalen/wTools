//! # Interactive REPL Mode
//!
//! This example demonstrates the interactive Read-Eval-Print Loop (REPL) capabilities
//! of Unilang, including interactive argument handling and stateless operation.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::pipeline::Pipeline;
use unilang::error::Error;
#[ cfg( all( feature = "repl", not( feature = "enhanced_repl" ) ) ) ]
use std::io::{ self, Write };

#[ cfg( feature = "enhanced_repl" ) ]
use rustyline::DefaultEditor;
#[ cfg( feature = "enhanced_repl" ) ]
use rustyline::error::ReadlineError;
#[ cfg( feature = "enhanced_repl" ) ]
use rustyline::history::History;
#[ cfg( feature = "enhanced_repl" ) ]

#[ cfg( feature = "repl" ) ]
fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== Interactive REPL Mode Demo ===\n" );

  let mut registry = CommandRegistry::new();

  // Step 1: Register commands with interactive arguments
  register_interactive_commands( &mut registry )?;

  // Step 2: Create stateless pipeline for REPL
  let pipeline = Pipeline::new( registry );
  println!( "✓ Initialized stateless pipeline for REPL operation\n" );

  // Step 3: Start interactive session
  println!( "🚀 Starting Interactive REPL Session" );
  
  #[ cfg( feature = "enhanced_repl" ) ]
  println!( "Enhanced REPL: Arrow keys, command history, and auto-completion enabled" );
  
  #[ cfg( all( feature = "repl", not( feature = "enhanced_repl" ) ) ) ]
  println!( "Basic REPL: Standard input/output (no arrow key support)" );
  
  println!( "Type commands or 'help' for available commands, 'quit' to exit\n" );

  #[ cfg( feature = "enhanced_repl" ) ]
  run_enhanced_repl( &pipeline )?;
  
  #[ cfg( all( feature = "repl", not( feature = "enhanced_repl" ) ) ) ]
  run_basic_repl( &pipeline )?;

  Ok( () )
}

#[ cfg( not( feature = "repl" ) ) ]
fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== Interactive REPL Mode Demo ===\n" );
  println!( "❌ REPL functionality is not enabled." );
  println!( "This example requires the 'repl' feature to be enabled." );
  println!();
  println!( "Available options:" );
  println!( "  cargo run --example 15_interactive_repl_mode --features repl" );
  println!( "  cargo run --example 15_interactive_repl_mode --features enhanced_repl" );
  println!( "  cargo run --example 15_interactive_repl_mode  (default includes repl)" );
  println!();
  println!( "💡 The 'repl' feature provides basic REPL functionality" );
  println!( "💡 The 'enhanced_repl' feature adds arrow keys, history, and tab completion" );
  
  Ok( () )
}

/// Register commands that demonstrate interactive argument handling
#[ cfg( feature = "repl" ) ]
fn register_interactive_commands( registry : &mut CommandRegistry ) -> Result< (), Error >
{
  // Command with interactive password input
  let login_cmd = CommandDefinition::former()
  .name( "login" )
  .namespace( ".auth" )
  .description( "User authentication with interactive password".to_string() )
  .hint( "Secure login command" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "signin".to_string() ] )
  .tags( vec![ "auth".to_string(), "security".to_string() ] )
  .permissions( vec![] )
  .idempotent( false )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec![ "auth.login username::john".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "username".to_string(),
      description: "User login name".to_string(),
      kind: Kind::String,
      hint: "Your username".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "user".to_string() ],
      tags: vec![ "required".to_string() ],
    },
    ArgumentDefinition {
      name: "password".to_string(),
      description: "User password (will be requested interactively)".to_string(),
      kind: Kind::String,
      hint: "Your password (interactive input)".to_string(),
      attributes: ArgumentAttributes { 
        optional: false, 
        interactive: true, 
        sensitive: true,
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "pass".to_string() ],
      tags: vec![ "required".to_string(), "secure".to_string() ],
    },
  ])
  .end();

  let login_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    // In a real implementation, this would handle the interactive password request
    println!( "🔐 Processing login for user: {}", 
      cmd.arguments.get( "username" ).map_or( "unknown".to_string(), std::string::ToString::to_string ) );
    
    // Simulate authentication
    println!( "✓ Authentication successful (demo mode)" );
    
    Ok( OutputData
    {
      content : "Login successful".to_string(),
      format : "text".to_string(),
    })
  });

  registry.command_add_runtime( &login_cmd, login_routine )?;

  // Command with optional interactive input
  let config_cmd = CommandDefinition::former()
  .name( "configure" )
  .namespace( ".system" )
  .description( "System configuration with optional interactive setup".to_string() )
  .hint( "Configuration management" )
  .status( "stable" )
  .version( "2.1.0" )
  .aliases( vec![ "config".to_string(), "setup".to_string() ] )
  .tags( vec![ "config".to_string(), "system".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "PUT".to_string() )
  .examples( vec![ "system.configure host::localhost port::8080".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "host".to_string(),
      description: "Server hostname".to_string(),
      kind: Kind::String,
      hint: "Server address".to_string(),
      attributes: ArgumentAttributes { 
        optional: true,
        default: Some("localhost".to_string()),
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "h".to_string() ],
      tags: vec![ "network".to_string() ],
    },
    ArgumentDefinition {
      name: "port".to_string(),
      description: "Server port number".to_string(),
      kind: Kind::Integer,
      hint: "Port number (1-65535)".to_string(),
      attributes: ArgumentAttributes { 
        optional: true,
        default: Some("8080".to_string()),
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "p".to_string() ],
      tags: vec![ "network".to_string() ],
    },
    ArgumentDefinition {
      name: "api_key".to_string(),
      description: "API key for authentication (interactive if not provided)".to_string(),
      kind: Kind::String,
      hint: "Secret API key".to_string(),
      attributes: ArgumentAttributes { 
        optional: true, 
        interactive: true, 
        sensitive: true,
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "key".to_string() ],
      tags: vec![ "security".to_string() ],
    },
  ])
  .end();

  let config_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "⚙️ Configuring system:" );
    for ( name, value ) in &cmd.arguments
    {
      let display_value = if name == "api_key" { "[HIDDEN]" } else { &value.to_string() };
      println!( "  • {name}: {display_value}" );
    }
    
    Ok( OutputData
    {
      content : "Configuration updated successfully".to_string(),
      format : "text".to_string(),
    })
  });

  registry.command_add_runtime( &config_cmd, config_routine )?;

  // Regular command for comparison
  let info_cmd = CommandDefinition::former()
  .name( "info" )
  .namespace( ".system" )
  .description( "Display system information".to_string() )
  .hint( "System info display" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "status".to_string() ] )
  .tags( vec![ "info".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "system.info".to_string() ] )
  .arguments( vec![] )
  .end();

  let info_routine = Box::new( | _cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "🖥️ System Information:" );
    println!( "  • OS: Linux" );
    println!( "  • Architecture: x86_64" );
    println!( "  • Unilang Version: 0.5.0" );
    println!( "  • REPL Mode: Active" );
    
    Ok( OutputData
    {
      content : "System info displayed".to_string(),
      format : "text".to_string(),
    })
  });

  registry.command_add_runtime( &info_cmd, info_routine )?;

  // Note: .version is a static command that appears in help but has no executable routine
  // This is a limitation of the static command system - we can only add routines to dynamic commands

  println!( "✓ Registered {} interactive commands", registry.commands().len() );

  Ok( () )
}

/// Run the basic interactive REPL loop (standard input/output)
#[ cfg( all( feature = "repl", not( feature = "enhanced_repl" ) ) ) ]
fn run_basic_repl( pipeline : &Pipeline ) -> Result< (), Box< dyn std::error::Error > >
{
  let mut command_history = Vec::new();
  let mut session_counter = 0u32;

  loop
  {
    // Display prompt
    print!( "unilang[{session_counter}]> " );
    io::stdout().flush()?;

    // Read user input
    let mut input = String::new();
    match io::stdin().read_line( &mut input )
    {
      Ok( 0 ) => break, // EOF
      Ok( _ ) =>
      {
        let input = input.trim();
        
        // Handle special REPL commands
        match input
        {
          "" => continue, // Empty input
          "quit" | "exit" | "q" =>
          {
            println!( "👋 Goodbye! Executed {} commands this session.", command_history.len() );
            break;
          },
          "help" | "h" =>
          {
            display_repl_help( pipeline.registry() );
            continue;
          },
          "history" =>
          {
            display_command_history( &command_history );
            continue;
          },
          "clear" =>
          {
            print!( "{}[2J{}[1;1H", 27 as char, 27 as char ); // ANSI clear screen
            continue;
          },
          _ => {}
        }

        // Add to history
        command_history.push( input.to_string() );
        session_counter += 1;

        // Process command through pipeline
        println!( "🔄 Processing: {input}" );
        let context = unilang::interpreter::ExecutionContext::default();
        let result = pipeline.process_command( input, context );
        match result.error
        {
          None =>
          {
            if !result.outputs.is_empty()
            {
              for output in &result.outputs
              {
                if !output.content.is_empty()
                {
                  println!( "✅ {}", output.content );
                }
              }
            }
          },
          Some( error ) =>
          {
            if error.contains( "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" ) || error.contains( "Interactive Argument Required" )
            {
              println!( "🔒 Interactive input required for secure argument" );
              println!( "💡 In a real application, this would prompt for secure input" );
              
              // Simulate interactive input (in real implementation, would use secure input)
              print!( "Enter value securely: " );
              io::stdout().flush()?;
              let mut _secure_input = String::new();
              io::stdin().read_line( &mut _secure_input )?;
              
              println!( "✓ Interactive input received (demo mode)" );
              println!( "  In production: password would be masked, API keys validated" );
            }
            else if error.contains( "No executable routine found" ) && input == ".version"
            {
              println!( "❌ The .version command is a static command without an executable routine" );
              println!( "💡 This is a known limitation - static commands appear in help but can't be executed" );
              println!( "📝 Framework Version: 0.7.0 (demo mode)" );
            }
            else if error.contains( "Available commands:" )
            {
              // Special handling for help-like error messages - convert to user-friendly help
              if input == "."
              {
                println!( "📋 Available Commands:" );
                // Extract and display just the command list from the error message
                let lines : Vec< &str > = error.lines().collect();
                for line in lines.iter().skip( 1 ) // Skip the first "Available commands:" line
                {
                  if line.trim().is_empty()
                  {
                    continue;
                  }
                  if line.contains( "Use '<command> ?' to get detailed help" )
                  {
                    break;
                  }
                  println!( "{line}" );
                }
                println!( "\n💡 Use 'help' for detailed information about each command" );
              }
              else
              {
                println!( "❌ Command not found: '{input}'" );
                println!( "💡 Type 'help' to see available commands, or '.' for a quick list" );
              }
            }
            else
            {
              println!( "❌ Error: {error}" );
              println!( "💡 Tip: Type 'help' for available commands" );
            }
          }
        }
        
        println!(); // Add spacing
      },
      Err( error ) =>
      {
        println!( "❌ Input error: {error}" );
        break;
      }
    }
  }

  Ok( () )
}

/// Run the enhanced interactive REPL loop (with rustyline for history/arrows)
#[ cfg( feature = "enhanced_repl" ) ]
fn run_enhanced_repl( pipeline : &Pipeline ) -> Result< (), Box< dyn core::error::Error > >
{
  let mut rl = DefaultEditor::new()?;
  let mut session_counter = 0u32;
  
  // Add command completion
  // TODO: Implement custom completer for command names
  
  println!( "🎨 Enhanced REPL Features:" );
  println!( "  • ↑/↓ Arrow keys for command history" );
  println!( "  • Tab completion (basic)" );
  println!( "  • Ctrl+C to quit, Ctrl+L to clear" );
  println!();
  
  // Check if we're running in an interactive terminal
  let is_tty = atty::is( atty::Stream::Stdin );
  
  if is_tty
  {
    println!( "💡 Arrow Key Usage:" );
    println!( "  • Enter some commands first" );
    println!( "  • Then use ↑ to go back through history" );
    println!( "  • Use ↓ to go forward through history" );
    println!( "  • Press Enter to execute the recalled command" );
  }
  else
  {
    println!( "⚠️  Note: Arrow keys only work in interactive terminals" );
    println!( "   Current session: Non-interactive (piped input detected)" );
    println!( "   For arrow key support, run directly in terminal" );
  }
  println!();

  loop
  {
    let prompt = format!( "unilang[{session_counter}]> " );
    
    match rl.readline( &prompt )
    {
      Ok( input ) =>
      {
        let input = input.trim();
        
        // Handle special REPL commands (don't add these to command history)
        match input
        {
          "" => continue, // Empty input
          "quit" | "exit" | "q" =>
          {
            println!( "👋 Goodbye! Executed {session_counter} commands this session." );
            break;
          },
          "help" | "h" =>
          {
            display_repl_help( pipeline.registry() );
            continue;
          },
          "history" =>
          {
            display_rustyline_history( &rl );
            continue;
          },
          "clear" =>
          {
            print!( "{}[2J{}[1;1H", 27 as char, 27 as char ); // ANSI clear screen
            continue;
          },
          _ => {
            // Only add real commands to history, not REPL meta-commands
            rl.add_history_entry( input )?;
            session_counter += 1;
          }
        }

        // Process command through pipeline
        println!( "🔄 Processing: {input}" );
        let context = unilang::interpreter::ExecutionContext::default();
        let result = pipeline.process_command( input, context );
        
        match result.error
        {
          None =>
          {
            if !result.outputs.is_empty()
            {
              for output in &result.outputs
              {
                if !output.content.is_empty()
                {
                  println!( "✅ {}", output.content );
                }
              }
            }
          },
          Some( error ) =>
          {
            if error.contains( "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" ) || error.contains( "Interactive Argument Required" )
            {
              println!( "🔒 Interactive input required for secure argument" );
              println!( "💡 In a real application, this would prompt for secure input" );
              
              // Simulate interactive input (in real implementation, would use secure input)
              match rl.readline( "Enter value securely: " )
              {
                Ok( secure_input ) =>
                {
                  rl.add_history_entry( "[INTERACTIVE INPUT]" )?; // Don't store actual secure input
                  println!( "✓ Interactive input received (demo mode)" );
                  println!( "  In production: password would be masked, API keys validated" );
                  println!( "  Entered: {} characters", secure_input.len() );
                },
                Err( _ ) =>
                {
                  println!( "❌ Interactive input cancelled" );
                }
              }
            }
            else if error.contains( "No executable routine found" ) && input == ".version"
            {
              println!( "❌ The .version command is a static command without an executable routine" );
              println!( "💡 This is a known limitation - static commands appear in help but can't be executed" );
              println!( "📝 Framework Version: 0.7.0 (demo mode)" );
            }
            else if error.contains( "Available commands:" )
            {
              // Special handling for help-like error messages - convert to user-friendly help
              if input == "."
              {
                println!( "📋 Available Commands:" );
                // Extract and display just the command list from the error message
                let lines : Vec< &str > = error.lines().collect();
                for line in lines.iter().skip( 1 ) // Skip the first "Available commands:" line
                {
                  if line.trim().is_empty()
                  {
                    continue;
                  }
                  if line.contains( "Use '<command> ?' to get detailed help" )
                  {
                    break;
                  }
                  println!( "{line}" );
                }
                println!( "\n💡 Use 'help' for detailed information about each command" );
              }
              else
              {
                println!( "❌ Command not found: '{input}'" );
                println!( "💡 Type 'help' to see available commands, or '.' for a quick list" );
              }
            }
            else
            {
              println!( "❌ Error: {error}" );
              println!( "💡 Tip: Type 'help' for available commands" );
            }
          }
        }
        
        println!(); // Add spacing
      },
      Err( ReadlineError::Interrupted ) => // Ctrl+C
      {
        println!( "👋 Goodbye! (Ctrl+C)" );
        break;
      },
      Err( ReadlineError::Eof ) => // Ctrl+D or EOF
      {
        println!( "👋 Goodbye! (EOF)" );
        break;
      },
      Err( error ) =>
      {
        println!( "❌ Input error: {error}" );
        break;
      }
    }
  }

  Ok( () )
}

/// Display rustyline command history
#[ cfg( feature = "enhanced_repl" ) ]
fn display_rustyline_history( rl : &DefaultEditor )
{
  let history = rl.history();
  if history.is_empty()
  {
    println!( "📝 No commands in history" );
    return;
  }

  println!( "📝 Command History ({} commands):", history.len() );
  for ( i, cmd ) in history.iter().enumerate()
  {
    println!( "  {:3}: {cmd}", i + 1 );
  }
}

/// Display REPL help information
#[ cfg( feature = "repl" ) ]
fn display_repl_help( registry : &CommandRegistry )
{
  println!( "=== REPL Help ===" );
  println!( "🎯 Available Commands:" );
  
  for ( name, command ) in registry.commands()
  {
    let interactive_args = command.arguments.iter()
      .filter( |arg| arg.attributes.interactive )
      .count();
    
    let interactive_marker = if interactive_args > 0 { " 🔒" } else { "" };
    
    println!( "  • {name}{interactive_marker}" );
    println!( "    {}", command.description );
    
    if !command.aliases.is_empty()
    {
      println!( "    Aliases: {}", command.aliases.join( ", " ) );
    }
    
    if interactive_args > 0
    {
      println!( "    Note: Contains {interactive_args} interactive argument(s)" );
    }
    println!();
  }

  println!( "🛠️ REPL Commands:" );
  println!( "  • help, h        - Show this help" );
  println!( "  • history        - Show command history" );
  println!( "  • clear          - Clear screen" );
  println!( "  • quit, exit, q  - Exit REPL" );

  println!( "\n💡 Interactive Arguments:" );
  println!( "  Commands marked with 🔒 have interactive arguments that will" );
  println!( "  prompt for secure input when the argument is marked as required" );
  println!( "  but not provided in the command line." );

  println!( "\n🔍 Examples:" );
  println!( "  system.info                    # Simple command" );
  println!( "  auth.login username::john      # Will prompt for password" );
  println!( "  system.configure host::example.com port::9000  # Optional interactive" );
}

/// Display command history
#[ cfg( all( feature = "repl", not( feature = "enhanced_repl" ) ) ) ]
fn display_command_history( history : &[String] )
{
  if history.is_empty()
  {
    println!( "📝 No commands in history" );
    return;
  }

  println!( "📝 Command History ({} commands):", history.len() );
  for ( i, cmd ) in history.iter().enumerate()
  {
    println!( "  {:3}: {cmd}", i + 1 );
  }
}

/// Main REPL mode features demonstrated:
#[ cfg( feature = "repl" ) ]
#[allow(dead_code)]
fn repl_features_summary()
{
  println!( "=== REPL Mode Features ===\n" );
  
  println!( "🔄 Stateless Operation:" );
  println!( "  • Pipeline components are reusable across commands" );
  println!( "  • No shared state between command executions" );
  println!( "  • Each command is processed independently" );
  println!( "  • Memory efficient: no accumulating state" );

  println!( "\n🔒 Interactive Argument Support:" );
  println!( "  • UNILANG_ARGUMENT_INTERACTIVE_REQUIRED error signaling" );
  println!( "  • Secure input prompting for passwords/API keys" );
  println!( "  • Optional interactive arguments with defaults" );
  println!( "  • Sensitive argument masking in logs/history" );

  println!( "\n🎯 REPL-Specific Features:" );
  println!( "  • Command history tracking" );
  println!( "  • Built-in help system" );
  println!( "  • Clear screen functionality" );
  println!( "  • Graceful error handling" );
  println!( "  • Session management" );

  println!( "\n⚡ Performance Benefits:" );
  println!( "  • Static command registry: zero-cost lookups" );
  println!( "  • Reusable parser and analyzer instances" );
  println!( "  • No startup cost per command" );
  println!( "  • Efficient memory usage" );

  println!( "\n🛠️ Developer Experience:" );
  println!( "  • Real-time command testing" );
  println!( "  • Interactive development workflow" );
  println!( "  • Easy debugging and experimentation" );
  println!( "  • Live error feedback" );
}