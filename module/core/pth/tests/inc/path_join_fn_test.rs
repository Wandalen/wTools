use super::*;
use std::path::PathBuf;

#[ test ]
fn join_empty()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "".into(), vec![ "".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn join_several_empties()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "".into(), vec![ "".into(), "".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn root_with_absolute()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/".into(), "/a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn root_with_relative()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/".into(), "a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn dir_with_absolute()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/dir".into(), "/a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn dir_with_relative()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/dir/a/b".into(), vec![ "/dir".into(), "a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn trailed_dir_with_absolute()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/dir/".into(), "/a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn trailed_dir_with_relative()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/dir/a/b".into(), vec![ "/dir/".into(), "a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn dir_with_down()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/dir".into(), "../a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn trailed_dir_with_down()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/dir/a/b".into(), vec![ "/dir/".into(), "../a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn dir_with_several_down()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/dir/dir2".into(), "../../a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn trailed_dir_with_several_down()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/dir/".into(), "../../a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn dir_with_several_down_go_out_of_root()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/../a/b".into(), vec![ "/dir".into(), "../../a/b".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn trailed_absolute_with_trailed_down()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b/".into(), vec![ "/a/b/".into(), "../".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn absolute_with_trailed_down()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/".into(), vec![ "/a/b".into(), "../".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn trailed_absolute_with_down()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/a/b/".into(), "..".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn trailed_absolute_with_trailed_here()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b/".into(), vec![ "/a/b/".into(), "./".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn absolute_with_trailed_here()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b/".into(), vec![ "/a/b".into(), "./".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn trailed_absolute_with_here()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b".into(), vec![ "/a/b/".into(), ".".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn join_with_empty()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/a/b/c".into(), vec![ "".into(), "a/b".into(), "".into(), "c".into(), "".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn join_windows_os_paths()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/c:/foo/bar/".into(), vec![ "c:\\".into(), "foo\\".into(), "bar\\".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn join_unix_os_paths()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/baz/foo".into(), vec![ "/bar/".into(), "/baz".into(), "foo/".into(), ".".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn join_unix_os_paths_2()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/baz/foo/z".into(), vec![ "/bar/".into(), "/baz".into(), "foo/".into(), ".".into(), "z".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn more_complicated_cases_1()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/aa/bb//cc".into(), vec![ "/aa".into(), "bb//".into(), "cc".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn more_complicated_cases_2()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/bb/cc".into(), vec![ "/aa".into(), "/bb".into(), "cc".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn more_complicated_cases_3()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "//aa/bb//cc//".into(), vec![ "//aa".into(), "bb//".into(), "cc//".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn more_complicated_cases_4()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "/aa/bb//cc".into(), vec![ "/aa".into(), "bb//".into(), "cc".into(), ".".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}

#[ test ]
fn more_complicated_cases_5()
{
  let ( expected, paths ) : ( PathBuf, Vec< PathBuf > ) = ( "//b//d/..e".into(), vec![ "/".into(), "a".into(), "//b//".into(), "././c".into(), "../d".into(), "..e".into() ] );
  let result = the_module::path::iter_join( paths.iter().map( |p| p.as_path() ) );
  assert_eq!
  (
    result,
    expected,
    "Test failed. Paths: '{:?}', Expected: '{}', Got: '{}'",
    paths,
    expected.display(),
    result.to_string_lossy(),
  );
}