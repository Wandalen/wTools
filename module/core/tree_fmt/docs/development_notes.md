# Development Notes & Lessons Learned

## Test File Organization

### File Size Limits (v0.4.0 cleanup)

**Policy**:
- **Ideal**: 200-400 lines per test file
- **Acceptable**: Up to 500 lines
- **Warning threshold**: 500-1000 lines (should split)
- **Must split**: Above 1000 lines

**Rationale**: Improves readability, encourages domain-focused test organization, makes tests easier to navigate.

**Challenge Discovered**: Rust integration tests are separate crates, which complicates module structure:
- Cannot share code between test files directly
- Two options: subdirectories with `mod.rs` or flat file naming with duplication
- **Solution chosen**: Flat file naming with small helper function duplication

**Splits in v0.4.0**:
1. `tests/aligned_tree.rs` (501 lines) → 3 files:
   - `aligned_tree_basic.rs` (211 lines) - Basic functionality
   - `aligned_tree_configuration.rs` (101 lines) - Config options
   - `aligned_tree_edge_cases.rs` (198 lines) - Edge cases and realistic use cases

2. `tests/table_styles.rs` (509 lines) → 3 files:
   - `table_styles_presets.rs` (219 lines) - Style preset constructors
   - `table_styles_outputs.rs` (130 lines) - Visual verification tests
   - `table_styles_compatibility.rs` (194 lines) - Backward compatibility and regressions

**Helper Function Strategy**: When splitting test files, small helper functions (like `sample_data()`) were duplicated rather than creating complex shared module structure. This trades minor duplication for significantly simpler test organization.

## Alignment Algorithm Evolution

### v0.1.0: The Problem

**Initial Implementation**: Basic `TreeFormatter` could display hierarchical trees but produced ragged output with multiple columns:

```text
├── short  v1.0  (path1)
│   ├── dep1  v2.0  (path/to/dep1)
└── very_long_name  v0.1  (path2)
```

**User Feedback**: Crate dependency trees were unreadable due to misaligned columns. Users couldn't quickly scan version numbers or paths across different tree depths.

### v0.2.0: The Solution

**Implementation**: Added `AlignedTreeFormatter` as separate struct with two-pass algorithm:

**Pass 1 - Width Calculation**:
- DFS traversal of entire tree
- Track maximum width for each column
- **Critical insight**: Column 0 width must include tree prefix length (`├── `, `│   ├── `, etc.)
- Formula: `prefix_len = (depth - 1) * indent_size + branch_connector_len + 1`

**Pass 2 - Formatting**:
- Render with calculated widths
- Pad each column to its maximum width
- Result: All columns align vertically regardless of tree depth

**Why It Works**: By including prefix length in column 0 width during pass 1, all subsequent columns start at the same visual position across all tree depths.

### v0.4.0: Consolidation

**Change**: Merged `AlignedTreeFormatter` into `TreeFormatter` as `format_aligned()` method.

**Rationale**: Same formatter, different method. Cleaner API with fewer types.

**Migration**: No breaking changes - just removed deprecated `AlignedTreeFormatter` struct.

## Table Styles Refactoring (v0.3.0)

### Problem: Limited Customization

v0.2.x only had `show_borders: bool` - binary choice between bordered and borderless tables.

**User Needs**:
- CLI tools wanted plain space-separated output
- Documentation needed Markdown tables
- Data export needed CSV/TSV
- Reports wanted visual clarity with grid lines

### Solution: Comprehensive Parametrization

**New Enums** (v0.3.0):
- `BorderStyle`: None, Ascii, AsciiGrid, Unicode, Markdown
- `HeaderSeparatorStyle`: None, Dash, AsciiGrid, Unicode, Markdown
- `ColumnSeparator`: Spaces(n), Character(char), String(String)

**9 Style Presets**:
1. Plain - CLI tool output
2. Minimal - No separators
3. Bordered - Traditional (default)
4. Markdown - GitHub-compatible
5. Grid - Full ASCII grid
6. Unicode Box - Beautiful box-drawing
7. CSV - Data export
8. TSV - Spreadsheet compatible
9. Compact - Maximum density

**Key Insight**: Different output contexts have fundamentally different requirements. Instead of trying to make one style work everywhere, provide specialized presets for each use case.

