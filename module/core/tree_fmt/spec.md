# tree_fmt Specification

## Overview

Generic multi-format data visualization library with unified format interface supporting eight output formats: **Table**, **Expanded**, **Tree**, **Logfmt**, **JSON**, **YAML**, **TOML**, and **Text** (NEW in v0.4.0+). Granular feature flags enable zero-cost abstractions with minimal dependencies. Core visual formatters have zero dependencies.

## Purpose

Provide reusable formatters for displaying data in multiple formats with seamless conversion between representations. Enables displaying the same data as horizontal tables, vertical records, or hierarchical trees across wflow and other projects.

## Design Principles

1. **Single Data Structure**: TreeNode<T> for all data (hierarchical and tabular)
2. **Unified Format Interface**: Same API for all formatters via Format trait (NEW v0.4.0)
3. **Canonical Data Format**: TableView struct for format-agnostic code (NEW v0.4.0)
4. **Granular Features**: Zero-cost abstractions with optional formatters (NEW v0.4.0)
5. **Mutual Replaceability**: Any data can be displayed in any format
6. **Minimal Dependencies**: Core has zero dependencies, formatters are optional
7. **Generic**: Works with any data type via `TreeNode<T>`
8. **ANSI-Aware**: Proper alignment with color codes
9. **Flexible Output**: String return and io::Write support
10. **Helper Traits**: Ergonomic builders and traits for table-shaped trees
11. **Modular Architecture**: Separated concerns across 16 source modules

## Display Formats

### Table Format (Horizontal Tabular)

Standard row-and-column table layout.

```text
 sid | sname | gap
-----+-------+-----
   3 | Alice |   5
   6 | Joe   |   1
  10 | Boris |   5
```

**Characteristics:**
- Horizontal layout
- Column headers with separator line
- Fixed-width columns with alignment
- Optional borders
- Compact representation for many rows

**Use cases:** Database results, comparison tables, spreadsheet-like data

### Table Styles (NEW in v0.3.0)

The `TableFormatter` supports 9 distinct table styles through comprehensive configuration options. Each style is optimized for specific use cases and output formats.

#### 1. Plain Style (Process Monitoring)

Clean space-separated columns with dash separator. Ideal for CLI tools output.

```text
 COUNT  MEMORY  NAMES  COMMANDS                        PATH
 -----  ------  -----  ------------------------------  ----
    45   12.4GB      3  claude,npm exec firecr,Main...  /home/user1/.nvm
    68    7.1GB      1  cargo                           /home/user1/.rustup
```

**Usage**:
```rust
let formatter = TableFormatter::with_config( TableConfig::plain() );
```

**Characteristics**:
- No vertical borders
- Dash-only header separator
- Space-separated columns (2 spaces default)
- Minimal padding
- Highly readable for terminal output

**Use cases**: Process monitors (ps, top), system utilities, log analysis, CLI tool output

#### 2. Minimal Style (No Separator)

Space-separated columns with no header separator. Maximum simplicity.

```text
 COUNT  MEMORY  NAMES  COMMANDS                        PATH
    45   12.4GB      3  claude,npm exec firecr,Main...  /home/user1/.nvm
    68    7.1GB      1  cargo                           /home/user1/.rustup
```

**Usage**:
```rust
let formatter = TableFormatter::with_config( TableConfig::minimal() );
```

**Characteristics**:
- No borders or separators
- Space-separated columns
- Cleanest possible output
- Maximum information density

**Use cases**: Simple reports, data exports, piping to other tools

#### 3. Bordered Style (Default)

Traditional pipe-separated table with full borders. Current default behavior.

```text
 COUNT  | MEMORY  | NAMES  | COMMANDS                        | PATH
--------+---------+--------+---------------------------------+-----------
    45  | 12.4GB  |     3  | claude,npm exec firecr,Main...  | /home/...
    68  | 7.1GB   |     1  | cargo                           | /home/...
```

**Usage**:
```rust
let formatter = TableFormatter::new();
// or
let formatter = TableFormatter::with_config( TableConfig::bordered() );
```

**Characteristics**:
- Pipe-separated columns (` | `)
- Dash+plus separator (` --------+--------- `)
- Clear visual boundaries
- Traditional database output style

**Use cases**: Database query results, formal reports, PostgreSQL-style output

#### 4. Markdown Style

GitHub-flavored Markdown table format.

```text
| COUNT | MEMORY | NAMES | COMMANDS                        | PATH      |
|-------|--------|-------|----------------------------------|-----------|
| 45    | 12.4GB | 3     | claude,npm exec firecr,Main...  | /home/... |
| 68    | 7.1GB  | 1     | cargo                            | /home/... |
```

**Usage**:
```rust
let formatter = TableFormatter::with_config( TableConfig::markdown() );
```

**Characteristics**:
- Pipes at start and end of each line
- Dash-only separator
- Compatible with Markdown renderers
- GitHub/GitLab documentation ready

**Use cases**: README files, documentation, GitHub issues, Markdown reports

#### 5. Grid Style (ASCII Box)

Full ASCII box drawing with intersections.

```text
+-------+--------+-------+----------------------------------+-----------+
| COUNT | MEMORY | NAMES | COMMANDS                         | PATH      |
+-------+--------+-------+----------------------------------+-----------+
| 45    | 12.4GB | 3     | claude,npm exec firecr,Main...  | /home/... |
+-------+--------+-------+----------------------------------+-----------+
| 68    | 7.1GB  | 1     | cargo                            | /home/... |
+-------+--------+-------+----------------------------------+-----------+
```

**Usage**:
```rust
let formatter = TableFormatter::with_config( TableConfig::grid() );
```

**Characteristics**:
- Full box with top/bottom borders
- Plus sign intersections
- Row separators between data rows
- Maximum visual clarity

**Use cases**: Formal reports, printed output, ASCII art dashboards

#### 6. Unicode Box Style

Full Unicode box drawing characters.

```text
┌───────┬────────┬───────┬──────────────────────────────────┬───────────┐
│ COUNT │ MEMORY │ NAMES │ COMMANDS                         │ PATH      │
├───────┼────────┼───────┼──────────────────────────────────┼───────────┤
│ 45    │ 12.4GB │ 3     │ claude,npm exec firecr,Main...  │ /home/... │
├───────┼────────┼───────┼──────────────────────────────────┼───────────┤
│ 68    │ 7.1GB  │ 1     │ cargo                            │ /home/... │
└───────┴────────┴───────┴──────────────────────────────────┴───────────┘
```

**Usage**:
```rust
let formatter = TableFormatter::with_config( TableConfig::unicode_box() );
```

**Characteristics**:
- Unicode box-drawing characters (┌─┬─┐)
- Smooth, professional appearance
- Top and bottom borders
- Row separators

**Use cases**: Terminal UIs, modern CLI tools, rich console output

#### 7. CSV Style

Comma-separated values with proper quoting.

```text
COUNT,MEMORY,NAMES,COMMANDS,PATH
45,12.4GB,3,"claude,npm exec firecr,Main...",/home/user1/.nvm
68,7.1GB,1,cargo,/home/user1/.rustup
```

**Usage**:
```rust
let formatter = TableFormatter::with_config( TableConfig::csv() );
```

**Characteristics**:
- Comma-separated
- Automatic quoting of values containing commas
- Standard CSV format
- No padding or alignment

**Use cases**: Data export, Excel import, database loading, data analysis

#### 8. TSV Style

Tab-separated values for clipboard and spreadsheet compatibility.

```text
COUNT→MEMORY→NAMES→COMMANDS→PATH
45→12.4GB→3→claude,npm exec firecr,Main...→/home/user1/.nvm
68→7.1GB→1→cargo→/home/user1/.rustup
```

Note: `→` represents tab character (`\t`) in actual output.

**Usage**:
```rust
let formatter = TableFormatter::with_config( TableConfig::tsv() );
```

**Characteristics**:
- Tab-separated (`\t`)
- No quoting needed
- Excel/Google Sheets compatible
- Preserves commas in data

**Use cases**: Spreadsheet paste, clipboard data, simple data exchange

#### 9. Compact Style

Minimal spacing for maximum information density.

```text
COUNT MEMORY NAMES COMMANDS                        PATH
    45 12.4GB     3 claude,npm exec firecr,Main... /home/user1/.nvm
    68  7.1GB     1 cargo                          /home/user1/.rustup
```

**Usage**:
```rust
let formatter = TableFormatter::with_config( TableConfig::compact() );
```

**Characteristics**:
- Single space column separator
- No padding around separators
- Maximum density
- No borders or separators

**Use cases**: Space-constrained displays, embedded systems, narrow terminals

### Table Formatter Parameters

The table formatter parameters system is built on three enums that can be mixed and matched:

#### BorderVariant Enum

```rust
pub enum BorderVariant
{
  /// No borders, space-separated columns
  None,

  /// ASCII borders with pipes: | + -
  Ascii,

  /// Full ASCII grid with row separators: +---+
  AsciiGrid,

  /// Unicode box drawing: ┌─┬─┐ ├─┼─┤ └─┴─┘
  Unicode,

  /// Markdown table format: | col | col |
  Markdown,
}
```

#### HeaderSeparatorVariant Enum

```rust
pub enum HeaderSeparatorVariant
{
  /// No separator line below header
  None,

  /// Dashes only: -----
  Dash,

  /// ASCII grid separator: +-----+
  AsciiGrid,

  /// Unicode separator: ├─────┤
  Unicode,

  /// Markdown separator: |-----|
  Markdown,
}
```

#### ColumnSeparator Enum

```rust
pub enum ColumnSeparator
{
  /// N spaces between columns
  Spaces( usize ),

  /// Single character separator (|, ,, \t, etc.)
  Character( char ),

  /// Custom string separator
  String( String ),
}
```

### Advanced Table Formatter Parameters

Beyond style presets, `TableConfig` supports granular control over formatter parameters:

