# Parameter Spec: pretty::

### Scope

- **Element:** `parameter/pretty`
- **Source:** `docs/cli/param.md#parameter--11-pretty`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-29 | pretty_one_produces_indented_json | nominal | ✅ |
| EC-30 | pretty_zero_produces_compact_json | nominal | ✅ |

---

### EC-29: pretty one produces indented json

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::"out.json" pretty::1` is run
- **Then:** Exit code 0; output file contains JSON with newlines and indentation
- **Tests:** `tests/archive_commands_test.rs`

### EC-30: pretty zero produces compact json

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::"out.json" pretty::0` is run
- **Then:** Exit code 0; output file is a single-line compact JSON with no extra whitespace
- **Tests:** `tests/archive_commands_test.rs`
