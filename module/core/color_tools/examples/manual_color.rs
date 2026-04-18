//! Manual color verification — run in a real terminal to visually inspect ANSI rendering.
//!
//! ```bash
//! cargo run --example manual_color --features "enabled,serde_support"
//! ```

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  use color_tools::DecoratedText;

  println!( "=== 1. Basic Color Output ===" );

  let plain = DecoratedText::from( "status: ok" );
  println!( "[plain_no_color]            {plain}" );

  let warn = DecoratedText::from( "status: warn" ).with_color( "\x1b[33m" );
  println!( "[yellow_warn]              {warn}" );

  let err = DecoratedText::from( "error" ).with_color( "\x1b[31m" );
  println!( "[red_error]                {err}" );

  println!( "[reset_restores_default]   {warn} <- must be default color" );

  println!();
  println!( "=== 2. Empty Text Edge Cases ===" );

  let empty_plain = DecoratedText::from( "" );
  println!( "[empty_plain_render]       >>{empty_plain}<< (nothing between markers)" );

  let empty_colored = DecoratedText::from( "" ).with_color( "\x1b[33m" );
  println!( "[empty_colored_render]     >>{empty_colored}<< (no visible text)" );

  println!();
  println!( "=== 3. Multiline Handling ===" );

  let ml_plain = DecoratedText::from( "line1\nline2\nline3" );
  println!( "[multiline_uncolored]" );
  println!( "{ml_plain}" );

  let ml_colored = DecoratedText::from( "line1\nline2" ).with_color( "\x1b[33m" );
  println!( "[multiline_colored_single_reset]" );
  println!( "{ml_colored}" );
  println!( "<- this line must be default color (reset worked)" );

  println!();
  println!( "=== 4. Display / String Conversion ===" );

  let ct = DecoratedText::from( "display-test" ).with_color( "\x1b[34m" );
  println!( "[display_equals_render]    {ct}" );

  let s : String = DecoratedText::from( "into-test" ).with_color( "\x1b[32m" ).into();
  println!( "[string_from_ct]           debug: {s:?}" );

  #[ cfg( feature = "serde_support" ) ]
  {
    println!();
    println!( "=== 5. Serde Feature ===" );

    let plain = DecoratedText::from( "hello" );
    println!( "[serde_json_plain]         {}", serde_json::to_string( &plain ).unwrap() );

    let colored = DecoratedText::from( "warn" ).with_color( "\x1b[33m" );
    println!( "[serde_json_colored]       {}", serde_json::to_string( &colored ).unwrap() );

    let json = serde_json::to_string( &colored ).unwrap();
    let restored : DecoratedText = serde_json::from_str( &json ).unwrap();
    println!( "[serde_roundtrip_visual]   {restored}" );
  }

  println!();
  println!( "=== Done — verify colors visually ===" );
}
