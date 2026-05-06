/// Internal namespace.
mod private
{
  use std::borrow::Cow;

  /// Captured output from a completed script execution.
  ///
  /// Holds the exit status code, complete standard output, and complete standard
  /// error as raw byte sequences. Assertion methods on this type are designed for
  /// use in Rust test functions — they panic with a descriptive message on failure
  /// rather than returning a `Result`.
  #[ derive( Debug, Clone, Default ) ]
  pub struct CapturedOutput
  {
    /// Exit code of the child process. Zero indicates success.
    pub exit_status : i32,
    /// Complete standard output captured from the process.
    pub stdout : Vec< u8 >,
    /// Complete standard error captured from the process.
    pub stderr : Vec< u8 >,
  }

  impl CapturedOutput
  {
    /// Decode stdout as UTF-8, lossily replacing invalid byte sequences.
    #[ must_use ]
    pub fn stdout_str( &self ) -> Cow< '_, str >
    {
      String::from_utf8_lossy( &self.stdout )
    }

    /// Decode stderr as UTF-8, lossily replacing invalid byte sequences.
    #[ must_use ]
    pub fn stderr_str( &self ) -> Cow< '_, str >
    {
      String::from_utf8_lossy( &self.stderr )
    }

    // ── Predicates ──────────────────────────────────────────────────────────────

    /// Returns `true` if exit status is zero.
    #[ must_use ]
    pub fn exit_ok( &self ) -> bool
    {
      self.exit_status == 0
    }

    /// Returns `true` if stdout exactly equals `expected` after UTF-8 decoding.
    #[ must_use ]
    pub fn stdout_eq( &self, expected : &str ) -> bool
    {
      self.stdout_str() == expected
    }

    /// Returns `true` if stderr exactly equals `expected` after UTF-8 decoding.
    #[ must_use ]
    pub fn stderr_eq( &self, expected : &str ) -> bool
    {
      self.stderr_str() == expected
    }

    /// Returns `true` if stdout contains `needle` as a substring.
    #[ must_use ]
    pub fn stdout_contains( &self, needle : &str ) -> bool
    {
      self.stdout_str().contains( needle )
    }

    /// Returns `true` if stderr contains `needle` as a substring.
    #[ must_use ]
    pub fn stderr_contains( &self, needle : &str ) -> bool
    {
      self.stderr_str().contains( needle )
    }

    // ── Assertions (panic on failure) ────────────────────────────────────────────

    /// Asserts that exit status is zero.
    ///
    /// # Panics
    ///
    /// Panics with exit code and stderr content when exit status is non-zero.
    pub fn assert_exit_ok( &self )
    {
      let exit_status = self.exit_status;
      let stderr = self.stderr_str();
      assert!
      (
        self.exit_ok(),
        "expected exit status 0, got {exit_status}\nstderr:\n{stderr}",
      );
    }

    /// Asserts that stdout exactly equals `expected`.
    ///
    /// # Panics
    ///
    /// Panics with expected/actual diff when stdout does not exactly match.
    pub fn assert_stdout_eq( &self, expected : &str )
    {
      let actual = self.stdout_str();
      assert!
      (
        actual == expected,
        "stdout mismatch\nexpected : {expected:?}\nactual   : {actual:?}",
      );
    }

    /// Asserts that stderr exactly equals `expected`.
    ///
    /// # Panics
    ///
    /// Panics with expected/actual diff when stderr does not exactly match.
    pub fn assert_stderr_eq( &self, expected : &str )
    {
      let actual = self.stderr_str();
      assert!
      (
        actual == expected,
        "stderr mismatch\nexpected : {expected:?}\nactual   : {actual:?}",
      );
    }

    /// Asserts that stdout contains `needle`.
    ///
    /// # Panics
    ///
    /// Panics showing the needle and full stdout content when not found.
    pub fn assert_stdout_contains( &self, needle : &str )
    {
      let actual = self.stdout_str();
      assert!
      (
        actual.contains( needle ),
        "stdout does not contain {needle:?}\nstdout : {actual:?}",
      );
    }

    /// Asserts that stderr contains `needle`.
    ///
    /// # Panics
    ///
    /// Panics showing the needle and full stderr content when not found.
    pub fn assert_stderr_contains( &self, needle : &str )
    {
      let actual = self.stderr_str();
      assert!
      (
        actual.contains( needle ),
        "stderr does not contain {needle:?}\nstderr : {actual:?}",
      );
    }

    /// Asserts that stdout is empty (zero bytes).
    ///
    /// # Panics
    ///
    /// Panics showing the actual stdout content when non-empty.
    pub fn assert_stdout_empty( &self )
    {
      let actual = self.stdout_str();
      assert!( actual.is_empty(), "expected empty stdout, got : {actual:?}" );
    }

    /// Asserts that stderr is empty (zero bytes).
    ///
    /// # Panics
    ///
    /// Panics showing the actual stderr content when non-empty.
    pub fn assert_stderr_empty( &self )
    {
      let actual = self.stderr_str();
      assert!( actual.is_empty(), "expected empty stderr, got : {actual:?}" );
    }
  }
}

mod_interface::mod_interface!
{
  exposed use private::CapturedOutput;
  prelude use private::CapturedOutput;
}
