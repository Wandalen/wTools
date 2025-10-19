#![allow(clippy::all)]
//! # Help Conventions Demo
//!
//! This example demonstrates the standardized help conventions implemented in Unilang:

#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::cast_lossless ) ]
#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::too_many_lines ) ]
#![ allow( clippy::unnecessary_map_or ) ]
//! 1. Automatic `.command.help` generation for every registered command
//! 2. Universal `??` parameter support for alternative help access
//! 3. Developer-friendly APIs for help configuration
//!
//! ## Key Features Demonstrated:
//! - Mandatory automatic help command generation
//! - Per-command help configuration
//! - Multiple help access methods (?, ??, .command.help)
//! - Comprehensive help content formatting

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;
use unilang::error::Error;

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== Help Conventions Demo ===\n" );

  // Step 1: Create registry - help conventions are now mandatory and always enabled
  let mut registry = CommandRegistry::new();

  println!( "✓ Created registry - help conventions are mandatory (always enabled)\n" );

  // Step 2: Register commands with different help configurations
  register_demo_commands( &mut registry )?;

  // Step 3: Create pipeline for command processing
  let pipeline = Pipeline::new( registry );

  println!( "✓ Registered demo commands and created pipeline\n" );

  // Step 4: Demonstrate help conventions in action
  demonstrate_help_access_methods( &pipeline )?;

  println!( "\n🎉 Help conventions demo completed successfully!" );
  println!( "Key takeaways:" );
  println!( "  • Every command automatically gets a .command.help counterpart (mandatory)" );
  println!( "  • Users can access help via: ?, ??, or .command.help" );
  println!( "  • Help content is comprehensive and consistently formatted" );
  println!( "  • Help generation is now mandatory - no opt-out mechanism" );

  Ok( () )
}

