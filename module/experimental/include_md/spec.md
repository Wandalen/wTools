# include_md

Include markdown file or its section at compile time.

## Overview

`include_md` is a placeholder crate intended to provide macros for including markdown files or specific sections of markdown files at compile time. This is useful for embedding documentation, README content, or other markdown resources directly into Rust code.

**Note**: This crate is currently a placeholder with minimal implementation.

### Scope

#### Responsibility

include_md is intended to be responsible for providing compile-time macros to include markdown content from external files into Rust source code.

#### In-Scope (Planned)

- **`include_md!` macro**: Include entire markdown file
- **Section extraction**: Include specific sections by heading
- **Compile-time processing**: File reading at build time
- **Doc comment integration**: Use markdown in doc comments

#### Out-of-Scope

- **Runtime file reading**: Compile-time only
- **Markdown rendering**: Raw text inclusion
- **File watching**: No incremental compilation support
- **Remote files**: Local files only

#### Boundaries

- **Upstream**: No dependencies (uses std file operations)
- **Downstream**: Used for documentation embedding
- **Build-time only**: Macro expansion at compile time

## Architecture

### Module Structure

```
include_md/
├── src/
│   └── _blank/             # Placeholder implementation
│       └── standard_lib.rs
├── Cargo.toml
├── readme.md
└── spec.md
```

## Public API (Planned)

### Macros

#### `include_md!`

Include entire markdown file.

**Path Resolution**: Paths are resolved relative to the source file containing the macro invocation (same behavior as `include_str!`).

**Error Handling**: Missing or unreadable files produce compile-time errors with file path and reason.

**Size Limits**: Files larger than 10MB will produce compile-time error to prevent excessive compile times.

```rust
use include_md::include_md;

// Include entire README (relative to this source file)
const README: &str = include_md!("../../readme.md");

// Use in doc comment
#[doc = include_md!("docs/usage.md")]
pub struct MyType;
```

#### `include_md_section!`

Include specific section by heading.

**Heading Matching**:
- Case-sensitive exact match required (e.g., `"## Usage"` does NOT match `"## usage"`)
- Includes all content until next heading of same or higher level
- Includes nested subsections within extracted section
- Heading level determined by leading `#` count (e.g., `##` is level 2)

**Duplicate Headings**: If multiple headings match, extracts first occurrence only.

**Missing Headings**: Compile-time error if specified heading not found in file.

**Path Resolution**: Same as `include_md!` - relative to source file.

**Error Handling**: Same as `include_md!` - compile-time errors for issues.

```rust
use include_md::include_md_section;

// Include only "Usage" section from README (case-sensitive)
const USAGE: &str = include_md_section!("readme.md", "## Usage");

// Includes content until next ## or # heading
// Includes any ### subsections within the Usage section
```

## Implementation Details

### Path Resolution

All file paths are resolved relative to the source file containing the macro invocation, consistent with Rust's `include_str!` and `include_bytes!` behavior.

Example:
```rust
// In src/lib.rs
const DOC: &str = include_md!("../docs/api.md");
// Resolves to: <project_root>/docs/api.md
```

### Error Handling

All errors occur at compile time via procedural macro expansion:
- **File not found**: Clear error message with attempted path
- **File unreadable**: Error with file path and permission issue
- **File too large**: Error with size limit (10MB)
- **Heading not found** (for `include_md_section!`): Error with heading string
- **Invalid UTF-8**: Error indicating encoding issue

No panics or `Result` types - macros follow Rust convention of compile-time errors.

### Size Limits

**Maximum File Size**: 10MB
- Larger files produce compile-time error
- Rationale: Prevent excessive compile times and memory usage
- Most markdown documentation files are <1MB

### Section Extraction Behavior

**Heading Matching**:
- **Case Sensitive**: `"## API"` does NOT match `"## api"`
- **Exact Match**: Full heading text must match (e.g., `"## Usage Examples"` vs `"## Usage"`)
- **Level Aware**: `##` matches only level-2 headings, not `#` or `###`

**Content Boundaries**:
- Extraction starts immediately after matched heading line
- Extraction stops at next heading of same or higher level
- Includes all nested subsections within boundary

Example:
```markdown
## API

This is the API section.

### Functions

These are functions (INCLUDED).

### Types

These are types (INCLUDED).

## Examples

This starts next section (EXCLUDED).
```

Extracting `"## API"` includes "This is the API section" plus both ### subsections, but stops before "## Examples".

**Duplicate Headings**: First occurrence extracted if multiple matches exist.

### Line Ending Handling

Line endings are preserved as-is from source file:
- CRLF (`\r\n`) remains CRLF
- LF (`\n`) remains LF
- No normalization applied

### Unicode Support

Full UTF-8 support:
- Emoji, non-ASCII characters, RTL text preserved exactly
- Invalid UTF-8 produces compile-time error

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |
| `no_std` | - | no_std compatibility |
| `use_alloc` | - | Use alloc in no_std |

## Dependencies and Consumers

### Dependencies

None - placeholder crate.

### Potential Consumers

- Crates embedding README in lib.rs
- Documentation generators
- Build scripts needing markdown content

## Design Rationale

### Why This Crate?

Standard `include_str!` works for raw inclusion, but lacks:
1. Section extraction by heading
2. Markdown-aware processing
3. Relative path resolution improvements

### Current Status

This crate is a placeholder for future development. Current functionality is minimal.

## Future Considerations

### Potential Enhancements

1. **Section extraction**: Parse markdown headings
2. **Link rewriting**: Adjust relative links
3. **Code block extraction**: Include only code examples
4. **Proc-macro implementation**: Full macro functionality

### Known Limitations

1. **Placeholder status**: Minimal implementation
2. **No tests**: Test suite disabled

## Adoption Guidelines

### When to Use

- Currently not recommended due to placeholder status
- Consider `include_str!` for basic needs

### When Implemented

- Embedding README sections in crate documentation
- Including usage examples from markdown files
- Generating documentation from external sources

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `doc-comment` | Alternative for doc tests from readme |
| `readme-sync` | Keep readme in sync with docs |