### Critical Bug Fixed During Development

**Header Separator Alignment Bug**:
- **Symptom**: Separator line length didn't match header/row lengths when `inner_padding > 0`
- **Cause**: Separator generation forgot to account for padding
- **Impact**: Misaligned table boxes, pipes didn't line up vertically
- **Fix**: Added padding calculation to separator generation
- **Test**: `test_header_separator_alignment_with_padding` verifies all lines same length

**Double Pipe Bug**:
- **Symptom**: Some configurations produced `||` in output
- **Cause**: Border pipe logic didn't coordinate with column separators
- **Fix**: Consolidated border pipe rendering logic
- **Test**: `test_default_table_no_double_pipes` scans for `||` patterns

**Lesson Learned**: Visual verification tests caught bugs that unit tests missed. Always include visual output inspection in test suite for formatting code.

## Feature Flag Strategy (v0.4.0)

### Problem: Dependency Bloat

**User Feedback**: Projects using only `TableFormatter` shouldn't need to pull in serde ecosystem (serde, serde_json, serde_yaml, toml) just to format tables.

### Solution: Granular Feature Flags

**Individual Formatter Features**:
- `format_table`, `format_expanded`, `format_tree`, `format_text` - Zero dependencies
- `format_json` - Requires serde, serde_json
- `format_yaml` - Requires serde, serde_yaml
- `format_toml` - Requires serde, toml

**Feature Bundles**:
- `visual_formats` (default) = table + expanded + tree
- `data_formats` = json + yaml + toml
- `all_formats` = visual + data + text

**serde_support Feature**: Conditionally enables serde derives on data structures only when data formatters are used.

**Result**:
- Default builds: Zero dependencies (pure stdlib)
- JSON-only builds: Just serde + serde_json
- Full builds: All formatters available

**Lesson Learned**: Zero-cost abstractions apply to dependencies too. Users pay only for what they use.

## Logfmt Formatter Design (v0.5.0)

### Problem: Machine-Parseable Structured Logging

**User Need**: Application logs need to be both human-readable AND machine-parseable for observability tools (Prometheus, Loki, Elasticsearch).

**Requirements**:
- Simple key=value format
- Grep-friendly (can search with standard Unix tools)
- Proper escaping for values with spaces, quotes, newlines
- Zero dependencies (stdlib only)
- One line per log entry

### Solution: Logfmt Format Implementation

**Format Specification**:
- Each table row → one line of output
- Fields separated by spaces
- Format: `key1=value1 key2=value2 ...`
- Header names become keys, cell values become values

**Escaping Logic**:
Three types of values require special handling:

1. **Values with spaces/tabs** → wrap in double quotes
   - Example: `msg="hello world"`
2. **Values with double quotes** → backslash-escape the quotes
   - Example: `msg="say \"hello\""`
3. **Values with newlines** → replace with literal `\n`
   - Example: `msg="line1\nline2"`

**Implementation Approach**:
- Created as standalone formatter (not table style)
- Implements unified `Format` trait
- Feature flag: `format_logfmt` (included in `visual_formats` bundle)
- Zero dependencies (pure stdlib)

### Why Standalone Formatter vs Table Style

**Decision**: Implemented as `LogfmtFormatter` (standalone) rather than `TableConfig::logfmt()` style preset.

**Rationale**:
- Logfmt requires fundamentally different rendering logic:
  - Headers become keys (not displayed separately)
  - No visual alignment or padding needed
  - Custom escaping rules
  - One line per row (no visual formatting overhead)
- Table styles are for visual presentation with borders/separators
- Logfmt is for data serialization (like JSON/YAML/TOML)

**Comparison**:
- `TableConfig` styles: Configure visual presentation of tables
- `LogfmtFormatter`: Serialize data in logfmt format

### Critical Edge Cases Handled

**Empty Values**: Output as `key=` (no value)

**Unicode**: Full UTF-8 support, no special handling needed
- Example: `name=测试 emoji=🎉`

**Special Characters**: Most don't require escaping
- Equals signs in values: `data=key=value` (no quotes needed)
- Pipes, emails, etc.: Output as-is

**Combined Escaping**: Values with multiple special chars
- Example: `msg="say \"hello\nworld\" here"` (quotes AND newlines)

