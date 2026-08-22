//! Tests for `SqlFormatter::with_heading()` / `with_footer()` — the same `Heading`
//! titled-rule pattern as `table_heading_test.rs` / `table_footer_test.rs` /
//! `tree_heading_test.rs` / `expanded_heading_test.rs` / `text_heading_test.rs` /
//! `yaml_heading_test.rs` / `toml_heading_test.rs`, applied to `SqlFormatter`. SQL
//! uses `--` for line comments, so the rendered rule is wrapped with a `"-- "` prefix.
//! FT-47..FT-52 continue the ID sequence from `tests/docs/feature/007_table_heading.md`
//! (FT-42..FT-46 were Toml's). Unlike Yaml/Toml, `SqlFormatter::format()` has two
//! return points — the BUG-020 empty-rows early return and the final populated-rows
//! return — so FT-52 covers both, mirroring Tree's FT-24, Expanded's FT-30, and
//! Text's FT-36.

#![ cfg( feature = "enabled" ) ]

#[ cfg( feature = "format_sql" ) ]
mod sql_heading_tests
{
  use data_fmt::{ RowBuilder, SqlFormatter, Heading, Format };

  fn simple_view() -> data_fmt::TableView
  {
    RowBuilder::new( vec![ "name".into(), "age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view()
  }

  fn display_width( s : &str ) -> usize
  {
    s.chars().count()
  }

  /// FT-47 — `feature/007`: title-only heading renders a `--`-commented titled rule
  /// before SQL output.
  // test_kind: standard
  #[ test ]
  fn title_only_heading_renders_before_sql_output_ft47()
  {
    let formatter = SqlFormatter::new( "users" ).with_heading( Heading::new( "Users" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    assert!(
      first_line.starts_with( "-- ─── Users" ),
      "first line must be the commented heading; got: '{first_line}'",
    );
    assert!( output.contains( "INSERT INTO" ), "sql body must still render; got:\n{output}" );
  }

  /// FT-48 — `feature/007`: title-only footer renders a `--`-commented titled rule
  /// after SQL output.
  // test_kind: standard
  #[ test ]
  fn title_only_footer_renders_after_sql_output_ft48()
  {
    let formatter = SqlFormatter::new( "users" ).with_footer( Heading::new( "2 records" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let last_line = *lines.last().unwrap_or( &"" );
    assert!(
      last_line.starts_with( "-- ─── 2 records" ),
      "last line must be the commented footer; got: '{last_line}'",
    );
    assert!( output.contains( "INSERT INTO" ), "sql body must still render; got:\n{output}" );
  }

  /// FT-49 — `feature/007`: the commented heading line's total display width
  /// (comment prefix + rule) fills to the widest rendered SQL line, matching the
  /// same width invariant as Tree/Expanded/Text/Yaml/Toml.
  // test_kind: standard
  #[ test ]
  fn heading_fills_to_widest_sql_line_ft49()
  {
    let formatter = SqlFormatter::new( "users" ).with_heading( Heading::new( "H" ) );
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

  /// FT-50 — `feature/007`: no heading/footer configured leaves SQL output as a pure,
  /// unwrapped `INSERT INTO` statement — no comment line prepended or appended.
  // test_kind: standard
  #[ test ]
  fn no_heading_no_footer_is_pure_sql_passthrough_ft50()
  {
    let formatter = SqlFormatter::new( "users" );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    assert!( !output.starts_with( "--" ), "no heading configured — output must not start with a comment rule line; got:\n{output}" );
    assert!( output.starts_with( "INSERT INTO" ), "unwrapped output must be a plain INSERT statement; got:\n{output}" );
    assert!( output.ends_with( ';' ), "unwrapped output must still end with ';'; got:\n{output}" );
  }

  /// FT-51 — `feature/007`: heading and footer coexist on SQL output without
  /// interfering with each other or the body.
  // test_kind: standard
  #[ test ]
  fn heading_and_footer_coexist_on_sql_output_ft51()
  {
    let formatter = SqlFormatter::new( "users" )
      .with_heading( Heading::new( "Top" ) )
      .with_footer( Heading::new( "End" ) );
    let output = formatter.format( &simple_view() ).unwrap_or_default();

    let lines : Vec< &str > = output.lines().collect();
    let first_line = *lines.first().unwrap_or( &"" );
    let last_line = *lines.last().unwrap_or( &"" );

    assert!( first_line.starts_with( "-- ─── Top" ), "first line must be heading; got: '{first_line}'" );
    assert!( last_line.starts_with( "-- ─── End" ), "last line must be footer; got: '{last_line}'" );
    assert_eq!(
      display_width( first_line ), display_width( last_line ),
      "heading and footer must fill to the same width",
    );
    assert!( output.contains( "INSERT INTO" ), "sql body unaffected; got:\n{output}" );
  }

  /// FT-52 — `feature/007`: heading applies even when the BUG-020 empty-rows early
  /// return fires (`Format::format()` returns `Ok(String::new())` when `data.rows`
  /// is empty, to avoid emitting invalid `INSERT INTO ... VALUES;` SQL) — proves the
  /// wrap applies uniformly at both `Format::format()` return points, not only the
  /// populated-rows path. Mirrors Tree's FT-24, Expanded's FT-30, and Text's FT-36.
  // test_kind: standard
  #[ test ]
  fn heading_applies_to_empty_rows_early_return_ft52()
  {
    let empty_view = RowBuilder::new( vec![ "id".into(), "name".into() ] ).build_view();
    let formatter = SqlFormatter::new( "users" ).with_heading( Heading::new( "Empty" ) );
    let output = formatter.format( &empty_view ).unwrap_or_default();

    assert_eq!( output.lines().count(), 1, "expected exactly one line (heading only, no body); got:\n{output}" );
    assert!( output.starts_with( "-- ─── Empty" ), "first line must be the commented heading; got: '{output}'" );
  }
}
