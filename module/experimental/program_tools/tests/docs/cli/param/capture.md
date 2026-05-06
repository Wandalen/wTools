# Parameter Spec: capture

### Scope

- **Purpose**: Verify `--capture` switches output mode between forwarding (default) and buffered.
- **Responsibility**: Edge cases for absent (forwarding), present (buffered), non-zero exit with capture, `--env` interaction, stderr-only program, and unknown variant flag.
- **In Scope**: CLI forwarding default; capture mode stdout and stderr buffering; `--capture` is a boolean presence flag (no value argument accepted).
- **Out of Scope**: API default (`true` — see `docs/feature/005_configuration_surface.md`); timeout interaction with capture (→ `param/timeout.md` EC-2).

### EC-1 (Divergence A): Absent — output forwarded to terminal

**Given:** A program that prints to stdout; no `--capture` flag
**When:** `program_tools run prog.rs`
**Then:** Exit code `0`; output flows directly to the caller's terminal in real time; no buffering occurs
**Commands:** run

### EC-2 (Divergence B): Present — output captured and buffered

**Given:** A program that prints to stdout; `--capture` flag present (presence flag — no value argument)
**When:** `program_tools run --capture prog.rs`
**Then:** Exit code `0`; stdout and stderr are captured into the runner's buffers; the CLI re-emits stdout via print and stderr via eprint after execution completes
**Commands:** run

### EC-3: Capture with non-zero exit program

**Given:** A program that prints to stdout then exits non-zero; `--capture` flag
**When:** `program_tools run --capture exit_fail.rs`
**Then:** Exit code non-zero (forwarded from program); any output produced before exit is captured; capture mode does not suppress or alter the exit code
**Commands:** run

### EC-4: Capture with `--env` flag

**Given:** A program that reads an env var and prints it; `--capture` and `--env KEY=VALUE`
**When:** `program_tools run --capture --env KEY=VALUE prog.rs`
**Then:** Exit code `0`; the env var is visible to the child process; captured stdout contains the expected env var value
**Commands:** run

### EC-5: Capture mode — stderr written by program re-emitted via eprint

**Given:** A program that writes only to stderr; `--capture` flag
**When:** `program_tools run --capture stderr_prog.rs`
**Then:** Exit code `0`; stderr is captured into the buffer and re-emitted to the caller's terminal via eprint after execution completes; stdout is empty; both streams are re-emitted — not suppressed
**Commands:** run

### EC-6: Unknown variant flag `--no-capture` rejected

**Given:** Any compilable program; `--no-capture` flag (not a defined CLI flag)
**When:** `program_tools run --no-capture prog.rs`
**Then:** Exit code `2`; `stderr` contains a clap error referencing the unrecognised flag; `stdout` is empty
**Commands:** run