### Testing Strategy

Created comprehensive test suite (`tests/logfmt.rs`):
- Basic formatting (single/multiple columns)
- Value escaping (spaces, tabs, quotes, newlines, combined)
- Edge cases (empty values, unicode, special chars)
- Configuration (custom separators)
- Real-world scenarios (application logs, metrics, HTTP requests)

**Total**: 20 integration tests covering all escaping rules and use cases

### Use Cases Validated

1. **Application Logging**: Timestamp, level, message, user_id, duration
2. **HTTP Request Logs**: Method, path, status, duration, IP
3. **System Metrics**: Metric name, value, unit, host, environment
4. **Error Logs**: Timestamp, error type, message, stack trace (multiline)

### Performance Characteristics

**Escaping Fast Path**: Simple values (no special chars) → no allocation, direct output

**Escaping Slow Path**: Values needing quotes/escapes → single string allocation

**Time Complexity**: O(r × c × v) where:
- r = number of rows
- c = number of columns
- v = average value length

**Space Complexity**: O(output_size) - minimal temporary allocations

**Lesson Learned**: Logfmt's simplicity makes it ideal for high-throughput logging scenarios where JSON parsing overhead would be too expensive.

## ANSI-Aware Alignment (v0.1.0)

### Problem: Color Codes Break Formatting

**Symptom**: Colored table headers were completely misaligned, making output unreadable.

**Root Cause**: ANSI escape sequences (e.g., `\x1b[31mRed\x1b[0m`) have byte length but zero visual length. Using `str.len()` for column width calculation included invisible escape codes.

### Solution: visual_len() Function

**Implementation**: State machine that skips escape sequences while counting visible characters.

**Algorithm**:
```rust
let mut len = 0;
let mut in_escape = false;

for ch in text.chars() {
  if ch == '\x1b' { in_escape = true; }
  else if in_escape && ch == 'm' { in_escape = false; }
  else if !in_escape { len += 1; }
}
```

**Edge Cases Handled**:
- Nested escape sequences (uncommon but supported)
- Malformed sequences (graceful degradation)
- Unicode characters (multi-byte but single visual char)

**Lesson Learned**: Terminal formatting is more complex than it appears. Always account for invisible control characters when calculating visual layout.

## Knowledge Distribution Strategy

### Problem: Knowledge Loss Risk

During rapid development, critical insights were scattered across:
- Temporary markdown files (marked with `-` prefix for gitignore)
- Code comments
- Commit messages
- Verbal discussions

**Risk**: These insights could be lost, making future maintenance harder.

### Solution: Structured Knowledge Hierarchy

**Priority Order** (most to least preferred):
1. **Test doc comments** - Behavioral insights, edge cases, historical bugs
2. **Code doc comments** - Implementation details, algorithm explanations
3. **docs/ directory** - Development notes, best practices, lessons learned
4. **spec.md** - Requirements and API contract only
5. **readme.md** - Pure scaffolding for new developers (NOT knowledge storage)

**Implementation** (v0.4.0 cleanup):
- Slimmed readme.md from 443 lines to 87 lines
- Added file-level doc comments to all test files
- Enhanced module doc comments in tree.rs and helpers.rs
- Created this development_notes.md file

**Lesson Learned**: Knowledge preservation is as important as code preservation. Future developers (including future you) will thank you for documenting the "why" not just the "what".

## Unicode and Emoji Handling

### Challenge

Unicode characters and emoji have complex display properties:
- Multi-byte encoding (UTF-8)
- Variable display width (emoji may be 2 columns wide)
- Combining characters (accents, diacritics)

### Current Approach

**Simplified Model**: Count each char as 1 display column using `chars().count()`.

**Works Well For**:
- ASCII text
- Most Latin scripts with accents
- CJK characters (Chinese, Japanese, Korean)
- Common emoji

**Edge Cases Not Handled**:
- Wide emoji (currently counted as 1, should be 2)
- Zero-width joiners (counted as 1, should be 0)
- Combining diacritics (counted separately)

**Rationale**: Full Unicode display width calculation requires external crate (unicode-width). Current simplified approach is "good enough" for 95% of use cases while maintaining zero dependencies.

