# Parameter Spec: name

### Scope

- **Purpose**: Verify `--name` sets the binary crate name in the generated manifest.
- **Responsibility**: Edge cases for absent (default name `script`), custom name, name with hyphens, empty value, and invalid identifier.
- **In Scope**: Manifest crate name field; default name `script`; custom name overriding the default.
- **Out of Scope**: Target directory (→ `param/target_dir.md`); output capture (→ `param/capture.md`).

### EC-1 (Divergence A): Absent — default name `script` used

**Given:** A single-file program at `main.rs`
**When:** `program_tools run main.rs` (no `--name`)
**Then:** Exit code `0`; generated manifest uses `script` as the crate name; program runs correctly
**Commands:** run

### EC-2 (Divergence B): Custom name supplied

**Given:** A single-file program at `main.rs`
**When:** `program_tools run --name my_tool main.rs`
**Then:** Exit code `0`; generated manifest uses `my_tool` as the crate name; program runs correctly
**Commands:** run

### EC-3: Name with hyphens

**Given:** A single-file program
**When:** `program_tools run --name my-tool main.rs`
**Then:** Exit code `0`; hyphens are accepted (Cargo normalises hyphens to underscores internally); program runs correctly
**Commands:** run

### EC-4: Empty name

**Given:** Any compilable program
**When:** `program_tools run --name "" main.rs`
**Then:** Exit code `1`; `stderr` contains an error about empty or invalid name; `stdout` is empty
**Commands:** run

### EC-5: Name that is an invalid Rust identifier

**Given:** Any compilable program
**When:** `program_tools run --name "123invalid" main.rs`
**Then:** Exit code `1`; `stderr` contains an error about invalid crate name; `stdout` is empty
**Commands:** run

### EC-6: Duplicate flag — behaviour documented

**Given:** A compilable single-file program
**When:** `program_tools run --name foo --name bar main.rs`
**Then:** Exit code `0` with last value winning, OR exit code `1` with duplicate-flag error; behaviour is consistent and documented
**Commands:** run
