/// Internal namespace.
mod private
{
  /// Execution configuration for a single script run.
  ///
  /// All fields default to empty/absent values. The runner applies sensible
  /// defaults at execution time: `build_profile` defaults to `"debug"`,
  /// `cargo_path` to `"cargo"`, `edition` to `"2021"`, `package_name` to
  /// `"script"`. Capture and cleanup are enabled by default.
  #[ derive( Debug, Clone ) ]
  pub struct RunOptions
  {
    /// Cargo build profile: `"debug"` or `"release"`. Empty means `"debug"`.
    pub build_profile : String,
    /// Persistent artifact cache directory. `None` means a per-run ephemeral
    /// directory inside the temp workspace, removed during cleanup.
    pub target_dir : Option< String >,
    /// Path to the Cargo binary. Empty means `"cargo"` resolved via PATH.
    pub cargo_path : String,
    /// Maximum execution time in milliseconds. `None` means no limit.
    pub timeout_ms : Option< u64 >,
    /// Additional Cargo features to enable.
    pub features : Vec< String >,
    /// Environment variables for the subprocess, each as a `"KEY=VALUE"` string.
    pub env_vars : Vec< String >,
    /// Rust edition for generated manifests. Empty means `"2021"`.
    pub edition : String,
    /// Package name for generated manifests. Empty means `"script"`.
    pub package_name : String,
    /// When true, stdout and stderr are captured into buffers.
    /// When false, they are forwarded to the current terminal.
    pub capture : bool,
    /// When true, the temporary workspace is removed after the run completes.
    pub cleanup : bool,
  }

  impl Default for RunOptions
  {
    fn default() -> Self
    {
      Self
      {
        build_profile : String::new(),
        target_dir : None,
        cargo_path : String::new(),
        timeout_ms : None,
        features : vec![],
        env_vars : vec![],
        edition : String::new(),
        package_name : String::new(),
        capture : true,
        cleanup : true,
      }
    }
  }
}

mod_interface::mod_interface!
{
  exposed use private::RunOptions;
  prelude use private::RunOptions;
}
