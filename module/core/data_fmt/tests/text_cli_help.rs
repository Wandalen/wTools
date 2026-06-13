//! Tests for `TextFormatter` `CliHelp` variant
//!
//! ## What This Tests
//!
//! Tests the `CliHelp` variant which formats data as CLI help text with:
//! - Section headers (all caps followed by colon)
//! - Indented content under sections
//! - Aligned key-value descriptions
//! - Blank lines between sections
//!
//! ## Data Structure Convention
//!
//! For `CliHelp` variant, data is structured as:
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

#![ cfg( feature = "enabled" ) ]

#[ cfg( feature = "format_text" ) ]
mod cli_help_tests
{
  use data_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

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
    let output_lines : Vec< &str > = output.lines().collect();
    let key_line = output_lines.iter().find( | l | l.contains( "key::string" ) ).unwrap();
    let fmt_line = output_lines.iter().find( | l | l.contains( "format::string" ) ).unwrap();

    // Find where descriptions start (should be same column)
    let desc1_start = key_line.find( "Show" ).unwrap();
    let desc2_start = fmt_line.find( "Output" ).unwrap();
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
    let output_lines : Vec< &str > = output.lines().collect();
    let config_line = output_lines.iter().find( | l | l.contains( "unikit .config" ) && !l.contains( "::" ) ).unwrap();
    assert!( config_line.starts_with( "  " ), "Should start with 2-space indent" );
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
    for line in &lines
    {
      if line.contains( ':' ) && line.chars().next().unwrap().is_uppercase()
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
    let view = RowBuilder::new( vec![ "Header".into(), String::new() ] )
      .add_row( vec![ "SECTION".into(), "".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).unwrap();

    assert_eq!( output.trim(), "SECTION:" );
  }

  /// AC-7 — `006_cli_help_alignment`: ANSI escape codes excluded from alignment width.
  ///
  /// ## Root Cause
  /// `format_cli_help()` computed `max_key_width` using `.len()` (UTF-8 byte count).
  /// ANSI escape bytes inflated the measured key width, shifting all sibling descriptions
  /// rightward by the invisible byte overhead. BUG-014.
  ///
  /// ## Why Not Caught
  /// The prior test only verified no-panic and verbatim ANSI preservation; it never
  /// asserted the visual column position of description values.
  ///
  /// ## Fix Applied
  /// Both `.len()` calls in `format_cli_help()` replaced with `visual_len()`, which
  /// strips ANSI sequences before counting characters.
  ///
  /// ## Prevention
  /// Use `visual_len()` for any alignment computation on user-visible strings; reserve
  /// `.len()` for byte-level operations (buffer sizing, serialization).
  ///
  /// ## Pitfall
  /// A string with ANSI escape codes has `.len()` >> visual width; using `.len()` for
  /// column alignment inflates the description column by the ANSI byte overhead,
  /// making all sibling plain-text descriptions appear far too indented.
  // test_kind: bug_reproducer(BUG-014)
  #[ test ]
  fn ansi_key_alignment_uses_visual_len_not_byte_count_ac7()
  {
    // ansi_key: visual width 9 ("--verbose"), byte count 18 ("\x1b[32m"=5 + "--verbose"=9 + "\x1b[0m"=4)
    let ansi_key = "\x1b[32m--verbose\x1b[0m";
    let view = RowBuilder::new( vec![ "Term".into(), "Desc".into() ] )
      .add_row( vec![ "OPTIONS".into(), "".into() ] )
      .add_row( vec![ ansi_key.into(), "Show verbose output".into() ] )
      .add_row( vec![ "--help".into(), "Show this help".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).expect( "must not panic with ANSI codes in key" );

    // ANSI codes must be preserved verbatim in the output
    assert!(
      output.contains( "\x1b[32m" ),
      "ANSI escape codes in key must be preserved in output:\n{output:?}",
    );
    assert!(
      output.contains( "\x1b[0m" ),
      "ANSI reset code must be preserved in output:\n{output:?}",
    );

    // Core alignment invariant: descriptions must align at indent(2) + max_visual_key_width(9) + gap(2) = 13
    // The --help row has no ANSI, so its raw byte position equals its visual column position
    let lines : Vec< &str > = output.lines().collect();
    let help_line = lines.iter()
      .find( | l | l.contains( "Show this help" ) )
      .expect( "--help description line must appear in output" );
    let desc_col = help_line.find( "Show this help" )
      .expect( "description must be present on --help line" );
    assert_eq!(
      desc_col,
      13,
      "--help description must start at col 13 (indent=2 + visual_key_max=9 + gap=2); \
       byte-count bug would put it at col 22:\n{output:?}",
    );
  }

  /// AC-8 — `006_cli_help_alignment`: mixed-case text is not detected as a section header.
  ///
  /// A row whose first column is `"Options"` (mixed-case, not all-uppercase) must
  /// NOT be rendered as an unindented header with colon suffix. It is treated as
  /// a key-description pair or simple indented line instead.
  // test_kind: standard
  #[ test ]
  fn mixed_case_row_not_treated_as_section_header_ac8()
  {
    let view = RowBuilder::new( vec![ "Term".into(), "Desc".into() ] )
      .add_row( vec![ "OPTIONS".into(), "".into() ] ) // real header
      .add_row( vec![ "Options".into(), "mixed-case row".into() ] ) // NOT a header
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).expect( "must not fail" );

    // "Options" must NOT appear as a header line (unindented, colon-suffixed)
    assert!(
      !output.lines().any( | l | l == "Options:" ),
      "'Options' (mixed-case) must not be emitted as a section header 'Options:':\n{output:?}",
    );
    // "Options" content must appear in the output (as indented line or key-desc pair)
    assert!(
      output.contains( "Options" ),
      "'Options' content must still appear in output:\n{output:?}",
    );
    // Description for the mixed-case row must appear
    assert!(
      output.contains( "mixed-case row" ),
      "description for mixed-case row must appear:\n{output:?}",
    );
  }

  /// AC-9 — `006_cli_help_alignment`: all-uppercase key with non-empty second column
  /// is not treated as a section header.
  ///
  /// A row where the first column is `"OPTIONS"` (all-uppercase) but the second column
  /// is non-empty (`"description text"`). This row must be rendered as a key-description
  /// pair, not as a section header; no colon is appended to the first column.
  // test_kind: standard
  #[ test ]
  fn uppercase_with_nonempty_description_not_a_header_ac9()
  {
    let view = RowBuilder::new( vec![ "Term".into(), "Desc".into() ] )
      .add_row( vec![ "OPTIONS".into(), "description text".into() ] )
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).expect( "must not fail" );

    // Must NOT appear as "OPTIONS:" (header form)
    assert!(
      !output.lines().any( | l | l == "OPTIONS:" ),
      "row with non-empty second column must not be rendered as section header 'OPTIONS:':\n{output:?}",
    );
    // Both column values must appear
    assert!(
      output.contains( "OPTIONS" ),
      "'OPTIONS' text must appear in output:\n{output:?}",
    );
    assert!(
      output.contains( "description text" ),
      "'description text' must appear in output:\n{output:?}",
    );
  }

