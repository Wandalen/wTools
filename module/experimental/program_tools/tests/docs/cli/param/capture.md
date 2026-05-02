# Parameter Spec: capture

### Scope

- **Purpose**: Verify `--capture` controls whether subprocess output is forwarded live or captured for programmatic access.
- **Responsibility**: Edge cases for absent (forwarding default), captured mode, piped vs terminal, and invalid value.
- **In Scope**: CLI default of `forwarding`; `captured` mode collecting stdout/stderr; invalid value rejection.
- **Out of Scope**: API default (API uses `captured` by default — see `docs/feature/005_configuration_surface.md`); output format (→ `command/run.md`).

### EC-1 (Divergence A): Absent — output forwarded to terminal

**Given:** A program that prints to stdout and stderr
**When:** `program_tools run prog.rs` (no `--capture`)
**Then:** Exit code `0`; output appears on the caller's terminal in real time; no buffering artefacts
**Commands:** run

### EC-2 (Divergence B): `--capture captured` — output collected

**Given:** The same program that prints to stdout and stderr
**When:** `program_tools run --capture captured prog.rs`
**Then:** Exit code `0`; stdout and stderr of the subprocess are captured and surfaced in the tool's result; output does not appear on terminal during execution
**Commands:** run

### EC-3: `--capture forwarding` — explicit forwarding

**Given:** A program that prints to stdout
**When:** `program_tools run --capture forwarding prog.rs`
**Then:** Exit code `0`; behaviour is identical to omitting the flag; output forwarded live
**Commands:** run

### EC-4: Invalid value

**Given:** Any compilable program
**When:** `program_tools run --capture buffered prog.rs`
**Then:** Exit code `1`; `stderr` contains an error describing the invalid capture mode; `stdout` is empty
**Commands:** run

### EC-5: Captured mode — stderr also captured

**Given:** A program that writes only to stderr
**When:** `program_tools run --capture captured stderr_prog.rs`
**Then:** Exit code `0`; stderr content is captured and accessible; nothing appears on the caller's terminal
**Commands:** run

### EC-6: Duplicate flag — behaviour documented

**Given:** A compilable single-file program
**When:** `program_tools run --capture forwarding --capture captured prog.rs`
**Then:** Exit code `0` with last value winning, OR exit code `1` with duplicate-flag error; behaviour is consistent and documented
**Commands:** run
