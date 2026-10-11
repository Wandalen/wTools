use std::path::{ Path, PathBuf };

use crate::hash;
use crate::scan::QuerySite;

/// The sites whose `query-<hash>.json` file (per
/// `docs/invariant/001_query_dispatch_invariant.md`, Hash Algorithm Parity)
/// exists in none of the `lookup` directories.
///
/// Cache entries with no matching call site are never flagged -- a shared
/// workspace `.sqlx/` legitimately holds sibling crates' entries.
#[ must_use ]
pub fn stale_sites< 'a >( sites : &'a [ QuerySite ], lookup : &[ PathBuf ] ) -> Vec< &'a QuerySite >
{
  // Fix(workspace_entry_mre_counts_as_fresh_beside_a_local_cache_dir): search every lookup
  // directory for each query, as sqlx does.
  // Root cause: only the first existing cache directory was checked, so an entry living in
  // the workspace-root .sqlx was reported stale whenever a crate-local .sqlx also existed.
  // Pitfall: sqlx resolves each query file independently -- one directory is not "the" cache.
  sites
  .iter()
  .filter( | site |
  {
    let filename = hash::cache_filename( &site.sql );
    !lookup.iter().any( | dir | dir.join( &filename ).is_file() )
  } )
  .collect()
}

/// One warning naming a stale call site.
#[ must_use ]
pub fn site_warning( site : &QuerySite ) -> String
{
  format!(
    "cargo:warning=sqlx_query: {}:{} -- no .sqlx entry matches this query's SQL",
    site.file.display(),
    site.line,
  )
}

/// The warning for a crate that has no offline cache at all.
#[ must_use ]
pub fn no_cache_dir_warning() -> String
{
  "cargo:warning=sqlx_query: no .sqlx directory found, so SQLX_OFFLINE is not forced; run `cargo sqlx prepare` once to create one".to_string()
}

/// The explanation that follows the site warnings when `SQLX_OFFLINE=true` is
/// forced over stale entries -- sqlx's own error then advises unsetting a
/// variable the user never set, so this names who set it and the two ways out.
/// `database_url_var` is the variable sqlx reads the database URL from;
/// `missing_offline_dir` is an explicit `SQLX_OFFLINE_DIR` that is not a
/// directory, which a soft-warn rebuild needs created first.
#[ must_use ]
pub fn forced_offline_summary( stale : usize, database_url_var : &str, missing_offline_dir : Option< &Path > ) -> Vec< String >
{
  let rebuild = match missing_offline_dir
  {
    None => format!( "rebuild with SQLX_QUERY_SOFT_WARN=1 to compile against {database_url_var} and refresh .sqlx in place" ),
    Some( dir ) => format!(
      "create {}, which SQLX_OFFLINE_DIR names, and rebuild with SQLX_QUERY_SOFT_WARN=1 to compile against {database_url_var} and write into it",
      dir.display(),
    ),
  };
  // Fix(forced_offline_summary_mre_names_the_way_out): explain the forced SQLX_OFFLINE.
  // Root cause: only per-site warnings were printed, while sqlx's error says to "unset
  // `SQLX_OFFLINE`" -- a variable the user never set and cannot unset.
  // Pitfall: the opt-out (SQLX_QUERY_SOFT_WARN=1) is undiscoverable unless the failing build
  // itself names it.
  vec!
  [
    format!(
      "cargo:warning=sqlx_query: forcing SQLX_OFFLINE=true, so sqlx rejects {}, exactly as an offline build (CI, a teammate without a database) would",
      wherever_compiled( stale ),
    ),
    format!( "cargo:warning=sqlx_query: fix with `cargo sqlx prepare` (add `-- --all-targets --all-features` when queries live in tests or behind features), or {rebuild}" ),
  ]
}

/// The explanation that follows the site warnings when `SQLX_QUERY_SOFT_WARN=1`
/// leaves the build live. Like `wherever_compiled`, it promises entries only
/// where the queries are compiled.
#[ must_use ]
pub fn soft_warn_summary( stale : usize, cache_dir : &Path, database_url_var : &str ) -> String
{
  let ( queries, entries, pronoun ) = if stale == 1
  {
    ( "the query above is".to_string(), "its missing entry", "it" )
  }
  else
  {
    ( format!( "the {stale} queries above are" ), "their missing entries", "them" )
  };
  format!(
    "cargo:warning=sqlx_query: SQLX_QUERY_SOFT_WARN=1, so SQLX_OFFLINE is not forced: wherever {queries} compiled against {database_url_var}, sqlx writes {entries} into {} (commit {pronoun}); without {database_url_var}, run `cargo sqlx prepare`",
    cache_dir.display(),
  )
}

/// The explanation that follows the site warnings when `SQLX_QUERY_SOFT_WARN=1`
/// leaves the build live but `SQLX_OFFLINE_DIR` names `dir`, which is not a
/// directory: sqlx never creates it, and fails every query it compiles live
/// instead of writing an entry.
#[ must_use ]
pub fn soft_warn_without_offline_dir_summary( dir : &Path, database_url_var : &str ) -> String
{
  format!(
    "cargo:warning=sqlx_query: SQLX_QUERY_SOFT_WARN=1, so SQLX_OFFLINE is not forced, but SQLX_OFFLINE_DIR names {}, which is not a directory: sqlx fails every query it compiles against {database_url_var} until you create it",
    dir.display(),
  )
}

/// The explanation that follows the site warnings when the environment or a
/// `.env` file already sets `SQLX_OFFLINE=true`.
#[ must_use ]
pub fn offline_already_set_summary( stale : usize ) -> String
{
  format!(
    "cargo:warning=sqlx_query: SQLX_OFFLINE is already true in the environment or a .env file, so sqlx rejects {}; run `cargo sqlx prepare`",
    wherever_compiled( stale ),
  )
}

/// "the query above wherever it is compiled", or the same for `count` queries.
fn wherever_compiled( count : usize ) -> String
{
  // Fix(test_only_query_mre_summaries_hold_for_builds_that_skip_it): speak of wherever the
  // queries are compiled, never of this build.
  // Root cause: the summaries said "sqlx rejects the query above" and "this build writes the
  // missing entry", but the scan also covers tests, examples and #[cfg]-gated code -- so
  // `cargo build` with a stale query in tests/ claimed a rejection and then finished, and
  // soft-warn promised an entry it never wrote.
  // Pitfall: Cargo runs a build script per package, not per target, so its output cannot depend
  // on which targets a command compiles.
  if count == 1
  {
    "the query above wherever it is compiled".to_string()
  }
  else
  {
    format!( "the {count} queries above wherever they are compiled" )
  }
}
