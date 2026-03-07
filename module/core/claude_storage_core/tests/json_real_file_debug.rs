//! Debug test for real file parse error
//!
//! This test attempts to parse the actual real file that's failing in production
//! to understand what's causing the error.

#[test]
#[ignore = "requires specific file path to exist on this machine"]
fn test_parse_real_problematic_file()
{
  use std::fs;
  use claude_storage_core::parse_json;

  let file_path = "/home/user1/.claude/projects/-home-user1-pro-lib-willbe-module-api-gmail--default-topic/95f9efbf-bab9-4fec-94ac-4a0024b02499.jsonl";

  let content = fs::read_to_string( file_path )
    .expect( "Failed to read test file" );

  let lines: Vec< &str > = content.lines().collect();

  println!( "Total lines in file: {}", lines.len() );

  // Try parsing each line
  for (i, line) in lines.iter().enumerate()
  {
    let result = parse_json( line );
    if let Err( e ) = result
    {
      println!( "Line {i}: FAILED with error: {e:?}" );
      println!( "Line length: {}", line.len() );

      // Try to find what's at the error position
      if let Some( pos_str ) = e.to_string().split( "position " ).nth( 1 )
      {
        if let Some( pos ) = pos_str.split( ':' ).next().and_then( | s | s.parse::< usize >().ok() )
        {
          if pos < line.len()
          {
            let start = pos.saturating_sub( 20 );
            let end = ( pos + 20 ).min( line.len() );
            let surrounding = &line[ start..end ];
            println!( "Context around position {pos}: ...{surrounding}..." );

            // Show actual bytes
            let bytes = &line.as_bytes()[ start..end ];
            println!( "Bytes (hex): {:?}", bytes.iter().map( | b | format!( "{b:02x}" ) ).collect::< Vec< _ > >() );
          }
        }
      }

      panic!( "Failed to parse line {i}" );
    }
    else
    {
      println!( "Line {i}: OK" );
    }
  }
}
