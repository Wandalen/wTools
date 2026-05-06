/// Internal namespace.
mod private
{
  use error_tools::*;
  use std::path::{ Path, PathBuf };
  use std::process::{ Command, Stdio };
  use std::sync::mpsc;
  use core::time::Duration;
  use std::time::{ Instant, SystemTime, UNIX_EPOCH };

  use crate::program::{ Plan, Program };
  use crate::run_options::RunOptions;
  use crate::output::CapturedOutput;

  // ── Internal helpers ──────────────────────────────────────────────────────────

  fn effective_profile( opts : &RunOptions ) -> &str
  {
    if opts.build_profile.is_empty() { "debug" } else { &opts.build_profile }
  }

  fn effective_cargo( opts : &RunOptions ) -> &str
  {
    if opts.cargo_path.is_empty() { "cargo" } else { &opts.cargo_path }
  }

  fn effective_edition( opts : &RunOptions ) -> &str
  {
    if opts.edition.is_empty() { "2021" } else { &opts.edition }
  }

  fn effective_package_name( opts : &RunOptions ) -> &str
  {
    if opts.package_name.is_empty() { "script" } else { &opts.package_name }
  }

  fn create_temp_workspace() -> Result< PathBuf >
  {
    let nanos = SystemTime::now()
      .duration_since( UNIX_EPOCH )
      .unwrap_or_default()
      .subsec_nanos();
    let pid = std::process::id();
    let dir_name = format!( "program_tools_{pid}_{nanos}" );
    let workspace_dir = std::env::temp_dir().join( dir_name );
    std::fs::create_dir_all( &workspace_dir )
      .map_err( | e | format_err!( "failed to create temp workspace '{}': {e}", workspace_dir.display() ) )?;
    Ok( workspace_dir )
  }

  fn write_source_file( workspace : &Path, rel_path : &str, data : &str ) -> Result< () >
  {
    let abs_path = workspace.join( rel_path );
    if let Some( parent ) = abs_path.parent()
    {
      std::fs::create_dir_all( parent )
        .map_err( | e | format_err!( "failed to create directory '{}': {e}", parent.display() ) )?;
    }
    std::fs::write( &abs_path, data )
      .map_err( | e | format_err!( "failed to write '{}': {e}", abs_path.display() ) )?;
    Ok( () )
  }

  fn generate_manifest( opts : &RunOptions ) -> String
  {
    format!
    (
      "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"{}\"\n\n[dependencies]\n",
      effective_package_name( opts ),
      effective_edition( opts )
    )
  }

  // ── Shared execution core ─────────────────────────────────────────────────────

