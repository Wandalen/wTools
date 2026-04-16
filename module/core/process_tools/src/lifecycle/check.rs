/// Define a private namespace for all its items.
mod private
{
  use std ::io;
  use std ::path ::Path;
  use core ::time ::Duration;
  use std ::time ::Instant;

  ///
  /// Checks whether a process with the given PID is alive.
  ///
  /// Uses the POSIX `kill(pid, 0)` syscall to probe process existence
  /// without delivering a signal.
  ///
  /// # Arguments
  /// - `pid` — The process ID to check. Must be positive.
  ///
  /// # Returns
  /// - `Ok(true)` — Process exists (even if caller lacks permission to signal it).
  /// - `Ok(false)` — No process with this PID exists.
  /// - `Err(_)` — PID is invalid (≤ 0), platform unsupported, or unexpected system error.
  ///
  /// # Errors
  ///
  /// Returns `Err` when `pid` ≤ 0, the platform is unsupported (non-Unix),
  /// or an unexpected `errno` is encountered.
  ///
  /// # Pitfalls
  ///
  /// **PID reuse:** Between checking and acting on the result, the kernel
  /// may reuse the PID for a different process. This is inherent to PID-based
  /// APIs and cannot be prevented at the application level.
  ///
  /// **EPERM means alive:** When `kill(pid, 0)` returns EPERM, the process
  /// IS alive — the caller just lacks permission to signal it. Treating all
  /// errors as "not alive" is a common mistake.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # #[ cfg( unix ) ]
  /// # {
  /// use process_tools ::lifecycle ::check;
  ///
  /// let pid = i32 ::try_from( std ::process ::id() ).unwrap();
  /// let alive = check ::is_process_alive( pid ).unwrap();
  /// assert!( alive );
  /// # }
  /// ```
  ///
  #[ cfg( unix ) ]
  #[ allow( unsafe_code ) ]
  pub fn is_process_alive( pid : i32 ) -> io ::Result< bool >
  {
    if pid <= 0
    {
      return Err( io ::Error ::new
      (
        io ::ErrorKind ::InvalidInput,
        format!( "PID must be positive, got {pid}" ),
      ));
    }

    // SAFETY: `libc::kill(pid, 0)` sends signal 0 (null signal) to the process.
    // This is a read-only probe — it does not modify any process state.
    // The pid argument is validated to be positive above.
    let result = unsafe { libc ::kill( pid, 0 ) };

    if result == 0
    {
      Ok( true )
    }
    else
    {
      let err = io ::Error ::last_os_error();
      match err.raw_os_error()
      {
        Some( libc ::ESRCH ) => Ok( false ),
        Some( libc ::EPERM ) => Ok( true ),
        _ => Err( err ),
      }
    }
  }

  /// Unsupported-platform fallback returning `Err`.
  #[ cfg( not( unix ) ) ]
  pub fn is_process_alive( _pid : i32 ) -> io ::Result< bool >
  {
    Err( io ::Error ::new
    (
      io ::ErrorKind ::Unsupported,
      "is_process_alive requires Unix (libc::kill)",
    ))
  }

  ///
  /// Polls [`is_process_alive`] until the process exits or timeout expires.
  ///
  /// # Arguments
  /// - `pid` — The process ID to monitor.
  /// - `timeout` — Maximum time to wait before returning an error.
  ///
  /// # Returns
  /// - `Ok(())` — Process exited within the timeout.
  /// - `Err(TimedOut)` — Timeout expired while the process is still alive.
  /// - `Err(_)` — System error from [`is_process_alive`].
  ///
  /// # Errors
  ///
  /// Returns `Err(TimedOut)` when the timeout expires while the process is
  /// still alive. Propagates any `Err` from [`is_process_alive`].
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use process_tools ::lifecycle ::check;
  /// use std ::time ::Duration;
  ///
  /// // Wait up to 5 seconds for PID 12345 to exit.
  /// check ::wait_for_exit( 12345, Duration ::from_secs( 5 ) ).ok();
  /// ```
  ///
  #[ must_use = "returns `Err` on timeout — ignoring may hide alive processes" ]
  pub fn wait_for_exit( pid : i32, timeout : Duration ) -> io ::Result< () >
  {
    let start = Instant ::now();
    let poll_interval = Duration ::from_millis( 50 );

    loop
    {
      if !is_process_alive( pid )?
      {
        return Ok( () );
      }
      if start.elapsed() >= timeout
      {
        return Err( io ::Error ::new
        (
          io ::ErrorKind ::TimedOut,
          format!( "process {pid} still alive after {timeout:?}" ),
        ));
      }
      std ::thread ::sleep( poll_interval );
    }
  }

  ///
  /// Reads a PID from a file and checks whether that process is alive.
  ///
  /// The file must contain a decimal integer (optionally surrounded by whitespace).
  ///
  /// # Arguments
  /// - `path` — Path to the PID file.
  ///
  /// # Returns
  /// - `Ok(true)` — File contains a valid PID and the process is alive.
  /// - `Ok(false)` — File contains a valid PID but the process is dead.
  /// - `Err(_)` — File doesn't exist, is not a valid integer, or system error.
  ///
  /// # Errors
  ///
  /// Returns `Err` when the file cannot be read, its contents are not a
  /// valid integer, or [`is_process_alive`] fails.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use process_tools ::lifecycle ::check;
  ///
  /// let alive = check ::is_pidfile_alive( std ::path ::Path ::new( "/var/run/daemon.pid" ) );
  /// ```
  ///
  pub fn is_pidfile_alive( path : &Path ) -> io ::Result< bool >
  {
    let content = std ::fs ::read_to_string( path )?;
    let pid : i32 = content.trim().parse().map_err( | e |
    {
      io ::Error ::new( io ::ErrorKind ::InvalidData, format!( "invalid PID in file: {e}" ) )
    })?;
    is_process_alive( pid )
  }
}

crate ::mod_interface!
{
  own use is_process_alive;
  own use wait_for_exit;
  own use is_pidfile_alive;
}
