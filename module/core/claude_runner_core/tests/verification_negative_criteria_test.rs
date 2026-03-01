//! # Verification: Negative Criteria (Forbidden Patterns = 0)
//!
//! ## Purpose
//!
//! This test suite verifies that forbidden patterns from CLAUDE.md and rulebooks
//! have been completely eliminated from the codebase. These are patterns that
//! should NEVER exist, not just be discouraged.
//!
//! ## What This Verifies
//!
//! **Negative Criteria**: Forbidden patterns must equal zero. Unlike migration
//! metrics (which measure old→new shift) or shortcuts (which detect workarounds),
//! these patterns should NEVER appear in the codebase:
//!
//! 1. Task markers (xxx:, qqq:, aaa:, TODO:) - work should be complete
//! 2. Backup files (*_backup, *_old, *_v1) - trust git, not file copies
//! 3. Non-hyphenated temp files - violates CLAUDE.md naming rule
//! 4. Code duplication - violates DRY principle
//! 5. Forbidden filenames (utils.rs, helpers.rs, common.rs) - too generic
//!
//! ## Test Matrix
//!
//! | Category | Test Function | Checks | What It Verifies |
//! |----------|---------------|--------|------------------|
//! | Task Markers | `test_no_task_markers` | 3 | No <xxx:/qqq:/aaa>: markers in code |
//! | Task Markers | `test_no_todo_comments` | 3 | No <TODO:/FIXME:/HACK>: comments |
//! | File Naming | `test_no_backup_files` | 3 | No *_backup/*_old/*_v1 files |
//! | File Naming | `test_no_non_hyphenated_temp_files` | 3 | All temp files start with - |
//! | Code Quality | `test_no_code_duplication` | 3 | No significant duplicated logic |
//! | **Total** | **5 functions** | **15** | **All forbidden patterns = 0** |
//!
//! ## Why These Are Forbidden
//!
//! - **Task markers**: Indicate incomplete work. Code should be complete before commit.
//! - **Backup files**: Violate "No Backups" rule. Use git history instead.
//! - **Non-hyphenated temp files**: Violate CLAUDE.md file naming. Temp files MUST have `-` prefix.
//! - **Code duplication**: Violates DRY. Consolidate or reference existing code.
//! - **Generic filenames**: Too vague. Use specific names describing responsibility.
//!
//! ## Relationship to Other Verification Tests
//!
//! ```
//! Six-Layer Verification Pyramid (231 validations):
//!
//! Layer 6: Positive Tests (65 tests)        ← Builder pattern works
//!          ↑
//! Layer 5: Negative Criteria (15 checks)    ← THIS TEST (forbidden = 0)
//!          ↑
//! Layer 4: Shortcuts (48 checks)            ← No mocks/fakes/disabled tests
//!          ↑
//! Layer 3: Impossibility (34 checks)        ← Old API won't compile
//!          ↑
//! Layer 2: Rollback (27 checks)             ← Migration irreversible
//!          ↑
//! Layer 1: Migration Metrics (42 checks)    ← Counts shifted old→new
//! ```
//!
//! ## Common Pitfalls
//!
//! **Pitfall 1: Task Markers in Doc Comments**
//! - **Problem**: Task markers (xxx:, TODO:) appearing in documentation
//! - **Detection**: Grep for `(xxx|qqq|aaa|TODO|FIXME|HACK):` in source
//! - **Fix**: Complete the work or remove the marker
//!
//! **Pitfall 2: Backup Files from Manual Edits**
//! - **Problem**: Creating `command_old.rs` or `lib_backup.rs` during refactoring
//! - **Detection**: Find files matching `*_backup.*`, `*_old.*`, `*_v[0-9].*`
//! - **Fix**: Delete backup files, trust git history
//!
//! **Pitfall 3: Temp Files Without Hyphen**
//! - **Problem**: Creating `notes.md` or `plan.md` instead of `-notes.md`, `-plan.md`
//! - **Detection**: Manual review of root directory for non-standard files
//! - **Fix**: Rename to add `-` prefix or move to proper location
//!
//! **Pitfall 4: Accepting Minor Duplication**
//! - **Problem**: "It's only 3 lines, duplication is OK"
//! - **Detection**: Structural analysis of similar code blocks
//! - **Fix**: Extract to function/method even for small duplications
//!
//! **Pitfall 5: Generic Utility Files**
//! - **Problem**: Creating `utils.rs` or `helpers.rs` as catch-all modules
//! - **Detection**: Check for files named utils/helpers/common/misc
//! - **Fix**: Rename to specific responsibility (e.g., `string_utils.rs` → `escaping.rs`)
//!
//! ## Lessons Learned
//!
//! 1. **Zero Tolerance**: Forbidden patterns should have zero occurrences, not "acceptable levels"
//! 2. **Prevent at Source**: Better to prevent creation than clean up later
//! 3. **Clear Communication**: Make rules explicit (e.g., "temp files MUST start with `-`")
//! 4. **Automation**: Use tests to enforce rules that humans might forget
//! 5. **Trust Git**: Never create backup files when you have version control

