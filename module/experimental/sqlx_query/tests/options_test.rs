//! How `sqlx_query::Options::resolve` reads its settings the way sqlx's macros
//! read them: from the environment, from `.env` files, and from the crate's
//! `sqlx.toml` -- `SQLX_OFFLINE` exactly as sqlx parses it, the database URL as
//! sqlx counts it.
//!
//! `Options::from_env` is `resolve` over the real process environment; it is
//! exercised end to end by the `offline_*_test.rs` builds, never here, since
//! changing this process's environment would race with parallel tests.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `empty_database_url_mre_counts_as_unset` | sqlx ignores an empty URL (MRE) | `DATABASE_URL=` | `database_url_set == false` | ✅ |
//! | `database_url_with_a_value_counts_as_set` | Happy path | `DATABASE_URL=sqlite://db.sqlite` | `true` | ✅ |
//! | `offline_true_spellings_parse_as_offline` | sqlx's `is_truthy_bool` | `true`, `TRUE`, `True`, `1` | `Some( true )` | ✅ |
//! | `offline_other_values_parse_as_online` | Anything else is an explicit `false` | `false`, `0`, empty, `yes`, `on` | `Some( false )` | ✅ |
//! | `offline_unset_parses_as_none` | Variable absent | — | `None` | ✅ |
//! | `soft_warn_needs_exactly_one` | Documented value only | `1` vs `true`, `yes`, empty | `true` only for `1` | ✅ |
//! | `auto_regen_needs_exactly_one` | Documented value only | `1` vs `true`, `yes`, empty | `true` only for `1` | ✅ |
//! | `offline_dir_is_kept_as_given` | Resolution happens later, against the workspace root | `SQLX_OFFLINE_DIR=cache` | `Some( "cache" )` | ✅ |
//! | `dotenv_mre_supplies_what_sqlx_reads_from_it` | Settings kept in `.env` (MRE) | `.env` with all three variables, empty environment | all three take effect | ✅ |
//! | `database_url_var_mre_follows_sqlx_toml` | `sqlx.toml` renames the variable (MRE) | `database-url-var = "APP_DATABASE_URL"` | only that variable counts | ✅ |
//! | `environment_beats_dotenv` | sqlx's precedence | both set, different values | the environment's | ✅ |
//! | `empty_database_url_in_environment_masks_dotenv` | Same, empty value | `DATABASE_URL=` vs `.env` URL | `database_url_set == false` | ✅ |
//! | `outermost_dotenv_wins` | Two `.env` files up the tree | crate and parent `.env` | parent's value; both recorded, crate's first | ✅ |
//! | `dotenv_never_sets_the_crates_own_switches` | Switches sqlx never reads | `SQLX_QUERY_*=1` in `.env` | both off | ✅ |
//! | `malformed_dotenv_is_read_up_to_the_error` | dotenvy parse error | bad second line | first line counts; file recorded | ✅ |
//! | `dotenv_directory_is_not_a_settings_file` | A virtualenv named `.env` | `.env/` | nothing recorded | ✅ |
//! | `database_url_var_is_read_from_dotenv_too` | Renamed variable in `.env` | `APP_DATABASE_URL` in `.env` | set; both files recorded | ✅ |
//! | `database_url_var_accepts_every_toml_form` | TOML syntax | dotted key, inline table, quoted key | same variable | ✅ |
//! | `database_url_var_defaults_to_database_url` | No usable key | other table, broken TOML, non-string | `DATABASE_URL`; file recorded | ✅ |
//! | `ancestor_sqlx_toml_is_ignored` | sqlx reads the crate's own only | `sqlx.toml` in the parent | `DATABASE_URL`; nothing recorded | ✅ |

#![ cfg( feature = "enabled" ) ]

mod support;

use std::path::{ Path, PathBuf };

use support::{ options_for, PackageDir };

/// Options for a crate at a path that does not exist -- so there is no
/// `sqlx.toml` or `.env` file to read -- with `env` as the entire environment.
fn resolve( env : &[ ( &str, &str ) ] ) -> sqlx_query::Options
{
  options_for( Path::new( "/nonexistent/crate" ), env )
}

