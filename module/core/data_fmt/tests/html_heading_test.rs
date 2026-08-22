//! Tests for `HtmlFormatter::with_heading()` / `with_footer()` — the same `Heading`
//! titled-rule pattern as `table_heading_test.rs` / `table_footer_test.rs` /
//! `tree_heading_test.rs` / `expanded_heading_test.rs` / `text_heading_test.rs` /
//! `yaml_heading_test.rs` / `toml_heading_test.rs` / `sql_heading_test.rs`, applied to
//! `HtmlFormatter`. Unlike `#`/`--` line comments, HTML uses a delimited comment
//! (`<!-- -->`), so the rendered rule is wrapped with both a `"<!-- "` prefix AND a
//! `" -->"` suffix — the first formatter to need `render_commented_rule_if_present`'s
//! `comment_suffix` parameter — to stay valid HTML rather than silently swallowing the
//! `<table>` markup that follows an unclosed comment.
//! FT-53..FT-58 continue the ID sequence from `tests/docs/feature/007_table_heading.md`
//! (FT-47..FT-52 were Sql's). `HtmlFormatter::format()` has a single return point (no
//! per-variant branches like Tree/Expanded/Text/Sql), so there is no branch-coverage
//! case here — instead FT-58 covers HTML's own genuinely distinct structural variant:
//! the optional `include_wrapper` prelude, proving the heading wraps the entire output
//! rather than being inserted between the wrapper and the `<table>` tag.

#![ cfg( feature = "enabled" ) ]

#[ cfg( feature = "format_html" ) ]
mod html_heading_tests
{
  use data_fmt::{ RowBuilder, HtmlFormatter, Heading, Format };

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

  /// FT-53 — `feature/007`: title-only heading renders an `<!-- -->`-commented titled rule
  /// before HTML output. Both the opening `<!--` and the closing `-->` are asserted
  /// explicitly — unlike `#`/`--` line comments, a delimited HTML comment without its
  /// closing marker would silently swallow the `<table>` markup that follows.
  // test_kind: standard
  #[ test ]
  fn title_only_heading_renders_before_html_output_ft53()
  {
    let formatter = HtmlFormatter::new().with_heading( Heading::new( "Users" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    assert!(
      first_line.starts_with( "<!-- ─── Users" ),
      "first line must be the commented heading; got: '{first_line}'",
    );
    assert!( first_line.ends_with( "-->" ), "heading comment must be closed on the same line; got: '{first_line}'" );
    assert!( output.contains( "<table>" ), "table markup must survive uncorrupted after the closed comment; got:\n{output}" );
    assert!( output.contains( "Alice" ), "html body must still render; got:\n{output}" );
  }

  /// FT-54 — `feature/007`: title-only footer renders an `<!-- -->`-commented titled rule
  /// after HTML output, on its own line — `HtmlFormatter`'s body (like `SqlFormatter`'s)
  /// ends with a bare closing tag and no trailing newline, so the wrap must insert a
  /// separating `\n` before the footer rather than joining it onto `</table>`'s line.
  // test_kind: standard
  #[ test ]
  fn title_only_footer_renders_after_html_output_ft54()
  {
    let formatter = HtmlFormatter::new().with_footer( Heading::new( "2 records" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let last_line = *lines.last().unwrap_or( &"" );
    assert!(
      last_line.starts_with( "<!-- ─── 2 records" ),
      "last line must be the commented footer; got: '{last_line}'",
    );
    assert!( last_line.ends_with( "-->" ), "footer comment must be closed on the same line; got: '{last_line}'" );
    let second_to_last = lines.get( lines.len().saturating_sub( 2 ) ).copied().unwrap_or( "" );
    assert!(
      second_to_last.ends_with( "</table>" ),
      "footer must be on its own line, not joined onto </table>'s line; got: '{second_to_last}'",
    );
    assert!( output.contains( "Bob" ), "html body must still render; got:\n{output}" );
  }

  /// FT-55 — `feature/007`: the commented heading line's total display width
  /// (`<!-- ` prefix + rule + ` -->` suffix) fills to the widest rendered HTML line,
  /// proving both delimiters' widths are subtracted before the rule is sized — matching
  /// the same width invariant as Tree/Expanded/Text/Yaml/Toml/Sql.
  // test_kind: standard
  #[ test ]
  fn heading_fills_to_widest_html_line_ft55()
  {
    let formatter = HtmlFormatter::new().with_heading( Heading::new( "H" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let body_lines = &lines[ 1.. ];
    let expected_width = body_lines.iter().map( | l | display_width( l ) ).max().unwrap_or( 0 );
    let heading_width = display_width( lines[ 0 ] );

    assert_eq!(
      heading_width, expected_width,
      "commented heading must fill to the widest body line ({expected_width}), got {heading_width}",
    );
  }

  /// FT-56 — `feature/007`: no heading/footer configured leaves HTML output as pure,
  /// unwrapped table markup — no comment line prepended or appended.
  // test_kind: standard
  #[ test ]
  fn no_heading_no_footer_is_pure_html_passthrough_ft56()
  {
    let formatter = HtmlFormatter::new();
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    assert!( !output.starts_with( "<!--" ), "no heading configured — output must not start with a comment rule line; got:\n{output}" );
    assert!( output.starts_with( "<table" ), "unwrapped output must start with the table tag; got:\n{output}" );
    assert!( output.ends_with( "</table>" ), "unwrapped output must still end with </table>; got:\n{output}" );
  }

  /// FT-57 — `feature/007`: heading and footer coexist on HTML output without
  /// interfering with each other or the body.
  // test_kind: standard
  #[ test ]
  fn heading_and_footer_coexist_on_html_output_ft57()
  {
    let formatter = HtmlFormatter::new()
      .with_heading( Heading::new( "Top" ) )
      .with_footer( Heading::new( "End" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    let last_line = *lines.last().unwrap_or( &"" );

    assert!( first_line.starts_with( "<!-- ─── Top" ), "first line must be heading; got: '{first_line}'" );
    assert!( last_line.starts_with( "<!-- ─── End" ), "last line must be footer; got: '{last_line}'" );
    assert_eq!(
      display_width( first_line ), display_width( last_line ),
      "heading and footer must fill to the same width",
    );
    assert!( output.contains( "Alice" ) && output.contains( "Bob" ), "html body unaffected; got:\n{output}" );
  }

  /// FT-58 — `feature/007`: when `include_wrapper` is enabled, the heading wraps the
  /// ENTIRE rendered output (including the `<!DOCTYPE>`/`<html>`/`<body>` prelude) —
  /// per the design decision documented in `docs/algorithm/007_heading_rendering.md`,
  /// matching every other formatter's "wrap the whole rendered body" pattern rather than
  /// being inserted between the wrapper and the `<table>` tag.
  // test_kind: standard
  #[ test ]
  fn heading_wraps_entire_output_including_wrapper_ft58()
  {
    let formatter = HtmlFormatter::new()
      .with_include_wrapper( true )
      .with_heading( Heading::new( "Report" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    assert!(
      first_line.starts_with( "<!-- ─── Report" ),
      "heading must be the very first line, before <!DOCTYPE html>; got: '{first_line}'",
    );
    let doctype_pos = output.find( "<!DOCTYPE html>" ).expect( "wrapper must still be present" );
    let table_pos = output.find( "<table" ).expect( "table markup must still be present" );
    assert!( doctype_pos < table_pos, "wrapper must precede the table tag; got:\n{output}" );
    assert!( output.contains( "Alice" ), "html body must still render; got:\n{output}" );
  }
}