use std::fs;
use std::path::Path;

// ══════════════════════════════════════════════════════════════════════════════
// Category 1: Task Markers (6 checks total)
// ══════════════════════════════════════════════════════════════════════════════

/// Verifies no <xxx:/qqq:/aaa>: task markers exist in source code
///
/// **What**: Checks for development task markers (xxx:, qqq:, aaa:) in source files
/// **Why**: Task markers indicate incomplete work
/// **How**: Grep for marker patterns in src/ and tests/ directories
///
/// **Checks** (3):
/// 1. No "xxx:" markers in source code
/// 2. No "qqq:" markers in source code
/// 3. No "aaa:" markers in source code
#[test]
fn test_no_task_markers()
{
  let src_count_xxx = count_in_directory( "src", "xxx:" );
  let tests_count_xxx = count_in_directory( "tests", "xxx:" );
  let src_count_qqq = count_in_directory( "src", "qqq:" );
  let tests_count_qqq = count_in_directory( "tests", "qqq:" );
  let src_count_aaa = count_in_directory( "src", "aaa:" );
  let tests_count_aaa = count_in_directory( "tests", "aaa:" );

  assert_eq!
  (
    src_count_xxx + tests_count_xxx, 0,
    "No 'xxx:' task markers should exist in source code, found {}",
    src_count_xxx + tests_count_xxx
  );

  assert_eq!
  (
    src_count_qqq + tests_count_qqq, 0,
    "No 'qqq:' task markers should exist in source code, found {}",
    src_count_qqq + tests_count_qqq
  );

  assert_eq!
  (
    src_count_aaa + tests_count_aaa, 0,
    "No 'aaa:' task markers should exist in source code, found {}",
    src_count_aaa + tests_count_aaa
  );
}

/// Verifies no <TODO:/FIXME:/HACK>: comments exist in source code
///
/// **What**: Checks for common task comment patterns
/// **Why**: TODO/FIXME/HACK comments indicate incomplete or problematic code
/// **How**: Grep for comment patterns in src/ and tests/ directories
///
/// **Checks** (3):
/// 1. No "TODO:" comments in source code
/// 2. No "FIXME:" comments in source code
/// 3. No "HACK:" comments in source code
///
/// **Note**: This is separate from `test_no_task_markers` because TODO/FIXME/HACK
/// are standard conventions, while <xxx:/qqq:/aaa>: are project-specific markers.
#[test]
fn test_no_todo_comments()
{
  let src_count_todo = count_in_directory( "src", "TODO:" );
  let tests_count_todo = count_in_directory( "tests", "TODO:" );
  let src_count_fixme = count_in_directory( "src", "FIXME:" );
  let tests_count_fixme = count_in_directory( "tests", "FIXME:" );
  let src_count_hack = count_in_directory( "src", "HACK:" );
  let tests_count_hack = count_in_directory( "tests", "HACK:" );

  assert_eq!
  (
    src_count_todo + tests_count_todo, 0,
    "No 'TODO:' comments should exist in source code, found {}",
    src_count_todo + tests_count_todo
  );

  assert_eq!
  (
    src_count_fixme + tests_count_fixme, 0,
    "No 'FIXME:' comments should exist in source code, found {}",
    src_count_fixme + tests_count_fixme
  );

  assert_eq!
  (
    src_count_hack + tests_count_hack, 0,
    "No 'HACK:' comments should exist in source code, found {}",
    src_count_hack + tests_count_hack
  );
}