```rust
pub struct TableConfig
{
  // Existing fields
  pub column_widths : Vec< usize >,
  pub align_right : Vec< bool >,

  // NEW in v0.3.0
  pub border_variant : BorderVariant,
  pub header_separator_variant : HeaderSeparatorVariant,
  pub column_separator : ColumnSeparator,
  pub outer_padding : bool,
  pub inner_padding : usize,
  pub colorize_header : bool,
  pub header_color : String,
  pub alternating_rows : bool,
  pub row_color1 : String,
  pub row_color2 : String,
  pub min_column_width : usize,
  pub max_column_width : Option< usize >,
  pub truncation_marker : String,
}
```

**Builder Methods**:
```rust
impl TableConfig
{
  // Style presets (NEW)
  pub fn plain() -> Self;
  pub fn minimal() -> Self;
  pub fn bordered() -> Self;
  pub fn markdown() -> Self;
  pub fn grid() -> Self;
  pub fn unicode_box() -> Self;
  pub fn csv() -> Self;
  pub fn tsv() -> Self;
  pub fn compact() -> Self;

  // Existing builders
  pub fn column_widths( self, widths : Vec< usize > ) -> Self;
  pub fn align_right( self, align : Vec< bool > ) -> Self;

  // NEW builders
  pub fn border_variant( self, variant : BorderVariant ) -> Self;
  pub fn header_separator_variant( self, variant : HeaderSeparatorVariant ) -> Self;
  pub fn column_separator( self, sep : ColumnSeparator ) -> Self;
  pub fn outer_padding( self, enabled : bool ) -> Self;
  pub fn inner_padding( self, spaces : usize ) -> Self;
  pub fn colorize_header( self, enabled : bool ) -> Self;
  pub fn header_color( self, color : String ) -> Self;
  pub fn alternating_rows( self, enabled : bool ) -> Self;
  pub fn row_colors( self, color1 : String, color2 : String ) -> Self;
  pub fn min_column_width( self, width : usize ) -> Self;
  pub fn max_column_width( self, width : Option< usize > ) -> Self;
  pub fn truncation_marker( self, marker : String ) -> Self;
}
```

### Custom Table Styles

Mix configuration options for custom styles:

```rust
// Custom: Cyan header with bold text and alternating row colors
let formatter = TableFormatter::with_config(
  TableConfig::plain()
    .colorize_header( true )
    .header_color( "\x1b[1;36m".to_string() )  // Bold cyan
    .alternating_rows( true )
    .row_colors(
      "\x1b[0m".to_string(),        // Normal
      "\x1b[48;5;236m".to_string()  // Dark gray background
    )
);

// Custom: Narrow terminal with truncation
let formatter = TableFormatter::with_config(
  TableConfig::minimal()
    .max_column_width( Some( 30 ) )
    .truncation_marker( "…".to_string() )
);

// Custom: Double-space separated with custom separator
let formatter = TableFormatter::with_config(
  TableConfig::new()
    .border_variant( BorderVariant::None )
    .header_separator_variant( HeaderSeparatorVariant::Dash )
    .column_separator( ColumnSeparator::Spaces( 4 ) )
);
```

### Logfmt Format (Structured Logging) - NEW v0.5.0

Machine-parseable structured logging format where each table row becomes one line of space-separated `key=value` pairs. Ideal for log aggregation tools and grep-friendly logging output.

```text
timestamp=2025-01-15T10:30:00Z level=info msg="user login" user_id=12345 duration=0.043
timestamp=2025-01-15T10:30:01Z level=error msg="database timeout" user_id=67890 duration=5.234
```

**Characteristics:**
- One line per data row
- Space-separated key=value pairs
- Header names become keys, cell values become values
- Automatic value escaping (spaces → quotes, quotes → backslashes)
- No visual formatting overhead (pure data)
- Human-readable AND machine-parseable

**Usage:**
```rust
let formatter = LogfmtFormatter::new();
// or with custom separator
let formatter = LogfmtFormatter::with_separator( ":" );
```

**Escaping Rules:**
- Values containing spaces/tabs → wrapped in double quotes
- Values containing quotes → quotes escaped with backslash
- Values containing newlines → newlines replaced with `\n` literal

**Example:**
```rust
let data = RowBuilder::new( vec![ "name".into(), "status".into(), "message".into() ] )
  .add_row( vec![ "server1".into(), "ok".into(), "running".into() ] )
  .add_row( vec![ "server2".into(), "error".into(), "disk full".into() ] )
  .build();

let formatter = LogfmtFormatter::new();
let output = formatter.format( &data );
```

**Output:**
```text
name=server1 status=ok message=running
name=server2 status=error message="disk full"
```

**Use cases:**
- Structured application logging
- Observability tool ingestion (Prometheus, Loki, Elasticsearch)
- Grep-friendly log formats
- Log aggregation pipelines
- CLI tool output for parsing

**Feature Flag:** Included in default `visual_formats` bundle (zero dependencies)

### Html Format (Web Tables) - NEW v0.5.0

Semantic HTML table output with CSS theme support for web dashboards, documentation, and email reports.

```html
<table class="table table-striped">
  <thead>
    <tr><th>Name</th><th>Age</th><th>City</th></tr>
  </thead>
  <tbody>
    <tr><td>Alice</td><td>30</td><td>NYC</td></tr>
    <tr><td>Bob</td><td>25</td><td>LA</td></tr>
  </tbody>
</table>
```

**Characteristics:**
- Semantic HTML5 markup (`<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>`)
- Multiple CSS themes (Bootstrap, Tailwind, Minimal, Custom)
- Automatic HTML escaping (`<`, `>`, `&`, quotes)
- Optional inline CSS inclusion
- Optional ANSI-to-HTML color conversion
- Clean, accessible HTML output

**Usage:**
```rust
let formatter = HtmlFormatter::new();  // Minimal theme
// or with Bootstrap theme
let formatter = HtmlFormatter::with_theme( HtmlTheme::Bootstrap );
// or with custom CSS classes
let formatter = HtmlFormatter::with_table_class( "my-table" );
```

**Themes:**
- **Minimal**: Basic `<table>` with no classes (pure semantic HTML)
- **Bootstrap**: Bootstrap 5 classes (`table table-striped table-hover`)
- **Tailwind**: Tailwind CSS classes (`min-w-full divide-y divide-gray-200`)
- **Custom**: User-provided CSS class string

**HTML Escaping:**
- `<` → `&lt;`
- `>` → `&gt;`
- `&` → `&amp;`
- `"` → `&quot;`
- `'` → `&#x27;`

**Example:**
```rust
let data = RowBuilder::new( vec![ "Name".into(), "Status".into() ] )
  .add_row( vec![ "Task 1".into(), "Done".into() ] )
  .add_row( vec![ "Task <2>".into(), "Pending & active".into() ] )
  .build_view();

let formatter = HtmlFormatter::with_theme( HtmlTheme::Bootstrap );
let html = formatter.format( &data )?;
```

**Output:**
```html
<table class="table table-striped">
  <thead>
    <tr>
      <th>Name</th>
      <th>Status</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Task 1</td>
      <td>Done</td>
    </tr>
    <tr>
      <td>Task &lt;2&gt;</td>
      <td>Pending &amp; active</td>
    </tr>
  </tbody>
</table>
```

**Use cases:**
- Web dashboards and admin panels
- Static site generation
- HTML email reports
- Documentation generation (embeddable tables)
- Export data for web display

**Feature Flag:** `format_html` (zero dependencies, part of optional bundle)

### Sql Format (Database INSERT Statements) - NEW v0.5.0

Generate SQL INSERT statements for database loading from tabular data.

```sql
INSERT INTO users (name, age, city) VALUES
  ('Alice', 30, 'NYC'),
  ('Bob', 25, 'LA');
```

**Characteristics:**
- Multi-row INSERT statement generation
- SQL dialect support (PostgreSQL, MySQL, SQLite, ANSI)
- Automatic SQL escaping (quotes, special chars)
- Configurable table name
- NULL value handling
- Batch insert optimization

**Usage:**
```rust
let formatter = SqlFormatter::new( "users" );  // Table name
// or with specific dialect
let formatter = SqlFormatter::with_dialect( "users", SqlDialect::PostgreSQL );
```

**Dialects:**
- **ANSI** (default): Standard SQL compliant
- **PostgreSQL**: PostgreSQL-specific syntax
- **MySQL**: MySQL/MariaDB syntax
- **SQLite**: SQLite3 syntax

**SQL Escaping:**
- Single quotes doubled: `'` → `''`
- Backslashes escaped: `\` → `\\` (MySQL)
- NULL values: Empty strings → `NULL`
- Numbers: Unquoted

**Example:**
```rust
let data = RowBuilder::new( vec![ "name".into(), "age".into(), "email".into() ] )
  .add_row( vec![ "Alice".into(), "30".into(), "alice@example.com".into() ] )
  .add_row( vec![ "Bob O'Brien".into(), "25".into(), "bob@example.com".into() ] )
  .build_view();

let formatter = SqlFormatter::new( "customers" );
let sql = formatter.format( &data )?;
```

**Output:**
```sql
INSERT INTO customers (name, age, email) VALUES
  ('Alice', 30, 'alice@example.com'),
  ('Bob O''Brien', 25, 'bob@example.com');
```

**PostgreSQL-specific features:**
- `ON CONFLICT` clause support
- `RETURNING` clause support
- Array literal support

**MySQL-specific features:**
- Backtick quoting for identifiers
- `ON DUPLICATE KEY UPDATE` support

**Use cases:**
- Database seeding and migrations
- ETL pipelines (extract-transform-load)
- Data export for database loading
- Test data generation
- Quick INSERT statement creation

**Feature Flag:** `format_sql` (zero dependencies, part of optional `data_formats` bundle)

### Expanded Format (Vertical Records)

Each row displayed vertically as key-value pairs. Supports two styles: PostgreSQL `\x` mode and property list.

**PostgreSQL Style (default):**
```text
-[ RECORD 1 ]
sid   | 3
sname | Alice
gap   | 5
-[ RECORD 2 ]
sid   | 6
sname | Joe
gap   | 1
```

**Property List Style (with default gray keys):**
```text
Command:           sleep 10 && echo hello1
Working Directory: /
Status:            Completed
Duration:          10 seconds

