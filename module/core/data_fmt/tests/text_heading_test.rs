//! Tests for `TextFormatter::with_heading()` / `with_footer()` — the same `Heading`
//! titled-rule pattern as `table_heading_test.rs` / `table_footer_test.rs` /
//! `tree_heading_test.rs` / `expanded_heading_test.rs`, applied to `TextFormatter`.
//! Like Tree and Expanded, Text output has no fixed column width, so the rule fills
//! to the widest rendered line's display width instead of a precomputed `table_width`.
//! FT-31..FT-36 continue the ID sequence from `tests/docs/feature/007_table_heading.md`
//! (FT-25..FT-30 were Expanded's). `TextFormatter` has no separate config type — fields
//! are added directly to the formatter (matching its own pre-existing all-`pub`-fields,
//! no-accessor convention), per the task's explicit YAGNI decision against inventing a
//! `TextConfig` type that nothing else needs.

#![ cfg( feature = "enabled" ) ]

#[ cfg( feature = "format_text" ) ]
mod text_heading_tests
{
  use data_fmt::{ RowBuilder, TextFormatter, TextVariant, Heading, Format };

  fn simple_view() -> data_fmt::TableView
  {
    RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view()
  }

  fn display_width( s : &str ) -> usize
  {
    s.chars().count()
  }

  /// FT-31 — `feature/007`: title-only heading renders titled rule before text output.
  // test_kind: standard
  #[ test ]
  fn title_only_heading_renders_before_text_output_ft31()
  {
    let formatter = TextFormatter::bullets().with_heading( Heading::new( "Users" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    assert!(
      first_line.starts_with( "─── Users" ),
      "first line must be the heading; got: '{first_line}'",
    );
    assert!( output.contains( "Alice" ), "text body must still render; got:\n{output}" );
  }

  /// FT-32 — `feature/007`: title-only footer renders titled rule after text output.
  // test_kind: standard
  #[ test ]
  fn title_only_footer_renders_after_text_output_ft32()
  {
    let formatter = TextFormatter::bullets().with_footer( Heading::new( "2 records" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let last_line = *lines.last().unwrap_or( &"" );
    assert!(
      last_line.starts_with( "─── 2 records" ),
      "last line must be the footer; got: '{last_line}'",
    );
    assert!( output.contains( "Bob" ), "text body must still render; got:\n{output}" );
  }

  /// FT-33 — `feature/007`: heading/footer fill to the widest rendered text line, not
  /// a fixed table-style width — text lines are ragged (length varies by variant and
  /// content), so the rule must track the actual body content rather than any
  /// config-declared width.
  // test_kind: standard
  #[ test ]
  fn heading_fills_to_widest_text_line_ft33()
  {
    let formatter = TextFormatter::bullets().with_heading( Heading::new( "H" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let body_lines = &lines[ 1.. ];
    let expected_width = body_lines.iter().map( | l | display_width( l ) ).max().unwrap_or( 0 );
    let heading_width = display_width( lines[ 0 ] );

    assert_eq!(
      heading_width, expected_width,
      "heading must fill to the widest body line ({expected_width}), got {heading_width}",
    );
  }

  /// FT-34 — `feature/007`: no heading/footer configured produces byte-identical output
  /// to a `TextFormatter` baseline (passthrough — Invariant 1 equivalent for Text).
  // test_kind: standard
  #[ test ]
  fn no_heading_no_footer_output_unchanged_ft34()
  {
    let plain = TextFormatter::bullets().format( &simple_view() ).unwrap_or_default();
    let baseline = TextFormatter::new( TextVariant::Bullets ).format( &simple_view() ).unwrap_or_default();

    assert_eq!( plain, baseline, "TextFormatter::bullets() with no heading/footer must match TextFormatter::new(Bullets) baseline" );
    assert!( !plain.starts_with( '─' ), "no heading configured — first line must not be a rule line" );
  }

  /// FT-35 — `feature/007`: heading and footer coexist on text output without interfering.
  // test_kind: standard
  #[ test ]
  fn heading_and_footer_coexist_on_text_output_ft35()
  {
    let formatter = TextFormatter::bullets()
      .with_heading( Heading::new( "Top" ) )
      .with_footer( Heading::new( "End" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    let last_line = *lines.last().unwrap_or( &"" );

    assert!( first_line.starts_with( "─── Top" ), "first line must be heading; got: '{first_line}'" );
    assert!( last_line.starts_with( "─── End" ), "last line must be footer; got: '{last_line}'" );
    assert_eq!(
      display_width( first_line ), display_width( last_line ),
      "heading and footer must fill to the same width",
    );
    assert!( output.contains( "Alice" ) && output.contains( "Bob" ), "text body unaffected; got:\n{output}" );
  }

  /// FT-36 — `feature/007`: heading applies even when `CliHelp`'s own internal
  /// early-return fires (`format_cli_help` returns `String::new()` when `data.rows`
  /// is empty) — proves the wrap applies uniformly at the `Format::format()` funnel
  /// point regardless of which variant's match arm produced the body, not only the
  /// populated-rows path. Mirrors Tree's FT-24 and Expanded's FT-30.
  // test_kind: standard
  #[ test ]
  fn heading_applies_to_empty_cli_help_rows_ft36()
  {
    let empty_view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] ).build_view();
    let formatter = TextFormatter::new( TextVariant::CliHelp ).with_heading( Heading::new( "Empty" ) );
    let output = formatter.format( &empty_view ).unwrap_or_default();

    assert_eq!( output.lines().count(), 1, "expected exactly one line (heading only, no body); got:\n{output}" );
    assert!( output.starts_with( "─── Empty" ), "first line must be the heading; got: '{output}'" );
  }
}
