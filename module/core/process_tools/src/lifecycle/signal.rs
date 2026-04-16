/// Define a private namespace for all its items.
mod private
{

  /// Standard POSIX signals (Linux signal numbers).
  ///
  /// Single source of truth for both `signal_name` and `signal_number` — prevents
  /// the forward and reverse mappings from drifting out of sync.
  ///
  /// Signal numbers are Linux-specific. macOS/BSD differ for some signals
  /// (e.g., SIGUSR1 = 10 on Linux, 30 on macOS).
  const SIGNALS : &[ ( i32, &str ) ] =
  &[
    ( 1, "SIGHUP" ),
    ( 2, "SIGINT" ),
    ( 3, "SIGQUIT" ),
    ( 4, "SIGILL" ),
    ( 5, "SIGTRAP" ),
    ( 6, "SIGABRT" ),
    ( 7, "SIGBUS" ),
    ( 8, "SIGFPE" ),
    ( 9, "SIGKILL" ),
    ( 10, "SIGUSR1" ),
    ( 11, "SIGSEGV" ),
    ( 12, "SIGUSR2" ),
    ( 13, "SIGPIPE" ),
    ( 14, "SIGALRM" ),
    ( 15, "SIGTERM" ),
    ( 16, "SIGSTKFLT" ),
    ( 17, "SIGCHLD" ),
    ( 18, "SIGCONT" ),
    ( 19, "SIGSTOP" ),
    ( 20, "SIGTSTP" ),
    ( 21, "SIGTTIN" ),
    ( 22, "SIGTTOU" ),
    ( 23, "SIGURG" ),
    ( 24, "SIGXCPU" ),
    ( 25, "SIGXFSZ" ),
  ];

  ///
  /// Returns the name of a POSIX signal given its number.
  ///
  /// Performs a forward lookup in the signal table. Returns `"UNKNOWN"` for
  /// unmapped signal numbers.
  ///
  /// # Arguments
  /// - `signal` — The signal number (e.g., 9 for SIGKILL).
  ///
  /// # Examples
  ///
  /// ```rust
  /// use process_tools ::lifecycle ::signal;
  ///
  /// assert_eq!( signal ::signal_name( 9 ), "SIGKILL" );
  /// assert_eq!( signal ::signal_name( 15 ), "SIGTERM" );
  /// assert_eq!( signal ::signal_name( 999 ), "UNKNOWN" );
  /// ```
  ///
  #[ must_use ]
  pub fn signal_name( signal : i32 ) -> &'static str
  {
    SIGNALS
    .iter()
    .find( | &&( n, _ ) | n == signal )
    .map_or( "UNKNOWN", | &( _, name ) | name )
  }

  ///
  /// Returns the signal number for a given POSIX signal name.
  ///
  /// Performs a reverse lookup in the signal table. Returns `None` for
  /// unrecognized names. Name matching is exact (case-sensitive).
  ///
  /// # Arguments
  /// - `name` — The signal name (e.g., `"SIGKILL"`).
  ///
  /// # Examples
  ///
  /// ```rust
  /// use process_tools ::lifecycle ::signal;
  ///
  /// assert_eq!( signal ::signal_number( "SIGKILL" ), Some( 9 ) );
  /// assert_eq!( signal ::signal_number( "NONEXISTENT" ), None );
  /// ```
  ///
  #[ must_use ]
  pub fn signal_number( name : &str ) -> Option< i32 >
  {
    SIGNALS
    .iter()
    .find( | &&( _, n ) | n == name )
    .map( | &( num, _ ) | num )
  }

  ///
  /// Returns a reference to the complete signal table.
  ///
  /// Each entry is a `( number, name )` pair. Useful for enumeration or
  /// display purposes.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use process_tools ::lifecycle ::signal;
  ///
  /// let signals = signal ::all_signals();
  /// assert!( signals.len() >= 25 );
  /// ```
  ///
  #[ must_use ]
  pub fn all_signals() -> &'static [ ( i32, &'static str ) ]
  {
    SIGNALS
  }
}

crate ::mod_interface!
{
  own use signal_name;
  own use signal_number;
  own use all_signals;
}
