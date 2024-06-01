use super::*;


#[ test ]
fn join_empty()
{
  let ( expected, paths ) = ( "", vec![ "" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}

#[ test ]
fn join_several_empties()
{
  let ( expected, paths ) = ( "", vec![ "", "" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn root_with_absolute()
{
  let ( expected, paths ) = ( "/a/b", vec![ "/", "/a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn root_with_relative()
{
  let ( expected, paths ) = ( "/a/b", vec![ "/", "a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn dir_with_absolute()
{
  let ( expected, paths ) = ( "/a/b", vec![ "/dir", "/a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}



#[ test ]
fn dir_with_relative()
{
  let ( expected, paths ) = ( "/dir/a/b", vec![ "/dir", "a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn trailed_dir_with_absolute()
{
  let ( expected, paths ) =  ( "/a/b", vec![ "/dir/", "/a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}

#[ test ]
fn trailed_dir_with_relative()
{
  let ( expected, paths ) = ( "/dir/a/b", vec![ "/dir/", "a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn dir_with_down()
{
  let ( expected, paths ) = ( "/a/b", vec![ "/dir", "../a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn trailed_dir_with_down()
{
  let ( expected, paths ) = ( "/dir/a/b", vec![ "/dir/", "../a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}



#[ test ]
fn dir_with_several_down()
{
  let ( expected, paths ) = ( "/a/b", vec![ "/dir/dir2", "../../a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn trailed_dir_with_several_down()
{
  let ( expected, paths ) = ( "/a/b", vec![ "/dir/", "../../a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn dir_with_several_down_go_out_of_root()
{
  let ( expected, paths ) = ( "/../a/b", vec![ "/dir", "../../a/b" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}

#[ test ]
fn trailed_absolute_with_trailed_down()
{
  let ( expected, paths ) = ( "/a/b/", vec![ "/a/b/", "../" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn absolute_with_trailed_down()
{
  let ( expected, paths ) = ( "/a/", vec![ "/a/b", "../" ]) ;
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn trailed_absolute_with_down()
{
  let ( expected, paths ) = ( "/a/b", vec![ "/a/b/", ".." ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn trailed_absolute_with_trailed_here()
{
  let ( expected, paths ) = ( "/a/b/", vec![ "/a/b/", "./" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}



#[ test ]
fn absolute_with_trailed_here()
{
  let ( expected, paths ) = ( "/a/b/", vec![ "/a/b", "./" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn trailed_absolute_with_here()
{
  let ( expected, paths ) = ( "/a/b", vec![ "/a/b/", "." ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn join_with_empty()
{
  let ( expected, paths ) = ( "/a/b/c", vec![ "", "a/b", "", "c", "" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}

#[ test ]
fn join_windows_os_paths()
{
  let ( expected, paths ) = ( "/c/foo/bar/", vec![ "c :\\", "foo\\", "bar\\" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn join_unix_os_paths()
{
  let ( expected, paths ) = ( "/baz/foo", vec![ "/bar/", "/baz", "foo/", "." ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn join_unix_os_paths_2()
{
  let ( expected, paths ) = ( "/baz/foo/z", vec![ "/bar/", "/baz", "foo/", ".", "z" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn more_complicated_cases_1()
{
  let ( expected, paths ) = ( "/aa/bb//cc", vec![ "/aa", "bb//", "cc" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}



#[ test ]
fn more_complicated_cases_2()
{
  let ( expected, paths ) = ( "/bb/cc", vec![ "/aa", "/bb", "cc" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn more_complicated_cases_3()
{
  let ( expected, paths ) = ( "//aa/bb//cc//", vec![ "//aa", "bb//", "cc//" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}


#[ test ]
fn more_complicated_cases_4()
{
  let ( expected, paths ) = ( "/aa/bb//cc", vec![ "/aa", "bb//", "cc", "." ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}

#[ test ]
fn more_complicated_cases_5()
{
  let ( expected, paths ) = ( "//b//d/..e", vec![ "/", "a", "//b//", "././c", "../d", "..e" ] );
  let result = the_module::path::join_paths( paths.clone().into_iter() );
  assert_eq!( result, std::path::PathBuf::from( expected ), "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'", paths, expected, result.to_string_lossy() );
}