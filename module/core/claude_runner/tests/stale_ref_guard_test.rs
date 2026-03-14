//! Guard tests: verify no stale `claude_runner_plugin` references remain in permanent files.
//!
//! ## Context
//!
//! `claude_runner_plugin` was removed from the willbe/dev workspace on 2026-03-09.
//! `dream` now links `dream_agent` directly as a compile-time dependency, aggregating
//! `claude.commands.yaml` via `build.rs` for PHF command registration.
//!
//! ## Tests
//!
//! - `no_plugin_ref_in_spec`: `spec.md` must have zero `claude_runner_plugin` matches
//!   (Note lines about proset preservation are exempt — those are intentional historical context)
//! - `no_plugin_ref_in_lib_rs`: `src/lib.rs` must have zero `claude_runner_plugin` matches
//! - `no_plugin_ref_in_readme`: `readme.md` must have zero `claude_runner_plugin` matches
//! - `no_src_readme_exists`: `src/readme.md` must not exist (violates src/ exception rule)
//!
//! ## Pitfalls
//!
//! **Proset ≠ deleted.** `claude_runner_plugin` still exists at `willbe/proset/module/`
//! as a reference implementation. It was only *removed from willbe/dev workspace*. Guard
//! tests that check for the plugin name must exempt deliberate Note lines documenting this
//! preservation — those are accurate history, not stale refs. Exempt pattern:
//! `!line.contains("removed from willbe/dev workspace")`.
//!
//! **MSRV 1.61.0: no `.is_some_and()`.** That method stabilised in 1.70.0. With
//! `-D warnings`, the `incompatible_msrv` clippy lint fires. Always use
//! `.map_or( false, |x| ... )` in this crate.
//!
//! **`display()` is not `Copy`.** `Path::display()` returns a `Display` wrapper that
//! borrows the path. Because it is not `Copy`, rustfmt/clippy cannot inline it into
//! format-string variable slots. The `uninlined_format_args` lint will fire on the
//! *other* arguments in the same call. Keep the explicit `{}` placeholder and add a
//! `// display() not Copy, can't inline` comment to explain why the lint is suppressed.
//!
//! **`doc_markdown` lint.** Bare crate/binary names in doc comments (e.g. `claude_runner`)
//! must be wrapped in backticks or the `doc_markdown` clippy lint fires with `-D warnings`.

use std::fs;
use std::path::Path;

fn collect_violations( file_path : &Path, pattern : &str ) -> Vec< String >
{
  let content = fs::read_to_string( file_path )
    .unwrap_or_else( |e| panic!( "Cannot read {}: {e}", file_path.display() ) );
  content
    .lines()
    .enumerate()
    .filter( |( _, line )| line.contains( pattern ) )
    .map( |( i, line )| format!( "  {}:{}: {}", file_path.display(), i + 1, line.trim() ) ) // display() not Copy, can't inline
    .collect()
}

#[test]
fn no_plugin_ref_in_spec()
{
  // Note lines about proset preservation are exempt: they document historical removal, not stale refs.
  // Exempt pattern: lines containing "> **Note:**" followed by "removed from willbe/dev workspace"
  let manifest = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let path = manifest.join( "spec.md" );
  let violations : Vec< String > = collect_violations( &path, "claude_runner_plugin" )
    .into_iter()
    .filter( |line| !line.contains( "removed from willbe/dev workspace" ) )
    .collect();
  assert!(
    violations.is_empty(),
    "Stale claude_runner_plugin references in spec.md:\n{}",
    violations.join( "\n" )
  );
}

#[test]
fn no_plugin_ref_in_lib_rs()
{
  let manifest = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let path = manifest.join( "src/lib.rs" );
  let violations = collect_violations( &path, "claude_runner_plugin" );
  assert!(
    violations.is_empty(),
    "Stale claude_runner_plugin references in src/lib.rs:\n{}",
    violations.join( "\n" )
  );
}

#[test]
fn no_plugin_ref_in_readme()
{
  let manifest = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let path = manifest.join( "readme.md" );
  let violations = collect_violations( &path, "claude_runner_plugin" );
  assert!(
    violations.is_empty(),
    "Stale claude_runner_plugin references in readme.md:\n{}",
    violations.join( "\n" )
  );
}

#[test]
fn no_src_readme_exists()
{
  let manifest = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let illegal = manifest.join( "src" ).join( "readme.md" );
  assert!(
    !illegal.exists(),
    "src/readme.md must not exist (violates src/ exception rule): {}",
    illegal.display()
  );
}
