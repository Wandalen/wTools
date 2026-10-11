# Manual Tests

### Scope

#### Responsibilities

Check by hand what the automated suite cannot drive: real `sqlx-cli`, sqlx itself reading `.env` and `sqlx.toml`, workspace layouts, and Cargo's own freshness decisions across consecutive builds -- each against a throwaway consumer crate wired to this checkout.

#### In Scope

- The fail-fast and soft-warn workflow, build by build.
- `cargo sqlx prepare` and `cargo sqlx prepare --check`.
- Settings in `.env` files and `sqlx.toml`, read by sqlx and by `sqlx_query` alike.
- Every query form, workspace layouts, nested packages and target directories.
- A query in code the current build does not compile.
- Rerun behavior: what re-runs the check, and what leaves a build fresh.
- `auto_regen` repairing a stale entry, in the library and in a test, and reporting a failed attempt.

#### Out of Scope

- Anything the automated tests already pin (see `../readme.md`); scenarios here repeat it only where a real tool is the point.
- Scripted assertions: `create_fixture_crate.sh` only sets a fixture up, and the reader judges every outcome.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Manual test plan: setup, scenarios, expected outcomes |
| `create_fixture_crate.sh` | Creates the throwaway consumer crate scenarios start from |

### Prerequisites

- A Rust toolchain, and `sqlite3` on `PATH`.
- For S2 only, `sqlx-cli` 0.9: `cargo install sqlx-cli --version '^0.9' --no-default-features --features rustls,sqlite`.
- A shell with `SQLX_OFFLINE`, `SQLX_OFFLINE_DIR`, `SQLX_QUERY_SOFT_WARN`, `SQLX_QUERY_AUTO_REGEN`, `DATABASE_URL` and `APP_DATABASE_URL` unset, except where a step sets them, and no `.env` file in any parent directory of the fixtures: sqlx and `sqlx_query` would both read it.
- Optional: one `CARGO_TARGET_DIR` exported for every scenario except S7, so sqlx compiles once instead of once per fixture.

### Setup

From this crate's directory:

```sh
fixture="$PWD/tests/manual/create_fixture_crate.sh"
"$fixture" /tmp/sq/s1                              # one fresh fixture per scenario
cd /tmp/sq/s1
export DATABASE_URL=sqlite:///tmp/sq/s1/fixture.sqlite
```

The script prints the `DATABASE_URL` to export. The fixture depends on this checkout of `sqlx_query` with `auto_regen` compiled in, calls `ensure_offline_mode()` from `build.rs`, and has:

- one query at `src/lib.rs:3`, `SELECT id FROM t`;
- `fixture.sqlite`, with table `t ( id INTEGER PRIMARY KEY NOT NULL, name TEXT )`;
- an empty `.sqlx/`.

Create fixtures outside any Cargo workspace; S5 nests them in workspaces of its own on purpose.

**Bootstrap** below means `SQLX_QUERY_SOFT_WARN=1 cargo build`, which writes the first entry: exit 0, the warning `src/lib.rs:3 -- no .sqlx entry matches this query's SQL`, and a summary saying sqlx writes the query's missing entry into `.sqlx` wherever it is compiled against `DATABASE_URL`.

**Fail-fast output** below means exit 101 with, in order: the site warning; `forcing SQLX_OFFLINE=true, so sqlx rejects the query above ...`; `fix with cargo sqlx prepare ... or rebuild with SQLX_QUERY_SOFT_WARN=1 ...`; then sqlx's own error, `SQLX_OFFLINE=true but there is no cached data for this query`.

**Did the check run?** Build with `-v`: `Running .../build-script-build` means the check ran; `Fresh sqlx_query_manual_fixture` means Cargo replayed its last output.

### Scenarios

#### S1 -- Fail fast, then repair with soft-warn

1. Bootstrap. `.sqlx/` now holds one entry.
2. `cargo build` → exit 0, no `sqlx_query` warning.
3. In `src/lib.rs`, change `SELECT id FROM t` to `SELECT id FROM t WHERE id > 0`. `cargo build` → fail-fast output, even though `DATABASE_URL` is set.
4. Change no file. `SQLX_QUERY_SOFT_WARN=1 cargo build -v` → the check runs again; exit 0; `.sqlx/` holds two entries.
5. `cargo build` → exit 0, silent.

#### S2 -- `cargo sqlx prepare`

**S2a, prepare after an edit.**

