# Invariant Spec: Security

### Scope

- **Element:** `invariant/004_security`
- **Source:** `docs/invariant/004_security.md`
- **Prefix:** `IN-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | dotdot_in_output_destination_rejected | security | ✅ |
| IN-02 | dotdot_in_archive_path_rejected | security | ✅ |
| IN-03 | sensitive_values_absent_from_error_output | security | 🔶 deferred |

---

### IN-01: `..` in materialize destination is rejected

- **Given:** An archive is loaded
- **When:** `.materialize destination::/tmp/safe/../../../etc` is run
- **Then:** Exit code 1; error output says path is invalid; no files written outside the intended directory
- **Behavioral Divergence:** destination with `..` → rejection; destination without `..` → success
- **Tests:** `tests/materialization_test.rs`

### IN-02: `..` in archive load path is rejected

- **Given:** genfile binary is available
- **When:** `.archive.load path::../../etc/passwd` is run
- **Then:** Exit code 1; error indicates path validation failure
- **Tests:** `tests/archive_commands_test.rs`

### IN-03: sensitive parameter values absent from error output

- **Given:** An archive with parameter `secret_key` set to `my-secret` is loaded
- **When:** A command fails and produces error output
- **Then:** The value `my-secret` does not appear in stdout or stderr
- **Tests:** none — see task/001_fill_test_surface_gaps.md
