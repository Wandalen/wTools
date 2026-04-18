//! Tests for `ColorfulText` type — covers all public invariants.
//!
//! ## Test Matrix
//!
//! | ID | Invariant Tested |
//! |----|-----------------|
//! | t01 | `From<String>` produces `color: None` |
//! | t02 | `From<&str>` produces `color: None` |
//! | t03 | `.with_color()` sets color field |
//! | t04 | `.render()` uncolored returns raw text with no escape codes |
//! | t05 | `.render()` colored starts with code and ends with `\x1b[0m]` |
//! | t06 | `From<ColorfulText> for String` equals `.render()` |
//! | t07 | `Display` output equals `.render()` |
//! | t08 | `Default` produces empty text, no color |
//! | t09 | Chaining: full round-trip with color |
//! | t10 | Round-trip: `String -> CT -> String` preserves text when uncolored |
//! | t11 | Empty text uncolored: `.render()` returns `""` |
//! | t12 | `is_colored()` returns true only when color is set |
//! | t13 | `is_empty()` checks text field, not render output |
//! | t14 | `.with_color()` second call overrides first |
//! | t15 | `.render()` on empty colored text emits `color + reset` with no content |
//! | t16 | `.render()` on multiline uncolored preserves `\n` verbatim |
//! | t17 | `.render()` on multiline colored emits exactly ONE reset — not per-line |
//! | t18 | Serde round-trip: serialize → deserialize preserves both fields |

use color_tools::ColorfulText;

// =============================================================================
// t01 — From<String> produces color: None
// =============================================================================

#[ test ]
fn t01_from_string_no_color()
{
  let ct = ColorfulText::from( "hello".to_string() );
  assert_eq!( ct.text, "hello" );
  assert_eq!( ct.color, None, "From<String> must not set a color" );
}

// =============================================================================
// t02 — From<&str> produces color: None
// =============================================================================

#[ test ]
fn t02_from_str_no_color()
{
  let ct : ColorfulText = "world".into();
  assert_eq!( ct.text, "world" );
  assert_eq!( ct.color, None, "From<&str> must not set a color" );
}

// =============================================================================
// t03 — .with_color() sets color field
// =============================================================================

#[ test ]
fn t03_with_color_sets_field()
{
  let ct = ColorfulText::from( "x" ).with_color( "\x1b[31m" );
  assert_eq!( ct.color, Some( "\x1b[31m".to_string() ) );
  assert_eq!( ct.text, "x" );
}

// =============================================================================
// t04 — .render() uncolored: raw text, no escape codes
// =============================================================================

#[ test ]
fn t04_render_uncolored_no_escape()
{
  let ct = ColorfulText::from( "plain" );
  let rendered = ct.render();
  assert_eq!( rendered, "plain", "uncolored render must equal raw text" );
  assert!( !rendered.contains( '\x1b' ), "no escape codes in uncolored render" );
}

// =============================================================================
// t05 — .render() colored: starts with code, ends with reset
// =============================================================================

#[ test ]
fn t05_render_colored_has_reset()
{
  let ct = ColorfulText::from( "warn" ).with_color( "\x1b[33m" );
  let rendered = ct.render();
  assert!( rendered.starts_with( "\x1b[33m" ), "must start with color code" );
  assert!( rendered.contains( "warn" ), "must contain the text" );
  assert!( rendered.ends_with( "\x1b[0m" ), "must end with reset" );
}

// =============================================================================
// t06 — From<ColorfulText> for String delegates to .render()
// =============================================================================

#[ test ]
fn t06_from_ct_to_string_is_render()
{
  let ct = ColorfulText::from( "msg" ).with_color( "\x1b[32m" );
  let expected = ct.render();
  let actual : String = ct.into();
  assert_eq!( actual, expected );
}

// =============================================================================
// t07 — Display output equals .render()
// =============================================================================

#[ test ]
fn t07_display_equals_render()
{
  let ct = ColorfulText::from( "display-test" ).with_color( "\x1b[34m" );
  assert_eq!( format!( "{ct}" ), ct.render() );
}

// =============================================================================
// t08 — Default: empty text, no color, render returns ""
// =============================================================================

#[ test ]
fn t08_default_empty()
{
  let ct = ColorfulText::default();
  assert_eq!( ct.text, "" );
  assert_eq!( ct.color, None );
  assert_eq!( ct.render(), "" );
}

// =============================================================================
// t09 — Chaining: color attach then render produces correct escape sequence
// =============================================================================

#[ test ]
fn t09_chain_color_render()
{
  let result = ColorfulText::from( "x" ).with_color( "\x1b[31m" ).render();
  assert_eq!( result, "\x1b[31mx\x1b[0m" );
}

// =============================================================================
// t10 — Round-trip: String → ColorfulText → String preserves text (uncolored)
// =============================================================================

#[ test ]
fn t10_roundtrip_uncolored()
{
  let original = "hello".to_string();
  let ct = ColorfulText::from( original.clone() );
  let back : String = ct.into();
  assert_eq!( back, original );
}

// =============================================================================
// t11 — Empty text uncolored: render returns ""
// =============================================================================

#[ test ]
fn t11_empty_text_render()
{
  let ct = ColorfulText::from( "" );
  assert_eq!( ct.render(), "" );
}