/// An empty `DATABASE_URL` is no database: sqlx skips it and stays offline.
///
/// ## Root Cause
///
/// `Options::from_env` in `src/options.rs` set `live_db_reachable` from
/// `DATABASE_URL`'s presence alone, so `DATABASE_URL=` -- a common way to
/// "unset" a variable inherited from a shell profile or CI template -- counted
/// as a live database, and `auto_regen` spent a nested `cargo check` that
/// could never connect.
///
/// ## Why Not Caught
///
/// Tests set `DATABASE_URL` to a real URL or not at all.
///
/// ## Fix Applied
///
/// `Options::resolve` sets `database_url_set` only for a non-empty value,
/// matching sqlx's own condition for compiling live.
///
/// ## Prevention
///
/// Mirror sqlx's own reading of each variable instead of testing presence.
///
/// ## Pitfall
///
/// An empty `SQLX_OFFLINE=` is the opposite case: sqlx reads it as an
/// explicit `false` (`offline_other_values_parse_as_online`).
// test_kind: mre
#[ test ]
fn empty_database_url_mre_counts_as_unset()
{
  assert!( !resolve( &[ ( "DATABASE_URL", "" ) ] ).database_url_set );
}

/// Any non-empty `DATABASE_URL` is one sqlx would try to compile against.
#[ test ]
fn database_url_with_a_value_counts_as_set()
{
  assert!( resolve( &[ ( "DATABASE_URL", "sqlite://db.sqlite" ) ] ).database_url_set );
}

/// sqlx counts `true` in any letter case, and `1`, as offline.
#[ test ]
fn offline_true_spellings_parse_as_offline()
{
  for value in [ "true", "TRUE", "True", "1" ]
  {
    assert_eq!( resolve( &[ ( "SQLX_OFFLINE", value ) ] ).offline, Some( true ), "SQLX_OFFLINE={value:?}" );
  }
}

/// Every other value -- even `yes`, `on`, or the empty string -- is an
/// explicit `false` to sqlx, which makes `sqlx_query` stand aside.
#[ test ]
fn offline_other_values_parse_as_online()
{
  for value in [ "false", "0", "", "yes", "on" ]
  {
    assert_eq!( resolve( &[ ( "SQLX_OFFLINE", value ) ] ).offline, Some( false ), "SQLX_OFFLINE={value:?}" );
  }
}

/// Without `SQLX_OFFLINE` there is no explicit choice to respect.
#[ test ]
fn offline_unset_parses_as_none()
{
  assert_eq!( resolve( &[] ).offline, None );
}

/// `SQLX_QUERY_SOFT_WARN` is on only for its documented value, `1`.
#[ test ]
fn soft_warn_needs_exactly_one()
{
  assert!( resolve( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] ).soft_warn );
  for value in [ "true", "yes", "" ]
  {
    assert!( !resolve( &[ ( "SQLX_QUERY_SOFT_WARN", value ) ] ).soft_warn, "SQLX_QUERY_SOFT_WARN={value:?}" );
  }
}

/// `SQLX_QUERY_AUTO_REGEN` is on only for its documented value, `1`.
#[ test ]
fn auto_regen_needs_exactly_one()
{
  assert!( resolve( &[ ( "SQLX_QUERY_AUTO_REGEN", "1" ) ] ).auto_regen );
  for value in [ "true", "yes", "" ]
  {
    assert!( !resolve( &[ ( "SQLX_QUERY_AUTO_REGEN", value ) ] ).auto_regen, "SQLX_QUERY_AUTO_REGEN={value:?}" );
  }
}

/// `SQLX_OFFLINE_DIR` is stored as given; a relative value is resolved later,
/// against the workspace root (`cache_dir_test.rs`).
#[ test ]
fn offline_dir_is_kept_as_given()
{
  assert_eq!( resolve( &[ ( "SQLX_OFFLINE_DIR", "cache" ) ] ).cache_dir_override, Some( PathBuf::from( "cache" ) ) );
}

