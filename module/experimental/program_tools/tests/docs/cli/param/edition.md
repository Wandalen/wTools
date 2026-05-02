# Parameter Spec: edition

### Scope

- **Purpose**: Verify `--edition` selects the correct Rust edition for compilation.
- **Responsibility**: Edge cases for 2021 edition, 2024 edition, omission, and invalid value.
- **In Scope**: Edition flag forwarding to `rustc`/Cargo; default edition; invalid value rejection.
- **Out of Scope**: Profile selection (→ `param/profile.md`); feature gates (→ `param/feature.md`).

### EC-1 (Divergence A): `--edition 2021`

**Given:** A single-file program using `2021` edition syntax (e.g., `use std::collections::HashMap;`)
**When:** `program_tools run --edition 2021 main.rs`
**Then:** Exit code `0`; program compiles and runs with Rust 2021 edition rules; output correct
**Commands:** run

### EC-2 (Divergence B): `--edition 2024`

**Given:** A single-file program using `2024` edition syntax or features
**When:** `program_tools run --edition 2024 main.rs`
**Then:** Exit code `0`; program compiles and runs with Rust 2024 edition rules; output correct
**Commands:** run

### EC-3: Omitted — defaults to 2021

**Given:** A compilable single-file program
**When:** `program_tools run main.rs` (no `--edition`)
**Then:** Exit code `0`; behaviour is identical to `--edition 2021`; the default edition is 2021
**Commands:** run

### EC-4: Invalid value

**Given:** A compilable single-file program
**When:** `program_tools run --edition 2019 main.rs`
**Then:** Exit code `1`; `stderr` contains an error describing the invalid edition value; `stdout` is empty
**Commands:** run

### EC-5: Edition-specific syntax compiled with correct edition

**Given:** A program that uses edition-2021-specific resolver behaviour
**When:** `program_tools run --edition 2021 main.rs` vs `program_tools run --edition 2018 main.rs` (if 2018 supported)
**Then:** `2021` exits `0`; `2018` may exit non-zero if syntax is incompatible; editions are not interchangeable
**Commands:** run

### EC-6: Duplicate flag — behaviour documented

**Given:** A compilable single-file program
**When:** `program_tools run --edition 2021 --edition 2024 main.rs`
**Then:** Exit code `0` with last value winning, OR exit code `1` with duplicate-flag error; behaviour is consistent and documented
**Commands:** run