Command:           sleep 5 && echo hello2
Working Directory: /home
Status:            Failed
Duration:          3 seconds
```

Note: Keys appear in gray in terminal output by default when using `property_style()`.

**Characteristics:**
- Vertical layout
- One record per block
- Key-value pairs with alignment
- Configurable record separators (or none)
- Configurable padding location (before or after separator)
- Easy reading for wide tables
- Optional ANSI color support for keys (terminal output)

**Formatter Parameters:**
- `padding_side`: Where to place alignment padding (`BeforeSeparator` or `AfterSeparator`)
- `colorize_keys`: Enable/disable key coloring (default: `false`)
- `key_color`: ANSI color code for keys (default: `\x1b[90m` gray)
- `record_separator`: Format string for section headers (empty string to disable)
- `key_value_separator`: Separator between key and value (` | ` or `: `)
- `show_record_numbers`: Show record numbers in separator (only when separator non-empty)

**Padding Styles:**
- `BeforeSeparator`: Keys padded before separator - `Name   | Value`
- `AfterSeparator`: Values padded after separator - `Name: Value`

**Use cases:** Wide tables with many columns, detailed record inspection, configuration display, command output, terminal logs, property files

### Tree Format (Hierarchical Outline)

Hierarchical tree structure with parent-child relationships.

```text
project
├── src
│   ├── main.rs (1024 bytes)
│   └── lib.rs (2048 bytes)
└── Cargo.toml (256 bytes)
```

**Characteristics:**
- Hierarchical layout
- Parent-child relationships
- Box-drawing characters
- Unlimited nesting depth
- Shows containment structure

**Use cases:** File systems, call graphs, org charts, nested categories

### Aligned Tree Format (Column-Aligned Hierarchical) - NEW v0.2.0

Hierarchical tree with multi-column attributes aligned vertically across all levels.

```text
├── api_ollama  v0.1.0  (api/ollama)
├── as_curl     v0.1.0  (module/as_curl)
│   ├── dep1  v2.0.0  (path/to/dep1)
│   └── dep2  v1.5.0  (path/to/dep2)
└── unikit      v0.1.0  (service/unikit)
```

**Characteristics:**
- Hierarchical layout with tree symbols
- Multiple columns per node (name, version, path, etc.)
- Vertical column alignment across all tree depths
- Automatic width calculation
- Configurable column separator
- Preserves tree structure (├──, │, └──)

**Use cases:** Package dependency trees, file listings with metadata, process trees with stats, any hierarchical data with aligned attributes

**Key Difference from Regular Tree**: Regular tree shows single data value per node; aligned tree shows multiple columns with vertical alignment, making it much easier to scan and compare attributes across different tree levels.

### Unified Format Interface (NEW in v0.4.0)

The Format trait provides a unified interface for all output formatters, enabling format-agnostic code with granular feature flags for minimal dependencies.

**Design Goals:**
1. **Unified Interface**: Same API for all formatters (table, json, yaml, toml, text)
2. **Canonical Data Format**: TableView struct as common interchange format
3. **Granular Features**: Each formatter behind optional feature flag
4. **Zero-Cost Abstractions**: Unused formatters compile to zero code/deps

**Core Types:**

```rust
/// Canonical data format for all formatters
pub struct TableView
{
  pub metadata : TableMetadata,
  pub rows : Vec< Vec< String > >,
}

/// Unified formatting interface
pub trait Format
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >;
}
```

**Available Formatters:**

| Formatter | Feature Flag | Dependencies | Use Case |
|-----------|-------------|--------------|----------|
| TableFormatter | `format_table` | None | Visual table output |
| ExpandedFormatter | `format_expanded` | None | Vertical records |
| TreeFormatter | `format_tree` | None | Hierarchical display |
| LogfmtFormatter | `format_logfmt` | None | Structured logging |
| HtmlFormatter | `format_html` | None | Web tables (HTML) |
| SqlFormatter | `format_sql` | None | SQL INSERT statements |
| JsonFormatter | `format_json` | serde, serde_json | Data interchange, APIs |
| YamlFormatter | `format_yaml` | serde, serde_yaml | Configuration files |
| TomlFormatter | `format_toml` | serde, toml | Rust config files |
| TextFormatter | `format_text` | None | Human-readable lists |

**Feature Bundles:**
- `visual_formats` = `format_table` + `format_expanded` + `format_tree` + `format_logfmt` (default)
- `web_formats` = `format_html` + `format_sql`
- `data_formats` = `format_json` + `format_yaml` + `format_toml`
- `all_formats` = `visual_formats` + `web_formats` + `data_formats` + `format_text`

**Usage Pattern:**

```rust
use tree_fmt::{ RowBuilder, Format };

// Build data once
let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  .add_row( vec![ "Alice".into(), "30".into() ] )
  .build_view();  // NEW: Returns TableView instead of TreeNode

// Use with different formatters through unified interface
#[ cfg( feature = "format_json" ) ]
{
  use tree_fmt::JsonFormatter;
  let json = JsonFormatter::new();
  let output = Format::format( &json, &view )?;
}

#[ cfg( feature = "format_table" ) ]
{
  use tree_fmt::TableFormatter;
  let table = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &table, &view )?;
}
```

**Migration from TreeNode:**
- Existing API unchanged: `RowBuilder::build()` still returns `TreeNode<String>`
- New API available: `RowBuilder::build_view()` returns `TableView`
- Backward compatible: `TableView::to_tree_node()` converts back to TreeNode

**Key Benefits:**
- Write format-agnostic rendering code
- Minimal dependencies with granular features
- Easy addition of new format types
- Consistent error handling across formatters

### Color Themes (Advanced Feature) - NEW v0.5.0

Predefined color schemes for visual formatters with consistent styling across Table, Expanded, and Tree formatters.

**Purpose**: Provide easy-to-use, professionally designed color schemes for terminal output without manual ANSI code management.

**Available Themes:**

1. **Dark** - High contrast for dark terminals
   - Headers: Bright cyan
   - Borders: Dim white
   - Alternating rows: Default + dark gray background
   - Tree branches: Cyan

2. **Light** - Optimized for light terminals
   - Headers: Dark blue
   - Borders: Dark gray
   - Alternating rows: White + light gray background
   - Tree branches: Blue

3. **Monokai** - Popular code editor theme
   - Headers: Bright magenta
   - Borders: Dark gray
   - Alternating rows: Black + dark gray background
   - Tree branches: Green

4. **Solarized** - Low-contrast scientific palette
   - Headers: Yellow
   - Borders: Base01
   - Alternating rows: Base03 + base02 background
   - Tree branches: Cyan

5. **Nord** - Arctic-inspired cool palette
   - Headers: Frost blue
   - Borders: Polar night
   - Alternating rows: Default + polar night background
   - Tree branches: Frost green

6. **Dracula** - Dark theme with vibrant colors
   - Headers: Purple
   - Borders: Comment gray
   - Alternating rows: Background + selection background
   - Tree branches: Pink

**Usage:**

```rust
use tree_fmt::{ RowBuilder, TableFormatter, TableConfig, ColorTheme };

let data = RowBuilder::new( vec![ "Name".into(), "Status".into() ] )
  .add_row( vec![ "Alice".into(), "Active".into() ] )
  .add_row( vec![ "Bob".into(), "Pending".into() ] )
  .build_view();

// Apply theme to table
let formatter = TableFormatter::with_config(
  TableConfig::bordered().with_theme( ColorTheme::Dark )
);

// Apply theme to expanded format
let formatter = ExpandedFormatter::with_config(
  ExpandedConfig::postgres_style().with_theme( ColorTheme::Monokai )
);

// Apply theme to tree format
let formatter = TreeFormatter::with_config(
  TreeConfig::new().with_theme( ColorTheme::Nord )
);
```

**Theme Application:**

Themes automatically configure:
- **TableConfig**: `header_color`, `alternating_rows`, `row_color1`, `row_color2`, border colors
- **ExpandedConfig**: `key_color`, record separator colors
- **TreeConfig**: Branch symbol colors, data colors

**Custom Theme Creation:**

```rust
use tree_fmt::ColorTheme;

let custom_theme = ColorTheme::custom()
  .header_color( "\x1b[38;5;208m" )  // Orange
  .border_color( "\x1b[38;5;240m" )  // Gray
  .row_color1( "\x1b[0m" )           // Default
  .row_color2( "\x1b[48;5;235m" )    // Dark gray bg
  .build();

