#[ allow( unused_imports ) ]
use super::*;


#[ test ]
fn test_with_empty_array() 
{
  let paths : Vec< &str > = vec![];
  let got = the_module::path::path_common( paths.into_iter() );
  assert_eq!( got, None );
}

// absolute-absolute

#[ test ]
fn test_absolute_absolute_have_common_dir() 
{
  let got = the_module::path::path_common( vec![ "/a1/b2", "/a1/a" ].into_iter() ).unwrap();
  assert_eq!( got, "/a1/" );
}

#[ test ]
fn test_absolute_absolute_have_common_dir_2() 
{
  let got = the_module::path::path_common( vec![ "/a1/b1/c", "/a1/b1/d", "/a1/b2" ].into_iter() ).unwrap();
  assert_eq!( got, "/a1/" );
}

#[ test ]
fn test_absolute_absolute_have_common_dir_and_part_of_name() 
{
  let got = the_module::path::path_common( vec![ "/a1/b2", "/a1/b1" ].into_iter() ).unwrap();
  assert_eq!( got, "/a1/" );
}

#[ test ]
fn test_absolute_absolute_one_path_has_dots_identical_paths() 
{
  let got = the_module::path::path_common( vec![ "/a1/x/../b1", "/a1/b1" ].into_iter() ).unwrap();
  assert_eq!( got, "/a1/b1" );
}

#[ test ]
fn test_absolute_absolute_more_than_one_dir_in_common_path() 
{
  let got = the_module::path::path_common( vec![ "/a1/b1/c1", "/a1/b1/c" ].into_iter() ).unwrap();
  assert_eq!( got, "/a1/b1/" );
}