/// The `DATABASE_URL`, `SQLX_OFFLINE` and `SQLX_OFFLINE_DIR` of a `.env` file
/// count exactly as sqlx's macros count them.
///
/// ## Root Cause
///
/// `Options::resolve` in `src/options.rs` read only the process environment,
/// while sqlx's macros also load every `.env` file from the crate directory up.
/// With sqlx's standard setup -- settings kept in `.env` -- `auto_regen` was
/// skipped for want of a `DATABASE_URL`, a `SQLX_OFFLINE=false` there was
/// overridden by the forced `true`, and soft-warn builds exported
/// `SQLX_OFFLINE_DIR` over the directory `.env` named, so sqlx wrote the
/// entries somewhere else.
///
/// ## Why Not Caught
///
/// Every test passed its settings in the environment; manual scenarios S3b,
/// S3c and S3d ran them from `.env`, and were documented as known gaps instead
/// of fixed.
///
/// ## Fix Applied
///
/// `Options::resolve` reads every `.env` file from the crate directory up to
/// the filesystem root with `dotenvy`, the parser sqlx uses, and records each
/// one in `Options::config_files`, which `build_script_directives` declares to
/// Cargo.
///
/// ## Prevention
///
/// Read every input sqlx reads, from the same places and in the same order,
/// instead of documenting the difference.
///
/// ## Pitfall
///
/// sqlx's precedence runs against intuition: a process variable, even an empty
/// one, beats every file, and among the files the one nearest the filesystem
/// root wins.
// test_kind: mre
#[ test ]
fn dotenv_mre_supplies_what_sqlx_reads_from_it()
{
  let package = PackageDir::empty();
  package.write( ".env", "DATABASE_URL=sqlite://db.sqlite\nSQLX_OFFLINE=true\nSQLX_OFFLINE_DIR=cache\n" );

  let options = options_for( package.root(), &[] );
  assert!( options.database_url_set );
  assert_eq!( options.offline, Some( true ) );
  assert_eq!( options.cache_dir_override, Some( PathBuf::from( "cache" ) ) );
}

/// A crate's `sqlx.toml` can name the variable sqlx reads the database URL
/// from; that variable, not `DATABASE_URL`, is the one that counts.
///
/// ## Root Cause
///
/// `Options::resolve` in `src/options.rs` hard-coded `DATABASE_URL`, while
/// sqlx 0.9 reads the variable named by `[common] database-url-var` in the
/// crate's `sqlx.toml`. With it set, `auto_regen` was skipped although sqlx had
/// a database, or attempted against `DATABASE_URL` although sqlx had none; the
/// advice named a variable sqlx ignores; and changing the real one never re-ran
/// the check.
///
/// ## Why Not Caught
///
/// No test or manual scenario used a `sqlx.toml`; the gap was documented as
/// affecting the `auto_regen` gate alone, missing the rerun trigger.
///
/// ## Fix Applied
///
/// `Options::resolve` reads `database-url-var` from the crate's `sqlx.toml`
/// with a TOML parser into `Options::database_url_var`, and looks that variable
/// up in the environment and in `.env` files. `build_script_directives`
/// declares it, and `sqlx.toml` itself, to Cargo, and the messages name it.
///
/// ## Prevention
///
/// Treat every file sqlx reads its configuration from as an input of the
/// check.
///
/// ## Pitfall
///
/// sqlx reads only the crate's own `sqlx.toml` -- never one in an ancestor
/// directory or at the workspace root.
// test_kind: mre
#[ test ]
fn database_url_var_mre_follows_sqlx_toml()
{
  let package = PackageDir::empty();
  package.write( "sqlx.toml", "[common]\ndatabase-url-var = \"APP_DATABASE_URL\"\n" );

  assert!( options_for( package.root(), &[ ( "APP_DATABASE_URL", "sqlite://db.sqlite" ) ] ).database_url_set );
  assert!( !options_for( package.root(), &[ ( "DATABASE_URL", "sqlite://db.sqlite" ) ] ).database_url_set );
}

/// A variable in the environment beats the same variable in any `.env` file.
#[ test ]
fn environment_beats_dotenv()
{
  let package = PackageDir::empty();
  package.write( ".env", "SQLX_OFFLINE=true\nSQLX_OFFLINE_DIR=from_file\n" );

  let options = options_for( package.root(), &[ ( "SQLX_OFFLINE", "false" ), ( "SQLX_OFFLINE_DIR", "from_env" ) ] );
  assert_eq!( options.offline, Some( false ) );
  assert_eq!( options.cache_dir_override, Some( PathBuf::from( "from_env" ) ) );
}

/// Even an empty `DATABASE_URL` in the environment beats a `.env` file's
/// value -- and then sqlx has no database.
#[ test ]
fn empty_database_url_in_environment_masks_dotenv()
{
  let package = PackageDir::empty();
  package.write( ".env", "DATABASE_URL=sqlite://db.sqlite\n" );

  assert!( !options_for( package.root(), &[ ( "DATABASE_URL", "" ) ] ).database_url_set );
}

/// Every `.env` from the crate directory up is read, in that order, and the
/// last value read wins: the file nearest the filesystem root, as in sqlx.
#[ test ]
fn outermost_dotenv_wins()
{
  let tree = PackageDir::empty();
  let outer = tree.write( ".env", "SQLX_OFFLINE_DIR=outer\n" );
  let inner = tree.write( "member/.env", "SQLX_OFFLINE_DIR=inner\nDATABASE_URL=sqlite://db.sqlite\n" );

  let options = options_for( &tree.root().join( "member" ), &[] );
  assert_eq!( options.cache_dir_override, Some( PathBuf::from( "outer" ) ) );
  assert!( options.database_url_set, "a value only the inner file sets still counts" );
  assert_eq!( options.config_files, [ inner, outer ] );
}