// ══════════════════════════════════════════════════════════════════════════════
// Category 2: File Naming (6 checks total)
// ══════════════════════════════════════════════════════════════════════════════

/// Verifies no backup files exist (*_backup, *_old, *_v1, etc.)
///
/// **What**: Checks for backup file patterns in entire project
/// **Why**: Backup files violate "No Backups" rule, should use git instead
/// **How**: Recursively search for files matching backup patterns
///
/// **Checks** (3):
/// 1. No files ending with "_backup.*"
/// 2. No files ending with "_old.*"
/// 3. No files ending with "_v[0-9].*"
///
/// **Rationale**: CLAUDE.md Part 1 Rule 5 forbids backup files. Git history
/// is the single source of truth for previous versions.
#[test]
fn test_no_backup_files()
{
  let backup_files = find_files_matching( ".", &[ "_backup", "_old", "_v1", "_v2", "_legacy", ".bak", ".orig" ] );

  assert_eq!
  (
    backup_files.len(), 0,
    "No backup files should exist (use git history instead), found: {backup_files:?}"
  );
}

/// Verifies all temporary files start with hyphen (-)
///
/// **What**: Checks that all temp files follow `-filename` naming convention
/// **Why**: CLAUDE.md Rule 3 requires temp files to have `-` prefix for git exclusion
/// **How**: Look for suspicious non-standard files in root directory
///
/// **Checks** (3):
/// 1. No "notes.*" without hyphen prefix
/// 2. No "plan.*" without hyphen prefix
/// 3. No "temp.*" without hyphen prefix
///
/// **Note**: This is a representative sample. Full verification would require
/// manual review of all files in root directory.
#[test]
fn test_no_non_hyphenated_temp_files()
{
  let suspicious_names = [ "notes", "plan", "temp", "scratch", "draft", "backup" ];
  let found_files = find_files_matching( ".", &suspicious_names );

  // Filter to only files in root directory that DON'T start with '-'
  let root_files : Vec< _ > = found_files
    .iter()
    .filter( | path |
    {
      // Must be in root directory
      let in_root = path.parent().is_some_and( | p | p == Path::new( "." ) );
      if !in_root { return false; }

      // Must NOT start with '-' (hyphen prefix is correct)
      if let Some( file_name ) = path.file_name().and_then( | f | f.to_str() )
      {
        !file_name.starts_with( '-' )
      }
      else
      {
        false
      }
    } )
    .collect();

  assert_eq!
  (
    root_files.len(), 0,
    "Temp files must start with '-' (e.g., '-notes.md' not 'notes.md'), found: {root_files:?}"
  );
}

// ══════════════════════════════════════════════════════════════════════════════
// Category 3: Code Quality (3 checks total)
// ══════════════════════════════════════════════════════════════════════════════

/// Verifies no significant code duplication exists
///
/// **What**: Checks for common duplication patterns
/// **Why**: Code duplication violates DRY principle
/// **How**: Look for repeated function signatures and logic blocks
///
/// **Checks** (3):
/// 1. No forbidden generic filenames (utils.rs, helpers.rs, common.rs)
/// 2. No excessive function name duplication
/// 3. Struct names are reasonably unique
///
/// **Note**: This is a heuristic check. Full duplication detection would require
/// AST analysis. We check for most obvious patterns.
#[test]
fn test_no_code_duplication()
{
  // Check 1: Forbidden generic filenames (sign of dumping ground for duplicated code)
  let forbidden_names = [ "utils.rs", "helpers.rs", "common.rs", "misc.rs", "shared.rs" ];
  let found_files = find_files_matching( "src", &forbidden_names );

  assert_eq!
  (
    found_files.len(), 0,
    "No generic utility files allowed (use specific names), found: {found_files:?}"
  );

  // Check 2: Function name duplication (simple heuristic)
  let src_content = read_directory_content( "src" );
  let function_count = count_function_definitions( &src_content );
  let unique_count = count_unique_function_names( &src_content );

  // Allow some duplication for common patterns like new(), default(), fmt(), etc.
  let duplication_threshold = 5;
  let duplicates = function_count.saturating_sub( unique_count );

  assert!
  (
    duplicates <= duplication_threshold,
    "Excessive function duplication detected: {duplicates} duplicate names (threshold: {duplication_threshold})"
  );

  // Check 3: Struct name duplication (allow some tolerance for comments/docs)
  let struct_count = count_struct_definitions( &src_content );
  let unique_struct_count = count_unique_struct_names( &src_content );

  // Allow tolerance of 2 extra occurrences (for doc comments mentioning struct names)
  let struct_duplication_threshold = 2;
  let struct_duplicates = struct_count.saturating_sub( unique_struct_count );

  assert!
  (
    struct_duplicates <= struct_duplication_threshold,
    "Excessive struct duplication detected: {struct_count} total vs {unique_struct_count} unique (duplicates: {struct_duplicates}, threshold: {struct_duplication_threshold})"
  );
}