/// Register demo commands showcasing different help convention features
fn register_demo_commands( registry : &mut CommandRegistry ) -> Result< (), Error >
{
  // Command 1: File system command with comprehensive help
  let fs_list_cmd = CommandDefinition::former()
    .name( ".fs.list" )
    .namespace( ".fs" )
    .description( "List files and directories with advanced filtering options" )
    .hint( "Advanced file listing" )
    .status( "stable" )
    .version( "2.1.0" )
    .tags( vec![ "filesystem".to_string(), "utility".to_string() ] )
    .aliases( vec![ ".ls".to_string(), ".dir".to_string() ] )
    .examples( vec![
      ".fs.list path::/home/user".to_string(),
      ".fs.list /tmp show_hidden::true".to_string(),
      ".ls -a".to_string()
    ])
    .arguments( vec![
      ArgumentDefinition {
        name : "path".to_string(),
        description : "Directory path to list (defaults to current directory)".to_string(),
        kind : Kind::Directory,
        hint : "Directory path".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( ".".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "p".to_string(), "dir".to_string() ],
        tags : vec![ "filesystem".to_string() ],
      },
      ArgumentDefinition {
        name : "show_hidden".to_string(),
        description : "Include hidden files in the listing".to_string(),
        kind : Kind::Boolean,
        hint : "Show hidden files".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "false".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "a".to_string(), "all".to_string() ],
        tags : vec![ "visibility".to_string() ],
      },
      ArgumentDefinition {
        name : "max_depth".to_string(),
        description : "Maximum directory depth to traverse".to_string(),
        kind : Kind::Integer,
        hint : "Max depth".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "1".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![
          ValidationRule::Min( 1.0 ),
          ValidationRule::Max( 10.0 )
        ],
        aliases : vec![ "d".to_string(), "depth".to_string() ],
        tags : vec![ "traversal".to_string() ],
      }
    ])
    .end();

  // Manually enable auto-help since builder method doesn't work yet
  let mut fs_list_cmd = fs_list_cmd;
  fs_list_cmd.auto_help_enabled = true;

  let fs_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let path = cmd.arguments.get( "path" ).map_or( ".".to_string(), std::string::ToString::to_string );
    let show_hidden = cmd.arguments.get( "show_hidden" ).map_or( false, | v | v.to_string() == "true" );
    let max_depth = cmd.arguments.get( "max_depth" ).map_or( 1, | v | v.to_string().parse().unwrap_or( 1 ) );

    println!( "📁 Listing directory: {}", path );
    println!( "   Show hidden: {}", show_hidden );
    println!( "   Max depth: {}", max_depth );

    let simulated_files = if show_hidden {
      vec![ ".hidden_config", "document.txt", "script.sh", ".env" ]
    } else {
      vec![ "document.txt", "script.sh" ]
    };

    for file in &simulated_files {
      println!( "   {}", file );
    }

    Ok( OutputData {
      content : format!( "Listed {} items in {}", simulated_files.len(), path ),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  registry.register_with_auto_help( fs_list_cmd, fs_routine )?;

  // Command 2: Network utility command
  let net_ping_cmd = CommandDefinition::former()
    .name( ".net.ping" )
    .namespace( ".net" )
    .description( "Ping a host to test network connectivity" )
    .hint( "Network ping utility" )
    .status( "stable" )
    .version( "1.5.0" )
    .tags( vec![ "network".to_string(), "diagnostic".to_string() ] )
    .aliases( vec![ ".ping".to_string() ] )
    .examples( vec![
      ".net.ping host::google.com".to_string(),
      ".ping 8.8.8.8 count::5".to_string()
    ])
    .arguments( vec![
      ArgumentDefinition {
        name : "host".to_string(),
        description : "Hostname or IP address to ping".to_string(),
        kind : Kind::String,
        hint : "Target host".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![
          ValidationRule::MinLength( 1 ),
          ValidationRule::MaxLength( 255 )
        ],
        aliases : vec![ "h".to_string(), "target".to_string() ],
        tags : vec![ "network".to_string() ],
      },
      ArgumentDefinition {
        name : "count".to_string(),
        description : "Number of ping packets to send".to_string(),
        kind : Kind::Integer,
        hint : "Packet count".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "4".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![
          ValidationRule::Min( 1.0 ),
          ValidationRule::Max( 100.0 )
        ],
        aliases : vec![ "c".to_string(), "n".to_string() ],
        tags : vec![ "quantity".to_string() ],
      }
    ])
    .end();

  // Manually enable auto-help since builder method doesn't work yet
  let mut net_ping_cmd = net_ping_cmd;
  net_ping_cmd.auto_help_enabled = true;

  let ping_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let host = cmd.arguments.get( "host" ).map_or( "localhost".to_string(), std::string::ToString::to_string );
    let count = cmd.arguments.get( "count" ).map_or( 4, | v | v.to_string().parse().unwrap_or( 4 ) );

    println!( "🌐 Pinging {} ({} packets)", host, count );

    for i in 1..=count {
      let response_time = 10.0 + ( i as f64 * 2.5 ); // Simulated response time
      println!( "   #{}: {} ms", i, response_time );
    }

    Ok( OutputData {
      content : format!( "Ping completed: {} packets sent to {}", count, host ),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  registry.register_with_auto_help( net_ping_cmd, ping_routine )?;

  // Command 3: System info command (minimal help example)
  let sys_info_cmd = CommandDefinition::former()
    .name( ".sys.info" )
    .namespace( ".sys" )
    .description( "Display system information" )
    .hint( "System info" )
    .status( "stable" )
    .version( "1.0.0" )
    .examples( vec![ ".sys.info".to_string() ] )
    .end();

  // Manually enable auto-help since builder method doesn't work yet
  let mut sys_info_cmd = sys_info_cmd;
  sys_info_cmd.auto_help_enabled = true;

  let info_routine = Box::new( | _cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "💻 System Information:" );
    println!( "   OS: Linux" );
    println!( "   Architecture: x86_64" );
    println!( "   Memory: 16 GB" );
    println!( "   CPU: 8 cores" );

    Ok( OutputData {
      content : "System information displayed".to_string(),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  registry.register_with_auto_help( sys_info_cmd, info_routine )?;

  println!( "✓ Registered 3 demo commands with automatic help generation:" );
  println!( "  • .fs.list (comprehensive arguments and validation)" );
  println!( "  • .net.ping (network utility with aliases)" );
  println!( "  • .sys.info (minimal command example)" );

  Ok( () )
}

/// Demonstrate all the different ways to access help
fn demonstrate_help_access_methods( pipeline : &Pipeline ) -> Result< (), Box< dyn core::error::Error > >
{
  println!( "🔍 Demonstrating Help Access Methods\n" );

  let context = ExecutionContext::default();

  // Method 1: Traditional ? operator
  println!( "=== Method 1: Traditional ? Operator ===" );
  println!( "Command: .fs.list ?" );
  let result1 = pipeline.process_command( ".fs.list ?", context.clone() );
  display_result( &result1, "Traditional ? operator" );

  println!( "\n" );

  // Method 2: New ?? parameter
  println!( "=== Method 2: New ?? Parameter ===" );
  println!( "Command: .fs.list ??" );
  let result2 = pipeline.process_command( ".fs.list \"??\"", context.clone() );
  display_result( &result2, "?? parameter" );

  println!( "\n" );

  // Method 3: Automatic .command.help
  println!( "=== Method 3: Automatic .command.help ===" );
  println!( "Command: .fs.list.help" );
  let result3 = pipeline.process_command( ".fs.list.help", context.clone() );
  display_result( &result3, ".command.help" );

  println!( "\n" );

  // Method 4: ?? mixed with other arguments
  println!( "=== Method 4: ?? Mixed with Arguments ===" );
  println!( "Command: .net.ping host::example.com ??" );
  let result4 = pipeline.process_command( ".net.ping host::example.com \"??\"", context.clone() );
  display_result( &result4, "?? with other arguments" );

  println!( "\n" );

  // Method 5: ?? as named parameter
  println!( "=== Method 5: ?? as Named Parameter ===" );
  println!( "Command: .sys.info help::??" );
  let result5 = pipeline.process_command( ".sys.info help::\"??\"", context.clone() );
  display_result( &result5, "?? as named parameter" );

  println!( "\n" );

  // Verification: All methods produce equivalent help
  println!( "=== Verification: Content Equivalence ===" );
  if result1.success && result2.success && result3.success {
    let content1 = &result1.outputs[0].content;
    let content2 = &result2.outputs[0].content;
    let content3 = &result3.outputs[0].content;

    if content1 == content2 && content2 == content3 {
      println!( "✅ All help access methods produce identical content" );
    } else {
      println!( "⚠️  Help content differs between methods" );
    }
  }

  // Interactive demo
  println!( "\n=== Interactive Demo ===" );
  println!( "Try these commands to explore help functionality:" );
  println!( "  .fs.list path::/tmp ??          # Get help with partial arguments" );
  println!( "  .net.ping.help                   # Direct help command access" );
  println!( "  .sys.info ?                      # Traditional help operator" );
  println!( "  .                                # List all available commands" );

  Ok( () )
}

/// Display command result in a user-friendly format
fn display_result( result : &unilang::pipeline::CommandResult, method_name : &str )
{
  if result.success {
    println!( "✅ {} succeeded", method_name );
    if !result.outputs.is_empty() {
      let help_content = &result.outputs[0].content;

      // Show a preview of the help content
      let lines : Vec< &str > = help_content.lines().take( 8 ).collect();
      println!( "📄 Help content preview:" );
      for line in lines {
        println!( "   {}", line );
      }

      let total_lines = help_content.lines().count();
      if total_lines > 8 {
        println!( "   ... ({} more lines)", total_lines - 8 );
      }
    }
  } else {
    println!( "❌ {} failed", method_name );
    if let Some( error ) = &result.error {
      println!( "   Error: {}", error );
    }
  }
}