#[ test ]
fn test_absolute_absolute_one_path_have_dots_no_common_dirs() 
{
  let got = the_module::path::path_common( vec![ "/a1/../../b1/c1", "/a1/b1/c1" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_absolute_dir_name_is_part_of_another_dir_name() 
{
  let got = the_module::path::path_common( vec![ "/abcd", "/ab" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_absolute_dir_names_has_dots_have_common_path() 
{
  let got = the_module::path::path_common( vec![ "/.a./.b./.c.", "/.a./.b./.c" ].into_iter() ).unwrap();
  assert_eq!( got, "/.a./.b./" );
}

#[ test ]
fn test_absolute_absolute_one_path_has_several_slashes_the_other_has_not_not_identical() 
{
  let got = the_module::path::path_common( vec![ "//a//b//c", "/a/b" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_absolute_identical_paths_with_several_slashes() 
{
  let got = the_module::path::path_common( vec![ "/a//b", "/a//b" ].into_iter() ).unwrap();
  assert_eq!( got, "/a//b" );
}

#[ test ]
fn test_absolute_absolute_identical_paths_with_several_slashes_2() 
{
  let got = the_module::path::path_common( vec![ "/a//", "/a//" ].into_iter() ).unwrap();
  assert_eq!( got, "/a//" );
}

#[ test ]
fn test_absolute_absolute_one_path_has_here_token_dirs_identical_paths() 
{
  let got = the_module::path::path_common( vec![ "/./a/./b/./c", "/a/b" ].into_iter() ).unwrap();
  assert_eq!( got, "/a/b" );
}

#[ test ]
fn test_absolute_absolute_different_case_in_path_name_not_identical() 
{
  let got = the_module::path::path_common( vec![ "/A/b/c", "/a/b/c" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_absolute_one_path_is_root_directory_common_root_directory() 
{
  let got = the_module::path::path_common( vec![ "/", "/x" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_absolute_different_paths_in_root_directory_common_root_directory() 
{
  let got = the_module::path::path_common( vec![ "/a", "/x" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}


// more than 2 path in arguments

#[ test ]
fn test_absolute_absolute_more_than_2_path_in_arguments() 
{
  let got = the_module::path::path_common( vec![ "/a/b/c", "/a/b/c", "/a/b/c", "/a/b/c" ].into_iter() ).unwrap();
  assert_eq!( got, "/a/b/c" );
}

#[ test ]
fn test_absolute_absolute_more_than_2_path_in_arguments_variant2() 
{
  let got = the_module::path::path_common( vec![ "/a/b/c", "/a/b/c", "/a/b" ].into_iter() ).unwrap();
  assert_eq!( got, "/a/b" );
}

#[ test ]
fn test_absolute_absolute_more_than_2_path_in_arguments_variant3() 
{
  let got = the_module::path::path_common( vec![ "/a/b/c", "/a/b/c", "/a/b1" ].into_iter() ).unwrap();
  assert_eq!( got, "/a/" );
}

#[ test ]
fn test_absolute_absolute_more_than_2_path_in_arguments_variant4() 
{
  let got = the_module::path::path_common( vec![ "/a/b/c", "/a/b/c", "/a" ].into_iter() ).unwrap();
  assert_eq!( got, "/a" );
}

#[ test ]
fn test_absolute_absolute_more_than_2_path_in_arguments_variant5() 
{
  let got = the_module::path::path_common( vec![ "/a/b/c", "/a/b/c", "/x" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_absolute_more_than_2_path_in_arguments_variant6() 
{
  let got = the_module::path::path_common( vec![ "/a/b/c", "/a/b/c", "/" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}









// absolute-relative 

#[ test ]
fn test_absolute_relative_root_and_down_token() 
{
  let got = the_module::path::path_common( vec![ "/", ".." ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_relative_root_and_here_token() 
{
  let got = the_module::path::path_common( vec![ "/", "." ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_relative_root_and_some_relative_directory() 
{
  let got = the_module::path::path_common( vec![ "/", "x" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_relative_root_and_double_down_token_in_path() 
{
  let got = the_module::path::path_common( vec![ "/", "../.." ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_relative_root_with_here_token_and_down_token() 
{
  let got = the_module::path::path_common( vec![ "/.", ".." ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_relative_root_with_here_token_and_here_token() 
{
  let got = the_module::path::path_common( vec![ "/.", "." ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_relative_root_with_here_token_and_some_relative_directory() 
{
  let got = the_module::path::path_common( vec![ "/.", "x" ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}

#[ test ]
fn test_absolute_relative_root_with_here_token_and_double_down_token_in_path() 
{
  let got = the_module::path::path_common( vec![ "/.", "../.." ].into_iter() ).unwrap();
  assert_eq!( got, "/" );
}







// relative - relative
#[ test ]
fn test_relative_relative_common_dir() 
{
  let got = the_module::path::path_common( vec![ "a1/b2", "a1/a" ].into_iter() ).unwrap();
  assert_eq!( got, "a1/" );
}

#[ test ]
fn test_relative_relative_common_dir_and_part_of_dir_names() 
{
  let got = the_module::path::path_common( vec![ "a1/b2", "a1/b1" ].into_iter() ).unwrap();
  assert_eq!( got, "a1/" );
}

#[ test ]
fn test_relative_relative_one_path_with_down_token_dir_identical_paths() 
{
  let got = the_module::path::path_common( vec![ "a1/x/../b1", "a1/b1" ].into_iter() ).unwrap();
  assert_eq!( got, "a1/b1" );
}

#[ test ]
fn test_relative_relative_paths_begins_with_here_token_directory_dots_identical_paths() 
{
  let got = the_module::path::path_common( vec![ "./a1/x/../b1", "./a1/b1" ].into_iter() ).unwrap();
  assert_eq!( got, "a1/b1" );
}

#[ test ]
fn test_relative_relative_one_path_begins_with_here_token_dir_another_down_token() 
{
  let got = the_module::path::path_common( vec![ "./a1/x/../b1", "../a1/b1" ].into_iter() ).unwrap();
  assert_eq!( got, ".." );
}

#[ test ]
fn test_relative_relative_here_token_and_down_token() 
{
  let got = the_module::path::path_common( vec![ ".", ".." ].into_iter() ).unwrap();
  assert_eq!( got, ".." );
}

#[ test ]
fn test_relative_relative_different_paths_start_with_here_token_dir() 
{
  let got = the_module::path::path_common( vec![ "./b/c", "./x" ].into_iter() ).unwrap();
  assert_eq!( got, "." );
}




//combinations of paths with dots

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots() 
{
  let got = the_module::path::path_common( vec![ "./././a", "./a/b" ].into_iter() ).unwrap();
  assert_eq!( got, "a" );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant2() 
{
  let got = the_module::path::path_common( vec![ "./a/./b", "./a/b" ].into_iter() ).unwrap();
  assert_eq!( got, "a/b" );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant3() 
{
  let got = the_module::path::path_common( vec![ "./a/./b", "./a/c/../b" ].into_iter() ).unwrap();
  assert_eq!( got, "a/b" );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant4() 
{
  let got = the_module::path::path_common( vec![ "../b/c", "./x" ].into_iter() ).unwrap();
  assert_eq!( got, ".." );
}



#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant9() 
{
  let got = the_module::path::path_common( vec![ "../../..", "./../../.." ].into_iter() ).unwrap();
  assert_eq!( got, "../../.." );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant10() 
{
  let got = the_module::path::path_common( vec![ "./../../..", "./../../.." ].into_iter() ).unwrap();
  assert_eq!( got, "../../.." );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant11() 
{
  let got = the_module::path::path_common( vec![ "../../..", "../../.." ].into_iter() ).unwrap();
  assert_eq!( got, "../../.." );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant12() 
{
  let got = the_module::path::path_common( vec![ "../b", "../b" ].into_iter() ).unwrap();
  assert_eq!( got, "../b" );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant13() 
{
  let got = the_module::path::path_common( vec![ "../b", "./../b" ].into_iter() ).unwrap();
  assert_eq!( got, "../b" );
}


// several relative paths

#[ test ]
fn test_relative_relative_several_relative_paths() 
{
  let got = the_module::path::path_common( vec![ "a/b/c", "a/b/c", "a/b/c" ].into_iter() ).unwrap();
  assert_eq!( got, "a/b/c" );
}

#[ test ]
fn test_relative_relative_several_relative_paths_variant2() 
{
  let got = the_module::path::path_common( vec![ "a/b/c", "a/b/c", "a/b" ].into_iter() ).unwrap();
  assert_eq!( got, "a/b" );
}

#[ test ]
fn test_relative_relative_several_relative_paths_variant3() 
{
  let got = the_module::path::path_common( vec![ "a/b/c", "a/b/c", "a/b1" ].into_iter() ).unwrap();
  assert_eq!( got, "a/" );
}

#[ test ]
fn test_relative_relative_several_relative_paths_variant4() 
{
  let got = the_module::path::path_common( vec![ "a/b/c", "a/b/c", "." ].into_iter() ).unwrap();
  assert_eq!( got, "." );
}

#[ test ]
fn test_relative_relative_several_relative_paths_variant5() 
{
  let got = the_module::path::path_common( vec![ "a/b/c", "a/b/c", "x" ].into_iter() ).unwrap();
  assert_eq!( got, "." );
}

#[ test ]
fn test_relative_relative_several_relative_paths_variant6() 
{
  let got = the_module::path::path_common( vec![ "a/b/c", "a/b/c", "./" ].into_iter() ).unwrap();
  assert_eq!( got, "." );
}

#[ test ]
fn test_relative_relative_several_relative_paths_variant7() 
{
  let got = the_module::path::path_common( vec![ "../a/b/c", "a/../b/c", "a/b/../c" ].into_iter() ).unwrap();
  assert_eq!( got, ".." );
}



#[ test ]
fn test_relative_relative_dot_and_double_up_and_down_tokens() 
{
  let got = the_module::path::path_common( vec![ ".", "./", ".." ].into_iter() ).unwrap();
  assert_eq!( got, ".." );
}



/* 

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant5() 
{
  let got = the_module::path::path_common( vec![ "../../b/c", "../b" ].into_iter() ).unwrap();
  assert_eq!( got, "../.." );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant6() 
{
  let got = the_module::path::path_common( vec![ "../../b/c", "../../../x" ].into_iter() ).unwrap();
  assert_eq!( got, "../../.." );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant7() 
{
  let got = the_module::path::path_common( vec![ "../../b/c/../../x", "../../../x" ].into_iter() ).unwrap();
  assert_eq!( got, "../../.." );
}

#[ test ]
fn test_relative_relative_combinations_of_paths_with_dots_variant8() 
{
  let got = the_module::path::path_common( vec![ "./../../b/c/../../x", "./../../../x" ].into_iter() ).unwrap();
  assert_eq!( got, "../../.." );
}


#[ test ]
fn test_relative_relative_dot_and_double_up_and_down_tokens_variant2() 
{
  let got = the_module::path::path_common( vec![ ".", "./../..", ".." ].into_iter() ).unwrap();
  assert_eq!( got, "../.." );
}

#[ test ]
fn test_relative_relative_several_relative_paths_variant8() 
{
  let got = the_module::path::path_common( vec![ "./a/b/c", "../../a/b/c", "../../../a/b" ].into_iter() ).unwrap();
  assert_eq!( got, "../../.." );
}









#[ test ]
#[ should_panic ]
fn test_first_path_is_absolute_another_is_dots() 
{
  the_module::path::path_common( vec![ "/a", ".."]);
}

#[ test ]
#[ should_panic ]
fn test_first_path_is_dots_and_absolute_path() 
{
  the_module::path::path_common( vec![ "..", "../../b/c", "/a"]);
}

#[ test ]
#[ should_panic ]
fn test_first_path_is_dots_and_absolute_path_variant2() 
{
  the_module::path::path_common( vec![ "../..", "../../b/c", "/a"]);
}

#[ test ]
#[ should_panic ]
fn test_unknown_path() 
{
  the_module::path::path_common( vec![ "/a", "x"]);
}

#[ test ]
#[ should_panic ]
fn test_unknown_path_variant2() 
{
  the_module::path::path_common( vec![ "x", "/a/b/c", "/a"]);
}  */