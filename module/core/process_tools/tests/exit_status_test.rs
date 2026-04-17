#![ allow( missing_docs ) ]

include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use ::process_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools :: *;

#[ cfg( feature = "enabled" ) ]
mod inc
{
  use super :: *;
  use the_module ::exit_status;

  /// T01: Zero exit code produces successful `ExitStatus` with `.code()` == `Some(0)`.
  #[ test ]
  fn synthetic_exit_status_zero_is_success()
  {
    let status = exit_status ::synthetic_exit_status( 0 );
    assert!( status.success(), "exit code 0 must be success" );
    assert_eq!( status.code(), Some( 0 ) );
  }

  /// T02: Exit code 1 produces failure `ExitStatus` with `.code()` == `Some(1)`.
  #[ test ]
  fn synthetic_exit_status_one_is_failure()
  {
    let status = exit_status ::synthetic_exit_status( 1 );
    assert!( !status.success(), "exit code 1 must not be success" );
    assert_eq!( status.code(), Some( 1 ) );
  }

  /// T03: Arbitrary exit code 42 encodes correctly.
  #[ test ]
  fn synthetic_exit_status_arbitrary_code()
  {
    let status = exit_status ::synthetic_exit_status( 42 );
    assert_eq!( status.code(), Some( 42 ) );
  }

  /// T04: `synthetic_success_status()` is equivalent to `synthetic_exit_status(0)`.
  #[ test ]
  fn synthetic_success_status_returns_zero()
  {
    let status = exit_status ::synthetic_success_status();
    assert!( status.success() );
    assert_eq!( status.code(), Some( 0 ) );
  }

  /// T05: `synthetic_failure_status()` is equivalent to `synthetic_exit_status(1)`.
  #[ test ]
  fn synthetic_failure_status_returns_one()
  {
    let status = exit_status ::synthetic_failure_status();
    assert!( !status.success() );
    assert_eq!( status.code(), Some( 1 ) );
  }

  // --- Corner cases ---

  /// Exit code 255 (max single-byte) encodes correctly on all platforms.
  #[ test ]
  fn synthetic_exit_status_max_byte_255()
  {
    let status = exit_status ::synthetic_exit_status( 255 );
    assert!( !status.success(), "code 255 must be failure" );
    assert_eq!( status.code(), Some( 255 ) );
  }

  /// Exit code 127 ("command not found" convention) encodes correctly.
  #[ test ]
  fn synthetic_exit_status_command_not_found_127()
  {
    let status = exit_status ::synthetic_exit_status( 127 );
    assert!( !status.success() );
    assert_eq!( status.code(), Some( 127 ) );
  }

  /// Exit code 128+9=137 (killed by SIGKILL convention) encodes correctly.
  #[ test ]
  fn synthetic_exit_status_signal_killed_137()
  {
    let status = exit_status ::synthetic_exit_status( 137 );
    assert!( !status.success() );
    assert_eq!( status.code(), Some( 137 ) );
  }

  /// Negative exit code -1: on Unix wraps to 255 due to POSIX 8-bit encoding.
  #[ cfg( unix ) ]
  #[ test ]
  fn synthetic_exit_status_negative_one_wraps()
  {
    let status = exit_status ::synthetic_exit_status( -1 );
    // (-1) << 8 = 0xFFFFFF00; WEXITSTATUS extracts bits 8-15 = 0xFF = 255
    assert_eq!( status.code(), Some( 255 ) );
  }

  /// Exit code 256 wraps to extracted code 0 on Unix, but `success()` remains false.
  ///
  /// POSIX `WEXITSTATUS` extracts only bits 8-15, giving 0.
  /// However, `ExitStatus::success()` checks the full raw status word (!= 0),
  /// so the result has code()==Some(0) yet success()==false — a broken invariant
  /// demonstrating why codes must be in range 0-255.
  #[ cfg( unix ) ]
  #[ test ]
  fn synthetic_exit_status_256_wraps_inconsistently()
  {
    let status = exit_status ::synthetic_exit_status( 256 );
    // 256 << 8 = 0x10000; WEXITSTATUS = (0x10000 >> 8) & 0xFF = 0
    assert_eq!( status.code(), Some( 0 ), "WEXITSTATUS extracts 0" );
    // But success() checks raw != 0, so it returns false despite code()==0.
    assert!( !status.success(), "raw status 0x10000 != 0 so not success" );
  }

  /// Exit code `i32::MAX` wraps to 255 on Unix due to 8-bit POSIX encoding.
  #[ cfg( unix ) ]
  #[ test ]
  fn synthetic_exit_status_i32_max_wraps()
  {
    let status = exit_status ::synthetic_exit_status( i32 ::MAX );
    // 0x7FFFFFFF << 8 = 0xFFFFFF00; WEXITSTATUS = 0xFF = 255
    assert_eq!( status.code(), Some( 255 ) );
  }

  /// Exit code `i32::MIN` wraps to raw status 0 on Unix (the only out-of-range
  /// code that accidentally produces a valid success status).
  #[ cfg( unix ) ]
  #[ test ]
  fn synthetic_exit_status_i32_min_wraps()
  {
    let status = exit_status ::synthetic_exit_status( i32 ::MIN );
    // 0x80000000 << 8 = 0x00000000 (wrapping); raw status == 0
    assert_eq!( status.code(), Some( 0 ) );
    assert!( status.success(), "raw status happens to be 0" );
  }

  /// PATH regression: synthetic helpers must not inspect PATH.
  ///
  /// Before the BB3 fix, `ExitStatus` construction in `wrun_core` called `/usr/bin/true` and
  /// `/usr/bin/false`. If `PATH` was cleared those helpers returned `Err`, causing an infinite spin.
  /// This test confirms the `process_tools` API works regardless of `PATH`.
  // test_kind: bug_reproducer(issue-bb3-synthetic-exit-status)
  #[ cfg( unix ) ]
  #[ test ]
  fn synthetic_status_works_with_empty_path()
  {
    let orig = std::env::var( "PATH" ).unwrap_or_default();
    std::env::set_var( "PATH", "/nonexistent" );

    let success = exit_status ::synthetic_success_status();
    let failure = exit_status ::synthetic_failure_status();

    std::env::set_var( "PATH", orig );

    assert!( success.success(), "success must not depend on PATH" );
    assert!( !failure.success(), "failure must not depend on PATH" );
  }
}
