# genfile Manual Test Plan

Manual testing procedures for scenarios not covered by automated integration tests.

### Responsibility Table

| File | Responsibility |
|------|----------------|

## Trigger

Run when: CLI behavior changes, REPL interaction changes, or before release.

## Procedures

### 1. REPL Session Smoke Test

Verify interactive REPL starts, accepts commands, and exits cleanly.

```bash
cargo run --bin genfile
```

Expected: prompt appears, accepts commands, `exit` terminates cleanly with exit code 0.

### 2. Archive Roundtrip via REPL

Verify complete create → add files → save → load → materialize cycle.

```bash
cargo run --bin genfile -- .archive.new name::test-project
cargo run --bin genfile -- .file.add path::src/main.rs content::"fn main() {}"
cargo run --bin genfile -- .archive.save path::/tmp/test.json
cargo run --bin genfile -- .archive.load path::/tmp/test.json
cargo run --bin genfile -- .materialize destination::/tmp/test-output
```

Expected: `/tmp/test-output/src/main.rs` contains `fn main() {}`.

### 3. Parameter Substitution End-to-End

Verify `{{variable}}` placeholders are replaced during materialization.

```bash
cargo run --bin genfile -- .archive.new name::paramtest
cargo run --bin genfile -- .file.add path::hello.txt content::"Hello, {{name}}!"
cargo run --bin genfile -- .value.set name::name value::World
cargo run --bin genfile -- .materialize destination::/tmp/param-output
```

Expected: `/tmp/param-output/hello.txt` contains `Hello, World!`.

### 4. Error Exit Codes

Verify CLI exits with non-zero on error.

```bash
cargo run --bin genfile -- .archive.load path::/nonexistent.json
echo "Exit code: $?"
```

Expected: error message printed to stderr, exit code 1.

### 5. Pack/Unpack Roundtrip

Verify pack creates a portable archive and unpack restores files without rendering.

```bash
mkdir -p /tmp/template-src
echo "Hello {{name}}" > /tmp/template-src/hello.txt
cargo run --bin genfile -- .pack input::/tmp/template-src output::/tmp/packed.json
cargo run --bin genfile -- .archive.load path::/tmp/packed.json
cargo run --bin genfile -- .unpack destination::/tmp/unpacked
```

Expected: `/tmp/unpacked/hello.txt` contains `Hello {{name}}` (placeholders preserved).
