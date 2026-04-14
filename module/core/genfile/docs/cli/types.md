# Types Reference

Type system documentation for genfile CLI parameters (15 semantic newtypes).

**Purpose:** This documentation serves implementers and developers. For user-focused parameter documentation, see [params.md](params.md).

## Quick Navigation

**By Category:**
- [Integer Types](#category-integer-types) (1) - Constrained numeric types
- [Boolean Types](#category-boolean-types) (4) - Flag types
- [String Types](#category-string-types) (4) - Text types with validation
- [Path Types](#category-path-types) (3) - Filesystem path types
- [Enum Types](#category-enum-types) (3) - Multiple-choice types

## Types Index

| # | Type | Purpose | Fundamental | Constraints | Used By |
|---|------|---------|-------------|-------------|---------|
| 1 | [VerbosityLevel](#type-verbositylevel) | Output detail control | u8 | 0-5 range | verbosity:: |
| 2 | [DryRunFlag](#type-dryrunflag) | Preview mode flag | bool | true/false | dry:: |
| 3 | [FilePath](#type-filepath) | File system path | PathBuf | Valid UTF-8 | path::, from_file:: |
| 4 | [OutputPath](#type-outputpath) | Writable output path | PathBuf | Writable | destination::, output_dir::, output:: |
| 5 | [DirectoryPath](#type-directorypath) | Directory path | PathBuf | Must exist | source::, input:: |
| 6 | [IdentifierString](#type-identifierstring) | Entity identifier | String | Alphanumeric+underscore | name:: |
| 7 | [DescriptionText](#type-descriptiontext) | Description text | String | Any UTF-8 | description:: |
| 8 | [PatternString](#type-patternstring) | Glob pattern | String | Valid glob | include_pattern::, exclude_pattern::, filter:: |
| 9 | [ContentString](#type-contentstring) | Content data | String | Any UTF-8 | value::, default::, content:: |
| 10 | [ContentMode](#type-contentmode) | Storage strategy | enum | inline \| reference | mode:: |
| 11 | [SerializationFormat](#type-serializationformat) | Data format | enum | json \| yaml | format:: |
| 12 | [WriteMode](#type-writemode) | Write behavior | enum | rewrite \| append \| skip | write_mode:: |
| 13 | [RecursiveFlag](#type-recursiveflag) | Traversal flag | bool | true/false | recursive:: |
| 14 | [PrettyPrintFlag](#type-prettyprintflag) | Formatting flag | bool | true/false | pretty:: |
| 15 | [MandatoryFlag](#type-mandatoryflag) | Requirement flag | bool | true/false | mandatory:: |

## Type Categories

### Category: Integer Types

Numeric types with constrained ranges.

| Type | Range | Default | Purpose |
|------|-------|---------|---------|
| [VerbosityLevel](#type-verbositylevel) | 0-5 | 1 | Output verbosity |

### Category: Boolean Types

Binary flag types.

| Type | Default | Purpose |
|------|---------|---------|
| [DryRunFlag](#type-dryrunflag) | false | Preview mode |
| [RecursiveFlag](#type-recursiveflag) | true | Directory traversal |
| [PrettyPrintFlag](#type-prettyprintflag) | true | JSON formatting |
| [MandatoryFlag](#type-mandatoryflag) | false | Parameter requirement |

### Category: String Types

Text types with validation.

| Type | Validation | Purpose |
|------|------------|---------|
| [IdentifierString](#type-identifierstring) | Alphanumeric+underscore | Entity names |
| [DescriptionText](#type-descriptiontext) | Any UTF-8 | Descriptions |
| [PatternString](#type-patternstring) | Valid glob | File patterns |
| [ContentString](#type-contentstring) | Any UTF-8 | Content data |

### Category: Path Types

Filesystem path types.

| Type | Validation | Purpose |
|------|------------|---------|
| [FilePath](#type-filepath) | Valid path | File references |
| [OutputPath](#type-outputpath) | Writable path | Output targets |
| [DirectoryPath](#type-directorypath) | Existing directory | Input sources |

### Category: Enum Types

Multiple-choice types.

| Type | Options | Purpose |
|------|---------|---------|
| [ContentMode](#type-contentmode) | inline \| reference | Storage strategy |
| [SerializationFormat](#type-serializationformat) | json \| yaml | Data format |
| [WriteMode](#type-writemode) | rewrite \| append \| skip | Write behavior |

---

## Type Specifications

### Type :: `VerbosityLevel`

Semantic newtype wrapping u8 to represent CLI output verbosity levels with constrained range. Prevents invalid values and centralizes verbosity checking through semantic methods.

**Purpose:** Type-safe verbosity control with validation
**Fundamental Type:** `u8`

**Constants:**
```rust
pub const MIN: u8 = 0;  // Silent (errors only)
pub const MAX: u8 = 5;  // Ultra-trace
pub const DEFAULT: u8 = 1;  // Normal output
```

**Constraints:**
- Value must be in range 0-5 (inclusive)
- Construction fails for out-of-range values
- Default is 1 (normal verbosity)

**Parsing:**
```rust
impl TryFrom<u8> for VerbosityLevel
{
  type Error = ValidationError;

  fn try_from( level: u8 ) -> Result< Self, Self::Error >
  {
    if level > 5
    {
      Err( ValidationError::OutOfRange { value: level, min: 0, max: 5 } )
    }
    else
    {
      Ok( Self( level ) )
    }
  }
}

impl FromStr for VerbosityLevel
{
  type Err = ValidationError;

  fn from_str( s: &str ) -> Result< Self, Self::Err >
  {
    let level: u8 = s.parse()
      .map_err( | _ | ValidationError::InvalidInteger( s.to_string() ) )?;
    Self::try_from( level )
  }
}
```

**Methods:**
```rust
impl VerbosityLevel
{
  pub fn is_silent( &self ) -> bool { self.0 == 0 }
  pub fn is_normal( &self ) -> bool { self.0 == 1 }
  pub fn is_verbose( &self ) -> bool { self.0 >= 2 }
  pub fn is_debug( &self ) -> bool { self.0 >= 3 }
  pub fn as_u8( &self ) -> u8 { self.0 }
}
```

**Used By:** [verbosity::](params.md#parameter-1-verbosity) parameter (24 commands)

---

### Type :: `DryRunFlag`

Boolean newtype for preview mode control. Ensures type safety for dry-run operations.

**Purpose:** Type-safe dry-run flag
**Fundamental Type:** `bool`

**Constants:**
```rust
pub const EXECUTE: bool = false;  // Real execution
pub const PREVIEW: bool = true;   // Dry run
pub const DEFAULT: bool = false;  // Execute by default
```

**Parsing:**
```rust
impl From<bool> for DryRunFlag
{
  fn from( flag: bool ) -> Self { Self( flag ) }
}

impl FromStr for DryRunFlag
{
  type Err = ValidationError;

  fn from_str( s: &str ) -> Result< Self, Self::Err >
  {
    match s
    {
      "0" | "false" => Ok( Self( false ) ),
      "1" | "true" => Ok( Self( true ) ),
      _ => Err( ValidationError::InvalidBoolean( s.to_string() ) ),
    }
  }
}
```

**Methods:**
```rust
impl DryRunFlag
{
  pub fn is_dry_run( &self ) -> bool { self.0 }
  pub fn is_execution( &self ) -> bool { !self.0 }
}
```

**Used By:** [dry::](params.md#parameter-2-dry) parameter (6 commands)

---

### Type :: `FilePath`

Path newtype for file system paths with UTF-8 validation.

**Purpose:** Type-safe file paths
**Fundamental Type:** `PathBuf`

**Constraints:**
- Must be valid UTF-8
- No specific existence requirement (context-dependent)

**Parsing:**
```rust
impl From<PathBuf> for FilePath
{
  fn from( path: PathBuf ) -> Self { Self( path ) }
}

impl FromStr for FilePath
{
  type Err = ValidationError;

  fn from_str( s: &str ) -> Result< Self, Self::Err >
  {
    Ok( Self( PathBuf::from( s ) ) )
  }
}
```

**Methods:**
```rust
impl FilePath
{
  pub fn as_path( &self ) -> &Path { &self.0 }
  pub fn exists( &self ) -> bool { self.0.exists() }
  pub fn to_string_lossy( &self ) -> String { self.0.to_string_lossy().to_string() }
}
```

**Used By:** [path::](params.md#parameter-3-path), [from_file::](params.md#parameter-18-fromfile)

---

### Type :: `OutputPath`

Path newtype for output locations with writability validation.

**Purpose:** Type-safe output paths
**Fundamental Type:** `PathBuf`

**Constraints:**
- Must be valid UTF-8
- Parent directory must be writable (or creatable)

**Parsing:**
```rust
impl From<PathBuf> for OutputPath
{
  fn from( path: PathBuf ) -> Self { Self( path ) }
}

impl OutputPath
{
  pub fn validate_writable( &self ) -> Result< (), ValidationError >
  {
    let parent = self.0.parent()
      .ok_or_else( || ValidationError::InvalidPath( "No parent directory".to_string() ) )?;

    if parent.exists() && !parent.metadata()?.permissions().readonly()
    {
      Ok( () )
    }
    else
    {
      Err( ValidationError::NotWritable( parent.to_path_buf() ) )
    }
  }
}
```

**Used By:** [destination::](params.md#parameter-5-destination), [output_dir::](params.md#parameter-12-outputdir), [output::](params.md#parameter-13-output)

---

### Type :: `DirectoryPath`

Path newtype for directory paths that must exist.

**Purpose:** Type-safe directory paths
**Fundamental Type:** `PathBuf`

**Constraints:**
- Must be valid UTF-8
- Must exist
- Must be directory (not file)

**Parsing:**
```rust
impl TryFrom<PathBuf> for DirectoryPath
{
  type Error = ValidationError;

  fn try_from( path: PathBuf ) -> Result< Self, Self::Error >
  {
    if !path.exists()
    {
      return Err( ValidationError::PathNotFound( path ) );
    }

    if !path.is_dir()
    {
      return Err( ValidationError::NotDirectory( path ) );
    }

    Ok( Self( path ) )
  }
}
```

**Used By:** [source::](params.md#parameter-9-source), [input::](params.md#parameter-16-input)

---

### Type :: `IdentifierString`

String newtype for identifiers with alphanumeric+underscore validation.

**Purpose:** Type-safe identifiers
**Fundamental Type:** `String`

**Constraints:**
- Non-empty
- Alphanumeric + underscore only
- No spaces or special characters

**Parsing:**
```rust
impl TryFrom<String> for IdentifierString
{
  type Error = ValidationError;

  fn try_from( s: String ) -> Result< Self, Self::Error >
  {
    if s.is_empty()
    {
      return Err( ValidationError::EmptyIdentifier );
    }

    if !s.chars().all( | c | c.is_alphanumeric() || c == '_' )
    {
      return Err( ValidationError::InvalidIdentifier( s ) );
    }

    Ok( Self( s ) )
  }
}
```

**Used By:** [name::](params.md#parameter-4-name)

---

### Type :: `DescriptionText`

String newtype for description text with no specific constraints.

**Purpose:** Type-safe descriptions
**Fundamental Type:** `String`

**Constraints:** Any UTF-8 text (empty allowed)

**Parsing:**
```rust
impl From<String> for DescriptionText
{
  fn from( s: String ) -> Self { Self( s ) }
}
```

**Used By:** [description::](params.md#parameter-6-description)

---

### Type :: `PatternString`

String newtype for glob patterns with validation.

**Purpose:** Type-safe glob patterns
**Fundamental Type:** `String`

**Constraints:** Valid glob pattern syntax

**Parsing:**
```rust
impl TryFrom<String> for PatternString
{
  type Error = ValidationError;

  fn try_from( s: String ) -> Result< Self, Self::Error >
  {
    // Validate glob pattern syntax
    glob::Pattern::new( &s )
      .map_err( | e | ValidationError::InvalidPattern( s.clone(), e.to_string() ) )?;

    Ok( Self( s ) )
  }
}
```

**Used By:** [include_pattern::](params.md#parameter-17-includepattern), [exclude_pattern::](params.md#parameter-21-excludepattern), [filter::](params.md#parameter-20-filter)

---

### Type :: `ContentString`

String newtype for content data with no constraints.

**Purpose:** Type-safe content strings
**Fundamental Type:** `String`

**Constraints:** Any UTF-8 text (can be multiline)

**Parsing:**
```rust
impl From<String> for ContentString
{
  fn from( s: String ) -> Self { Self( s ) }
}
```

**Used By:** [value::](params.md#parameter-8-value), [default::](params.md#parameter-22-default), [content::](params.md#parameter-23-content)

---

### Type :: `ContentMode`

Enum newtype for content storage strategy.

**Purpose:** Type-safe content mode
**Fundamental Type:** `enum`

**Constants:**
```rust
pub enum ContentMode
{
  Inline,    // Content embedded in archive
  Reference, // Content stored as file paths
}

pub const DEFAULT: ContentMode = ContentMode::Reference;
```

**Parsing:**
```rust
impl FromStr for ContentMode
{
  type Err = ValidationError;

  fn from_str( s: &str ) -> Result< Self, Self::Err >
  {
    match s.to_lowercase().as_str()
    {
      "inline" => Ok( ContentMode::Inline ),
      "reference" => Ok( ContentMode::Reference ),
      _ => Err( ValidationError::InvalidContentMode( s.to_string() ) ),
    }
  }
}
```

**Methods:**
```rust
impl ContentMode
{
  pub fn is_inline( &self ) -> bool { matches!( self, ContentMode::Inline ) }
  pub fn is_reference( &self ) -> bool { matches!( self, ContentMode::Reference ) }
}
```

**Used By:** [mode::](params.md#parameter-14-mode)

---

### Type :: `SerializationFormat`

Enum newtype for archive serialization format.

**Purpose:** Type-safe serialization format
**Fundamental Type:** `enum`

**Constants:**
```rust
pub enum SerializationFormat
{
  Json, // JSON format
  Yaml, // YAML format
}

pub const DEFAULT: SerializationFormat = SerializationFormat::Json;
```

**Parsing:**
```rust
impl FromStr for SerializationFormat
{
  type Err = ValidationError;

  fn from_str( s: &str ) -> Result< Self, Self::Err >
  {
    match s.to_lowercase().as_str()
    {
      "json" => Ok( SerializationFormat::Json ),
      "yaml" | "yml" => Ok( SerializationFormat::Yaml ),
      _ => Err( ValidationError::InvalidFormat( s.to_string() ) ),
    }
  }
}

impl SerializationFormat
{
  pub fn from_path_extension( path: &Path ) -> Option< Self >
  {
    path.extension()
      .and_then( | ext | ext.to_str() )
      .and_then( | ext | Self::from_str( ext ).ok() )
  }
}
```

**Used By:** [format::](params.md#parameter-19-format)

---

### Type :: `WriteMode`

Enum newtype for file write behavior.

**Purpose:** Type-safe write mode
**Fundamental Type:** `enum`

**Constants:**
```rust
pub enum WriteMode
{
  Rewrite, // Overwrite existing file
  Append,  // Append to existing file
  Skip,    // Skip if file exists
}
```

**Parsing:**
```rust
impl FromStr for WriteMode
{
  type Err = ValidationError;

  fn from_str( s: &str ) -> Result< Self, Self::Err >
  {
    match s.to_lowercase().as_str()
    {
      "rewrite" | "overwrite" => Ok( WriteMode::Rewrite ),
      "append" => Ok( WriteMode::Append ),
      "skip" => Ok( WriteMode::Skip ),
      _ => Err( ValidationError::InvalidWriteMode( s.to_string() ) ),
    }
  }
}
```

**Used By:** [write_mode::](params.md#parameter-7-writemode)

---

### Type :: `RecursiveFlag`

Boolean newtype for recursive traversal control.

**Purpose:** Type-safe recursive flag
**Fundamental Type:** `bool`

**Constants:**
```rust
pub const RECURSIVE: bool = true;
pub const FLAT: bool = false;
pub const DEFAULT: bool = true;
```

**Parsing:** Same as [DryRunFlag](#type-dryrunflag)

**Used By:** [recursive::](params.md#parameter-10-recursive)

---

### Type :: `PrettyPrintFlag`

Boolean newtype for JSON formatting control.

**Purpose:** Type-safe pretty-print flag
**Fundamental Type:** `bool`

**Constants:**
```rust
pub const PRETTY: bool = true;
pub const COMPACT: bool = false;
pub const DEFAULT: bool = true;
```

**Parsing:** Same as [DryRunFlag](#type-dryrunflag)

**Used By:** [pretty::](params.md#parameter-11-pretty)

---

### Type :: `MandatoryFlag`

Boolean newtype for parameter requirement control.

**Purpose:** Type-safe mandatory flag
**Fundamental Type:** `bool`

**Constants:**
```rust
pub const MANDATORY: bool = true;
pub const OPTIONAL: bool = false;
pub const DEFAULT: bool = false;
```

**Parsing:** Same as [DryRunFlag](#type-dryrunflag)

**Used By:** [mandatory::](params.md#parameter-15-mandatory)

---

## Type Safety Principles

**Why Semantic Newtypes:**
1. **Prevents mixing incompatible values** (cant use VerbosityLevel as DryRunFlag)
2. **Centralizes validation** (parse once at input boundary)
3. **Self-documenting code** (function signature shows intent)
4. **Compile-time safety** (Rust type checker prevents misuse)

**Example:**
```rust
// Bad: Primitive types (no type safety)
fn execute_command( verbosity: u8, dry: bool ) -> Result< () >
{
  // Can accidentally pass verbosity as dry flag
}

// Good: Semantic newtypes (type-safe)
fn execute_command( verbosity: VerbosityLevel, dry: DryRunFlag ) -> Result< () >
{
  // Compiler prevents type mismatches
}
```

---

## See Also

- [Parameters Reference](params.md) - Parameter specifications (user-focused)
- [Commands Reference](commands.md) - Command documentation
- [Parameter Groups](parameter_groups.md) - Shared parameter sets
