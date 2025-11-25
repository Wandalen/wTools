//! Tests for TextFormatter CliHelp variant
//!
//! ## What This Tests
//!
//! Tests the CliHelp variant which formats data as CLI help text with:
//! - Section headers (all caps followed by colon)
//! - Indented content under sections
//! - Aligned key-value descriptions
//! - Blank lines between sections
//!
//! ## Data Structure Convention
//!
//! For CliHelp variant, data is structured as:
//! - Row with uppercase first column + empty second column = section header
//! - Row with both columns = key-description pair (aligned)
//! - Row with content in first column only = simple indented line
//!
//! ## Expected Output Format
//!
//! ```text
//! SECTION HEADER:
//!   key-term          Description text here
//!   another-key       More description
//!
//! ANOTHER SECTION:
//!   simple line
//!   another line
//! ```

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

#[ cfg( feature = "format_text" ) ]
mod cli_help_tests
{
  use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

  #[ test ]
  fn test_cli_help_basic_section_with_aligned_pairs()
  {
    let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
      .add_row( vec![ "PARAMETERS".into(), "".into() ] )
      .add_row( vec![ "key::string".into(), "Show specific config key (optional)".into() ] )
      .add_row( vec![ "format::string".into(), "Output format: table|json|yaml (default: table)".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    // Should have section header
    assert!( output.contains( "PARAMETERS:" ) );

    // Should have indented and aligned content
    assert!( output.contains( "  key::string" ) );
    assert!( output.contains( "Show specific config key" ) );
    assert!( output.contains( "  format::string" ) );
    assert!( output.contains( "Output format:" ) );

    // Descriptions should align vertically
    let lines : Vec< &str > = output.lines().collect();
    let line1 = lines.iter().find( | l | l.contains( "key::string" ) ).unwrap();
    let line2 = lines.iter().find( | l | l.contains( "format::string" ) ).unwrap();

    // Find where descriptions start (should be same column)
    let desc1_start = line1.find( "Show" ).unwrap();
    let desc2_start = line2.find( "Output" ).unwrap();
    assert_eq!( desc1_start, desc2_start, "Descriptions should align vertically" );
  }

  #[ test ]
  fn test_cli_help_multiple_sections_with_blank_lines()
  {
    let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
      .add_row( vec![ "USAGE".into(), "".into() ] )
      .add_row( vec![ "unikit .config [options]".into(), "".into() ] )
      .add_row( vec![ "PARAMETERS".into(), "".into() ] )
      .add_row( vec![ "key::string".into(), "Show specific key".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    // Should have both section headers
    assert!( output.contains( "USAGE:" ) );
    assert!( output.contains( "PARAMETERS:" ) );

    // Should have blank line between sections
    let lines : Vec< &str > = output.lines().collect();
    let usage_idx = lines.iter().position( | l | l.contains( "USAGE:" ) ).unwrap();
    let params_idx = lines.iter().position( | l | l.contains( "PARAMETERS:" ) ).unwrap();

    // There should be at least one line gap between sections
    assert!( params_idx > usage_idx + 1, "Should have blank line between sections" );
  }

  #[ test ]
  fn test_cli_help_section_with_simple_lines()
  {
    let view = RowBuilder::new( vec![ "Content".into(), "Extra".into() ] )
      .add_row( vec![ "EXAMPLES".into(), "".into() ] )
      .add_row( vec![ "unikit .config".into(), "".into() ] )
      .add_row( vec![ "unikit .config key::max_tokens".into(), "".into() ] )
      .add_row( vec![ "unikit .config format::json".into(), "".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    assert!( output.contains( "EXAMPLES:" ) );
    assert!( output.contains( "  unikit .config" ) );
    assert!( output.contains( "  unikit .config key::max_tokens" ) );
    assert!( output.contains( "  unikit .config format::json" ) );

    // Simple lines should not have extra alignment padding
    let lines : Vec< &str > = output.lines().collect();
    let line1 = lines.iter().find( | l | l.contains( "unikit .config" ) && !l.contains( "::" ) ).unwrap();
    assert!( line1.starts_with( "  " ), "Should start with 2-space indent" );
  }

  #[ test ]
  fn test_cli_help_mixed_content_types()
  {
    let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
      .add_row( vec![ "USAGE".into(), "".into() ] )
      .add_row( vec![ "command [options]".into(), "".into() ] )
      .add_row( vec![ "OPTIONS".into(), "".into() ] )
      .add_row( vec![ "--verbose".into(), "Enable verbose output".into() ] )
      .add_row( vec![ "--quiet".into(), "Suppress all output".into() ] )
      .add_row( vec![ "EXAMPLES".into(), "".into() ] )
      .add_row( vec![ "command --verbose".into(), "".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    // Should have all three sections
    assert!( output.contains( "USAGE:" ) );
    assert!( output.contains( "OPTIONS:" ) );
    assert!( output.contains( "EXAMPLES:" ) );

    // Simple line under USAGE
    assert!( output.contains( "  command [options]" ) );

    // Aligned pairs under OPTIONS
    assert!( output.contains( "  --verbose" ) );
    assert!( output.contains( "Enable verbose output" ) );

    // Simple line under EXAMPLES
    assert!( output.contains( "  command --verbose" ) );
  }

  #[ test ]
  fn test_cli_help_custom_indent()
  {
    let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
      .add_row( vec![ "SECTION".into(), "".into() ] )
      .add_row( vec![ "item".into(), "description".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp )
      .with_indent( 4 );
    let output = formatter.format( &view ).unwrap();

    // Should use 4-space indent instead of default 2
    assert!( output.contains( "    item" ) );
  }

  #[ test ]
  fn test_cli_help_long_keys_increase_alignment()
  {
    let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
      .add_row( vec![ "OPTIONS".into(), "".into() ] )
      .add_row( vec![ "short".into(), "Short key description".into() ] )
      .add_row( vec![ "very-long-option-name".into(), "Long key description".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    let lines : Vec< &str > = output.lines().collect();
    let short_line = lines.iter().find( | l | l.contains( "short" ) && l.contains( "Short key" ) ).unwrap();
    let long_line = lines.iter().find( | l | l.contains( "very-long" ) ).unwrap();

    // Both descriptions should start at same column (aligned to longest key)
    let desc1_start = short_line.find( "Short key" ).unwrap();
    let desc2_start = long_line.find( "Long key" ).unwrap();
    assert_eq!( desc1_start, desc2_start, "Descriptions should align to longest key" );
  }

  #[ test ]
  fn test_cli_help_section_header_formats()
  {
    let view = RowBuilder::new( vec![ "Header".into(), "Content".into() ] )
      .add_row( vec![ "USAGE".into(), "".into() ] )
      .add_row( vec![ "PARAMETERS".into(), "".into() ] )
      .add_row( vec![ "SOURCE TYPES".into(), "".into() ] )
      .add_row( vec![ "SPECIAL VALUES".into(), "".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    // All should be formatted as section headers
    assert!( output.contains( "USAGE:" ) );
    assert!( output.contains( "PARAMETERS:" ) );
    assert!( output.contains( "SOURCE TYPES:" ) );
    assert!( output.contains( "SPECIAL VALUES:" ) );

    // Headers should not be indented
    let lines : Vec< &str > = output.lines().collect();
    for line in lines.iter()
    {
      if line.contains( ":" ) && line.chars().next().unwrap().is_uppercase()
      {
        assert!( !line.starts_with( ' ' ), "Section headers should not be indented" );
      }
    }
  }

  #[ test ]
  fn test_cli_help_exact_unikit_format()
  {
    // This test reproduces the exact format from the user's example
    let view = RowBuilder::new( vec![ "Section".into(), "Content".into() ] )
      .add_row( vec![ "USAGE".into(), "".into() ] )
      .add_row( vec![ "unikit .config [key::key-name] [format::output-format]".into(), "".into() ] )
      .add_row( vec![ "PARAMETERS".into(), "".into() ] )
      .add_row( vec![ "key::string".into(), "Show specific config key (optional)".into() ] )
      .add_row( vec![ "format::string".into(), "Output format: table|json|yaml (default: table)".into() ] )
      .add_row( vec![ "SOURCE TYPES".into(), "".into() ] )
      .add_row( vec![ "runtime".into(), "CLI parameter override (highest priority)".into() ] )
      .add_row( vec![ "env".into(), "Environment variable (UNIKIT_*)".into() ] )
      .add_row( vec![ "file".into(), "Config file (workspace or user)".into() ] )
      .add_row( vec![ "default".into(), "Built-in default value (lowest priority)".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    // Verify structure matches expected format
    assert!( output.starts_with( "USAGE:" ) );
    assert!( output.contains( "\n  unikit .config" ) );
    assert!( output.contains( "\nPARAMETERS:" ) );
    assert!( output.contains( "\n  key::string" ) );
    assert!( output.contains( "\nSOURCE TYPES:" ) );
    assert!( output.contains( "\n  runtime" ) );

    // Verify alignment
    let lines : Vec< &str > = output.lines().collect();
    let params_section_start = lines.iter().position( | l | l.contains( "PARAMETERS:" ) ).unwrap();
    let key_line = &lines[ params_section_start + 1 ];
    let format_line = &lines[ params_section_start + 2 ];

    // Both should have descriptions starting at same position
    let key_desc_pos = key_line.find( "Show" ).unwrap();
    let format_desc_pos = format_line.find( "Output" ).unwrap();
    assert_eq!( key_desc_pos, format_desc_pos );

    println!( "Generated CLI Help output:\n{output}" );
  }

  #[ test ]
  fn test_cli_help_empty_view()
  {
    let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    assert_eq!( output, "" );
  }

  #[ test ]
  fn test_cli_help_single_section_only()
  {
    let view = RowBuilder::new( vec![ "Header".into(), "".into() ] )
      .add_row( vec![ "SECTION".into(), "".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    assert_eq!( output.trim(), "SECTION:" );
  }
}
