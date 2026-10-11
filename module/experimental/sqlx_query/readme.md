# Module :: sqlx_query
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=sqlx_query)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/sqlx_query?color=e3e8f0&logo=docs.rs)](https://docs.rs/sqlx_query) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Keep sqlx's offline query cache (`.sqlx/`) honest. Called from your `build.rs`, `sqlx_query` builds your crate with `SQLX_OFFLINE=true` -- the way CI and teammates without a database build it -- and names every query that has no `.sqlx/` entry, so a stale cache fails on your machine instead of theirs.

## Why

With `DATABASE_URL` set, sqlx's query macros check every query against the live database and never read `.sqlx/`. Change a query's SQL, forget `cargo sqlx prepare`, and every local build still passes. The first build to notice is one without a database -- in CI, or on a teammate's machine, long after the change.

`sqlx_query` makes your own build the one that notices. It finds every sqlx query macro in your crate, looks each query's entry up the way sqlx does, and then builds offline, exactly as CI will.

## Usage

Add it as a build dependency:

```sh
cargo add --build sqlx_query
```

Then call it from `build.rs`:

```rust,no_run
// build.rs
fn main()
{
  sqlx_query::ensure_offline_mode();
}
```

Your query call sites don't change. Before you start, run `cargo sqlx prepare` once to create `.sqlx/`: `sqlx_query` never creates it, and without it, it only warns.

When a query has no entry -- its SQL changed, or the entry was deleted -- the build stops at that query, whether or not a database is available:

```text
warning: my_crate@0.1.0: sqlx_query: src/lib.rs:3 -- no .sqlx entry matches this query's SQL
warning: my_crate@0.1.0: sqlx_query: forcing SQLX_OFFLINE=true, so sqlx rejects the query above wherever it is compiled, exactly as an offline build (CI, a teammate without a database) would
warning: my_crate@0.1.0: sqlx_query: fix with `cargo sqlx prepare` (add `-- --all-targets --all-features` when queries live in tests or behind features), or rebuild with SQLX_QUERY_SOFT_WARN=1 to compile against DATABASE_URL and refresh .sqlx in place
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
 --> src/lib.rs:3:13
```

Either fix brings the build back. Commit the new entries:

```sh
cargo sqlx prepare                    # regenerate .sqlx/ with sqlx-cli
SQLX_QUERY_SOFT_WARN=1 cargo build    # or build once against your database, which writes the missing entries
```

## Behavior

| Situation | Result |
|---|---|
| Every query has an entry | Builds offline, silently -- the same build CI runs |
| A query has no entry | Warns at the query, then fails wherever it's compiled, as an offline build would |
| ... with `SQLX_QUERY_SOFT_WARN=1` | Warns, then builds live if sqlx finds a database -- writing the missing entries of the queries it compiles -- and offline otherwise |
| `SQLX_OFFLINE` set by you, or by `cargo sqlx prepare` | Never overridden; with `true`, missing entries are still reported |
| No `.sqlx/` directory yet | One warning; the build is otherwise untouched |

Changing any setting below, any source file, or any `.sqlx/` entry re-runs the check on the next build.

## Configuration

| Variable | Effect |
|---|---|
| `SQLX_QUERY_SOFT_WARN=1` | Don't force offline mode over missing entries; build live and let sqlx write them |
| `SQLX_QUERY_AUTO_REGEN=1` | With the `auto_regen` feature: before deciding, repair missing entries with a nested live `cargo check` of every target and feature |
| `SQLX_OFFLINE` | An explicit value is never overridden |
| `SQLX_OFFLINE_DIR` | Searched first and written to, as by sqlx, which never creates it; a relative path starts at the workspace root |
| `DATABASE_URL` | The database `auto_regen` compiles against; your `sqlx.toml` can rename it, as for sqlx |

The last three are read exactly as sqlx reads them: from the environment, or else from the `.env` files in your crate's directory and its parents. The two `SQLX_QUERY_*` switches are read from the environment only.

Cargo features: `enabled` (default) is the check itself. `auto_regen` adds the nested `cargo check`, which builds your dependencies, dev-dependencies included, again in a separate target directory. `full` enables both.

## Limits

- **sqlx 0.9.** Macro names, cache-file names, directory lookup and variable parsing all follow sqlx 0.9.
- **Missing entries only.** `sqlx_query` checks that every query has an entry, not that the entry still matches your schema -- that's what `cargo sqlx prepare --check` is for.
- **Your crate's own sources.** It checks all twelve query macros -- `query!`, `query_as!`, `query_scalar!`, their `query_file` forms, and the `_unchecked` variant of each -- in the crate's own `.rs` files, including tests, examples and feature-gated code the current build may not compile. Macros are recognized by name. Git-ignored files, nested packages and code generated into `OUT_DIR` aren't scanned, and neither is SQL passed in through a `macro_rules!` parameter.
- **Hands off the cache.** It never creates `.sqlx/`, and never deletes or edits an entry.

Design notes, invariants and the manual test plan live in the crate's `docs/` and `tests/` directories.