1. Bootstrap, then change the query as in S1 step 3.
2. `cargo sqlx prepare` → exit 0, `query data written to .sqlx in the current directory`. `.sqlx/` holds exactly one entry, the new query's: prepare deletes the old ones first.
3. `cargo build` → exit 0, silent.

**S2b, `--check` catches drift.** In a fresh fixture, replace `src/lib.rs` with:

```rust
pub async fn names( pool : &sqlx::SqlitePool ) -> Result< (), sqlx::Error >
{
  let row = sqlx::query!( "SELECT name FROM t" ).fetch_one( pool ).await?;
  let _ = row;
  Ok( () )
}
```

1. Bootstrap.
2. In the entry under `.sqlx/`, flip each value of `describe.nullable` (`true` ↔ `false`), standing in for schema drift.
3. `cargo sqlx prepare --check` → exit 1, `prepare check failed: one or more query files differ`.

#### S3 -- Settings in `.env` and `sqlx.toml`

Each part uses a fresh fixture. A setting kept in `.env` must act exactly as it does in the environment.

- **S3a.** `echo "DATABASE_URL=$DATABASE_URL" > .env; unset DATABASE_URL`, then bootstrap → exit 0, and the entry is written.
- **S3b.** The same `.env`, `DATABASE_URL` unset; `SQLX_QUERY_AUTO_REGEN=1 cargo build` → exit 0, with ``regenerated stale .sqlx entries via an isolated live `cargo check` ``; `.sqlx/` holds the entry.
- **S3c.** `printf 'DATABASE_URL=%s\nSQLX_OFFLINE=false\n' "$DATABASE_URL" > .env; unset DATABASE_URL`.
  1. `cargo build` → exit 0, no `sqlx_query` warning: the check stands aside, as for `SQLX_OFFLINE=false` in the environment.
  2. `sed -i '/^SQLX_OFFLINE=/d' .env`, then `cargo build -v` → the check runs again, since `.env` is watched → fail-fast output: the live build of step 1 wrote no entry.
- **S3d.** `mkdir custom_cache; echo "SQLX_OFFLINE_DIR=$PWD/custom_cache" > .env`, keep `DATABASE_URL` exported.
  1. Bootstrap → exit 0; the summary names `custom_cache`, the entry lands there, and `.sqlx/` stays empty.
  2. `cargo build` → exit 0, silent.
- **S3e.** `sed -i 's/"macros" ]/"macros", "sqlx-toml" ]/' Cargo.toml; printf '[common]\ndatabase-url-var = "APP_DATABASE_URL"\n' > sqlx.toml; export APP_DATABASE_URL=$DATABASE_URL; unset DATABASE_URL`.
  1. Bootstrap → exit 0; the summary says the query is compiled against `APP_DATABASE_URL`, and the entry is written.
  2. Change the query as in S1 step 3; `SQLX_QUERY_AUTO_REGEN=1 cargo build` → exit 0, with ``regenerated stale .sqlx entries via an isolated live `cargo check` ``.
  3. `export DATABASE_URL=$APP_DATABASE_URL; unset APP_DATABASE_URL`, change the query to `SELECT id FROM t WHERE id > 1`; `SQLX_QUERY_AUTO_REGEN=1 cargo build` → `auto_regen skipped: APP_DATABASE_URL is unset or empty`, then the fail-fast output, its advice naming `APP_DATABASE_URL`.

#### S4 -- Every query form

In a fresh fixture, run `mkdir queries; printf 'SELECT id AS from_file FROM t' > queries/from_file.sql`, and replace `src/lib.rs` with:

```rust
macro_rules! inside_macro_rules
{
  () => { sqlx::query!( "SELECT id AS inside_macro_rules FROM t" ) };
}

pub struct Row { pub id : i64 }

pub async fn sites( pool : &sqlx::SqlitePool ) -> Result< (), sqlx::Error >
{
  let _ = sqlx::query!( "SELECT id AS plain FROM t" ).fetch_one( pool ).await?;
  let _ = vec![ sqlx::query!( "SELECT id AS inside_vec FROM t" ) ];
  let _ = inside_macro_rules!().fetch_one( pool ).await?;
  let _ = sqlx::query!( "SELECT id " + "AS concatenated FROM t" ).fetch_one( pool ).await?;
  let _ = sqlx::query_unchecked!( "SELECT id AS unchecked FROM t" ).fetch_one( pool ).await?;
  let _ = sqlx::query_scalar_unchecked!( "SELECT id AS scalar_unchecked FROM t" ).fetch_one( pool ).await?;
  let _ = sqlx::query_as_unchecked!( Row, "SELECT id FROM t WHERE id > -1" ).fetch_one( pool ).await?;
  let _ = sqlx::query!( r#"SELECT id AS "raw_string" FROM t"# ).fetch_one( pool ).await?;
  let _ = sqlx::query_file!( "queries/from_file.sql" ).fetch_one( pool ).await?;
  Ok( () )
}
```