/// The crate's own switches come from the environment alone: sqlx never reads
/// them from `.env`, and the nested regeneration check removes
/// `SQLX_QUERY_AUTO_REGEN` from its environment to stop recursing.
#[ test ]
fn dotenv_never_sets_the_crates_own_switches()
{
  let package = PackageDir::empty();
  package.write( ".env", "SQLX_QUERY_SOFT_WARN=1\nSQLX_QUERY_AUTO_REGEN=1\n" );

  let options = options_for( package.root(), &[] );
  assert!( !options.soft_warn );
  assert!( !options.auto_regen );
}

/// A `.env` file dotenvy cannot parse fails every sqlx query macro anyway; the
/// lines before the error still count, and the file is recorded, so fixing it
/// re-runs the check.
#[ test ]
fn malformed_dotenv_is_read_up_to_the_error()
{
  let package = PackageDir::empty();
  let dotenv = package.write( ".env", "SQLX_OFFLINE=true\nnot a setting\nSQLX_OFFLINE_DIR=after\n" );

  let options = options_for( package.root(), &[] );
  assert_eq!( options.offline, Some( true ) );
  assert_eq!( options.cache_dir_override, None );
  assert_eq!( options.config_files, [ dotenv ] );
}

/// A `.env` directory -- a Python virtualenv, say -- is not a settings file,
/// and is never recorded: watching it would re-run the check on every install.
#[ test ]
fn dotenv_directory_is_not_a_settings_file()
{
  let package = PackageDir::empty();
  package.write( ".env/bin/activate", "" );

  assert_eq!( options_for( package.root(), &[] ).config_files, Vec::< PathBuf >::new() );
}

/// The variable `sqlx.toml` names is read from `.env` files too.
#[ test ]
fn database_url_var_is_read_from_dotenv_too()
{
  let package = PackageDir::empty();
  let config = package.write( "sqlx.toml", "[common]\ndatabase-url-var = \"APP_DATABASE_URL\"\n" );
  let dotenv = package.write( ".env", "APP_DATABASE_URL=sqlite://db.sqlite\n" );

  let options = options_for( package.root(), &[] );
  assert_eq!( options.database_url_var, "APP_DATABASE_URL" );
  assert!( options.database_url_set );
  assert_eq!( options.config_files, [ config, dotenv ] );
}

/// Dotted keys, an inline table and the other string forms are the same TOML.
#[ test ]
fn database_url_var_accepts_every_toml_form()
{
  for config in
  [
    "common.database-url-var = \"APP_DATABASE_URL\"\n",
    "common = { database-url-var = 'APP_DATABASE_URL' }\n",
    "[common]\n\"database-url-var\" = \"\"\"APP_DATABASE_URL\"\"\"\n",
  ]
  {
    let package = PackageDir::empty();
    package.write( "sqlx.toml", config );
    assert_eq!( options_for( package.root(), &[] ).database_url_var, "APP_DATABASE_URL", "{config}" );
  }
}

/// Without the key, or with a `sqlx.toml` sqlx cannot parse -- which fails
/// every query macro anyway -- the variable is `DATABASE_URL`; an existing
/// `sqlx.toml` is recorded either way.
#[ test ]
fn database_url_var_defaults_to_database_url()
{
  for config in [ "[macros]\n", "[common\n", "[common]\ndatabase-url-var = 7\n" ]
  {
    let package = PackageDir::empty();
    let file = package.write( "sqlx.toml", config );

    let options = options_for( package.root(), &[] );
    assert_eq!( options.database_url_var, "DATABASE_URL", "{config}" );
    assert_eq!( options.config_files, [ file ], "{config}" );
  }
}

/// sqlx reads only the crate's own `sqlx.toml`, never an ancestor's.
#[ test ]
fn ancestor_sqlx_toml_is_ignored()
{
  let tree = PackageDir::empty();
  tree.write( "sqlx.toml", "[common]\ndatabase-url-var = \"APP_DATABASE_URL\"\n" );
  let member = tree.mkdir( "member" );

  let options = options_for( &member, &[] );
  assert_eq!( options.database_url_var, "DATABASE_URL" );
  assert_eq!( options.config_files, Vec::< PathBuf >::new() );
}
