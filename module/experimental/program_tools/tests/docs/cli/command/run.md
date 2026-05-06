# Command Spec: run

Verifies the end-to-end observable behavior of `program_tools run`. Covers happy paths,
error rejection, exit code forwarding, and infrastructure failures.

## TC-1 — Happy path: single Rust file

**Command:**
```
program_tools run main.rs
```
(where `main.rs` contains `fn main() { println!("hello"); }`)

**Expected behavior:**
- Exits with code `0`
- `stdout` contains the program's expected output (`hello\n`)
- `stderr` is empty or contains only Cargo build progress on first build

## TC-2 — Happy path: existing Cargo project directory

**Command:**
```
program_tools run ./my_project/
```
(where `./my_project/` contains a valid `Cargo.toml` and `src/main.rs`)

**Expected behavior:**
- Exits with code `0`
- `stdout` contains the project binary's output
- No manifest generation occurs; project's own `Cargo.toml` is used

## TC-3 — Missing `<TARGET>` argument

**Command:**
```
program_tools run
```

**Expected behavior:**
- Exits with code `2` (clap usage error)
- `stderr` contains a usage error referencing the missing required `<TARGET>` argument
- `stdout` is empty

## TC-4 — Non-existent `<TARGET>` path

**Command:**
```
program_tools run /nonexistent/path/script.rs
```

**Expected behavior:**
- Exits with code `1`
- `stderr` contains an error describing the path was not found
- `stdout` is empty

## TC-5 — Unknown flag rejected

**Command:**
```
program_tools run --not-a-real-flag main.rs
```

**Expected behavior:**
- Exits with code `2` (clap usage error)
- `stderr` contains an error referencing the unrecognised flag
- `stdout` is empty

## TC-6 — Target program exit code forwarded

**Command:**
```
program_tools run exit_42.rs
```
(where `exit_42.rs` contains `fn main() { std::process::exit(42); }`)

**Expected behavior:**
- Exits with code `42` (the target program's exact exit code, not normalised to `1`)
- `stdout` and `stderr` match whatever the target program produced
- The raw non-zero exit code is forwarded verbatim

## TC-7 — Infrastructure error: Cargo binary not found

**Command:**
```
program_tools run --cargo /nonexistent/cargo main.rs
```

**Expected behavior:**
- Exits with code `1`
- `stderr` contains a diagnostic about the Cargo binary not being found
- `stdout` is empty

## TC-8 — Compilation error: target program does not compile

**Command:**
```
program_tools run broken.rs
```
(where `broken.rs` contains `fn main() { this_does_not_compile }`)

**Expected behavior:**
- Exits with a non-zero code (Cargo's exit code for compilation failure)
- `stderr` contains Cargo's compiler diagnostic output (contains `error[E...]:` or similar)
- `stdout` is empty

## TC-9 — Top-level `--help` exits zero with usage text

**Command:**
```
program_tools --help
```

**Expected behavior:**
- Exits with code `0`
- `stdout` contains usage text
- `stderr` is empty

## TC-10 — `run --help` exits zero with subcommand usage text

**Command:**
```
program_tools run --help
```

**Expected behavior:**
- Exits with code `0`
- `stdout` contains the `run` subcommand's usage text
- `stderr` is empty

## TC-11 — No subcommand exits non-zero

**Command:**
```
program_tools
```

**Expected behavior:**
- Exits with a non-zero code
- `stderr` contains a clap usage error (subcommand required)
- `stdout` is empty or contains help text

## TC-12 — `--capture` flag enables buffered output mode

**Command:**
```
program_tools run --capture main.rs
```
(where `main.rs` prints to stdout)

**Expected behavior:**
- Exits with code `0`
- `stdout` contains the program's output (captured and re-emitted by the CLI)
- `--capture` is a boolean presence flag — no value argument required

## TC-13 — `--env KEY=VALUE` injects environment variable into script

**Command:**
```
program_tools run --capture --env PT_VAR=hello main.rs
```
(where `main.rs` reads and prints `PT_VAR`)

**Expected behavior:**
- Exits with code `0`
- `stdout` contains the env var value (`hello`)
- The env var is visible to the child process via `std::env::var`