1. Bootstrap → nine site warnings, at lines 3, 10, 11 and 13 to 18 (line 12 calls the `macro_rules!` macro, whose query is reported at line 3); `.sqlx/` holds nine entries.
2. Change `AS inside_vec` to `AS inside_vec_v2`, and bootstrap again → only line 11 is reported; ten entries.
3. Change only the SQL file: `printf 'SELECT id AS from_file_v2 FROM t' > queries/from_file.sql`. `cargo build` → fail-fast output for line 18 alone.

#### S5 -- Workspace layouts

**S5a to S5c, an entry at the workspace root.**

```sh
mkdir -p /tmp/sq/s5/.sqlx
printf '[workspace]\nmembers = [ "member" ]\nresolver = "2"\n' > /tmp/sq/s5/Cargo.toml
"$fixture" /tmp/sq/s5/member && cd /tmp/sq/s5/member && rmdir .sqlx
export DATABASE_URL=sqlite:///tmp/sq/s5/member/fixture.sqlite
```

1. Bootstrap → the summary names `/tmp/sq/s5/.sqlx`, and the entry lands there.
2. `cargo build` → exit 0, silent.
3. `rm /tmp/sq/s5/.sqlx/query-*.json`, change nothing else, `cargo build` → fail-fast output.

**S5n, nested workspaces.**

```sh
mkdir -p /tmp/sq/s5n/.sqlx /tmp/sq/s5n/inner/.sqlx
printf '[workspace]\nmembers = []\nexclude = [ "inner" ]\nresolver = "2"\n' > /tmp/sq/s5n/Cargo.toml
printf '[workspace]\nmembers = [ "m" ]\nresolver = "2"\n' > /tmp/sq/s5n/inner/Cargo.toml
"$fixture" /tmp/sq/s5n/inner/m && cd /tmp/sq/s5n/inner/m && rmdir .sqlx
export DATABASE_URL=sqlite:///tmp/sq/s5n/inner/m/fixture.sqlite
```

1. Bootstrap → the entry lands in `/tmp/sq/s5n/inner/.sqlx`, the root of the workspace the crate belongs to; the outer `.sqlx/` stays empty.

#### S6 -- A nested package

1. Bootstrap.
2. Add a package inside the fixture, with a query of its own:

   ```sh
   mkdir -p nested/src
   printf '[package]\nname = "nested_pkg"\nversion = "0.1.0"\nedition = "2021"\n' > nested/Cargo.toml
   printf 'pub fn f() { let _ = sqlx::query!( "SELECT id AS nested FROM t" ); }\n' > nested/src/lib.rs
   touch src/lib.rs
   ```

3. `cargo build` → exit 0, and no warning about the nested package's query.

#### S7 -- A target directory inside the package

In a fresh fixture, `export CARGO_TARGET_DIR=$PWD/build-out` -- a target directory that is not named `target`, so sqlx compiles from scratch into it.

1. Bootstrap.
2. `cargo build -v` → the check runs once more, since the environment changed.
3. `cargo build -v` → `Fresh sqlx_query_manual_fixture`: `build-out/` is recognized by its `CACHEDIR.TAG`, and never watched.

#### S8 -- A relative `SQLX_OFFLINE_DIR`

Continue in `/tmp/sq/s5/member` after S5.

1. Change the query to `SELECT id FROM t WHERE id > 1`. `SQLX_OFFLINE_DIR=.sqlx SQLX_QUERY_SOFT_WARN=1 cargo build` → the summary names `/tmp/sq/s5/.sqlx` -- relative to the workspace root, as for sqlx -- and the entry lands there; no `member/.sqlx/` appears.
2. `SQLX_OFFLINE_DIR=.sqlx cargo build` → exit 0.

#### S9 -- Nothing changed, nothing re-run

