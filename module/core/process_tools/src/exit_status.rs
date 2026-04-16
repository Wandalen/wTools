/// Define a private namespace for all its items.
mod private
{
  use std ::process ::ExitStatus;

  ///
  /// Constructs an [`ExitStatus`] from an exit code without spawning a process.
  ///
  /// Hides the platform-specific encoding:
  /// - **Unix:** POSIX `waitpid` status word format — `from_raw( code << 8 )`
  /// - **Windows:** Direct exit code — `from_raw( code as u32 )`
  ///
  /// # Arguments
  /// - `code` — The exit code to encode (0 = success, non-zero = failure).
  ///
  /// # Pitfalls
  ///
  /// **Valid range is 0–255.** On Unix, only the low 8 bits of `code` are
  /// preserved in the POSIX `waitpid` status word. Codes outside 0–255
  /// produce an [`ExitStatus`] with inconsistent semantics — for example,
  /// `code` 256 yields `code() == Some(0)` yet `success() == false`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use process_tools ::exit_status ::synthetic_exit_status;
  ///
  /// let success = synthetic_exit_status( 0 );
  /// assert!( success.success() );
  /// assert_eq!( success.code(), Some( 0 ) );
  ///
  /// let failure = synthetic_exit_status( 1 );
  /// assert!( !failure.success() );
  /// assert_eq!( failure.code(), Some( 1 ) );
  /// ```
  ///
  #[ must_use ]
  pub fn synthetic_exit_status( code : i32 ) -> ExitStatus
  {
    #[ cfg( unix ) ]
    {
      use std ::os ::unix ::process ::ExitStatusExt;
      ExitStatus ::from_raw( code << 8 )
    }
    #[ cfg( windows ) ]
    {
      use std ::os ::windows ::process ::ExitStatusExt;
      ExitStatus ::from_raw( code as u32 )
    }
  }

  ///
  /// Constructs a success [`ExitStatus`] (exit code 0) without spawning a process.
  ///
  /// Equivalent to `synthetic_exit_status( 0 )`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use process_tools ::exit_status ::synthetic_success_status;
  ///
  /// let status = synthetic_success_status();
  /// assert!( status.success() );
  /// ```
  ///
  #[ must_use ]
  pub fn synthetic_success_status() -> ExitStatus
  {
    synthetic_exit_status( 0 )
  }

  ///
  /// Constructs a failure [`ExitStatus`] (exit code 1) without spawning a process.
  ///
  /// Equivalent to `synthetic_exit_status( 1 )`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use process_tools ::exit_status ::synthetic_failure_status;
  ///
  /// let status = synthetic_failure_status();
  /// assert!( !status.success() );
  /// assert_eq!( status.code(), Some( 1 ) );
  /// ```
  ///
  #[ must_use ]
  pub fn synthetic_failure_status() -> ExitStatus
  {
    synthetic_exit_status( 1 )
  }
}

crate ::mod_interface!
{
  own use synthetic_exit_status;
  own use synthetic_success_status;
  own use synthetic_failure_status;
}
