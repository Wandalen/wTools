//! Tests for `TomlFormatter::with_heading()` / `with_footer()` — the same `Heading`
//! titled-rule pattern as `table_heading_test.rs` / `table_footer_test.rs` /
//! `tree_heading_test.rs` / `expanded_heading_test.rs` / `text_heading_test.rs` /
//! `yaml_heading_test.rs`, applied to `TomlFormatter`. TOML also uses `#` for
//! comments, so the rendered rule is wrapped with the same `"# "` prefix as YAML.
//! FT-42..FT-46 continue the ID sequence from `tests/docs/feature/007_table_heading.md`
//! (FT-37..FT-41 were Yaml's). `TomlFormatter` has a single code path through
//! `Format::format()` (no per-variant branches like Text/Sql), so there is no
//! separate branch-coverage case here — the no-heading passthrough test (FT-45)
//! already proves the wrap is a no-op on the only path that exists.

#![ cfg( feature = "enabled" ) ]

#[ cfg( feature = "format_toml" ) ]
mod toml_heading_tests
{
  use data_fmt::{ RowBuilder, TomlFormatter, Heading, Format };

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

  /// FT-42 — `feature/007`: title-only heading renders a `#`-commented titled rule
  /// before TOML output.
  // test_kind: standard
  #[ test ]
  fn title_only_heading_renders_before_toml_output_ft42()
  {
    let formatter = TomlFormatter::new().with_heading( Heading::new( "Users" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    assert!(
      first_line.starts_with( "# ─── Users" ),
      "first line must be the commented heading; got: '{first_line}'",
    );
    assert!( output.contains( "Alice" ), "toml body must still render; got:\n{output}" );
  }

  /// FT-43 — `feature/007`: title-only footer renders a `#`-commented titled rule
  /// after TOML output.
  // test_kind: standard
  #[ test ]
  fn title_only_footer_renders_after_toml_output_ft43()
  {
    let formatter = TomlFormatter::new().with_footer( Heading::new( "2 records" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let last_line = *lines.last().unwrap_or( &"" );
    assert!(
      last_line.starts_with( "# ─── 2 records" ),
      "last line must be the commented footer; got: '{last_line}'",
    );
    assert!( output.contains( "Bob" ), "toml body must still render; got:\n{output}" );
  }

  /// FT-44 — `feature/007`: the commented heading line's total display width
  /// (comment prefix + rule) fills to the widest rendered TOML line, matching the
  /// same width invariant as Tree/Expanded/Text/Yaml.
  // test_kind: standard
  #[ test ]
  fn heading_fills_to_widest_toml_line_ft44()
  {
    let formatter = TomlFormatter::new().with_heading( Heading::new( "H" ) );
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

  /// FT-45 — `feature/007`: no heading/footer configured leaves TOML output as pure,
  /// unwrapped `toml::to_string` output — no comment line prepended or appended.
  // test_kind: standard
  #[ test ]
  fn no_heading_no_footer_is_pure_toml_passthrough_ft45()
  {
    let formatter = TomlFormatter::new();
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    assert!( !output.starts_with( '#' ), "no heading configured — output must not start with a comment rule line; got:\n{output}" );
    assert!( output.trim_end().starts_with( "[[row]]" ), "unwrapped output must be plain TOML array-of-tables; got:\n{output}" );
    assert!( output.contains( "Alice" ) && output.contains( "Bob" ), "toml body must render unaffected; got:\n{output}" );
  }

  /// FT-46 — `feature/007`: heading and footer coexist on TOML output without
  /// interfering with each other or the body.
  // test_kind: standard
  #[ test ]
  fn heading_and_footer_coexist_on_toml_output_ft46()
  {
    let formatter = TomlFormatter::new()
      .with_heading( Heading::new( "Top" ) )
      .with_footer( Heading::new( "End" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    let last_line = *lines.last().unwrap_or( &"" );

    assert!( first_line.starts_with( "# ─── Top" ), "first line must be heading; got: '{first_line}'" );
    assert!( last_line.starts_with( "# ─── End" ), "last line must be footer; got: '{last_line}'" );
    assert_eq!(
      display_width( first_line ), display_width( last_line ),
      "heading and footer must fill to the same width",
    );
    assert!( output.contains( "Alice" ) && output.contains( "Bob" ), "toml body unaffected; got:\n{output}" );
  }
}
