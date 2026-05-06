# Invariant: PID File Format

### Scope

- **Purpose**: Guarantee that the PID file format written by the `daemon` sub-module is always readable by the `check` sub-module, and vice versa.
- **Responsibility**: Defines the canonical PID file format so two independently-authored sub-modules share a wire contract without drifting apart.
- **In Scope**: Format written by `daemon::write_pidfile()` and `daemonize()`, and format expected by `daemon::read_pidfile()` and `check::is_pidfile_alive()`.
- **Out of Scope**: PID file locking semantics (internal to `daemonize()`); file path conventions; external tools' PID file formats.

### Invariant Statement

A PID file managed by this crate contains exactly one decimal integer representing a process ID, optionally surrounded by ASCII whitespace (spaces, tabs, newlines). Any function that writes a PID file in this crate must write a plain decimal integer. Any function that reads a PID file must accept this format by trimming surrounding whitespace and parsing the result as a decimal integer. No other format — hex, binary, padded, quoted, newline-only — is defined or accepted.

### Enforcement Mechanism

`daemon::write_pidfile(path, pid)` writes `pid.to_string()` — a plain decimal integer with no newline or padding. `daemon::read_pidfile(path)` and `check::is_pidfile_alive(path)` both parse with `content.trim().parse()`. The trim accommodates trailing newlines written by external tools (system init scripts, shell `echo $PID`), not by this crate's own writes.

Verification:

```bash
grep -n "to_string\|trim.*parse\|parse.*trim" \
  src/lifecycle/daemon.rs src/lifecycle/check.rs
# Must show only decimal integer write/read patterns
# No hex formatting, no binary encoding
```

### Violation Consequences

If `write_pidfile` were changed to write a hex value, `read_pidfile` and `is_pidfile_alive` would silently return a parse error on the next read — there is no compile-time check for this contract. The cross-module format agreement has no type-level enforcement; this invariant is the sole documentation of the requirement.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/lifecycle/daemon.rs](../../src/lifecycle/daemon.rs) | `write_pidfile()`, `read_pidfile()`, `write_pidfile_locked()` |
| source | [src/lifecycle/check.rs](../../src/lifecycle/check.rs) | `is_pidfile_alive()` reads PID files written by the daemon module |
| doc | [api/006_daemon_api.md](../api/006_daemon_api.md) | PID file management function signatures |
| doc | [api/005_check_api.md](../api/005_check_api.md) | `is_pidfile_alive()` reads the format defined by this invariant |