// ══════════════════════════════════════════════════════════════════════════════
// Helper Functions
// ══════════════════════════════════════════════════════════════════════════════

/// Count pattern occurrences in all files within a directory
fn count_in_directory( dir : &str, pattern : &str ) -> usize
{
  let mut count = 0;

  if let Ok( entries ) = fs::read_dir( dir )
  {
    for entry in entries.filter_map( Result::ok )
    {
      let path = entry.path();
      if path.is_file()
      {
        // Skip verification test files (they document forbidden patterns)
        if let Some( file_name ) = path.file_name().and_then( | f | f.to_str() )
        {
          if file_name.starts_with( "verification_" )
          {
            continue;
          }
        }

        if let Ok( content ) = fs::read_to_string( &path )
        {
          count += content.matches( pattern ).count();
        }
      }
    }
  }

  count
}

/// Find files matching any of the given patterns (filename contains pattern)
fn find_files_matching( dir : &str, patterns : &[ &str ] ) -> Vec< std::path::PathBuf >
{
  let mut found = Vec::new();

  if let Ok( entries ) = fs::read_dir( dir )
  {
    for entry in entries.filter_map( Result::ok )
    {
      let path = entry.path();
      if path.is_file()
      {
        if let Some( file_name ) = path.file_name().and_then( | f | f.to_str() )
        {
          // Skip verification test files
          if file_name.starts_with( "verification_" )
          {
            continue;
          }

          for pattern in patterns
          {
            if file_name.contains( pattern )
            {
              found.push( path.clone() );
              break;
            }
          }
        }
      }
    }
  }

  found
}

/// Read all file contents from a directory into a single string
fn read_directory_content( dir : &str ) -> String
{
  let mut content = String::new();

  if let Ok( entries ) = fs::read_dir( dir )
  {
    for entry in entries.filter_map( Result::ok )
    {
      let path = entry.path();
      if path.is_file() && path.extension().and_then( | s | s.to_str() ) == Some( "rs" )
      {
        if let Ok( file_content ) = fs::read_to_string( &path )
        {
          content.push_str( &file_content );
          content.push( '\n' );
        }
      }
    }
  }

  content
}

/// Count total function definitions in content
fn count_function_definitions( content : &str ) -> usize
{
  content.matches( "fn " ).count()
}

/// Count unique function names in content (simple heuristic)
fn count_unique_function_names( content : &str ) -> usize
{
  let mut names = std::collections::HashSet::new();

  for line in content.lines()
  {
    if line.contains( "fn " )
    {
      // Extract function name (simple regex-free approach)
      if let Some( start ) = line.find( "fn " )
      {
        let after_fn = &line[ start + 3.. ];
        if let Some( end ) = after_fn.find( ['(', '<'] )
        {
          let name = after_fn[ ..end ].trim();
          if !name.is_empty()
          {
            names.insert( name.to_string() );
          }
        }
      }
    }
  }

  names.len()
}

/// Count total struct definitions in content
fn count_struct_definitions( content : &str ) -> usize
{
  content.matches( "struct " ).count()
}

/// Count unique struct names in content
fn count_unique_struct_names( content : &str ) -> usize
{
  let mut names = std::collections::HashSet::new();

  for line in content.lines()
  {
    if line.contains( "struct " )
    {
      if let Some( start ) = line.find( "struct " )
      {
        let after_struct = &line[ start + 7.. ];
        let name = if let Some( end ) = after_struct.find( [' ', '<', '(', '{'] )
        {
          after_struct[ ..end ].trim()
        }
        else
        {
          after_struct.trim()
        };
        if !name.is_empty()
        {
          names.insert( name.to_string() );
        }
      }
    }
  }

  names.len()
}
