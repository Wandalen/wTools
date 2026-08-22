//! Tests for `YamlFormatter::with_heading()` / `with_footer()` — the same `Heading`
//! titled-rule pattern as `table_heading_test.rs` / `table_footer_test.rs` /
//! `tree_heading_test.rs` / `expanded_heading_test.rs` / `text_heading_test.rs`,
//! applied to `YamlFormatter`. Unlike the earlier formatters, YAML has a comment
//! syntax (`#`), so the rendered rule is wrapped with a `"# "` prefix to stay valid
//! YAML rather than emitted as a bare rule line.
//! FT-37..FT-41 continue the ID sequence from `tests/docs/feature/007_table_heading.md`
//! (FT-31..FT-36 were Text's). `YamlFormatter` has a single code path through
//! `Format::format()` (no per-variant branches like Text/Sql), so there is no
//! separate branch-coverage case here — the no-heading passthrough test (FT-40)
//! already proves the wrap is a no-op on the only path that exists.

#![ cfg( feature = "enabled" ) ]

#[ cfg( feature = "format_yaml" ) ]
mod yaml_heading_tests
{
  use data_fmt::{ RowBuilder, YamlFormatter, Heading, Format };

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

  /// FT-37 — `feature/007`: title-only heading renders a `#`-commented titled rule
  /// before YAML output.
  // test_kind: standard
  #[ test ]
  fn title_only_heading_renders_before_yaml_output_ft37()
  {
    let formatter = YamlFormatter::new().with_heading( Heading::new( "Users" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    assert!(
      first_line.starts_with( "# ─── Users" ),
      "first line must be the commented heading; got: '{first_line}'",
    );
    assert!( output.contains( "Alice" ), "yaml body must still render; got:\n{output}" );
  }

  /// FT-38 — `feature/007`: title-only footer renders a `#`-commented titled rule
  /// after YAML output.
  // test_kind: standard
  #[ test ]
  fn title_only_footer_renders_after_yaml_output_ft38()
  {
    let formatter = YamlFormatter::new().with_footer( Heading::new( "2 records" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let last_line = *lines.last().unwrap_or( &"" );
    assert!(
      last_line.starts_with( "# ─── 2 records" ),
      "last line must be the commented footer; got: '{last_line}'",
    );
    assert!( output.contains( "Bob" ), "yaml body must still render; got:\n{output}" );
  }

  /// FT-39 — `feature/007`: the commented heading line's total display width
  /// (comment prefix + rule) fills to the widest rendered YAML line, matching the
  /// same width invariant as Tree/Expanded/Text.
  // test_kind: standard
  #[ test ]
  fn heading_fills_to_widest_yaml_line_ft39()
  {
    let formatter = YamlFormatter::new().with_heading( Heading::new( "H" ) );
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

  /// FT-40 — `feature/007`: no heading/footer configured leaves YAML output as pure,
  /// unwrapped `serde_yaml_ng` output — no comment line prepended or appended.
  // test_kind: standard
  #[ test ]
  fn no_heading_no_footer_is_pure_yaml_passthrough_ft40()
  {
    let formatter = YamlFormatter::new();
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    assert!( !output.starts_with( '#' ), "no heading configured — output must not start with a comment rule line; got:\n{output}" );
    assert!( output.trim_end().starts_with( '-' ), "unwrapped output must be plain YAML list; got:\n{output}" );
    assert!( output.contains( "Alice" ) && output.contains( "Bob" ), "yaml body must render unaffected; got:\n{output}" );
  }

  /// FT-41 — `feature/007`: heading and footer coexist on YAML output without
  /// interfering with each other or the body.
  // test_kind: standard
  #[ test ]
  fn heading_and_footer_coexist_on_yaml_output_ft41()
  {
    let formatter = YamlFormatter::new()
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
    assert!( output.contains( "Alice" ) && output.contains( "Bob" ), "yaml body unaffected; got:\n{output}" );
  }
}
