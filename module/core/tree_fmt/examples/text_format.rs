//! Example demonstrating `TextFormatter` usage
//!
//! Shows all 6 text formatting variants for different output scenarios.
//!
//! Run with:
//! ```bash
//! cargo run --example text_format --features format_text
//! ```

#[ cfg( feature = "format_text" ) ]
use tree_fmt::{ RowBuilder, TextFormatter, Format };

#[ cfg( not( feature = "format_text" ) ) ]
fn main()
{
  println!( "This example requires the 'format_text' feature." );
  println!( "Run with: cargo run --example text_format --features format_text" );
}

#[ cfg( feature = "format_text" ) ]
fn main()
{
  println!( "=== TextFormatter Examples ===\n" );

  // Sample data for most examples
  let view = RowBuilder::new( vec![ "Task".into(), "Status".into(), "Priority".into() ] )
    .add_row( vec![ "Implement feature X".into(), "Done".into(), "High".into() ] )
    .add_row( vec![ "Fix bug #42".into(), "In Progress".into(), "Critical".into() ] )
    .add_row( vec![ "Write documentation".into(), "Pending".into(), "Medium".into() ] )
    .add_row( vec![ "Code review".into(), "Done".into(), "Low".into() ] )
    .build_view();

  // Example 1: Bullets (default)
  println!( "1. Bullets Variant (default):\n" );
  let formatter = TextFormatter::bullets();
  let text = formatter.format( &view ).unwrap();
  println!( "{text}" );
  println!( "Use case: Quick lists, task summaries, bullet points\n" );

  // Example 2: Numbered
  println!( "2. Numbered Variant:\n" );
  let formatter = TextFormatter::numbered();
  let text = formatter.format( &view ).unwrap();
  println!( "{text}" );
  println!( "Use case: Ordered lists, step-by-step instructions, rankings\n" );

  // Example 3: KeyValue
  println!( "3. KeyValue Variant:\n" );
  let formatter = TextFormatter::key_value();
  let text = formatter.format( &view ).unwrap();
  println!( "{text}" );
  println!( "Use case: Configuration display, property lists, metadata\n" );

  // Example 4: Sections
  println!( "4. Sections Variant:\n" );
  let formatter = TextFormatter::sections();
  let text = formatter.format( &view ).unwrap();
  println!( "{text}" );
  println!( "Use case: Grouped data, categorized output, headers with content\n" );

  // Example 5: Compact
  println!( "5. Compact Variant:\n" );
  let formatter = TextFormatter::compact();
  let text = formatter.format( &view ).unwrap();
  println!( "{text}" );
  println!( "Use case: Dense output, tags list, space-constrained displays\n" );

  // Example 6: CliHelp (NEW)
  println!( "6. CliHelp Variant (NEW):\n" );

  // CLI help text needs specific data structure
  let help_view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
    .add_row( vec![ "USAGE".into(), String::new() ] )
    .add_row( vec![ "myapp [command] [options]".into(), String::new() ] )
    .add_row( vec![ "COMMANDS".into(), String::new() ] )
    .add_row( vec![ "build".into(), "Build the project".into() ] )
    .add_row( vec![ "test".into(), "Run tests".into() ] )
    .add_row( vec![ "deploy".into(), "Deploy to production".into() ] )
    .add_row( vec![ "OPTIONS".into(), String::new() ] )
    .add_row( vec![ "--verbose".into(), "Enable verbose output".into() ] )
    .add_row( vec![ "--config FILE".into(), "Use custom config file".into() ] )
    .add_row( vec![ "--help".into(), "Show this help message".into() ] )
    .add_row( vec![ "EXAMPLES".into(), String::new() ] )
    .add_row( vec![ "myapp build --verbose".into(), String::new() ] )
    .add_row( vec![ "myapp test --config test.toml".into(), String::new() ] )
    .build_view();

  let formatter = TextFormatter::cli_help();
  let text = formatter.format( &help_view ).unwrap();
  println!( "{text}" );
  println!( "Use case: CLI help text, command documentation, usage guides" );
  println!( "Features: Section headers, aligned descriptions, blank line separators\n" );

  // Example 7: Custom indentation
  println!( "7. Custom Indentation (4 spaces):\n" );
  let formatter = TextFormatter::bullets().with_indent( 4 );
  let text = formatter.format( &view ).unwrap();
  println!( "{text}" );
  println!( "Use case: Nested content, increased visual hierarchy\n" );

  // Example 8: Custom separator
  println!( "8. Custom Separator (semicolon + space):\n" );
  let formatter = TextFormatter::compact().with_separator( "; ".to_string() );
  let text = formatter.format( &view ).unwrap();
  println!( "{text}" );
  println!( "Use case: Alternative list formats, custom delimiters\n" );

  // Example 9: Real-world CLI help example
  println!( "9. Real-World Example - Config Command Help:\n" );

  let config_help = RowBuilder::new( vec![ "Section".into(), "Content".into() ] )
    .add_row( vec![ "USAGE".into(), String::new() ] )
    .add_row( vec![ "unikit .config [key::key-name] [format::output-format]".into(), String::new() ] )
    .add_row( vec![ "PARAMETERS".into(), String::new() ] )
    .add_row( vec![ "key::string".into(), "Show specific config key (optional)".into() ] )
    .add_row( vec![ "format::string".into(), "Output format: table|json|yaml (default: table)".into() ] )
    .add_row( vec![ "SOURCE TYPES".into(), String::new() ] )
    .add_row( vec![ "runtime".into(), "CLI parameter override (highest priority)".into() ] )
    .add_row( vec![ "env".into(), "Environment variable (UNIKIT_*)".into() ] )
    .add_row( vec![ "file".into(), "Config file (workspace or user)".into() ] )
    .add_row( vec![ "default".into(), "Built-in default value (lowest priority)".into() ] )
    .build_view();

  let formatter = TextFormatter::cli_help();
  let text = formatter.format( &config_help ).unwrap();
  println!( "{text}" );
  println!( "Note: Descriptions automatically align to longest key + 2 spaces\n" );

  println!( "=== Usage Tips ===" );
  println!( "- Bullets: Default choice for general lists" );
  println!( "- Numbered: When order matters" );
  println!( "- KeyValue: For configuration/metadata display" );
  println!( "- Sections: When data has natural groupings" );
  println!( "- Compact: Space-constrained or inline display" );
  println!( "- CliHelp: Command-line help text with sections and alignment" );
  println!( "- Customize with .with_indent() and .with_separator() methods" );
}
