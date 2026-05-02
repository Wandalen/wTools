# Invariant Spec: output_determinism

### Scope

- **Purpose**: Verify that identical invocations of `program_tools run` produce identical stdout and stderr output.
- **Responsibility**: Repeated runs of a deterministic program yield the same output and exit code.
- **In Scope**: stdout determinism; stderr determinism; exit code stability across identical inputs.
- **Out of Scope**: Programs with intentional non-determinism (random seed, timestamps); build timing differences.

### IC-1: Identical runs produce identical stdout

**Given:** A program that prints a fixed string and exits `0`; same source, same flags, same environment
**When:** `program_tools run main.rs` invoked twice in sequence
**Then:** Both runs exit `0`; `stdout` content is byte-for-byte identical across both runs; the invariant holds for stdout

### IC-2: Identical runs produce identical stderr

**Given:** A program that writes a fixed diagnostic to stderr and exits `0`; same source, same flags
**When:** `program_tools run main.rs` invoked twice in sequence
**Then:** Both runs exit `0`; `stderr` content is identical across both runs (excluding any framework-level timing lines); the invariant holds for stderr
**Commands:** run
