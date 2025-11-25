# Cargo Feature Refactoring Plan

Pragmatic variant-level feature flags for tree_fmt.

## Goals

1. Enable variant-level granularity for formatters with multiple distinct variants
2. Allow minimal binary size by selecting only needed variants
3. Maintain pragmatic approach - avoid excessive fragmentation
4. Preserve backward compatibility via meta-features

## Proposed Feature Structure

### Core Philosophy

- **Variant-level features** for formatters where variants serve different use cases (Table, Html, Sql)
- **Formatter-level features** for formatters with runtime variant selection (Text, Json)
- **Single features** for single-variant formatters (Yaml, Toml, Logfmt)
- **Meta-features** for convenience (format_table, format_sql, all_formats)

## Feature Hierarchy

### TableFormatter (8 individual features)

**Rationale**: High diversity in use cases (export vs visual, ASCII vs Unicode)

```toml
table_plain = []           # Default: CLI tools (ps, top)
table_minimal = []         # No separators
table_bordered = []        # PostgreSQL-style pipes
table_markdown = []        # GitHub README tables
table_grid = []            # Full ASCII grid
table_unicode = []         # Unicode box-drawing
table_csv = []             # Data export to Excel
table_tsv = []             # Clipboard/spreadsheet paste
table_compact = []         # Narrow terminals

# Convenience meta-features
format_table_visual = ["table_plain", "table_minimal", "table_bordered", "table_markdown", "table_grid", "table_unicode", "table_compact"]
format_table_export = ["table_csv", "table_tsv"]
format_table = ["format_table_visual", "format_table_export"]
```

### ExpandedFormatter (2 individual features)

**Rationale**: Small overhead, optional granularity

```toml
expanded_postgres = []     # PostgreSQL \x mode (default)
expanded_property = []     # Property lists with colors

# Convenience meta-feature
format_expanded = ["expanded_postgres", "expanded_property"]
```

### TreeFormatter (3 individual features)

**Rationale**: Different complexity levels, users may want minimal subset

```toml
tree_hierarchical = []     # Standard tree (default)
tree_aligned = []          # Column-aligned with metadata
tree_aggregated = []       # With subtree totals

# Convenience meta-feature
format_tree = ["tree_hierarchical", "tree_aligned", "tree_aggregated"]
```

### TextFormatter (1 feature, 6 runtime variants)

**Rationale**: All variants share code, differ only in formatting logic

```toml
format_text = []           # Includes: Bullets, Numbered, Sections, KeyValue, Compact, CliHelp
```

**Note**: Variant selection via `TextVariant` enum at runtime

### JsonFormatter (1 feature, 2 runtime modes)

**Rationale**: Pretty vs Compact is runtime parameter, not separate implementations

```toml
format_json = []           # Includes: Pretty, Compact modes
```

**Note**: Mode selection via constructor parameter

### YamlFormatter, TomlFormatter, LogfmtFormatter (1 feature each)

**Rationale**: Single variant, no granularity possible

```toml
format_yaml = []           # Standard YAML
format_toml = []           # Standard TOML
format_logfmt = []         # Logfmt logging
```

### HtmlFormatter (4 individual features)

**Rationale**: Framework-specific variants may have different dependencies

```toml
html_minimal = []          # Plain HTML (default)
html_bootstrap = []        # Bootstrap 5 classes
html_tailwind = []         # Tailwind CSS classes
html_custom = []           # User-provided classes

# Convenience meta-features
format_html_basic = ["html_minimal", "html_custom"]
format_html_frameworks = ["html_bootstrap", "html_tailwind"]
format_html = ["format_html_basic", "format_html_frameworks"]
```

### SqlFormatter (4 individual features)

**Rationale**: Different SQL dialects for different databases, users need only one

```toml
sql_ansi = []              # Standard SQL (default)
sql_postgres = []          # PostgreSQL-specific syntax
sql_mysql = []             # MySQL/MariaDB syntax
sql_sqlite = []            # SQLite3 syntax

# Convenience meta-feature
format_sql = ["sql_ansi", "sql_postgres", "sql_mysql", "sql_sqlite"]
```

## Default Configuration

```toml
default = [
  "table_plain",
  "expanded_postgres",
  "tree_hierarchical",
  "format_logfmt"
]
```

**Total default formatters**: 4 formatters, 4 variants

## Meta-Features

```toml
all_formats = [
  "format_table",
  "format_expanded",
  "format_tree",
  "format_text",
  "format_json",
  "format_yaml",
  "format_toml",
  "format_logfmt",
  "format_html",
  "format_sql"
]
```

## Feature Count Summary

| Category | Granular Features | Meta-Features | Total |
|----------|------------------|---------------|-------|
| Table variants | 9 | 3 | 12 |
| Expanded variants | 2 | 1 | 3 |
| Tree variants | 3 | 1 | 4 |
| Text formatter | 1 | 0 | 1 |
| Json formatter | 1 | 0 | 1 |
| Yaml formatter | 1 | 0 | 1 |
| Toml formatter | 1 | 0 | 1 |
| Logfmt formatter | 1 | 0 | 1 |
| Html variants | 4 | 3 | 7 |
| Sql variants | 4 | 1 | 5 |
| **TOTAL** | **27** | **9** | **36** |

## Implementation Steps

### Phase 1: Cargo.toml Updates

