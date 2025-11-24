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

```rust
use include_md::include_md;

// Include entire README
const README: &str = include_md!("../../readme.md");

// Use in doc comment
#[doc = include_md!("docs/usage.md")]
pub struct MyType;
```

#### `include_md_section!`

Include specific section by heading.

```rust
use include_md::include_md_section;

// Include only "Usage" section from README
const USAGE: &str = include_md_section!("readme.md", "## Usage");
```

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