1. Bootstrap, then `cargo build -v` → the check runs once more, since the environment changed.
2. `cargo build -v` → `Fresh sqlx_query_manual_fixture`.

**S9b, git-ignored files.** In a fresh fixture, make the fixture a git repository that ignores `*.log`, and create `src/debug.log` before the first build. Repeat steps 1 and 2, then `echo x >> src/debug.log` and `cargo build -v` → still `Fresh`: git-ignored files are never watched.

#### S10 -- A deleted entry

1. Bootstrap, then `cargo build` → exit 0.
2. `rm .sqlx/query-*.json`, change nothing else, `cargo build` → fail-fast output.

#### S11 -- `auto_regen` repairs a stale entry

1. Bootstrap, then change the query as in S1 step 3.
2. `SQLX_QUERY_AUTO_REGEN=1 cargo build` → exit 0, with ``regenerated stale .sqlx entries via an isolated live `cargo check` ``; `.sqlx/` holds two entries. The first run compiles sqlx again, inside the nested check's own target directory.
3. `cargo build` → exit 0, silent.

#### S12 -- A query `cargo build` does not compile

1. Bootstrap, then add an integration test with a query of its own:

   ```sh
   mkdir tests
   printf '#[ test ]\nfn names()\n{\n  let _ = sqlx::query!( "SELECT name FROM t" );\n}\n' > tests/it.rs
   ```

2. `cargo build` → the site warning for `tests/it.rs:4`, then `forcing SQLX_OFFLINE=true, so sqlx rejects the query above wherever it is compiled, ...`, and exit 0: `cargo build` does not compile tests.
3. `SQLX_QUERY_SOFT_WARN=1 cargo build` → exit 0, with a summary saying sqlx writes the entry wherever the query is compiled against `DATABASE_URL`; `.sqlx/` still holds one entry.
4. `cargo test --no-run` → fail-fast output for `tests/it.rs:4`.
5. `SQLX_QUERY_SOFT_WARN=1 cargo test --no-run` → exit 0; `.sqlx/` holds two entries.
6. `cargo test --no-run` → exit 0, silent.

#### S13 -- `auto_regen` repairs a query in a test

1. Bootstrap, then add `tests/it.rs` as in S12 step 1.
2. `SQLX_QUERY_AUTO_REGEN=1 cargo build` → exit 0, with ``regenerated stale .sqlx entries via an isolated live `cargo check` ``; `.sqlx/` holds two entries. The nested check builds every target with every feature, so it compiles the test.
3. `cargo test --no-run` → exit 0, silent.
4. Append a query that no configuration compiles: `printf '\n#[ cfg( any() ) ]\nfn never()\n{\n  let _ = sqlx::query!( "SELECT id AS never_compiled FROM t" );\n}\n' >> tests/it.rs`. `SQLX_QUERY_AUTO_REGEN=1 cargo build` → exit 0, with ``regenerated 0 of 1 stale .sqlx entries via an isolated live `cargo check`, which compiles none of the code holding the rest``, then the site warning for `tests/it.rs:10` and the two fail-fast summary lines.

#### S14 -- A failed regeneration

In a fresh fixture, `export DATABASE_URL=sqlite://$PWD/missing_dir/none.sqlite`, a database sqlx cannot open.

1. `SQLX_QUERY_AUTO_REGEN=1 cargo build` → ``auto_regen's isolated `cargo check` exited with exit status: 101; .sqlx entries may still be stale``, then the fail-fast output, and none of the nested check's own errors.
2. `touch src/lib.rs`, then `SQLX_QUERY_AUTO_REGEN=1 cargo build -vv` → the nested check's own errors as well, each line prefixed `[sqlx_query_manual_fixture 0.1.0]`: `error returned from database: (code: 14) unable to open database file`, then `could not compile`.

### Last Run

Every scenario matched its expected outcome, with sqlx 0.9.0 and sqlx-cli 0.9.0. On 2026-10-11, S3b, S3e and S14 ran after the latest change, which passes the nested check's output on through the build script's stderr. On 2026-10-09, S1, all of S3 and S9 ran after the changes that made settings in `.env` files and `sqlx.toml` count and stopped promising entries in a `SQLX_OFFLINE_DIR` that is not a directory; S11, S12 and S13 ran before those, after the changes to the summaries' wording and to what `auto_regen` compiles. S9b has not been run by hand yet; `package_walk_test.rs` covers git-ignored files automatically.
