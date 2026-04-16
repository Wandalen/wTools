#![ allow( missing_docs ) ]

include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use ::process_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools :: *;

#[ cfg( all( feature = "enabled", unix ) ) ]
mod inc
{
  use super :: *;
  use the_module ::lifecycle ::daemon;

  /// T11: PID file round-trip — write, read, remove.
  ///
  /// Verifies that `write_pidfile` persists the PID, `read_pidfile` recovers
  /// the same value, and `remove_pidfile` deletes the file.
  #[ test ]
  fn pidfile_round_trip()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "test.pid" );
    let pid = std ::process ::id();

    daemon ::write_pidfile( &pidfile, pid ).expect( "write_pidfile failed" );
    assert!( pidfile.exists(), "PID file must exist after write" );

    let read_pid = daemon ::read_pidfile( &pidfile ).expect( "read_pidfile failed" );
    assert_eq!( read_pid, pid, "read PID must match written PID" );

    daemon ::remove_pidfile( &pidfile ).expect( "remove_pidfile failed" );
    assert!( !pidfile.exists(), "PID file must be gone after remove" );
  }

  // --- Corner cases: write_pidfile ---

  /// `write_pidfile` with PID 0 round-trips correctly.
  #[ test ]
  fn write_pidfile_zero()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "zero.pid" );
    daemon ::write_pidfile( &pidfile, 0 ).expect( "write_pidfile(0) must succeed" );
    let read_pid = daemon ::read_pidfile( &pidfile ).expect( "read_pidfile must succeed" );
    assert_eq!( read_pid, 0 );
  }

  /// `write_pidfile` with `u32::MAX` round-trips correctly.
  #[ test ]
  fn write_pidfile_u32_max()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "max.pid" );
    daemon ::write_pidfile( &pidfile, u32 ::MAX ).expect( "write_pidfile(MAX) must succeed" );
    let read_pid = daemon ::read_pidfile( &pidfile ).expect( "read_pidfile must succeed" );
    assert_eq!( read_pid, u32 ::MAX );
  }

  /// `write_pidfile` to nonexistent directory returns `Err`.
  #[ test ]
  fn write_pidfile_nonexistent_dir()
  {
    let result = daemon ::write_pidfile(
      std ::path ::Path ::new( "/nonexistent/dir/test.pid" ),
      12345,
    );
    assert!( result.is_err(), "writing to nonexistent dir must error" );
  }

  // --- Corner cases: read_pidfile ---

  /// `read_pidfile` with empty file returns `Err`.
  #[ test ]
  fn read_pidfile_empty()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "empty.pid" );
    std ::fs ::write( &pidfile, "" ).unwrap();
    let result = daemon ::read_pidfile( &pidfile );
    assert!( result.is_err(), "empty file must error" );
  }

  /// `read_pidfile` with non-numeric content returns `Err`.
  #[ test ]
  fn read_pidfile_non_numeric()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "bad.pid" );
    std ::fs ::write( &pidfile, "abc" ).unwrap();
    let result = daemon ::read_pidfile( &pidfile );
    assert!( result.is_err(), "non-numeric content must error" );
  }

  /// `read_pidfile` with negative value returns `Err` (`u32` cannot be negative).
  #[ test ]
  fn read_pidfile_negative()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "neg.pid" );
    std ::fs ::write( &pidfile, "-1" ).unwrap();
    let result = daemon ::read_pidfile( &pidfile );
    assert!( result.is_err(), "negative value must error for u32 parse" );
  }

  /// `read_pidfile` with overflow value (exceeds `u32`) returns `Err`.
  #[ test ]
  fn read_pidfile_overflow()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "big.pid" );
    std ::fs ::write( &pidfile, "99999999999999" ).unwrap();
    let result = daemon ::read_pidfile( &pidfile );
    assert!( result.is_err(), "overflow value must error" );
  }

  /// `read_pidfile` from nonexistent file returns `Err`.
  #[ test ]
  fn read_pidfile_missing_file()
  {
    let result = daemon ::read_pidfile( std ::path ::Path ::new( "/nonexistent/test.pid" ) );
    assert!( result.is_err(), "missing file must error" );
  }

  /// `read_pidfile` with whitespace-padded content parses correctly.
  #[ test ]
  fn read_pidfile_whitespace_padded()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "padded.pid" );
    std ::fs ::write( &pidfile, "  42  \n" ).unwrap();
    let read_pid = daemon ::read_pidfile( &pidfile ).expect( "whitespace-padded must parse" );
    assert_eq!( read_pid, 42 );
  }

  // --- Corner cases: remove_pidfile ---

  /// `remove_pidfile` on nonexistent file returns `Err`.
  #[ test ]
  fn remove_pidfile_nonexistent()
  {
    let result = daemon ::remove_pidfile(
      std ::path ::Path ::new( "/nonexistent/test.pid" ),
    );
    assert!( result.is_err(), "removing nonexistent file must error" );
  }

  /// `write_pidfile` overwrites existing file content.
  #[ test ]
  fn write_pidfile_overwrites()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "overwrite.pid" );
    daemon ::write_pidfile( &pidfile, 111 ).unwrap();
    daemon ::write_pidfile( &pidfile, 222 ).unwrap();
    let read_pid = daemon ::read_pidfile( &pidfile ).unwrap();
    assert_eq!( read_pid, 222, "second write must overwrite first" );
  }
}
