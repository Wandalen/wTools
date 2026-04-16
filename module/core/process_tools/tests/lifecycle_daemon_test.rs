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
}