1. **Define all granular features**
   - Add 27 individual variant/formatter features
   - Set up feature dependencies (variants depend on their formatter infrastructure)

2. **Define meta-features**
   - Add 9 convenience meta-features
   - Update `default` and `all_formats`

3. **Update dependencies**
   - Move serde_json, serde_yaml, toml, etc. to optional dependencies
   - Make each optional dependency controlled by its feature flag

### Phase 2: Code Reorganization

1. **src/formatters/table.rs**
   ```rust
   #[cfg(feature = "table_plain")]
   pub use config::TableConfig::plain;

   #[cfg(feature = "table_bordered")]
   pub use config::TableConfig::bordered;

   // ... etc for each variant
   ```

2. **src/formatters/html.rs**
   ```rust
   #[cfg(feature = "html_minimal")]
   pub const MINIMAL: HtmlVariant = HtmlVariant::Minimal;

   #[cfg(feature = "html_bootstrap")]
   pub const BOOTSTRAP: HtmlVariant = HtmlVariant::Bootstrap;

   // ... etc
   ```

3. **src/formatters/sql.rs**
   ```rust
   #[cfg(feature = "sql_ansi")]
   pub const ANSI: SqlVariant = SqlVariant::Ansi;

   // ... etc
   ```

4. **src/lib.rs**
   ```rust
   #[cfg(any(
     feature = "table_plain",
     feature = "table_minimal",
     // ... all table features
   ))]
   pub mod table;

   #[cfg(any(
     feature = "html_minimal",
     // ... all html features
   ))]
   pub mod html;
   ```

### Phase 3: Documentation Updates

1. **Update readme.md**
   - Add Feature Flag column to Table of Variants (already done)
   - Update Cargo Features section with new structure
   - Add usage examples for minimal configurations

2. **Update spec.md**
   - Document feature flag architecture
   - Add feature selection guide
   - Update version history

3. **Update variant descriptors** (docs/variant/*.md)
   - Update feature_flag attribute for each variant

4. **Create feature selection guide** (docs/feature_selection.md)
   - Decision tree for choosing features
   - Common use case examples
   - Binary size comparison table

### Phase 4: Testing Infrastructure

1. **Update CI configuration**
   - Test minimal feature set (default only)
   - Test each granular feature individually
   - Test common feature combinations
   - Test all_formats

2. **Add feature combination tests**
   ```bash
   # Minimal
   cargo test --no-default-features --features table_plain

   # Export only
   cargo test --no-default-features --features "table_csv,table_tsv,format_json"

   # Visual only
   cargo test --no-default-features --features format_table_visual

   # All
   cargo test --all-features
   ```

### Phase 5: Migration Guide

1. **Document breaking changes** (none expected)

2. **Provide migration examples**
   ```toml
   # Old (still works via meta-features)
   tree_fmt = { version = "0.4.0", features = ["format_json"] }

   # New (more granular)
   tree_fmt = { version = "0.5.0", features = ["table_csv", "format_json"] }

   # Minimal (smallest binary)
   tree_fmt = { version = "0.5.0", default-features = false, features = ["table_plain"] }
   ```

## Use Case Examples

### Minimal CLI Tool (just plain table)

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = ["table_plain"] }
```

### Data Export Pipeline

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = ["table_csv", "table_tsv", "format_json"] }
```

### Documentation Generator

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = ["table_markdown", "html_minimal"] }
```

### Database Tool (PostgreSQL)

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = ["table_bordered", "sql_postgres"] }
```

### Full-Featured Application

```toml
tree_fmt = { version = "0.5.0", features = ["all_formats"] }
```

## Binary Size Impact

Estimated size reduction from granular features (approximate):

| Configuration | Estimated Binary Size | Reduction |
|---------------|----------------------|-----------|
| all_formats | ~500 KB | baseline |
| default only | ~200 KB | 60% smaller |
| table_plain only | ~100 KB | 80% smaller |
| table_csv + format_json | ~150 KB | 70% smaller |

**Note**: Actual sizes depend on link-time optimization and target platform

## Backward Compatibility

All existing feature names remain valid via meta-features:

- `format_json` → still enables JsonFormatter
- `format_html` → enables all HtmlFormatter variants
- `format_sql` → enables all SqlFormatter variants
- `all_formats` → enables everything

**Migration impact**: NONE for existing users

## Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Feature flag explosion | Medium | Use meta-features for convenience |
| CI complexity | Low | Add feature matrix testing |
| Documentation overhead | Low | Auto-generate feature tables |
| User confusion | Medium | Clear feature selection guide |

## Timeline Estimate

| Phase | Effort | Duration |
|-------|--------|----------|
| Phase 1: Cargo.toml | Low | 1 hour |
| Phase 2: Code refactoring | Medium | 3 hours |
| Phase 3: Documentation | Medium | 2 hours |
| Phase 4: Testing | Medium | 2 hours |
| Phase 5: Migration guide | Low | 1 hour |
| **TOTAL** | | **9 hours** |

## Decision: Proceed?

**Recommendation**: YES

**Benefits**:
- Users can minimize binary size by 60-80%
- Clear feature boundaries align with use cases
- No breaking changes for existing users
- Maintains pragmatic balance (27 features, not 100)

**Concerns**:
- Slightly more complex Cargo.toml
- More CI test combinations

**Next Step**: Review plan, then implement Phase 1
