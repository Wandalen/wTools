
include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use proper_path_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

// #[ cfg( feature = "enabled" ) ]
// #[ test ]
// fn path_with_dotdot_segments_that_fully_resolve()
// {
//
//   let path = std::path::PathBuf::from( "a/b/c/../../.." );
//   let exp = ".";
//   let normalized = the_module::path::normalize( &path );
//   let got = normalized.to_str().unwrap();
//   a_id!( exp, got, "Failed: path_with_dotdot_segments_that_fully_resolve_in_relative_path. Expected: '{}', got: '{}'", exp, got );
//
// }
