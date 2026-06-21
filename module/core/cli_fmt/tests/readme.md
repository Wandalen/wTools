# Tests

This directory contains all automated tests for the cli_fmt crate.

## Scope

**Responsibilities:**
Validates CLI output processing and help template rendering. Tests are organized by functional domain, with comprehensive coverage of OutputConfig behavior, process_output integration, ANSI preservation, and CliHelpTemplate column alignment and section omission. Test spec documents in `docs/` map each doc entity instance to its test cases.

**In Scope:**
- Unit tests for OutputConfig builder pattern and configuration
- Integration tests for process_output function with various configurations
- Stream selection validation (stdout, stderr, both)
- Head/tail line filtering tests with edge cases
- ANSI-aware width truncation tests
- Bug reproducer tests (e.g., stderr ordering, width boundary detection)
- CliHelpTemplate rendering tests (column alignment, ANSI suppression, section omission, desc annotation, usage_lines, arguments, option_groups, per-group padding)
- Test spec documents mapping doc entity instances to test cases (`docs/`)

**Out of Scope:**
- Performance benchmarks (would belong in benches/ directory if created)
- Manual testing procedures (none required for this crate currently)
- ANSI escape code generation tests (belongs in strs_tools)
- General string manipulation tests (belongs in strs_tools)

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Document test organization and navigation |
| `output.rs` | Validate CLI output processing behavior |
| `help.rs` | Validate CliHelpTemplate rendering, CliHelpStyle defaults, OptionGroup, and CliHelpData::default() |
| `docs/` | Test specification documents for doc entity instances |

## Organization Principles

- **Flat Structure**: Single test file until >20 test files, then one-level domain nesting
- **Domain-Based**: Tests organized by functional domain (WHAT tested), not methodology (HOW tested)
- **One Aspect Per Test**: Each test validates single specific aspect of functionality
- **Explicit Parameters**: No fragile tests relying on default values
- **Bug Documentation**: Comprehensive 5-section documentation for bug reproducer tests

## Test Coverage

`output.rs` (46 tests):
- **OutputConfig Tests**: Default configuration, has_processing detection, builder pattern, is_default discriminant tests (stream_filter, width_suffix, unicode_aware, tail, width)
- **Stream Selection Tests**: stdout-only, stderr-only, both streams, both-with-empty-stdout, both-with-empty-stderr, stderr-before-stdout ordering
- **Head Tests**: Truncate to N lines, exceeds total, exact count, empty input
- **Tail Tests**: Last N lines, exceeds total, exact count, empty input
- **Combined Head+Tail Tests**: No-overlap filtering, overlapping windows (all retained), exact-fit boundary
- **Width Tests**: No truncation needed, truncation with arrow suffix, custom suffix, zero width handling, ANSI preservation with and without truncation, exact boundary (`len == max_width`)
- **Integration Tests**: Combined operations testing, combined both-streams+head+width, `lines_omitted` correctness via `process_output`
- **Stream Merge Edge Cases**: both streams with trailing newlines (no double-newline); stdout-trailing-newline with separator

`help.rs` (30 tests):
- **T01** Column alignment: cmd/opt names padded to configured widths, no ANSI in no-TTY mode
- **T02** No ANSI codes: `tty_detect=false` suppresses all escape sequences
- **T03** Explicit `tty_detect=false`: equivalent behavior to T01
- **T04** Name not truncated: `cmd_name_width` is minimum padding, not a hard cap
- **T05** No Options section: omitted when `options` vec is empty
- **T06** No Examples section: omitted when `examples` vec is empty
- **T07** Single group: `Usage: {binary}` and `Commands:` headers appear; group and command name appear; no ANSI
- **T08** `CliHelpStyle::default()` layout field values match reference implementation
- **T09** `ExampleEntry.desc` rendered: `Some` appends `# text`; `None` emits no `#` (bug reproducer)
- **T10** `CliHelpStyle::default()` color fields and `tty_detect` match API contract values
- **T11** Empty groups vec: render succeeds without panic; binary name and tagline appear
- **T12** `opt_name_width` is minimum padding, not a hard cap
- **T13** `CliHelpStyle::default()` (tty_detect=true) in non-TTY process: no ANSI codes in output
- **T14** `Cargo.toml` does not contain `"data_fmt"` — AC-4 dependency-absence regression guard
- **T-A01** `usage_lines` non-empty renders each line indented; empty `usage_lines` falls back to default header
- **T-A02** `arguments` non-empty renders Arguments section with column padding; empty omits section
- **T-A03** `option_groups` renders each group as `"{name}:"` header with padded entries
- **T-A04** `option_groups` empty + `options` non-empty renders legacy `"Options:"` section (backward compat)
- **T-A05** `option_groups` non-empty suppresses `options` field and `"Options:"` header
- **T-A06** Each `OptionGroup` computes column padding independently from its own entries only
- **T-A07** `CliHelpData::default()` constructs without panic; all Vec fields empty
- **T-A09** `examples/basic_usage.rs` compiles with `CliHelpData::default()` + field assignment under `#[non_exhaustive]` (`cargo test --examples`)
- **T-B01** 3 custom `usage_lines` all render indented; default `Usage:` header absent
- **T-B02** 2 arguments with different name lengths: shorter padded to longest
- **T-B03** `CommandGroup` with empty `entries` vec: group header appears, no commands
- **T-B04** Render with entirely empty `CliHelpData`: no panic; output contains `Usage:` and `Commands:`
- **T-B05** `ExampleEntry.desc=Some("")`: renders `# ` marker (annotation present even with empty text)
- **T-B06** `OptionGroup` with empty `entries` vec is silently skipped — no group header emitted
- **T-B07** `option_groups` with empty-entry group + non-empty `options`: options are suppressed (footgun)
- **T-B08** `Arguments:` section appears before command group entries in output when both set

Compile_fail doc test (in `src/help.rs`):
- **T-A08** Exhaustive external `CliHelpData` struct literal rejected by `#[non_exhaustive]`

Total: 76 integration tests + 6 doc tests = 82 tests

## Test Execution

```bash
# Run all tests
cargo test

# Run with level 3 verification (recommended)
w3 .test l::3
# OR
ctest3

# Run specific test file
cargo test --test output
```

## Navigation

- CLI output processing tests: `output.rs`
- CLI help template tests: `help.rs`
- Test spec documents (doc entity → test case mapping): `docs/`
- Test matrix and bug documentation: See `output.rs` file header (lines 5-79)
- Bug reproducer documentation: See `output.rs` lines 5-35 (width truncation boundary detection)
- Help test matrix: See `help.rs` file header (lines 8-34)