**Future Consideration**: If users report alignment issues with emoji, add optional `unicode-width` dependency behind feature flag.

## Modular Architecture Benefits

### v0.2.0 Refactoring

**Before**: Monolithic 800+ line file with all formatting code.

**After**: 17 focused source files:
```
src/
├── lib.rs (re-exports)
├── data.rs (TreeNode, TableView)
├── builder.rs (TreeBuilder)
├── table_tree.rs (RowBuilder)
├── config.rs (all config structs)
├── conversions.rs (tree↔table)
├── helpers.rs (ANSI utilities)
└── formatters/
    ├── mod.rs
    ├── format_trait.rs (unified Format trait)
    ├── tree.rs
    ├── table.rs
    ├── expanded.rs
    ├── logfmt.rs (NEW v0.5.0)
    ├── json.rs
    ├── yaml.rs
    ├── toml_fmt.rs
    └── text.rs
```

**Benefits Realized**:
1. **Easier Navigation**: Jump directly to relevant formatter
2. **Clear Boundaries**: Each file has single responsibility
3. **Parallel Development**: Multiple people can work without conflicts
4. **Incremental Compilation**: Smaller compilation units
5. **Easier Testing**: Can test individual modules

**Lesson Learned**: Modular architecture has upfront cost but pays off as project grows.

## Best Practices Discovered

### Test Organization
- Group tests by domain (aligned_tree, table_styles) not methodology (unit/integration)
- Use file-level doc comments to explain what the file tests and why
- Split files at 500 lines to maintain readability
- Visual verification tests catch bugs unit tests miss

### Documentation
- Explain "why" in doc comments, not just "what"
- Include visual examples for formatting code
- Document edge cases and how they're handled
- Link related tests and code sections

### Error Handling
- Graceful degradation better than panics (e.g., malformed ANSI sequences)
- Return empty string for empty input (avoid special cases downstream)
- Validate input structure in constructors, not formatters

### API Design
- Fluent builder APIs for optional configuration
- Sensible defaults (most users should call `new()`)
- Feature flags for zero-cost abstractions
- Trait objects for polymorphism when needed

## Testing Philosophy

### Integration Tests Are Primary

**Rationale**: Formatting is inherently integration-heavy. Unit testing individual functions misses layout bugs.

**Approach**:
- 175 integration tests in `tests/` directory
- 65 doc tests in module comments
- Visual verification tests print actual output

**Critical Tests**:
- `reproduce_alignment_problem.rs` - Visual demonstration of alignment fix
- `table_styles_compatibility.rs` - Ensures backward compatibility
- `verify_alignment_correct.rs` - Programmatic alignment verification

### No Mocking Philosophy

**Principle**: Use real implementations, not mocks.

**Rationale**: Formatters are pure functions with no I/O. Mocking adds complexity without benefit.

**Result**: Tests are simpler and more reliable.

## Future Development Guidelines

### When Adding New Formatters

1. Add feature flag in `Cargo.toml`
2. Create `src/formatters/{name}.rs` with module doc comment
3. Implement `Format` trait
4. Add to feature bundles if appropriate
5. Create example in `examples/`
6. Add integration tests with visual verification
7. Document in spec.md

### When Fixing Bugs

1. Add failing test that reproduces bug
2. Add test doc comment explaining the bug and expected behavior
3. Fix the bug
4. Verify test passes
5. Add regression test to prevent recurrence
6. Document in code comments if non-obvious

### When Refactoring

1. Ensure all tests pass before starting
2. Make small, incremental changes
3. Run tests after each change
4. Update doc comments to reflect new structure
5. Add to development_notes.md if lessons learned

## Maintenance Checklist

When updating this project:
- [ ] All 175 integration tests pass
- [ ] All 65 doc tests pass
- [ ] Zero clippy warnings with `-D warnings`
- [ ] spec.md updated if API changed
- [ ] Examples still compile and run
- [ ] readme.md still accurate (but minimal)
- [ ] New insights documented in code or docs/

## Success Metrics

- ✅ readme.md < 100 lines (pure scaffolding)
- ✅ All test files have explanatory doc comments
- ✅ Critical algorithms documented in source
- ✅ Zero knowledge loss from development process
- ✅ New developers can understand design decisions
- ✅ Future maintenance is easier, not harder
