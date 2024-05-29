#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn path_consisting_only_of_dot_segments()
{

  let path = std::path::PathBuf::from( "././." );
  let exp = ".";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_consisting_only_of_dot_segments. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "." );
  let exp = ".";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_consisting_only_of_dot_segments. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "./" );
  let exp = ".";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_consisting_only_of_dot_segments. Expected: '{}', got: '{}'", exp, got );

}

#[ test ]
fn path_consisting_only_of_dotdot_segments()
{
  let path = std::path::PathBuf::from( "../../.." );
  let exp = "../../..";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_consisting_only_of_dotdot_segments. Expected: '{}', got: '{}'", exp, got );
}

#[ test ]
fn dotdot_overflow()
{

  let path = std::path::PathBuf::from( "../../a" );
  let exp = "../../a";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "?. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "/../../a" );
  let exp = "/../../a";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "?. Expected: '{}', got: '{}'", exp, got );

}

#[ test ]
fn path_with_trailing_dot_or_dotdot_segments()
{

  let path = std::path::PathBuf::from( "/a/b/c/.." );
  let exp = "/a/b";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_trailing_dot_or_dotdot_segments. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "./a/b/c/.." );
  let exp = "./a/b";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_trailing_dot_or_dotdot_segments. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "a/b/c/.." );
  let exp = "a/b";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_trailing_dot_or_dotdot_segments. Expected: '{}', got: '{}'", exp, got );

}

#[ test ]
fn empty_path()
{
  let path = std::path::PathBuf::new();
  let exp = ".";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: empty_path. Expected: '{}', got: '{}'", exp, got );
}

#[ test ]
fn path_with_no_dot_or_dotdot_only_regular_segments()
{
  let path = std::path::PathBuf::from( "/a/b/c" );
  let exp = "/a/b/c";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_no_dot_or_dotdot_only_regular_segments. Expected: '{}', got: '{}'", exp, got );
}

#[ test ]
fn path_with_mixed_dotdot_segments_that_resolve_to_valid_path()
{
  let path = std::path::PathBuf::from( "/a/b/../c" );
  let exp = "/a/c";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_mixed_dotdot_segments_that_resolve_to_valid_path. Expected: '{}', got: '{}'", exp, got );
}

#[ test ]
fn path_with_dotdot_segments_at_the_beginning()
{
  let path = std::path::PathBuf::from( "../../a/b" );
  let exp = "../../a/b";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_dotdot_segments_at_the_beginning. Expected: '{}', got: '{}'", exp, got );
}

#[ test ]
fn path_with_dotdot_segments_that_fully_resolve()
{

  let path = std::path::PathBuf::from( "/a/b/c/../../.." );
  let exp = "/";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_dotdot_segments_that_fully_resolve_to_root. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "a/b/c/../../.." );
  let exp = ".";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_dotdot_segments_that_fully_resolve_in_relative_path. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "./a/b/c/../../.." );
  let exp = ".";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_dotdot_segments_and_initial_current_dir_that_fully_resolve. Expected: '{}', got: '{}'", exp, got );

}

#[ test ]
fn path_including_non_ascii_characters_or_spaces()
{
  let path = std::path::PathBuf::from( "/a/ö/x/../b/c" );
  let exp = "/a/ö/b/c";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_including_non_ascii_characters_or_spaces. Expected: '{}', got: '{}'", exp, got );
}

#[ test ]
fn path_with_dot_or_dotdot_embedded_in_regular_path_segments()
{

  let path = std::path::PathBuf::from( "/a/b..c/..d/d../x/../e" );
  let exp = "/a/b..c/..d/d../e";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_dot_or_dotdot_embedded_in_regular_path_segments. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "a/b..c/..d/d../x/../e" );
  let exp = "a/b..c/..d/d../e";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_dot_or_dotdot_embedded_in_regular_path_segments. Expected: '{}', got: '{}'", exp, got );

}

#[ test ]
fn path_with_multiple_dot_and_dotdot_segments()
{

  let path = std::path::PathBuf::from( "/a/./b/.././c/../../d" );
  let exp = "/d";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_multiple_dot_and_dotdot_segments. Expected: '{}', got: '{}'", exp, got );

  let path = std::path::PathBuf::from( "a/./b/.././c/../../d" );
  let exp = "d";
  let normalized = the_module::path::normalize( &path );
  let got = normalized.to_str().unwrap();
  a_id!( exp, got, "Failed: path_with_multiple_dot_and_dotdot_segments. Expected: '{}', got: '{}'", exp, got );

}
