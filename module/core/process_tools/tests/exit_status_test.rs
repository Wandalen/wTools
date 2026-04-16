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
}