// =============================================================================
// t12 — is_colored() returns true only when color is set
// =============================================================================

#[ test ]
fn t12_is_colored()
{
  let plain = ColorfulText::from( "x" );
  assert!( !plain.is_colored(), "uncolored must return false" );
  let colored = plain.with_color( "\x1b[35m" );
  assert!( colored.is_colored(), "colored must return true" );
}

// =============================================================================
// t13 — is_empty() checks text field, not render output
// =============================================================================

#[ test ]
fn t13_is_empty_checks_text()
{
  let empty_uncolored = ColorfulText::from( "" );
  assert!( empty_uncolored.is_empty(), "empty uncolored must be empty" );

  // Even with color set, empty text is considered empty
  let empty_colored = ColorfulText::from( "" ).with_color( "\x1b[33m" );
  assert!( empty_colored.is_empty(), "empty colored must also be empty" );

  let nonempty = ColorfulText::from( "x" );
  assert!( !nonempty.is_empty(), "non-empty text must not be empty" );
}

// =============================================================================
// t14 — .with_color() second call overrides the first
// =============================================================================

#[ test ]
fn t14_with_color_override()
{
  let ct = ColorfulText::from( "text" )
    .with_color( "\x1b[31m" )   // red — first call
    .with_color( "\x1b[33m" );  // yellow — second call must win
  assert_eq!(
    ct.color,
    Some( "\x1b[33m".to_string() ),
    "second with_color call must override the first",
  );
  assert!(
    ct.render().starts_with( "\x1b[33m" ),
    "render must use the overriding color",
  );
  assert!(
    !ct.render().contains( "\x1b[31m" ),
    "overridden color must not appear in render output",
  );
}

// =============================================================================
// t15 — .render() on empty colored text produces color prefix followed by reset
//
// is_empty() == true causes the formatter to skip rendering the detail line.
// render() itself does not check is_empty(); it always applies the color wrap.
// This test documents that design distinction.
// =============================================================================

#[ test ]
fn t15_render_empty_colored_text()
{
  let ct = ColorfulText::from( "" ).with_color( "\x1b[33m" );
  assert!( ct.is_empty(), "empty colored text must report is_empty == true" );
  assert_eq!(
    ct.render(),
    "\x1b[33m\x1b[0m",
    "render of empty colored text must produce color+reset with no content between them",
  );
}

// =============================================================================
// t16 — .render() on multiline uncolored text preserves newlines verbatim
// =============================================================================

#[ test ]
fn t16_render_multiline_uncolored()
{
  let ct = ColorfulText::from( "line1\nline2\nline3" );
  let rendered = ct.render();
  assert_eq!( rendered, "line1\nline2\nline3", "uncolored render must preserve newlines verbatim" );
  assert!( !rendered.contains( '\x1b' ), "no ANSI codes must appear in uncolored multiline render" );
}

// =============================================================================
// t17 — .render() on multiline colored text places exactly ONE reset at the end
//
// render() wraps the whole text block — it is NOT per-line aware.
// Per-line ANSI wrapping is the formatter's responsibility (see table.rs Algorithm 3).
// This test documents that design boundary explicitly.
// =============================================================================

#[ test ]
fn t17_render_multiline_colored_single_reset()
{
  let ct = ColorfulText::from( "line1\nline2" ).with_color( "\x1b[33m" );
  let rendered = ct.render();
  assert!( rendered.starts_with( "\x1b[33m" ), "must begin with color prefix" );
  assert!( rendered.ends_with( "\x1b[0m" ), "must end with ANSI reset" );
  assert!( rendered.contains( "line1\nline2" ), "text content including newline must be preserved" );
  assert_eq!(
    rendered.matches( "\x1b[0m" ).count(),
    1,
    "render() produces exactly ONE ANSI reset for the whole text — per-line wrapping is the formatter's job",
  );
}

// =============================================================================
// t18 — Serde round-trip: serialize → deserialize preserves both fields
//
// Tests both plain (color: None) and colored (color: Some) variants.
// Requires the `serde_support` feature to be enabled.
// =============================================================================

#[ cfg( feature = "serde_support" ) ]
#[ test ]
fn t18_serde_roundtrip()
{
  // Plain variant: color field must survive as None
  let plain = ColorfulText::from( "hello" );
  let json = serde_json::to_string( &plain ).expect( "serialize plain must succeed" );
  let restored : ColorfulText = serde_json::from_str( &json ).expect( "deserialize plain must succeed" );
  assert_eq!( restored.text, plain.text, "text field must survive serde round-trip" );
  assert_eq!( restored.color, plain.color, "color field must survive serde round-trip as None" );

  // Colored variant: color field must survive as Some(...)
  let colored = ColorfulText::from( "warn" ).with_color( "\x1b[33m" );
  let json = serde_json::to_string( &colored ).expect( "serialize colored must succeed" );
  let restored : ColorfulText = serde_json::from_str( &json ).expect( "deserialize colored must succeed" );
  assert_eq!( restored.text, colored.text, "text field must survive serde round-trip" );
  assert_eq!( restored.color, colored.color, "color field must survive serde round-trip as Some" );
  assert_eq!( restored.render(), colored.render(), "render output must be identical after round-trip" );
}
