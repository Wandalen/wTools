# Parameter Spec: env

### Scope

- **Purpose**: Verify `--env` injects environment variables into the executed program.
- **Responsibility**: Edge cases for absent (no injection), single variable, multiple variables, override of existing env, empty value, and variable with equals sign in value.
- **In Scope**: Variable injection into subprocess; collection merge semantics; overriding inherited env.
- **Out of Scope**: Feature activation via env (→ `param/feature.md`).

### EC-1 (Divergence A): Absent — no variables injected

**Given:** A program that reads `MY_VAR` from env and prints it, or prints `unset` if absent
**When:** `program_tools run env_read.rs` (no `--env`)
**Then:** Exit code `0`; `stdout` contains `unset`; no variable injection occurred
**Commands:** run

### EC-2 (Divergence B): Single variable injected

**Given:** The same program that reads `MY_VAR` from env
**When:** `program_tools run --env MY_VAR=hello env_read.rs`
**Then:** Exit code `0`; `stdout` contains `hello`; the variable is visible inside the program
**Commands:** run

### EC-3: Two variables — collection merge

**Given:** A program that reads `VAR_A` and `VAR_B` and prints both
**When:** `program_tools run --env VAR_A=foo --env VAR_B=bar env_read2.rs`
**Then:** Exit code `0`; both variables injected; output shows `foo` and `bar`; second `--env` appends rather than replaces
**Commands:** run

### EC-4: Override of inherited environment variable

**Given:** `MY_VAR` is set in the shell environment; a program that prints `MY_VAR`
**When:** `program_tools run --env MY_VAR=overridden env_read.rs`
**Then:** Exit code `0`; `stdout` contains `overridden` (not the shell value); `--env` takes precedence over inherited env
**Commands:** run

### EC-5: Empty variable value

**Given:** Any compilable program that reads `MY_VAR`
**When:** `program_tools run --env MY_VAR= env_read.rs`
**Then:** Exit code `0`; `MY_VAR` is set to the empty string; program sees empty string rather than unset
**Commands:** run

### EC-6: Value containing equals sign

**Given:** A program that reads `MY_VAR` and prints its full value
**When:** `program_tools run --env MY_VAR=a=b env_read.rs`
**Then:** Exit code `0`; `MY_VAR` is set to `a=b` (only the first `=` is the separator); value printed correctly
**Commands:** run
