# Implement multiline cell support and column size limits

## Description

Currently, tree_fmt v0.5.0 treats each table cell as a single-line string and lacks effective column size constraints. While `TableConfig` defines `max_column_width` and `truncation_marker` fields, they are not implemented in the table formatter. This forces users to manually truncate content before passing it to tree_fmt, reducing flexibility and increasing code duplication across projects.

This task implements two critical missing features:
1. **Multiline cell support**: Automatic text wrapping within cells based on column width limits
2. **Column size enforcement**: Proper implementation of `max_column_width` and `truncation_marker` with configurable truncation strategies

These features will eliminate the need for manual pre-processing in downstream projects like gmailer, wplan_client, and willbe, enabling tree_fmt to become a complete table formatting solution.

## Requirements

-   All work must strictly adhere to all applicable rulebooks
    (discover via `prompt .rulebooks.relevant`)
-   Maintain 100% backward compatibility with existing tree_fmt API
-   Achieve 100% test coverage for new features
-   Follow TDD methodology: write tests before implementation
-   Zero breaking changes to existing API surface
-   Performance: no more than 10% overhead for tables without new features
-   Documentation: comprehensive API docs and usage examples for both features

## Acceptance Criteria

-   **Multiline Cell Support:**
    -   `TableConfig` has new field: `enable_multiline: bool` (default: false for backward compatibility)
    -   `TableConfig` has new field: `wrap_mode: WrapMode` enum (Truncate, Wrap, WrapWithEllipsis)
    -   Cells with newlines (`\n`) render across multiple rows when `enable_multiline = true`
    -   Automatic word wrapping occurs when cell content exceeds column width
    -   Row heights automatically adjust to accommodate tallest cell in row
    -   Vertical alignment configurable: Top, Middle, Bottom (default: Top)
    -   Border characters extend vertically for multiline cells

-   **Column Size Enforcement:**
    -   `TableConfig.max_column_width` is actively enforced during rendering
    -   `TableConfig.truncation_marker` is used when truncating content (default: "...")
    -   New field: `truncation_strategy: TruncationStrategy` enum (Left, Right, Middle)
    -   Column widths respect both `min_column_width` and `max_column_width` constraints
    -   Truncation preserves ANSI color codes without breaking them mid-sequence
    -   Unicode-aware truncation respects grapheme cluster boundaries

-   **Test Coverage:**
    -   Unit tests for WrapMode::Truncate, Wrap, WrapWithEllipsis
    -   Unit tests for TruncationStrategy::Left, Right, Middle
    -   Integration tests with ANSI colors in multiline cells
    -   Integration tests with Unicode (emoji, CJK characters) in wrapped cells
    -   Property-based tests for edge cases (empty cells, very long words, mixed content)
    -   Regression tests ensuring backward compatibility (all existing tests pass)
    -   Performance benchmarks showing <10% overhead

-   **Documentation:**
    -   README examples demonstrating multiline usage
    -   README examples demonstrating truncation modes
    -   API docs for all new config fields with inline examples
    -   Migration guide for users currently doing manual truncation
    -   Performance characteristics documented

-   **Quality Gates:**
    -   All tests pass: `cargo test --all-features`
    -   Zero clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
    -   Documentation builds: `cargo doc --all-features --no-deps`
    -   Code coverage ≥95% for new code (measured with tarpaulin)
    -   Benchmarks show acceptable performance (documented in PR)