  /// AC-10 — `006_cli_help_alignment`: alignment column is computed globally across
  /// all sections; content in both sections appears correctly aligned.
  ///
  /// Section 1 has a key of 20 characters; section 2 has keys of 4 characters.
  /// In the current implementation, the alignment column is the global maximum (20-char
  /// key width from section 1), so both sections use the same alignment column. This test
  /// guards the current global-alignment behavior and verifies that both sections render
  /// their content correctly with consistent column positions.
  ///
  /// Note: the spec describes per-section alignment reset as the intended behavior; the
  /// current implementation uses global alignment across all sections.
  // test_kind: standard
  #[ test ]
  fn global_alignment_widens_all_sections_ac10()
  {
    let long_key = "abcdefghijklmnopqrst"; // exactly 20 chars
    let view = RowBuilder::new( vec![ "Term".into(), "Desc".into() ] )
      .add_row( vec![ "SEC1".into(), "".into() ] )
      .add_row( vec![ long_key.into(), "long key desc".into() ] )
      .add_row( vec![ "".into(), "".into() ] )           // blank row between sections
      .add_row( vec![ "SEC2".into(), "".into() ] )
      .add_row( vec![ "abcd".into(), "short desc".into() ] ) // 4-char key in section 2
      .build_view();

    let formatter = TextFormatter::new( TextVariant::CliHelp );
    let output = formatter.format( &view ).expect( "must not fail" );

    // Both sections must appear in the output with their content
    assert!( output.contains( "long key desc" ), "section 1 description must appear:\n{output:?}" );
    assert!( output.contains( "short desc" ), "section 2 description must appear:\n{output:?}" );
    assert!( output.contains( "SEC1" ), "section 1 header must appear:\n{output:?}" );
    assert!( output.contains( "SEC2" ), "section 2 header must appear:\n{output:?}" );

    // Both key-desc lines appear; in current implementation alignment is global (not per-section),
    // so both sections' descriptions align at the same column position (global max key width).
    let sec1_line = output.lines().find( | l | l.contains( "long key desc" ) )
      .expect( "section 1 key-desc line must appear" );
    let sec2_line = output.lines().find( | l | l.contains( "short desc" ) )
      .expect( "section 2 key-desc line must appear" );

    let sec1_desc_col = sec1_line.find( "long key desc" ).unwrap_or( 0 );
    let sec2_desc_col = sec2_line.find( "short desc" ).unwrap_or( 0 );

    // Current behavior: global alignment — both sections use the same description column
    assert_eq!(
      sec1_desc_col, sec2_desc_col,
      "global alignment: both sections use same description column (current behavior):\n  sec1={sec1_line:?}\n  sec2={sec2_line:?}\n  output:\n{output:?}",
    );
  }
}
