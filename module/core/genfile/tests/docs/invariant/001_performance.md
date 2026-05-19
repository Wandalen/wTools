# Invariant Spec: Performance

### Scope

- **Element:** `invariant/001_performance`
- **Source:** `docs/invariant/001_performance.md`
- **Prefix:** `IN-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | typical_command_within_latency_bound | performance | 🔶 deferred |
| IN-02 | repl_startup_within_500ms | performance | 🔶 deferred |

---

### IN-01: typical command completes within latency bound

- **Given:** genfile binary is built in release mode; archive exists on disk
- **When:** `.archive.load path::<file>` is run and execution time is measured
- **Then:** Elapsed time is under 100ms for the command execution (excluding `cargo run` overhead)
- **Tests:** none — see task/001_fill_test_surface_gaps.md

### IN-02: REPL startup completes within 500ms

- **Given:** genfile binary is built in release mode
- **When:** `genfile` is started and time-to-prompt is measured
- **Then:** First prompt appears within 500ms
- **Tests:** none — see task/001_fill_test_surface_gaps.md
