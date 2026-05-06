# Parameter Spec: profile

### Scope

- **Purpose**: Verify that `--profile` selects the correct Cargo build profile.
- **Responsibility**: Edge cases for debug and release profiles, omission, invalid value, and duplicate supply.
- **In Scope**: `debug` and `release` values; default behaviour; invalid value rejection.
- **Out of Scope**: Build artifact paths (→ `param/target_dir.md`); output content (→ `command/run.md`).

### EC-1 (Divergence A): `--profile debug`

**Given:** A compilable single-file Rust program
**When:** `program_tools run --profile debug main.rs`
**Then:** Exit code `0`; build proceeds without `--release` flag; binary runs and produces output
**Commands:** run

### EC-2 (Divergence B): `--profile release`

**Given:** A compilable single-file Rust program
**When:** `program_tools run --profile release main.rs`
**Then:** Exit code `0`; build proceeds with `cargo build --release`; binary runs and produces output
**Commands:** run

### EC-3: Omitted — defaults to debug

**Given:** A compilable single-file Rust program
**When:** `program_tools run main.rs` (no `--profile` flag)
**Then:** Exit code `0`; behaviour is identical to `--profile debug`
**Commands:** run

### EC-4: Invalid value

**Given:** A compilable single-file Rust program
**When:** `program_tools run --profile fast main.rs`
**Then:** Exit code `1`; `stderr` contains an error describing the invalid profile value; `stdout` is empty
**Commands:** run

### EC-5: Release and debug profiles produce identical output for deterministic program

**Given:** A program that prints a fixed string regardless of optimisation level
**When:** `program_tools run --profile release prog.rs` vs `program_tools run --profile debug prog.rs`
**Then:** Both exit `0`; both produce identical `stdout`; the profile controls the build, not the output content
**Commands:** run

### EC-6: Duplicate flag — behaviour documented

**Given:** A compilable single-file Rust program
**When:** `program_tools run --profile debug --profile release main.rs`
**Then:** Exit code `0` with last value winning, OR exit code `1` with duplicate-flag error; behaviour is consistent and documented
**Commands:** run
