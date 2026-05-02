# Parameter Spec: timeout

### Scope

- **Purpose**: Verify `--timeout` limits execution duration and surfaces timeout as an error.
- **Responsibility**: Edge cases for absent (no limit), short timeout, generous timeout, zero, negative, and non-numeric values.
- **In Scope**: No-limit default; timeout error on duration exceeded; invalid value rejection.
- **Out of Scope**: Build time (timeout governs the subprocess execution, not the compilation step — document which).

### EC-1 (Divergence A): Absent — no execution time limit

**Given:** A program that sleeps for 100ms
**When:** `program_tools run slow.rs` (no `--timeout`)
**Then:** Exit code `0`; program runs to completion; output appears after the sleep completes
**Commands:** run

### EC-2 (Divergence B): Short timeout — process terminated

**Given:** A program that sleeps for 500ms; `--timeout 1` (1ms limit)
**When:** `program_tools run --timeout 1 slow.rs`
**Then:** Exit code non-zero; `stderr` contains a timeout error; program output is absent or incomplete
**Commands:** run

### EC-3: Generous timeout — program completes within limit

**Given:** A program that completes in under 1s; `--timeout 60000` (60s)
**When:** `program_tools run --timeout 60000 fast.rs`
**Then:** Exit code `0`; program runs to completion; output matches expected; no timeout triggered
**Commands:** run

### EC-4: Zero timeout

**Given:** Any compilable program
**When:** `program_tools run --timeout 0 main.rs`
**Then:** Exit code non-zero; program either never starts or is immediately terminated; behaviour is documented and consistent across calls
**Commands:** run

### EC-5: Non-numeric value

**Given:** Any compilable program
**When:** `program_tools run --timeout abc main.rs`
**Then:** Exit code `1`; `stderr` contains a parse error; `stdout` is empty
**Commands:** run

### EC-6: Negative value

**Given:** Any compilable program
**When:** `program_tools run --timeout -1 main.rs`
**Then:** Exit code `1`; `stderr` contains an invalid value error; `stdout` is empty
**Commands:** run
