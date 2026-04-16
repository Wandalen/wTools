/// Define a private namespace for all its items.
#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{
  use std ::io ::{ self, Write };
  use std ::path ::{ Path, PathBuf };
  use former ::Former;

  ///
  /// Writes a PID to a file.
  ///
  /// # Arguments
  /// - `path` — Destination file path.
  /// - `pid` — Process ID to persist.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the file cannot be created or written.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use process_tools ::lifecycle ::daemon;
  ///
  /// daemon ::write_pidfile( std ::path ::Path ::new( "/tmp/test.pid" ), 12345 ).unwrap();
  /// ```
  ///
  pub fn write_pidfile( path : &Path, pid : u32 ) -> io ::Result< () >
  {
    std ::fs ::write( path, pid.to_string() )
  }

  ///
  /// Reads a PID from a file.
  ///
  /// The file must contain a decimal integer optionally surrounded by whitespace.
  ///
  /// # Arguments
  /// - `path` — Path to the PID file.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the file cannot be read or does not contain a valid integer.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use process_tools ::lifecycle ::daemon;
  ///
  /// let pid = daemon ::read_pidfile( std ::path ::Path ::new( "/tmp/test.pid" ) ).unwrap();
  /// ```
  ///
  pub fn read_pidfile( path : &Path ) -> io ::Result< u32 >
  {
    let content = std ::fs ::read_to_string( path )?;
    content.trim().parse().map_err( | e |
    {
      io ::Error ::new( io ::ErrorKind ::InvalidData, format!( "invalid PID in file: {e}" ) )
    })
  }

  ///
  /// Removes a PID file.
  ///
  /// # Arguments
  /// - `path` — Path to the PID file to delete.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the file does not exist or cannot be removed.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use process_tools ::lifecycle ::daemon;
  ///
  /// daemon ::remove_pidfile( std ::path ::Path ::new( "/tmp/test.pid" ) ).unwrap();
  /// ```
  ///
  pub fn remove_pidfile( path : &Path ) -> io ::Result< () >
  {
    std ::fs ::remove_file( path )
  }

  ///
  /// Configuration for the [`daemonize`] function.
  ///
  #[ derive( Debug, Former ) ]
  pub struct DaemonizeOptions
  {
    /// Path to write the daemon PID file. Skipped when `None`.
    pid_file : Option< PathBuf >,
    /// Working directory after daemonization. Defaults to `/`.
    #[ former( default = PathBuf ::from( "/" ) ) ]
    working_dir : PathBuf,
  }

  ///
  /// Daemonizes the current process using the POSIX double-fork pattern.
  ///
  /// After a successful return the calling code runs in a fully detached
  /// daemon process. The original (parent) process has already exited.
  ///
  /// # Known Pitfalls (from wplan source audit)
  ///
  /// ## Pitfall 1 — TOCTOU race in singleton check
  /// Multiple daemon instances can start simultaneously if the PID file
  /// check and write are not atomic. This implementation uses
  /// `flock(LOCK_EX | LOCK_NB)` before any file mutation.
  ///
  /// ## Pitfall 2 — Truncate-before-lock
  /// Truncating the PID file before acquiring the lock allows concurrent
  /// children to both see an empty file. This implementation acquires the
  /// lock first, then truncates and writes.
  ///
  /// ## Pitfall 3 — PID verification after IPC
  /// The parent may observe a socket created by a *different* daemon child.
  /// Callers must verify the PID file contains the expected child PID after
  /// observing readiness signals.
  ///
  /// ## Pitfall 4 — FD closure vs redirection
  /// Closing stderr (fd 2) allows client sockets to reuse that descriptor,
  /// causing `eprintln!()` to write to sockets. This implementation
  /// redirects stdin / stdout / stderr to `/dev/null` instead.
  ///
  /// ## Pitfall 5 — Inherited FD leak
  /// Inherited pipe FDs from the parent's `Command ::output()` are never
  /// closed in the daemon child, causing the parent to hang in nextest.
  /// This implementation closes all FDs from 3 to `sysconf(_SC_OPEN_MAX)`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if `fork`, `setsid`, or file-descriptor operations fail,
  /// or if another daemon instance already holds the PID file lock.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// # #[ cfg( unix ) ]
  /// # {
  /// use process_tools ::lifecycle ::daemon;
  ///
  /// let opts = daemon ::DaemonizeOptions ::former()
  ///   .pid_file( "/var/run/mydaemon.pid" )
  ///   .form();
  /// daemon ::daemonize( &opts ).expect( "daemonization failed" );
  /// // — running in daemon process now —
  /// # }
  /// ```
  ///
  #[ cfg( unix ) ]
  #[ allow( unsafe_code ) ]
  pub fn daemonize( options : &DaemonizeOptions ) -> io ::Result< () >
  {
    // --- First fork: detach from controlling terminal ---
    // SAFETY: fork() is a standard POSIX call. The process state is
    // well-defined at this point (single-threaded recommended).
    match unsafe { libc ::fork() }
    {
      -1 => return Err( io ::Error ::last_os_error() ),
      0 => {} // child continues
      _ =>
      {
        // SAFETY: _exit() terminates without running atexit handlers,
        // preventing double-flush of stdio buffers shared with the child.
        unsafe { libc ::_exit( 0 ) };
      }
    }

    // --- New session: become session leader ---
    // SAFETY: setsid() is a standard POSIX call, valid after fork.
    if unsafe { libc ::setsid() } == -1
    {
      return Err( io ::Error ::last_os_error() );
    }

    // --- Second fork: prevent reacquiring a controlling terminal ---
    // SAFETY: Same as the first fork.
    match unsafe { libc ::fork() }
    {
      -1 => return Err( io ::Error ::last_os_error() ),
      0 => {} // grandchild continues (this is the daemon)
      _ =>
      {
        // SAFETY: Same _exit rationale.
        unsafe { libc ::_exit( 0 ) };
      }
    }

    // --- Change working directory ---
    std ::env ::set_current_dir( &options.working_dir )?;

    // --- Reset umask ---
    // SAFETY: umask() is a trivial POSIX call with no failure mode.
    unsafe { libc ::umask( 0 ) };

    // --- Pitfall 4: Redirect stdin/stdout/stderr to /dev/null ---
    redirect_std_fds()?;

    // --- Pitfall 5: Close inherited FDs ---
    close_inherited_fds();

    // --- Pitfall 1 & 2: Write PID file with flock ---
    if let Some( ref pid_file ) = options.pid_file
    {
      write_pidfile_locked( pid_file )?;
    }

    Ok( () )
  }

  /// Redirects stdin, stdout, stderr to `/dev/null`.
  ///
  /// Pitfall 4 fix: redirect instead of close — prevents fd reuse by
  /// sockets, which would corrupt application I/O.
  #[ cfg( unix ) ]
  #[ allow( unsafe_code ) ]
  fn redirect_std_fds() -> io ::Result< () >
  {
    use std ::os ::unix ::io ::AsRawFd;

    let dev_null = std ::fs ::OpenOptions ::new()
      .read( true )
      .write( true )
      .open( "/dev/null" )?;
    let fd = dev_null.as_raw_fd();

    // SAFETY: dup2 duplicates a valid open fd to the target descriptor.
    // fds 0, 1, 2 are well-known standard descriptors.
    unsafe
    {
      libc ::dup2( fd, libc ::STDIN_FILENO );
      libc ::dup2( fd, libc ::STDOUT_FILENO );
      libc ::dup2( fd, libc ::STDERR_FILENO );
    }
    // `dev_null` is dropped here, closing the original fd — the dup'd
    // descriptors 0/1/2 remain valid as independent duplicates.
    Ok( () )
  }

  /// Closes all file descriptors from 3 to `sysconf(_SC_OPEN_MAX)`.
  ///
  /// Pitfall 5 fix: prevents inherited pipe FDs from keeping parent
  /// processes blocked on read.
  #[ cfg( unix ) ]
  #[ allow( unsafe_code ) ]
  fn close_inherited_fds()
  {
    // SAFETY: sysconf(_SC_OPEN_MAX) is a read-only query.
    let max_fd = unsafe { libc ::sysconf( libc ::_SC_OPEN_MAX ) };
    let max_fd = if max_fd <= 0 { 1024_i64 } else { max_fd };

    for fd in 3 ..i32 ::try_from( max_fd ).unwrap_or( 1024 )
    {
      // SAFETY: close() on an already-closed fd returns -1 but is harmless.
      unsafe { libc ::close( fd ) };
    }
  }

  /// Writes the current PID to a file with an exclusive non-blocking lock.
  ///
  /// Pitfall 1 fix: `flock(LOCK_EX | LOCK_NB)` prevents TOCTOU races.
  /// Pitfall 2 fix: lock acquired before truncation.
  #[ cfg( unix ) ]
  #[ allow( unsafe_code ) ]
  fn write_pidfile_locked( path : &Path ) -> io ::Result< () >
  {
    use std ::os ::unix ::io ::AsRawFd;

    let file = std ::fs ::OpenOptions ::new()
      .create( true )
      .write( true )
      .truncate( false ) // Pitfall 2: never truncate before locking
      .open( path )?;

    // SAFETY: flock() is a standard POSIX call on a valid fd.
    let ret = unsafe
    {
      libc ::flock( file.as_raw_fd(), libc ::LOCK_EX | libc ::LOCK_NB )
    };
    if ret == -1
    {
      return Err( io ::Error ::new
      (
        io ::ErrorKind ::AlreadyExists,
        "another daemon instance holds the PID file lock",
      ));
    }

    // Now safe to truncate and write while holding the lock.
    file.set_len( 0 )?;
    write!( &file, "{}", std ::process ::id() )?;

    // Intentionally leak the File handle so the flock is held for the
    // daemon's entire lifetime. The OS releases it on process exit.
    std ::mem ::forget( file );

    Ok( () )
  }
}

#[ cfg( unix ) ]
crate ::mod_interface!
{
  own use write_pidfile;
  own use read_pidfile;
  own use remove_pidfile;
  own use DaemonizeOptions;
  own use daemonize;
}
