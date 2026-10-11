#!/usr/bin/env bash
# Create a throwaway consumer crate wired to this checkout of sqlx_query, with a
# real SQLite database and an empty .sqlx/ directory -- the starting state for
# every scenario in tests/manual/readme.md. Orchestration only: it never builds
# or checks anything.
#
# usage: create_fixture_crate.sh <dir>
#   <dir> must not exist yet; create it outside any Cargo workspace (e.g. under /tmp).
#
# Prints the DATABASE_URL to export for the fixture on success.

set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <dir>" >&2
  exit 2
fi

if [ -e "$1" ]; then
  echo "error: $1 already exists; pick a fresh path" >&2
  exit 2
fi

command -v sqlite3 > /dev/null || { echo "error: sqlite3 CLI not found on PATH" >&2; exit 2; }

sqlx_query_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../.." && pwd )"

mkdir -p "$1/src" "$1/.sqlx"
fixture_dir="$( cd "$1" && pwd )"

cat > "$fixture_dir/Cargo.toml" << EOF
[package]
name = "sqlx_query_manual_fixture"
version = "0.1.0"
edition = "2021"

[dependencies]
sqlx = { version = "0.9", default-features = false, features = [ "sqlite", "runtime-tokio", "tls-rustls-ring", "macros" ] }

[build-dependencies]
sqlx_query = { path = "$sqlx_query_dir", features = [ "auto_regen" ] }
EOF

cat > "$fixture_dir/build.rs" << 'EOF'
fn main()
{
  sqlx_query::ensure_offline_mode();
}
EOF

cat > "$fixture_dir/src/lib.rs" << 'EOF'
pub async fn first_id( pool : &sqlx::SqlitePool ) -> Result< i64, sqlx::Error >
{
  let row = sqlx::query!( "SELECT id FROM t" ).fetch_one( pool ).await?;
  Ok( row.id )
}
EOF

sqlite3 "$fixture_dir/fixture.sqlite" "CREATE TABLE t ( id INTEGER PRIMARY KEY NOT NULL, name TEXT );"

echo "fixture created: $fixture_dir"
echo "export DATABASE_URL=sqlite://$fixture_dir/fixture.sqlite"
