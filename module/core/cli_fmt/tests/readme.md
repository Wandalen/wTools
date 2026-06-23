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
| `output_passthrough.rs` | Validate FT-41 passthrough path under `output_passthrough` feature (no `string_split`) |
| `help.rs` | Validate CliHelpTemplate rendering, CliHelpStyle defaults, OptionGroup, and CliHelpData::default() |
| `docs/` | Test specification documents for doc entity instances |

## Organization Principles

- **Flat Structure**: Single test file until >20 test files, then one-level domain nesting
- **Domain-Based**: Tests organized by functional domain (WHAT tested), not methodology (HOW tested)
- **One Aspect Per Test**: Each test validates single specific aspect of functionality
- **Explicit Parameters**: No fragile tests relying on default values
- **Bug Documentation**: Comprehensive 5-section documentation for bug reproducer tests

## Test Coverage

`output.rs` (59 tests):
- **OutputConfig Tests**: Default configuration, has_processing detection, builder pattern, is_default discriminant tests (stream_filter, width_suffix, unicode_aware, tail, width), new() constructor alias
- **Stream Selection Tests**: stdout-only, stderr-only, both streams, both-with-empty-stdout, both-with-empty-stderr, stderr-before-stdout ordering
- **Head Tests**: Truncate to N lines, exceeds total, exact count, empty input
- **Tail Tests**: Last N lines, exceeds total, exact count, empty input
- **Combined Head+Tail Tests**: No-overlap filtering, overlapping windows (all retained), exact-fit boundary
- **Width Tests**: No truncation needed, truncation with arrow suffix, custom suffix, zero width handling, empty suffix (no truncation marker), ANSI preservation with and without truncation, exact boundary (`len == max_width`)
- **Integration Tests**: Combined operations testing, combined both-streams+head+width, `lines_omitted` correctness via `process_output`
- **Stream Merge Edge Cases**: stderr trailing newline (no double-newline separator), both streams trailing newlines (no double-newline), stdout-trailing-newline with separator, merge_streams Stdout-only direct call (AP-14), merge_streams Stderr-only direct call (AP-15)
- **Combined Limit Tests** (FT-36..FT-44): stdout-filter+head, stderr-filter+head (FT-42 — symmetric counterpart), head+tail+width triple combination, empty stdout+non-empty stderr+head, width=0 disables truncation when head is active, unicode_aware=false char-not-byte counting (FT-43), line exactly 1 over max_width (FT-44)

`help.rs` (34 tests):
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
- **T-B09** `examples` entries render in declaration order (first declared appears at lower offset)
- **T-B10** tagline appears after `Usage:` line, separated by a blank line (`"\n\n"`)
- **T-B11** `col_gap=4` produces 4 spaces between padded name column and description (FT-31)
- **T-B12** `cmd_indent=2` produces 2-space leading indent instead of default 4 (FT-32)

Compile_fail doc test (in `src/help.rs`):
- **T-A08** Exhaustive external `CliHelpData` struct literal rejected by `#[non_exhaustive]`

Total: 93 integration tests + 6 doc tests = 99 tests

`output_passthrough.rs` (1 test — not in standard suite; run with `cargo nextest run --test output_passthrough --no-default-features --features output_passthrough`):
- `feature_flag_line_filtering_passthrough` (FT-41) — verifies `apply_line_filtering` passthrough branch returns content unchanged with `lines_omitted == 0` when compiled without `string_split`

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
- Test matrix: See `output.rs` file header (`## Test Matrix` section)
- Bug reproducer documentation: BUG-005 see `width_exact_boundary` in `output.rs`; BUG-006 see `merge_streams_ordering` in `output.rs`; BUG-007 see `test_example_desc_rendered` in `help.rs`
- Help test matrix: See `help.rs` file header (`## Test Matrix` section)