  /// Execute a fully-configured `Command` under the given options.
  ///
  /// Handles all four combinations of capture × timeout:
  ///
  /// - **capture + timeout**: background thread drains pipes; main thread waits on
  ///   a bounded channel receive.  Limitation (v0.1.0): on timeout expiry the child
  ///   is not killed (pipe ownership is in the thread); the orphaned process runs
  ///   until it exits naturally.
  /// - **capture, no timeout**: `cmd.output()` collects stdout and stderr.
  /// - **forward + timeout**: polling `try_wait()` with `child.kill()` on expiry;
  ///   the child IS killed because no pipes prevent the kill call.
  /// - **forward, no timeout**: `cmd.status()` with inherited stdio.
  fn execute_command( mut cmd : Command, opts : &RunOptions ) -> Result< CapturedOutput >
  {
    let cargo = effective_cargo( opts );

    if opts.capture
    {
      cmd.stdout( Stdio::piped() ).stderr( Stdio::piped() );

      if let Some( timeout_ms ) = opts.timeout_ms
      {
        let child = cmd.spawn()
          .map_err( | e | format_err!( "failed to invoke '{cargo}': {e}" ) )?;

        let ( tx, rx ) = mpsc::channel();
        std::thread::spawn( move ||
        {
          let _ = tx.send( child.wait_with_output() );
        } );

        return match rx.recv_timeout( Duration::from_millis( timeout_ms ) )
        {
          core::result::Result::Ok( inner ) => match inner
          {
            core::result::Result::Ok( output ) => Ok
            ( CapturedOutput
              {
                exit_status : output.status.code().unwrap_or( -1 ),
                stdout : output.stdout,
                stderr : output.stderr,
              }
            ),
            core::result::Result::Err( e ) =>
              Err( format_err!( "failed to invoke '{cargo}': {e}" ) ),
          },
          core::result::Result::Err( _ ) =>
            Err( format_err!( "process timed out after {timeout_ms} ms" ) ),
        };
      }

      let output = cmd.output()
        .map_err( | e | format_err!( "failed to invoke '{cargo}': {e}" ) )?;
      return Ok
      ( CapturedOutput
        {
          exit_status : output.status.code().unwrap_or( -1 ),
          stdout : output.stdout,
          stderr : output.stderr,
        }
      );
    }

    // Forwarding mode: inherit stdio so output flows directly to the caller's terminal.

    if let Some( timeout_ms ) = opts.timeout_ms
    {
      let mut child = cmd.spawn()
        .map_err( | e | format_err!( "failed to invoke '{cargo}': {e}" ) )?;

      let deadline = Instant::now() + Duration::from_millis( timeout_ms );
      loop
      {
        if let Some( status ) = child.try_wait()
          .map_err( | e | format_err!( "failed to poll '{cargo}': {e}" ) )?
        {
          return Ok
          ( CapturedOutput
            {
              exit_status : status.code().unwrap_or( -1 ),
              stdout : vec![],
              stderr : vec![],
            }
          );
        }
        if Instant::now() >= deadline
        {
          let _ = child.kill();
          return Err( format_err!( "process timed out after {timeout_ms} ms" ) );
        }
        std::thread::sleep( Duration::from_millis( 10 ) );
      }
    }

    let status = cmd.status()
      .map_err( | e | format_err!( "failed to invoke '{cargo}': {e}" ) )?;
    Ok
    ( CapturedOutput
      {
        exit_status : status.code().unwrap_or( -1 ),
        stdout : vec![],
        stderr : vec![],
      }
    )
  }

  fn execute_in_workspace( program : &Program, opts : &RunOptions, workspace_dir : &Path ) -> Result< CapturedOutput >
  {
    // Write source files into the workspace.
    for source in &program.source
    {
      write_source_file( workspace_dir, &source.file_path, &source.data )?;
    }

    // Write or generate the Cargo manifest.
    let manifest_content = program.manifest
      .as_deref()
      .map_or_else( || generate_manifest( opts ), String::from );
    let manifest_path = workspace_dir.join( "Cargo.toml" );
    std::fs::write( &manifest_path, &manifest_content )
      .map_err( | e | format_err!( "failed to write Cargo.toml: {e}" ) )?;

    // Determine the artifact cache directory.
    let target_dir : PathBuf = opts.target_dir
      .as_deref()
      .map_or_else( || workspace_dir.join( "target" ), PathBuf::from );

    let profile = effective_profile( opts );
    let cargo = effective_cargo( opts );

    let mut cmd = Command::new( cargo );
    cmd
      .arg( "run" )
      .arg( "--quiet" )
      .arg( "--manifest-path" ).arg( &manifest_path )
      .arg( "--target-dir" ).arg( &target_dir );

    if profile == "release"
    {
      cmd.arg( "--release" );
    }

    for feature in &opts.features
    {
      cmd.arg( "--features" ).arg( feature );
    }

    for env_var in &opts.env_vars
    {
      if let Some( ( key, value ) ) = env_var.split_once( '=' )
      {
        cmd.env( key, value );
      }
    }

    execute_command( cmd, opts )
  }

  // ── Public API ────────────────────────────────────────────────────────────────

