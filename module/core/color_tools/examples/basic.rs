//! Programmer-facing API reference for `DecoratedText`.
//!
//! Six sections cover the full public API surface with assertions so the
//! binary self-validates when run — not just a visual demo.
//!
//! ```bash
//! cargo run --example basic --features enabled
//! ```

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  use color_tools::DecoratedText;

  // =========================================================================
  // S1 — Plain construction
  // From<&str> and From<String> attach no color; render() returns raw text.
  // =========================================================================

  let plain_str  : DecoratedText = "hello".into();
  let plain_string = DecoratedText::from( "hello".to_string() );

  assert_eq!( plain_str.render(),    "hello", "From<&str> render must equal raw text" );
  assert_eq!( plain_string.render(), "hello", "From<String> render must equal raw text" );
  assert!(
    !plain_str.render().contains( '\x1b' ),
    "plain render must have no ANSI escape codes"
  );
  assert_eq!( plain_str.color, None, "From<&str> must not set a color" );

  println!( "S1 plain: {plain_str}" );

  // =========================================================================
  // S2 — 4-bit color encoding
  // SGR codes 30–37 (normal) and 90–97 (bright) are 4-bit foreground colors.
  // render() wraps: color_prefix + text + "\x1b[0m" (ANSI reset).
  // =========================================================================

  let yellow = DecoratedText::from( "warn" ).with_color( "\x1b[33m" );

  assert_eq!(
    yellow.render(),
    "\x1b[33mwarn\x1b[0m",
    "4-bit colored render must be: prefix + text + reset"
  );
  assert!( yellow.render().starts_with( "\x1b[33m" ), "must start with 4-bit code" );
  assert!( yellow.render().ends_with( "\x1b[0m" ),   "must end with ANSI reset" );

  println!( "S2 4-bit yellow: {yellow}" );

  // =========================================================================
  // S3 — 256-color encoding
  // Format: "\x1b[38;5;N m" where N is 0-255.
  // =========================================================================

  let orange_256 = DecoratedText::from( "info" ).with_color( "\x1b[38;5;208m" );

  assert!(
    orange_256.render().starts_with( "\x1b[38;5;208m" ),
    "256-color render must start with 38;5;N prefix"
  );
  assert!(
    orange_256.render().ends_with( "\x1b[0m" ),
    "256-color render must end with ANSI reset"
  );
  assert!(
    orange_256.render().contains( "info" ),
    "256-color render must contain original text"
  );

  println!( "S3 256-color orange: {orange_256}" );

  // =========================================================================
  // S4 — 24-bit true-color encoding
  // Format: "\x1b[38;2;R;G;B m" where R, G, B are 0-255.
  // =========================================================================

  let rgb_orange = DecoratedText::from( "ok" ).with_color( "\x1b[38;2;255;165;0m" );

  assert!(
    rgb_orange.render().starts_with( "\x1b[38;2;255;165;0m" ),
    "24-bit true-color render must start with 38;2;R;G;B prefix"
  );
  assert!(
    rgb_orange.render().ends_with( "\x1b[0m" ),
    "24-bit true-color render must end with ANSI reset"
  );

  println!( "S4 24-bit RGB orange: {rgb_orange}" );

  // =========================================================================
  // S5 — Query methods: is_colored(), is_empty()
  // is_empty() tests the text field, not render() — a colored empty text is
  // still considered empty because no visible content will be displayed.
  // =========================================================================

  // is_colored(): true only when a color prefix is attached
  let plain_query = DecoratedText::from( "x" );
  assert!( !plain_query.is_colored(), "plain must not be colored" );
  let colored_query = DecoratedText::from( "x" ).with_color( "\x1b[35m" );
  assert!( colored_query.is_colored(), "colored must report is_colored" );

  // is_empty(): tests text field only, independent of color
  let empty_plain   = DecoratedText::from( "" );
  let empty_colored = DecoratedText::from( "" ).with_color( "\x1b[33m" );
  let nonempty      = DecoratedText::from( "x" );

  assert!( empty_plain.is_empty(),   "empty uncolored must be empty" );
  assert!( empty_colored.is_empty(), "empty colored must still be empty" );
  assert!( !nonempty.is_empty(),     "non-empty text must not be empty" );

  // is_empty() == true does not mean render() returns "":
  // render() always applies the color wrap when color is set.
  assert_eq!( empty_plain.render(), "" );
  assert_eq!( empty_colored.render(), "\x1b[33m\x1b[0m" );

  println!( "S5 queries: is_colored={}, is_empty={}", colored_query.is_colored(), empty_plain.is_empty() );

  // =========================================================================
  // S6 — Conversion paths
  // All three string-producing conversions delegate to render(), ensuring
  // a single canonical rendering path (see invariant/004).
  // =========================================================================

  let source = DecoratedText::from( "msg" ).with_color( "\x1b[32m" );

  // Path A: String::from(ct)
  let as_string : String = source.clone().into();
  assert_eq!( as_string, source.render(), "String::from(ct) must equal render()" );

  // Path B: format!("{ct}") via Display
  let display_output = format!( "{source}" );
  assert_eq!( display_output, source.render(), "Display must equal render()" );

  // Path C: explicit .render() call (baseline)
  assert_eq!( source.render(), "\x1b[32mmsg\x1b[0m" );

  // All three agree
  assert_eq!( as_string, display_output, "all conversion paths must produce the same string" );

  println!( "S6 conversions: {source}" );
  println!( "examples/basic.rs: all assertions passed" );
}