let formatter = TableFormatter::with_config(
  TableConfig::bordered().with_theme( custom_theme )
);
```

**Color Reset:**

All themes include automatic color reset (`\x1b[0m`) after colored elements to prevent color bleeding into subsequent output.

**Terminal Compatibility:**

- Uses standard ANSI escape codes
- 256-color support (` \x1b[38;5;Nm` format)
- Gracefully degrades in non-color terminals
- No-color mode: `ColorTheme::None` disables all colors

**Feature Flag:** `themes` (zero dependencies, optional enhancement)

**Integration:**

```rust
#[ cfg( feature = "themes" ) ]
{
  use tree_fmt::ColorTheme;
  let config = TableConfig::bordered().with_theme( ColorTheme::Dark );
}
```

**Use Cases:**
- Professional terminal UIs
- Consistent branding across CLI tools
- Accessibility (high-contrast themes)
- Developer tools with syntax-like highlighting
- Log viewers and monitoring dashboards

## Architecture

### Modular File Structure

**v0.4.0 Modular Architecture** - Separated concerns across 16 source files:

```
src/
├── lib.rs                     # Re-exports public API
├── data.rs                    # TreeNode, TableView struct, TableShapedView trait
├── builder.rs                 # TreeBuilder (hierarchical)
├── table_tree.rs              # RowBuilder (table-shaped)
├── config.rs                  # TreeConfig, TableConfig, ExpandedConfig
├── conversions.rs             # Tree↔Table conversions, FlattenConfig
├── helpers.rs                 # visual_len, pad_to_width
└── formatters/
    ├── mod.rs                 # TableShapedFormatter trait, Format trait re-export
    ├── format_trait.rs        # Format trait, FormatError (NEW v0.4.0)
    ├── tree.rs                # TreeFormatter with format() and format_aligned() methods
    ├── table.rs               # TableFormatter
    ├── expanded.rs            # ExpandedFormatter
    ├── json.rs                # JsonFormatter (NEW v0.4.0)
    ├── yaml.rs                # YamlFormatter (NEW v0.4.0)
    ├── toml_fmt.rs            # TomlFormatter (NEW v0.4.0)
    └── text.rs                # TextFormatter (NEW v0.4.0)
```

### Layer 1: Single Data Structure

**TreeNode<T>** serves both hierarchical and tabular use cases.

```rust
/// Universal data structure for trees and tables
pub struct TreeNode<T> {
  pub name: String,
  pub data: Option<T>,
  pub children: Vec<TreeNode<T>>,
}
```

**Tabular data representation:**
```
root
├── 1 (row name)
│   ├── sid: "3"
│   ├── sname: "Alice"
│   └── gap: "5"
└── 2 (row name)
    ├── sid: "6"
    ├── sname: "Joe"
    └── gap: "1"
```

**Design invariants:**
- Hierarchical trees: Directories have `data = None`, files have `data = Some(T)`
- Table-shaped trees: Root has row nodes, row nodes have column-named children
- Table validation: All row nodes have identical child structure (same column names)

### Layer 2: Table Construction Helpers

Ergonomic builders and traits for working with table-shaped trees.

```rust
/// Fluent builder for table-shaped trees
pub struct RowBuilder {
  root: TreeNode<String>,
  headers: Vec<String>,
  row_count: usize,
}

impl RowBuilder {
  pub fn new(headers: Vec<String>) -> Self;

  // Fluent API (consuming self)
  pub fn add_row(self, row: Vec<String>) -> Self;
  pub fn add_row_with_name(self, row_name: String, row: Vec<String>) -> Self;

  // Mutable API (for loops)
  pub fn add_row_mut(&mut self, row: Vec<String>);
  pub fn add_row_with_name_mut(&mut self, row_name: String, row: Vec<String>);

  pub fn build(self) -> TreeNode<String>;
}

/// Generic trait for extracting table data from trees
pub trait TableShapedView {
  fn extract_headers(&self) -> Option<Vec<String>>;
  fn is_table_shaped(&self) -> bool;
  fn to_rows(&self) -> Vec<Vec<String>>;
}

impl<T: std::fmt::Display> TableShapedView for TreeNode<T> {
  // Extract column names from first row's children
  fn extract_headers(&self) -> Option<Vec<String>>;

  // Validate all rows have identical structure
  fn is_table_shaped(&self) -> bool;

  // Extract row data as Vec<Vec<String>> (converts T to String via Display)
  fn to_rows(&self) -> Vec<Vec<String>>;
}
```

**Table construction patterns:**
- **Fluent builder**: Use `RowBuilder::new().add_row().add_row().build()`
- **Mutable builder**: Use `builder.add_row_mut()` in loops
- **Manual construction**: Use `TreeBuilder` for custom row naming
- **Generic support**: TableShapedView works with any `T: Display`

### Layer 3: Display Formatters (Format-Specific)

Three formatters all working with TreeNode<T>.

```rust
/// Horizontal table formatter (uses TableShapedView trait)
pub struct TableFormatter {
  config: TableConfig,
}

/// Vertical record formatter (uses TableShapedView trait)
pub struct ExpandedFormatter {
  config: ExpandedConfig,
}

/// Hierarchical tree formatter (native TreeNode rendering)
pub struct TreeFormatter {
  config: TreeConfig,
  symbols: TreeSymbols,
}

/// Common trait for table-shaped formatters (polymorphism)
pub trait TableShapedFormatter {
  fn format(&self, tree: &TreeNode<String>) -> String;
}

impl TableShapedFormatter for TableFormatter { ... }
impl TableShapedFormatter for ExpandedFormatter { ... }
```

**Formatter design:**
- TableFormatter/ExpandedFormatter use `TableShapedView` trait to extract headers/rows
- TreeFormatter renders TreeNode<T> directly with method-level generics
- Configuration controls display options (borders, symbols, alignment)
- All formatters support both String return and Write trait output
- `format()` returns String, `write_to()` writes to any io::Write

**Configuration builder pattern:**
All config structs have fluent builder APIs:
```rust
let config = TreeConfig::new()
  .show_branches(false)
  .max_depth(Some(3));
```

## API Contract

### Data Representation Types

```rust
/// Universal hierarchical data structure
pub struct TreeNode<T> {
  pub name: String,
  pub data: Option<T>,
  pub children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
  pub const fn new(name: String, data: Option<T>) -> Self;
}

/// Tree builder for path-based construction
pub struct TreeBuilder<T> {
  root: TreeNode<T>,
}

impl<T> TreeBuilder<T> {
  pub fn new(root_name: impl Into<String>) -> Self;
  pub fn insert(self, path: &[&str], data: T) -> Self;
  pub fn build(self) -> TreeNode<T>;
}

impl<T: Clone> TreeBuilder<T> {
  pub fn from_items<I, P>(root_name: impl Into<String>, items: I) -> Self
  where
    I: IntoIterator<Item = (P, T)>,
    P: AsRef<[String]>;
}

/// Table construction helper with fluent API
pub struct RowBuilder {
  root: TreeNode<String>,
  headers: Vec<String>,
  row_count: usize,
}

impl RowBuilder {
  pub fn new(headers: Vec<String>) -> Self;

  // Fluent API (consuming, chainable)
  #[must_use]
  pub fn add_row(self, row: Vec<String>) -> Self;
  #[must_use]
  pub fn add_row_with_name(self, row_name: String, row: Vec<String>) -> Self;

  // Mutable API (for programmatic use)
  pub fn add_row_mut(&mut self, row: Vec<String>);
  pub fn add_row_with_name_mut(&mut self, row_name: String, row: Vec<String>);

  pub fn build(self) -> TreeNode<String>;
}

/// Generic trait for working with table-shaped trees
pub trait TableShapedView {
  fn extract_headers(&self) -> Option<Vec<String>>;
  fn is_table_shaped(&self) -> bool;
  fn to_rows(&self) -> Vec<Vec<String>>;
}

impl<T: std::fmt::Display> TableShapedView for TreeNode<T> {
  // Converts T to String via Display trait
  fn extract_headers(&self) -> Option<Vec<String>>;
  fn is_table_shaped(&self) -> bool;
  fn to_rows(&self) -> Vec<Vec<String>>;
}
```

### Conversion Utilities

```rust
/// Tree-to-table flattening configuration
pub struct FlattenConfig {
  pub include_path: bool,
  pub include_name: bool,
  pub include_depth: bool,
  pub include_data: bool,
  pub column_names: Option<(String, String, String, String)>,
}

impl FlattenConfig {
  pub fn new() -> Self;

  // Fluent builder methods
  #[must_use]
  pub fn include_path(self, include: bool) -> Self;
  #[must_use]
  pub fn include_name(self, include: bool) -> Self;
  #[must_use]
  pub fn include_depth(self, include: bool) -> Self;
  #[must_use]
  pub fn include_data(self, include: bool) -> Self;
  #[must_use]
  pub fn column_names(self, path: String, name: String, depth: String, data: String) -> Self;
}

/// Flatten hierarchical tree to table-shaped tree (default columns)
pub fn flatten_to_table_tree<T: Display>(tree: &TreeNode<T>) -> TreeNode<String>;

/// Flatten with custom column selection and naming
pub fn flatten_to_table_tree_with_config<T: Display>(
  tree: &TreeNode<T>,
  config: &FlattenConfig
) -> TreeNode<String>;
```

### Formatter Parameter Types

#### BorderVariant, HeaderSeparatorVariant, ColumnSeparator (NEW v0.3.0)

```rust
/// Border rendering variant for tables
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderVariant
{
  None,       // No borders, space-separated
  Ascii,      // Pipe borders: | + -
  AsciiGrid,  // Full ASCII grid: +---+
  Unicode,    // Unicode box drawing: ┌─┬─┐
  Markdown,   // Markdown table: | col |
}

/// Header separator line variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderSeparatorVariant
{
  None,       // No separator line
  Dash,       // Dash-only: -----
  AsciiGrid,  // ASCII grid: +-----+
  Unicode,    // Unicode: ├─────┤
  Markdown,   // Markdown: |-----|
}

/// Column separator configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnSeparator
{
  Spaces( usize ),      // N spaces between columns
  Character( char ),    // Single character (|, ,, \t)
  String( String ),     // Custom string
}
```

#### TableConfig

```rust
/// Formatter parameters for table output
#[derive(Debug, Clone)]
pub struct TableConfig {
  // Legacy fields (v0.1-v0.2)
  pub show_borders: bool,          // Deprecated: use border_variant
  pub column_widths: Vec<usize>,
  pub align_right: Vec<bool>,

  // NEW in v0.3.0
  pub border_variant: BorderVariant,
  pub header_separator_variant: HeaderSeparatorVariant,
  pub column_separator: ColumnSeparator,
  pub outer_padding: bool,
  pub inner_padding: usize,
  pub colorize_header: bool,
  pub header_color: String,
  pub alternating_rows: bool,
  pub row_color1: String,
  pub row_color2: String,
  pub min_column_width: usize,
  pub max_column_width: Option<usize>,
  pub truncation_marker: String,
}

impl TableConfig {
  // Construction
  pub fn new() -> Self;

  // Style presets (NEW v0.3.0)
  pub fn plain() -> Self;
  pub fn minimal() -> Self;
  pub fn bordered() -> Self;
  pub fn markdown() -> Self;
  pub fn grid() -> Self;
  pub fn unicode_box() -> Self;
  pub fn csv() -> Self;
  pub fn tsv() -> Self;
  pub fn compact() -> Self;

  // Legacy builders (v0.1-v0.2)
  #[must_use]
  #[deprecated(note = "Use border_variant() instead")]
  pub fn show_borders(self, show: bool) -> Self;
  #[must_use]
  pub fn column_widths(self, widths: Vec<usize>) -> Self;
  #[must_use]
  pub fn align_right(self, align: Vec<bool>) -> Self;

  // NEW builders (v0.3.0)
  #[must_use]
  pub fn border_variant(self, variant: BorderVariant) -> Self;
  #[must_use]
  pub fn header_separator_variant(self, variant: HeaderSeparatorVariant) -> Self;
  #[must_use]
  pub fn column_separator(self, sep: ColumnSeparator) -> Self;
  #[must_use]
  pub fn outer_padding(self, enabled: bool) -> Self;
  #[must_use]
  pub fn inner_padding(self, spaces: usize) -> Self;
  #[must_use]
  pub fn colorize_header(self, enabled: bool) -> Self;
  #[must_use]
  pub fn header_color(self, color: String) -> Self;
  #[must_use]
  pub fn alternating_rows(self, enabled: bool) -> Self;
  #[must_use]
  pub fn row_colors(self, color1: String, color2: String) -> Self;
  #[must_use]
  pub fn min_column_width(self, width: usize) -> Self;
  #[must_use]
  pub fn max_column_width(self, width: Option<usize>) -> Self;
  #[must_use]
  pub fn truncation_marker(self, marker: String) -> Self;
}

/// Formatter parameters for expanded output
#[derive(Debug, Clone)]
pub struct ExpandedConfig {
  pub record_separator: String,
  pub key_value_separator: String,
  pub show_record_numbers: bool,
}

impl ExpandedConfig {
  pub fn new() -> Self;
  #[must_use]
  pub fn record_separator(self, separator: String) -> Self;
  #[must_use]
  pub fn key_value_separator(self, separator: String) -> Self;
  #[must_use]
  pub fn show_record_numbers(self, show: bool) -> Self;
}

/// Formatter parameters for tree output
#[derive(Debug, Clone)]
pub struct TreeConfig {
  pub show_branches: bool,
  pub show_root: bool,
  pub indent_size: usize,
  pub max_depth: Option<usize>,
  pub column_separator: String,
  pub min_column_width: usize,
}

impl TreeConfig {
  pub fn new() -> Self;
  #[must_use]
  pub fn show_branches(self, show: bool) -> Self;
  #[must_use]
  pub fn show_root(self, show: bool) -> Self;
  #[must_use]
  pub fn indent_size(self, size: usize) -> Self;
  #[must_use]
  pub fn max_depth(self, depth: Option<usize>) -> Self;
  #[must_use]
  pub fn column_separator(self, separator: String) -> Self;
  #[must_use]
  pub fn min_column_width(self, width: usize) -> Self;
}

/// Tree box-drawing symbols
#[derive(Debug, Clone)]
pub struct TreeSymbols {
  pub branch: &'static str,
  pub last_branch: &'static str,
  pub vertical: &'static str,
  pub space: &'static str,
}
```

### Formatter APIs

#### TableFormatter (Horizontal Tabular Display)

```rust
impl TableFormatter {
  // Construction
  pub fn new() -> Self;
  pub const fn with_config(config: TableConfig) -> Self;

  // Format table-shaped tree (extracts headers/rows via TableShapedView)
  pub fn format(&self, tree: &TreeNode<String>) -> String;

  // Format hierarchical tree (flattens to path/name/depth/data)
  pub fn format_tree<T: Display>(&self, tree: &TreeNode<T>) -> String;

  // Write directly to io::Write
  pub fn write_to<W: std::io::Write>(&self, tree: &TreeNode<String>, writer: &mut W) -> std::io::Result<()>;
}

impl TableShapedFormatter for TableFormatter {
  fn format(&self, tree: &TreeNode<String>) -> String;
}
```

#### ExpandedFormatter (Vertical Record Display)

```rust
impl ExpandedFormatter {
  // Construction
  pub fn new() -> Self;
  pub const fn with_config(config: ExpandedConfig) -> Self;

  // Format table-shaped tree (extracts headers/rows via TableShapedView)
  pub fn format(&self, tree: &TreeNode<String>) -> String;

  // Format hierarchical tree (flattens to path/name/depth/data)
  pub fn format_tree<T: Display>(&self, tree: &TreeNode<T>) -> String;

  // Write directly to io::Write
  pub fn write_to<W: std::io::Write>(&self, tree: &TreeNode<String>, writer: &mut W) -> std::io::Result<()>;
}

impl TableShapedFormatter for ExpandedFormatter {
  fn format(&self, tree: &TreeNode<String>) -> String;
}
```

#### TreeFormatter (Hierarchical Tree Display)

```rust
impl TreeFormatter {
  // Construction
  pub fn new() -> Self;
  pub fn with_config(config: TreeConfig) -> Self;
  pub fn with_symbols(symbols: TreeSymbols) -> Self;

  // Format tree data with custom renderer (method-level generics)
  pub fn format<T, F>(&self, tree: &TreeNode<T>, render_item: F) -> String
  where F: Fn(&T) -> String;

  // Format tree with column-aligned data (NEW merged from AlignedTreeFormatter)
  pub fn format_aligned(&self, tree: &TreeNode<ColumnData>) -> String;

  // Format tree with aggregated directory totals
  pub fn format_with_aggregation<T, V, A, F, D, C>(
    &self,
    tree: &TreeNode<T>,
    grand_total: V,
    aggregate_fn: A,
    convert_to_f64: C,
    render_file: F,
    render_directory: D,
  ) -> String
  where
    V: Copy + Add<Output = V> + Default + Sum,
    A: Fn(&T) -> V,
    C: Fn(V) -> f64,
    F: Fn(&T, V, f64) -> String,
    D: Fn(&str, V, f64) -> String;

  // Write directly to io::Write
  pub fn write_to<T, F, W>(&self, tree: &TreeNode<T>, writer: &mut W, render_item: F) -> std::io::Result<()>
  where
    F: Fn(&T) -> String,
    W: std::io::Write;
}
```

#### TableShapedFormatter Trait (Polymorphism)

```rust
/// Common trait for table-shaped formatters
pub trait TableShapedFormatter {
  fn format(&self, tree: &TreeNode<String>) -> String;
}

// Usage with trait objects
let formatters: Vec<Box<dyn TableShapedFormatter>> = vec![
  Box::new(TableFormatter::new()),
  Box::new(ExpandedFormatter::new()),
];

for formatter in formatters {
  let output = formatter.format(&tree);
}
```

### ANSI Support

```rust
// Helper functions for ANSI color code handling
pub fn visual_len(text: &str) -> usize;
pub fn pad_to_width(text: &str, target_width: usize, align_right: bool) -> String;
```

## Functional Requirements

### FR-1: Data Representation

**FR-1.1**: `TreeNode<T>` - hierarchical data with generic type support
**FR-1.2**: `RowBuilder` - fluent builder for table-shaped trees
**FR-1.3**: Support unlimited nesting depth in trees
**FR-1.4**: Validate table row lengths match header count

### FR-2: Data Conversion

**FR-2.1**: Convert hierarchical tree to table-shaped tree (flatten)
**FR-2.2**: Support generic type `T` with `Display` trait for conversions
**FR-2.3**: Preserve all data during conversion
**FR-2.4**: Customizable column selection via `FlattenConfig`

### FR-3: Table Format (Horizontal Tabular Display)

**FR-3.1**: Format table-shaped tree as horizontal table
**FR-3.2**: Format hierarchical tree as table (auto-flatten)
**FR-3.3**: Unicode borders with configurable styles
**FR-3.4**: Automatic column width calculation
**FR-3.5**: Manual column width specification
**FR-3.6**: Per-column alignment (left/right)
**FR-3.7**: Header row with separator line

### FR-4: Expanded Format (Vertical Record Display)

**FR-4.1**: Format table-shaped tree as vertical records
**FR-4.2**: Format hierarchical tree as records (auto-flatten)
**FR-4.3**: Record separators with sequential numbering
**FR-4.4**: Key-value pair display for each field
**FR-4.5**: Configurable separators and styling
**FR-4.6**: Align keys and values properly

### FR-5: Tree Format (Hierarchical Display)

**FR-5.1**: Format `TreeNode<T>` with box-drawing characters
**FR-5.2**: Format table-shaped tree as hierarchical tree
**FR-5.3**: Unicode symbols (`├──`, `└──`, `│`)
**FR-5.4**: Customizable symbols and indentation
**FR-5.5**: Closure-based data rendering
**FR-5.6**: Display file nodes with data
**FR-5.7**: Display directory nodes without data
**FR-5.8**: Aggregated subtree totals with percentages

### FR-6: ANSI Color Support

**FR-6.1**: Calculate visual length excluding ANSI codes
**FR-6.2**: Pad strings accounting for ANSI codes
**FR-6.3**: Proper alignment with colored cells (all formats)
**FR-6.4**: Support common ANSI escape sequences (`\x1b[...m`)

### FR-7: Builder Patterns

**FR-7.1**: Fluent RowBuilder API (chainable methods)
**FR-7.2**: Mutable RowBuilder API (for loops)
**FR-7.3**: Config builder pattern for all configs
**FR-7.4**: `#[must_use]` attributes on consuming methods

### FR-8: Output Flexibility

**FR-8.1**: String return from `format()` methods
**FR-8.2**: Write trait support via `write_to()` methods
**FR-8.3**: Zero-allocation output to io::Write

### FR-9: Edge Cases

**FR-9.1**: Empty tables return empty string in all formats
**FR-9.2**: Empty trees return empty string when formatted
**FR-9.3**: Single-row tables display correctly in all formats
**FR-9.4**: Generic TableShapedView works with any `T: Display`

## Implementation Details

### Data Representation Structures

**TreeNode<T>**:
```rust
pub struct TreeNode<T> {
  pub name: String,           // Node name (file/directory)
  pub data: Option<T>,        // None for directories
  pub children: Vec<TreeNode<T>>,
}
```

**Invariant**: Directories have `data = None`, files have `data = Some(T)`

**RowBuilder** (replaces legacy DataTable):
```rust
pub struct RowBuilder {
  root: TreeNode<String>,
  headers: Vec<String>,
  row_count: usize,
}
```

**Invariant**: All rows have length equal to `headers.len()`

### Data Conversion Algorithms

#### Hierarchical Tree → Table-Shaped Tree Conversion

**Algorithm** (via `flatten_to_table_tree`):
```rust
pub fn flatten_to_table_tree<T: Display>(tree: &TreeNode<T>) -> TreeNode<String> {
  // 1. Create headers: ["path", "name", "depth", "data"]
  // 2. Create RowBuilder with headers
  // 3. DFS traversal:
  //    - Track current path
  //    - Track current depth
  //    - For each node, add row: [path, name, depth, data]
  // 4. Return table-shaped tree
}
```

**Customizable via FlattenConfig**:
```rust
let config = FlattenConfig::new()
  .include_path(false)       // Exclude path column
  .include_depth(false)      // Exclude depth column
  .column_names(             // Custom column names
    "File Path".into(),
    "File Name".into(),
    "Level".into(),
    "Size".into()
  );

let flattened = flatten_to_table_tree_with_config(&tree, &config);
```

**Example (default columns)**:
```text
Tree:
project
├── src
│   └── main.rs (100)

Table-shaped tree:
root
├── 1
│   ├── path: "project"
│   ├── name: "project"
│   ├── depth: "0"
│   └── data: ""
├── 2
│   ├── path: "project/src"
│   ├── name: "src"
│   ├── depth: "1"
│   └── data: ""
└── 3
    ├── path: "project/src/main.rs"
    ├── name: "main.rs"
    ├── depth: "2"
    └── data: "100"
```

**Example (custom columns - name and data only)**:
```text
Config:
  .include_path(false)
  .include_depth(false)

Table-shaped tree:
root
├── 1
│   ├── name: "project"
│   └── data: ""
├── 2
│   ├── name: "src"
│   └── data: ""
└── 3
    ├── name: "main.rs"
    └── data: "100"
```

### Path-Based Insertion

**Algorithm**:
1. Split path into components
2. Traverse tree, creating intermediate nodes
3. Insert data at leaf node
4. Return updated builder for chaining

**Example**:
```rust
TreeBuilder::new("root")
  .insert(&["src", "main.rs"], 150)
  .insert(&["src", "lib.rs"], 200)
```

Creates:
```
root/
├── src/
    ├── main.rs (150)
    └── lib.rs (200)
```

### Format Rendering Algorithms

#### Table Format Algorithm

**Algorithm**:
```rust
impl TableFormatter {
  pub fn format(&self, tree: &TreeNode<String>) -> String {
    // 1. Extract headers via TableShapedView trait
    // 2. Extract rows via TableShapedView trait
    // 3. Calculate column widths (max of header and all rows)
    // 4. Format header row with padding
    // 5. Format separator line
    // 6. Format each data row with padding
    // 7. Return concatenated string
  }
}
```

**Column Width Calculation**: `max(header_width, max(row[i]_width for all rows))`

#### Expanded Format Algorithm

**Algorithm**:
```rust
impl ExpandedFormatter {
  pub fn format(&self, tree: &TreeNode<String>) -> String {
    // 1. Extract headers via TableShapedView trait
    // 2. Calculate max key width (longest header name)
    // 3. For each row node (index i):
    //    - Output record separator: "-[ RECORD {i} ]"
    //    - For each cell child:
    //      - Pad cell name to max key width
    //      - Output: "{name} | {data}"
    // 4. Return concatenated string
  }
}
```

**Key Alignment**: All keys left-padded to same width for vertical alignment

#### Tree Format Algorithm

**Recursive DFS with method-level generics**:
```rust
fn format_node<T, F>(
  &self,
  node: &TreeNode<T>,
  output: &mut String,
  prefix: &str,
  is_last: bool,
  depth: usize,
  render: &F
)
where F: Fn(&T) -> String
```

**Prefix Accumulation**:
- Last child: `└── ` → `    ` (space for next level)
- Middle child: `├── ` → `│   ` (vertical for next level)

### Aggregation Algorithm

**Two-Pass**:
1. Calculate subtree totals (bottom-up)
2. Render with percentages (top-down)

**Percentage Calculation**:
```rust
percentage = (subtree_total / grand_total) * 100.0
```

### ANSI Handling

**Visual Length Calculation**:
```rust
pub fn visual_len(text: &str) -> usize {
  // Count chars, skip ANSI sequences (\x1b[...m)
  let mut len = 0;
  let mut in_escape = false;

  for ch in text.chars() {
    if ch == '\x1b' { in_escape = true; }
    else if in_escape && ch == 'm' { in_escape = false; }
    else if !in_escape { len += 1; }
  }
  len
}
```

**Padding with ANSI**:
- Calculate visible length (excluding ANSI)
- Add padding to reach target width
- Preserve ANSI codes in output

## Performance Characteristics

**Time Complexity**:
- Tree construction: O(n × d) where n = nodes, d = depth
- Tree rendering: O(n) single traversal
- Table rendering: O(r × c) where r = rows, c = columns
- Aggregation: O(n) two passes
- Flattening: O(n) DFS traversal

**Space Complexity**:
- Tree: O(n) for node storage
- Rendering: O(d) recursion depth
- Table: O(r × c) for output buffer

**Memory Usage**:
- Minimal allocations
- String concatenation only
- No temporary collections

## Non-Functional Requirements

**Performance**:
- Render 1,000 node tree < 1ms
- Render 100 row table < 1ms
- Zero heap allocations in hot path

**Portability**:
- Zero dependencies
- Pure Rust standard library
- Platform-independent
- `no_std` compatible (with alloc)

**Safety**:
- No unsafe code
- No unwrap in production paths
- No panic paths
- Generic over `T` (minimal trait bounds)

**Architecture**:
- Modular design (16 source files)
- Clear separation of concerns
- Trait-based polymorphism (TableShapedFormatter, Format)
- Builder pattern throughout
- Feature-gated compilation for zero-cost abstractions

## Unicode Box-Drawing Characters

### Tree Symbols
```
├──  U+251C BOX DRAWINGS LIGHT VERTICAL AND RIGHT
│    U+2502 BOX DRAWINGS LIGHT VERTICAL
└──  U+2514 BOX DRAWINGS LIGHT UP AND RIGHT
```

### Table Borders
```
┌─┬─┐  Top border
├─┼─┤  Middle separator
└─┴─┘  Bottom border
│      Vertical separator
```

## Testing Strategy

**Unit Tests** (85 total across 8 test files):

*tests/builder.rs (15 tests)*
- TreeBuilder construction and manipulation
- Path insertion (single, multiple, nested)
- Edge cases (duplicates, empty components, unicode)
- Batch creation (from_items, from_pathbufs)
- Structure tests (deep, wide, insertion order)

*tests/data.rs (14 tests)*
- TreeNode creation and manipulation
- RowBuilder construction and validation
- TableShapedView trait implementation
- Edge cases (empty, single node, large trees)

*tests/column_data.rs (18 tests) - NEW v0.2.0*
- ColumnData construction (new, from_pairs)
- Display trait implementation
- Clone behavior and independence
- Edge cases (empty, unicode, long strings, special chars)

*tests/aligned_tree.rs (15 tests) - NEW v0.2.0*
- Basic aligned formatting (single/multiple nodes)
- Multi-level trees (2-5 depth levels)
- Wide trees (10+ siblings)
- Mixed column counts
- Configuration options (separator, min width)
- Edge cases (empty, unicode, long values)
- Realistic use cases (crate dependency simulation)

*tests/reproduce_alignment_problem.rs (3 tests) - NEW v0.2.0*
- Demonstrates alignment problem and solution
- Programmatic alignment verification
- Side-by-side comparison (aligned vs unaligned)

*tests/fluent_api.rs (8 tests)*
- Fluent RowBuilder API (chainable methods)
- Config builder patterns (Tree, Table, Expanded)
- Integration of builders with formatters

*tests/formatters.rs (8 tests)*
- TableShapedFormatter trait polymorphism
- Generic TableShapedView (integers, floats, custom types)
- Write trait support (stdout, multiple formatters)
- Generic type integration with formatters

*tests/flatten_config.rs (4 tests)*
- FlattenConfig column selection
- Custom column naming
- Path-only flattening
- Integration with TableFormatter

**Doc Tests** (34):
- All public API examples
- Conversion examples
- Common patterns
- Builder patterns
- ColumnData examples (NEW v0.2.0)
- AlignedTreeFormatter examples (NEW v0.2.0)

**Coverage**: 100% of public API

**Total Test Count**: 119 tests (85 unit + 34 doc)

**Test Organization**:
- All integration tests are gated behind the `integration` feature
- Feature is enabled by default in `[features]` section
- Each test file includes `#![ cfg( feature = "integration" ) ]` attribute
- Allows selective test execution and faster builds when needed

## Cargo Features

**Available Features** (NEW in v0.4.0):

**Individual Formatters**:
- `format_table` - TableFormatter (no dependencies)
- `format_expanded` - ExpandedFormatter (no dependencies)
- `format_tree` - TreeFormatter (no dependencies)
- `format_text` - TextFormatter with 6 styles (no dependencies)
- `format_json` - JsonFormatter (requires: serde, serde_json)
- `format_yaml` - YamlFormatter (requires: serde, serde_yaml)
- `format_toml` - TomlFormatter (requires: serde, toml)

**Feature Bundles**:
- `visual_formats` = `format_table` + `format_expanded` + `format_tree` (default)
- `data_formats` = `format_json` + `format_yaml` + `format_toml`
- `all_formats` = `visual_formats` + `data_formats` + `format_text`

**Other Features**:
- `serde_support` - Enables serde derives on data structures (required for data formatters)
- `integration` (default) - Enables all integration tests in `tests/` directory

**Feature Configuration**:
```toml
[features]
default = [ "integration", "visual_formats" ]
integration = []
serde_support = [ "dep:serde" ]
format_table = []
format_expanded = []
format_tree = []
format_text = []
format_json = [ "serde_support", "dep:serde_json" ]
format_yaml = [ "serde_support", "dep:serde_yaml" ]
format_toml = [ "serde_support", "dep:toml" ]
visual_formats = [ "format_table", "format_expanded", "format_tree" ]
data_formats = [ "format_json", "format_yaml", "format_toml" ]
all_formats = [ "visual_formats", "data_formats", "format_text" ]
```

**Usage Examples**:
```toml
# Default: visual formatters only (table, expanded, tree)
tree_fmt = "0.4.0"

# Add JSON support
tree_fmt = { version = "0.4.0", features = [ "format_json" ] }

# All formatters
tree_fmt = { version = "0.4.0", features = [ "all_formats" ] }

# Minimal: only table formatter
tree_fmt = { version = "0.4.0", default-features = false, features = [ "format_table" ] }
```

## Dependencies

**Runtime**:
- **Core** (with default features): Zero dependencies (pure stdlib)
- **With `format_json`**: serde, serde_json
- **With `format_yaml`**: serde, serde_yaml
- **With `format_toml`**: serde, toml

**Dev**: serde, serde_json, serde_yaml, toml (for testing)

**Portability**:
- Visual formatters: Maximum (zero dependencies)
- Data formatters: Requires serde ecosystem

## Example Usage

### Fluent RowBuilder API

```rust
use tree_fmt::{ RowBuilder, TableFormatter };

// Fluent chainable API
let tree = RowBuilder::new(vec![ "Name".into(), "Age".into() ])
  .add_row(vec![ "Alice".into(), "30".into() ])
  .add_row(vec![ "Bob".into(), "25".into() ])
  .build();

let formatter = TableFormatter::new();
println!("{}", formatter.format(&tree));
```

```text
 Name  | Age
-------+-----
 Alice |  30
 Bob   |  25
```

### Polymorphic Formatting with Trait Objects

```rust
use tree_fmt::{ RowBuilder, formatters::TableShapedFormatter, TableFormatter, ExpandedFormatter };

let tree = RowBuilder::new(vec![ "Col".into() ])
  .add_row(vec![ "Data".into() ])
  .build();

// Use trait object for polymorphism
let formatters: Vec<Box<dyn TableShapedFormatter>> = vec![
  Box::new(TableFormatter::new()),
  Box::new(ExpandedFormatter::new()),
];

for formatter in formatters {
  let output = formatter.format(&tree);
  println!("{}", output);
}
```

### Generic TableShapedView (Works with any T: Display)

```rust
use tree_fmt::{ TreeNode, TableShapedView };

// Create tree with numeric data
let mut root = TreeNode::new("root".into(), None::<u64>);

let mut row1 = TreeNode::new("row1".into(), None);
row1.children.push(TreeNode::new("A".into(), Some(100u64)));
row1.children.push(TreeNode::new("B".into(), Some(200u64)));

root.children.push(row1);

// Extract as string rows (converts via Display)
let rows = root.to_rows();
assert_eq!(rows[0], vec!["100", "200"]);
```

### Column-Aligned Tree Formatting (NEW in v0.2.0)

`TreeFormatter` supports column-aligned formatting via the `format_aligned()` method, which displays hierarchical trees with multiple attributes per node where all columns align vertically across all tree levels. This solves the problem of ragged, hard-to-scan output when displaying structured data like dependency trees.

**Problem**: Without alignment, columns are ragged:
```text
├── api_ollama  api_ollama v0.1.0 (api/ollama)
├── as_curl  as_curl v0.1.0 (module/as_curl)
│   ├── dep1  dep1 v2.0.0 (path/to/dep1)
```

**Solution**: With `TreeFormatter::format_aligned()`, columns align vertically:
```text
├── api_ollama  v0.1.0  (api/ollama)
├── as_curl     v0.1.0  (module/as_curl)
│   ├── dep1  v2.0.0  (path/to/dep1)
```

**Code Example**:

```rust
use tree_fmt::{ TreeNode, ColumnData, TreeFormatter };

let mut root = TreeNode::new("workspace".to_string(), None);

// Add nodes with multi-column data
root.children.push(TreeNode::new(
  "api_ollama".to_string(),
  Some(ColumnData::new(vec![
    "api_ollama".to_string(),
    "v0.1.0".to_string(),
    "(api/ollama)".to_string()
  ]))
));

root.children.push(TreeNode::new(
  "as_curl".to_string(),
  Some(ColumnData::new(vec![
    "as_curl".to_string(),
    "v0.1.0".to_string(),
    "(module/as_curl)".to_string()
  ]))
));

// Format with aligned columns
let formatter = TreeFormatter::new();
let output = formatter.format_aligned(&root);
println!("{}", output);
```

**Output**:
```text
├── api_ollama  v0.1.0  (api/ollama)
└── as_curl     v0.1.0  (module/as_curl)
```

**Key Features**:
- Automatic column width calculation across entire tree
- Works with any number of columns
- Preserves tree structure (├──, │, └──)
- Configurable column separator (default: 2 spaces)
- Supports unicode and ANSI colors
- Two-pass algorithm: calculates widths, then formats

**Formatter Parameters**:
```rust
use tree_fmt::{ TreeFormatter, TreeConfig };

let config = TreeConfig::new()
  .column_separator(" | ".to_string())  // Pipe-separated columns
  .min_column_width(10);                // Minimum width per column

let formatter = TreeFormatter::with_config(config);
let output = formatter.format_aligned(&tree);
```

**Use Cases**:
- Package dependency trees (name, version, path)
- File trees (name, size, permissions, date)
- Process trees (name, PID, CPU%, memory)
- Any hierarchical data with multiple aligned attributes

### Expanded Format Styles (NEW in v0.2.3)

The `ExpandedFormatter` supports two distinct styles through the `PaddingSide` enum.

#### PaddingSide Enum

```rust
/// Where to place alignment padding in key-value pairs
pub enum PaddingSide
{
  /// Pad keys before separator: "Name   | Value"
  BeforeSeparator,

  /// Pad values after separator: "Name: Value"
  AfterSeparator,
}
```

#### PostgreSQL Style (Default)

**Usage**:
```rust
use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

let tree = RowBuilder::new(vec![ "Name".into(), "Status".into() ])
  .add_row(vec![ "server1".into(), "online".into() ])
  .build();

let formatter = ExpandedFormatter::new();
// or
let formatter = ExpandedFormatter::with_config( ExpandedConfig::postgres_style() );
let output = formatter.format(&tree);
```

**Output**:
```text
-[ RECORD 1 ]
Name   | server1
Status | online
```

**Features**:
- Record separator headers: `-[ RECORD N ]`
- Pipe separator: ` | `
- Keys padded before separator
- Values start immediately after separator

#### Property List Style

**Usage**:
```rust
use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

let tree = RowBuilder::new(vec![ "Command".into(), "Status".into(), "Duration".into() ])
  .add_row(vec![ "sleep 10".into(), "Completed".into(), "10 seconds".into() ])
  .build();

let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
let output = formatter.format(&tree);
```

**Output**:
```text
Command:  sleep 10
Status:   Completed
Duration: 10 seconds
```

**Features**:
- No record separator headers (blank lines between records)
- Colon separator: `: `
- Separators follow keys immediately
- Values padded after separator for alignment

#### Colored Keys

Both styles support optional ANSI coloring for keys:

**PostgreSQL style with gray keys**:
```rust
let formatter = ExpandedFormatter::with_config(
  ExpandedConfig::postgres_style()
    .colorize_keys(true)
);
```

**Property style with gray keys**:
```rust
let formatter = ExpandedFormatter::with_config(
  ExpandedConfig::property_style()
    .colorize_keys(true)
);
```

**Output** (property style with colors):
```text
Command:  sleep 10      # "Command:" in gray
Status:   Completed     # "Status:" in gray
Duration: 10 seconds    # "Duration:" in gray
```

**Custom colors**:
```rust
let formatter = ExpandedFormatter::with_config(
  ExpandedConfig::property_style()
    .colorize_keys(true)
    .key_color("\x1b[36m".to_string())  // Cyan
);
```

**Common ANSI Colors**:
- Gray: `\x1b[90m` (default)
- Blue: `\x1b[34m`
- Cyan: `\x1b[36m`
- Yellow: `\x1b[33m`

#### Custom Styles

Mix and match configuration options:

```rust
use tree_fmt::{ ExpandedConfig, PaddingSide };

let formatter = ExpandedFormatter::with_config(
  ExpandedConfig::new()
    .record_separator("=== Entry {} ===".to_string())
    .key_value_separator(" = ".to_string())
    .padding_side(PaddingSide::AfterSeparator)
    .colorize_keys(true)
);
```

**Output**:
```text
=== Entry 1 ===
Name     = Alice
Location = New York
```

### Write Trait Support (Zero-Allocation Output)

```rust
use tree_fmt::{ RowBuilder, TableFormatter };
use std::io::Cursor;

let tree = RowBuilder::new(vec![ "Col".into() ])
  .add_row(vec![ "Data".into() ])
  .build();

let formatter = TableFormatter::new();
let mut buffer = Cursor::new(Vec::new());

// Write directly to io::Write without intermediate String
formatter.write_to(&tree, &mut buffer).unwrap();

let output = String::from_utf8(buffer.into_inner()).unwrap();
assert!(output.contains("Data"));
```

### Config Builder Pattern

```rust
use tree_fmt::{ TreeConfig, TreeFormatter, TreeBuilder };

let config = TreeConfig::new()
  .show_branches(false)
  .show_root(true)
  .indent_size(2)
  .max_depth(Some(3));

let tree = TreeBuilder::new("root")
  .insert(&["dir", "file.txt"], 100)
  .build();

let formatter = TreeFormatter::with_config(config);
let output = formatter.format(&tree, |n| format!("{}", n));
```

### Customizable Tree Flattening

```rust
use tree_fmt::{ TreeBuilder, conversions::{ flatten_to_table_tree_with_config, FlattenConfig }, TableFormatter };

let tree = TreeBuilder::new("project")
  .insert(&["src", "main.rs"], 150)
  .insert(&["src", "lib.rs"], 200)
  .build();

// Only include name and data columns with custom names
let config = FlattenConfig::new()
  .include_path(false)
  .include_depth(false)
  .column_names("ignored".into(), "File".into(), "ignored2".into(), "Lines".into());

let flattened = flatten_to_table_tree_with_config(&tree, &config);
let formatter = TableFormatter::new();
let output = formatter.format(&flattened);

// Output contains only "File" and "Lines" columns
assert!(output.contains("File"));
assert!(output.contains("Lines"));
assert!(!output.contains("project/src")); // path excluded
```

### Hierarchical Data Example

```rust
use tree_fmt::{ TreeBuilder, TreeFormatter, TableFormatter, ExpandedFormatter };

// Create tree data
let tree = TreeBuilder::new("project")
  .insert(&["src", "main.rs"], 1024u64)
  .insert(&["src", "lib.rs"], 2048u64)
  .insert(&["Cargo.toml"], 256u64)
  .build();
```

#### Tree Format Output

```rust
let tree_fmt = TreeFormatter::new();
let output = tree_fmt.format(&tree, |bytes| format!("{} bytes", bytes));
println!("{}", output);
```

```text
project
├── src
│   ├── main.rs (1024 bytes)
│   └── lib.rs (2048 bytes)
└── Cargo.toml (256 bytes)
```

#### Table Format Output (Flattened)

```rust
let table_fmt = TableFormatter::new();
println!("{}", table_fmt.format_tree(&tree));
```

```text
 path                 | name       | depth | data
----------------------+------------+-------+------
 project              | project    | 0     |
 project/src          | src        | 1     |
 project/src/main.rs  | main.rs    | 2     | 1024
 project/src/lib.rs   | lib.rs     | 2     | 2048
 project/Cargo.toml   | Cargo.toml | 1     | 256
```

#### Expanded Format Output (Flattened)

```rust
let expanded_fmt = ExpandedFormatter::new();
println!("{}", expanded_fmt.format_tree(&tree));
```

```text
-[ RECORD 1 ]
path  | project
name  | project
depth | 0
data  |
-[ RECORD 2 ]
path  | project/src
name  | src
depth | 1
data  |
-[ RECORD 3 ]
path  | project/src/main.rs
name  | main.rs
depth | 2
data  | 1024
```

## Versioning

**Current**: v0.4.0 (unified format interface with granular feature flags)

**Semantic Versioning**:
- MAJOR: Breaking API changes
- MINOR: New features (formats, conversions)
- PATCH: Bug fixes, optimizations

## Changes in v0.2.0

**Architecture Refactoring**:
1. **Modular structure** - Split monolithic implementation into 11 focused modules
2. **Removed DataTable** - Replaced with RowBuilder + TableShapedView trait
3. **Removed PhantomData** - TreeFormatter now uses method-level generics
4. **Added TableShapedFormatter trait** - Polymorphic formatter interface
5. **Generic TableShapedView** - Works with TreeNode<T: Display>
6. **Fluent RowBuilder API** - Chainable builder methods
7. **Write trait support** - All formatters support write_to()
8. **Config builder pattern** - All configs have fluent APIs
9. **FlattenConfig** - Customizable tree-to-table conversion

**New Features**:
1. `RowBuilder` - Fluent builder for table-shaped trees
2. `TableShapedView` trait - Generic trait for table operations
3. `TableShapedFormatter` trait - Polymorphic formatter interface
4. `FlattenConfig` - Customizable flattening configuration
5. `flatten_to_table_tree_with_config()` - Custom column selection
6. `write_to()` methods - Zero-allocation io::Write output
7. Config builder methods - Fluent APIs for all configs
8. Generic TableShapedView - Works with any T: Display

**Breaking Changes from Legacy**:
- DataTable removed (use RowBuilder)
- format_with_row_names() removed (row naming handled by builder)
- TreeFormatter no longer generic over struct (method-level generics)
- All configs now have builder pattern

**Migration Guide**:
```rust
// Legacy (pre-refactor)
let mut data = DataTable::new(headers);
data.add_row(row);

// v0.2.0 (current)
let tree = RowBuilder::new(headers)
  .add_row(row)
  .build();
```

## Version History

**v0.1.0** (Linter improvements):
- Added ANSI-aware helper functions
- `visual_len()` - Calculate visible length
- `pad_to_width()` - Pad with ANSI support
- Enhanced table alignment with colors

**v0.2.0** (Modular refactored architecture):
- Modular file structure (11 source files)
- RowBuilder replaces DataTable
- TableShapedView trait for generic table operations
- TableShapedFormatter trait for polymorphism
- Fluent builder APIs throughout
- Write trait support for zero-allocation output
- FlattenConfig for customizable conversions
- Method-level generics in TreeFormatter
- 50 comprehensive tests across 3 test files

**v0.2.1** (Quality improvements):
- Fixed documentation accuracy in tests/readme.md to reflect actual test files
- Fixed documentation accuracy in examples/readme.md to reflect actual examples
- Removed dead code (unused traverse_and_flatten wrapper function)
- Code style improvements: all function signatures now comply with 110-char line limit
- All tests pass (50 unit tests + 30 doc tests)
- Zero clippy warnings

**v0.2.2** (Colored keys feature):
- Added ANSI color support for keys in ExpandedFormatter
- New `colorize_keys` config option (default: false for backward compatibility)
- New `key_color` config option (default: gray `\x1b[90m`)
- Builder methods: `colorize_keys()` and `key_color()`
- 4 new tests for color functionality
- Updated examples and documentation
- All tests pass (94 unit tests + 35 doc tests)
- Zero clippy warnings

**v0.2.3** (Property list style and padding parametrization):
- Added `PaddingSide` enum: `BeforeSeparator` (default) and `AfterSeparator`
- Parametrized record separator (empty string disables headers)
- New `ExpandedConfig::postgres_style()` convenience constructor
- New `ExpandedConfig::property_style()` convenience constructor (gray keys enabled by default)
- Builder method: `padding_side()`
- Updated `record_separator` to support format string with `{}` placeholder
- 10 new tests for property style and padding variations (including color defaults)
- Zero breaking changes (full backward compatibility)
- All tests pass (104 unit tests + 36 doc tests)
- Zero clippy warnings

**v0.3.0** (Table styles refactoring - comprehensive parametrization):
- Added `BorderVariant` enum: `None`, `Ascii`, `AsciiGrid`, `Unicode`, `Markdown`
- Added `HeaderSeparatorVariant` enum: `None`, `Dash`, `AsciiGrid`, `Unicode`, `Markdown`
- Added `ColumnSeparator` enum: `Spaces(usize)`, `Character(char)`, `String(String)`
- Comprehensive `TableConfig` refactoring with 11 new formatter parameters:
  - `border_variant`: Border rendering variant (replaces `show_borders`)
  - `header_separator_variant`: Header separator line variant
  - `column_separator`: Column separator configuration
  - `outer_padding`: Padding at table edges
  - `inner_padding`: Padding within cells
  - `colorize_header`: Enable header coloring
  - `header_color`: ANSI color for headers
  - `alternating_rows`: Enable alternating row colors
  - `row_color1`, `row_color2`: Colors for alternating rows
  - `min_column_width`: Minimum width per column
  - `max_column_width`: Maximum width with truncation
  - `truncation_marker`: String for truncated content ("...")
- 9 variant preset constructors for common table formats:
  - `TableConfig::plain()` - Space-separated with dash separator (CLI tools)
  - `TableConfig::minimal()` - Space-separated, no separator (maximum simplicity)
  - `TableConfig::bordered()` - Traditional pipe borders (default)
  - `TableConfig::markdown()` - GitHub-flavored Markdown tables
  - `TableConfig::grid()` - Full ASCII grid with intersections
  - `TableConfig::unicode_box()` - Unicode box-drawing characters
  - `TableConfig::csv()` - Comma-separated values
  - `TableConfig::tsv()` - Tab-separated values
  - `TableConfig::compact()` - Minimal spacing for density
- 13 new builder methods for fine-grained control
- Deprecated `show_borders()` (use `border_variant()` instead)
- Full backward compatibility (default behavior unchanged)
- 30+ new tests for all table variants and formatter parameters
- Zero breaking changes for existing code
- All tests pass (134+ unit tests + 40+ doc tests)
- Zero clippy warnings
- 2 new examples: `table_styles.rs`, `process_monitor.rs`

**v0.4.0** (Unified format interface with granular feature flags):
- Added `Format` trait - Unified interface for all formatters
- Added `TableView` struct - Canonical data format for format-agnostic code
- Added `FormatError` enum - Consistent error handling across formatters
- Added `JsonFormatter` - JSON output with pretty/compact modes (NEW)
- Added `YamlFormatter` - YAML output (NEW)
- Added `TomlFormatter` - TOML output (NEW)
- Added `TextFormatter` - Plain text output with 6 styles: bullets, numbered, sections, key-value, compact, cli-help (NEW)
- Added `RowBuilder::build_view()` - Returns TableView instead of TreeNode
- Added `TableView::to_tree_node()` - Backward compatibility converter
- Implemented `Format` trait for all formatters (Table, Expanded, Tree, JSON, YAML, TOML, Text)
- Merged `AlignedTreeFormatter` into `TreeFormatter` as `format_aligned()` method
- Added `column_separator` and `min_column_width` fields to `TreeConfig`
- Granular cargo features for zero-cost abstractions:
  - Individual formatter features: `format_table`, `format_expanded`, `format_tree`, `format_text`, `format_json`, `format_yaml`, `format_toml`
  - Feature bundles: `visual_formats` (default), `data_formats`, `all_formats`
  - Conditional serde support: `serde_support` feature
- Updated architecture to 16 source files (added 4 new formatters, merged aligned_tree.rs into tree.rs)
- Zero runtime dependencies for visual formatters
- Optional dependencies: serde, serde_json, serde_yaml, toml (only when features enabled)
- 21 new integration tests for unified format interface (`tests/unified_format_trait.rs`)
- Total: 175 tree_fmt tests + 18 will_crates tests + 43 doc tests = 236 tests
- Real-world integration: Updated will_crates to use unified format interface
- Full backward compatibility: Existing API unchanged, new `build_view()` method added alongside `build()`
- Created comprehensive 450+ line guide: `UNIFIED_FORMAT_GUIDE.md`
- Created example demonstrating unified interface: `examples/unified_formats.rs`
- Zero breaking changes for existing code
- All tests pass with -D warnings
- Zero clippy warnings
