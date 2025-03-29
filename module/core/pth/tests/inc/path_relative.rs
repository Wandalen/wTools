#[ allow( unused_imports ) ]
use super::*;
use std::path::PathBuf;


// absolute path relative

#[ test ]
fn test_absolute_a_minus_b()
{
  let from = "/a";
  let to = "/b";
  let expected = "../b";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( PathBuf::from( expected ) ) );
}

#[ test ]
fn test_absolute_root_minus_b()
{
  let from = "/";
  let to = "/b";
  let expected = "b";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_same_path()
{
  let from = "/aa/bb/cc";
  let to = "/aa/bb/cc";
  let expected = ".";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_same_path_with_trail()
{
  let from = "/aa/bb/cc";
  let to = "/aa/bb/cc/";
  let expected = "./";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_two_trailed_absolute_paths()
{
  let from = "/a/b/";
  let to = "/a/b/";
  let expected = "./";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_two_absolute_paths_with_trail()
{
  let from = "/a/b";
  let to = "/a/b/";
  let expected = "./";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_two_absolute_paths()
{
  let from = "/a/b/";
  let to = "/a/b";
  let expected = ".";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_same_path_trail_to_not()
{
  let from = "/aa/bb/cc/";
  let to = "/aa/bb/cc";
  let expected = ".";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_a_to_double_slash_b()
{
  let from = "/a";
  let to = "//b";
  let expected = "..//b";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}


#[ test ]
fn test_absolute_relative_to_nested()
{
  let from = "/foo/bar/baz/asdf/quux";
  let to = "/foo/bar/baz/asdf/quux/new1";
  let expected = "new1";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_out_of_relative_dir()
{
  let from = "/abc";
  let to = "/a/b/z";
  let expected = "../a/b/z";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_relative_root()
{
  let from = "/";
  let to = "/a/b/z";
  let expected = "a/b/z";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}


#[ test ]
fn test_long_not_direct()
{
  let from = "/a/b/xx/yy/zz";
  let to = "/a/b/files/x/y/z.txt";
  let expected = "../../../files/x/y/z.txt";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_relative_to_parent_directory()
{
  let from = "/aa/bb/cc";
  let to = "/aa/bb";
  let expected = "..";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_relative_to_parent_directory_file_trailed()
{
  let from = "/aa/bb/cc";
  let to = "/aa/bb/";
  let expected = "../";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_relative_root_to_root()
{
  let from = "/";
  let to = "/";
  let expected = ".";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_windows_disks()
{
  let from = "d:/";
  let to = "c:/x/y";
  let expected = "../c/x/y";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}


#[ test ]
fn test_absolute_relative_to_parent_directory_both_trailed()
{
  let from = "/aa/bb/cc/";
  let to = "/aa/bb/";
  let expected = "./../";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}


#[ test ]
fn test_absolute_a_with_trail_to_double_slash_b_with_trail()
{
  let from = "/a/";
  let to = "//b/";
  let expected = "./..//b/";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_4_down()
{
  let from = "/aa//bb/cc/";
  let to = "//xx/yy/zz/";
  let expected = "./../../../..//xx/yy/zz/";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_same_length_both_trailed()
{
  let from = "/aa//bb/cc/";
  let to = "//xx/yy/zz/";
  let expected = "./../../../..//xx/yy/zz/";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_absolute_relative_to_parent_directory_base_trailed()
{
  let from = "/aa/bb/cc/";
  let to = "/aa/bb";
  let expected = "./..";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}





// relative_path_relative

#[ test ]
fn test_relative_dot_to_dot()
{
  let from = ".";
  let to = ".";
  let expected = ".";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_to_b()
{
  let from = "a";
  let to = "b";
  let expected = "../b";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_b_to_b_c()
{
  let from = "a/b";
  let to = "b/c";
  let expected = "../../b/c";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_b_to_a_b_c()
{
  let from = "a/b";
  let to = "a/b/c";
  let expected = "c";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_b_c_to_a_b()
{
  let from = "a/b/c";
  let to = "a/b";
  let expected = "..";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_b_c_d_to_a_b_d_c()
{
  let from = "a/b/c/d";
  let to = "a/b/d/c";
  let expected = "../../d/c";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_to_dot_dot_a()
{
  let from = "a";
  let to = "../a";
  let expected = "../../a";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_slash_slash_b_to_a_slash_slash_c()
{
  let from = "a//b";
  let to = "a//c";
  let expected = "../c";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_dot_slash_b_to_a_dot_slash_c()
{
  let from = "a/./b";
  let to = "a/./c";
  let expected = "../c";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_dot_dot_slash_b_to_b()
{
  let from = "a/../b";
  let to = "b";
  let expected = ".";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_b_to_b_dot_dot_slash_b()
{
  let from = "b";
  let to = "b/../b";
  let expected = ".";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_to_dot_dot()
{
  let from = ".";
  let to = "..";
  let expected = "..";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_to_dot_dot_dot()
{
  let from = ".";
  let to = "../..";
  let expected = "../..";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_dot_to_dot_dot()
{
  let from = "..";
  let to = "../..";
  let expected = "..";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_dot_to_dot_dot_dot()
{
  let from = "..";
  let to = "..";
  let expected = ".";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_dot_a_b_to_dot_dot_c_d()
{
  let from = "../a/b";
  let to = "../c/d";
  let expected = "../../c/d";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_to_b()
{
  let from = ".";
  let to = "b";
  let expected = "b";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_slash_to_b()
{
  let from = "./";
  let to = "b";
  let expected = "./b";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_to_b_slash()
{
  let from = ".";
  let to = "b/";
  let expected = "b/";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_dot_slash_to_b_slash()
{
  let from = "./";
  let to = "b/";
  let expected = "./b/";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}

#[ test ]
fn test_relative_a_dot_dot_to_b_dot_dot()
{
  let from = "a/../b/..";
  let to = "b";
  let expected = "b";
  assert_eq!( the_module::path::path_relative( from, to ), PathBuf::from( expected ) );
}