  /// Execute a plan and return captured output.
  ///
  /// Creates a temporary workspace, writes sources, generates a Cargo manifest
  /// when absent, invokes Cargo, captures output, and cleans up. Returns `Err`
  /// only for infrastructure failures (workspace allocation, Cargo not found,
  /// file write errors). Compilation failures and non-zero exit codes are
  /// represented as `exit_status != 0` in the returned `CapturedOutput`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the temporary workspace cannot be created, a source file
  /// cannot be written, or the Cargo binary cannot be invoked.
  pub fn run( plan : Plan ) -> Result< CapturedOutput >
  {
    let opts = plan.run_options.unwrap_or_default();
    let program = plan.program;
    let workspace_dir = create_temp_workspace()?;
    let result = execute_in_workspace( &program, &opts, &workspace_dir );
    if opts.cleanup
    {
      let _ = std::fs::remove_dir_all( &workspace_dir );
    }
    result
  }

  /// Execute inline Rust source code and return captured output.
  ///
  /// Wraps `code` in a generated plan with the source at `src/main.rs` and
  /// delegates to `run`. This is the primary single-expression form for test use.
  ///
  /// # Errors
  ///
  /// Returns `Err` for the same infrastructure reasons as [`run`].
  pub fn run_source( code : &str ) -> Result< CapturedOutput >
  {
    let plan = Plan::former()
      .program()
        .source()
          .file_path( "src/main.rs".to_string() )
          .data( code.to_string() )
          .end()
        .end()
      .form();
    run( plan )
  }

  /// Read a Rust source file and execute it.
  ///
  /// Reads the file at `path` into memory and delegates to `run_source`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the file at `path` cannot be read, or for any
  /// infrastructure error propagated from [`run`].
  pub fn run_file( path : impl AsRef< Path > ) -> Result< CapturedOutput >
  {
    let path = path.as_ref();
    let code = std::fs::read_to_string( path )
      .map_err( | e | format_err!( "failed to read '{}': {e}", path.display() ) )?;
    run_source( &code )
  }

  /// Execute an existing Cargo project directory.
  ///
  /// Invokes `cargo run --quiet` with the project's `Cargo.toml`. No temporary
  /// workspace is created; the project's own layout and target directory are used
  /// unless `opts.target_dir` overrides them. All `RunOptions` fields apply:
  /// `cargo_path`, `build_profile`, `features`, `env_vars`, `capture`,
  /// `timeout_ms`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if no `Cargo.toml` is found in `dir` or if the `cargo`
  /// binary cannot be invoked.
  pub fn run_project( dir : impl AsRef< Path >, opts : &RunOptions ) -> Result< CapturedOutput >
  {
    let dir = dir.as_ref();
    let manifest = dir.join( "Cargo.toml" );
    if !manifest.exists()
    {
      bail!( "no Cargo.toml found in '{}'", dir.display() );
    }

    let profile = effective_profile( opts );
    let cargo = effective_cargo( opts );

    let mut cmd = Command::new( cargo );
    cmd
      .arg( "run" )
      .arg( "--quiet" )
      .arg( "--manifest-path" ).arg( &manifest );

    if profile == "release"
    {
      cmd.arg( "--release" );
    }

    for feature in &opts.features
    {
      cmd.arg( "--features" ).arg( feature );
    }

    for env_var in &opts.env_vars
    {
      if let Some( ( key, value ) ) = env_var.split_once( '=' )
      {
        cmd.env( key, value );
      }
    }

    // Override Cargo's default target dir only when explicitly requested.
    if let Some( ref target_dir ) = opts.target_dir
    {
      cmd.arg( "--target-dir" ).arg( target_dir );
    }

    execute_command( cmd, opts )
  }
}

mod_interface::mod_interface!
{
  exposed use private::{ run, run_source, run_file, run_project };
  prelude use private::{ run, run_source, run_file, run_project };
}
