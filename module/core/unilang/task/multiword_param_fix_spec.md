# Fix Specification: Quoted Multiword Parameter Parsing

**Goal:** Make `cmd param::"multi word value"` work without outer shell quotes

## Root Cause

**File:** `unilang_parser/src/parser_engine.rs:1163-1164`

```rust
let command_str = tokens.join( " " );
self.parse_single_instruction( &command_str )  // ← BUG: Re-tokenizes reconstructed string
```

**Problem:** After correctly reconstructing argv and adding quotes back, the code re-parses the joined string, which re-tokenizes on delimiters (including `::`), breaking the carefully reconstructed quote boundaries.

## Solution: Direct Instruction Building

Replace the string reconstruction + re-parsing with direct instruction building from parsed argv tokens.

### Implementation

**Location:** `unilang_parser/src/parser_engine.rs`

**Changes:**

1. **Rename current `parse_argv` to `parse_argv_legacy`** (keep for compatibility)

2. **Create new `parse_argv` that doesn't re-tokenize:**

```rust
pub fn parse_argv( &self, argv: &[ String ] ) -> Result< GenericInstruction, ParseError >
{
  if argv.is_empty()
  {
    return Err( ParseError::empty_input() );
  }

  // First element must be the command name
  let command_name = &argv[ 0 ];

  if !command_name.starts_with( '.' )
  {
    return Err( ParseError::command_must_start_with_dot( command_name ) );
  }

  // Parse remaining argv elements as arguments
  let mut i = 1;
  let mut arguments: Vec< ParsedArgument > = Vec::new();

  while i < argv.len()
  {
    let arg = &argv[ i ];

    // Check if this is a named argument (contains ::)
    if let Some( ( key, value_raw ) ) = arg.split_once( "::" )
    {
      // Start building the value
      let mut value = value_raw.to_string();

      // Combine subsequent argv elements until we hit another parameter or command
      while i + 1 < argv.len()
      {
        let next_arg = &argv[ i + 1 ];

        // Stop if next arg contains :: (it's another named argument)
        if next_arg.contains( "::" )
        {
          break;
        }

        // Stop if next arg starts with . (it's a command)
        if next_arg.starts_with( '.' )
        {
          break;
        }

        // Combine this argument into the value
        if !value.is_empty()
        {
          value.push( ' ' );
        }
        value.push_str( next_arg );
        i += 1;
      }

      // Strip surrounding quotes if present (bash may have left them)
      let final_value = strip_surrounding_quotes( &value );

      // Create argument directly - NO re-tokenization
      arguments.push( ParsedArgument
      {
        key: key.to_string(),
        value: final_value,
        source_location: SourceLocation::from_argv_index( i ),
      });
    }
    else
    {
      // Positional argument (no ::)
      let final_value = strip_surrounding_quotes( arg );

      arguments.push( ParsedArgument
      {
        key: String::new(),  // Positional arg has no key
        value: final_value,
        source_location: SourceLocation::from_argv_index( i ),
      });
    }

    i += 1;
  }

  // Build instruction directly from parsed arguments
  // NO string reconstruction, NO re-tokenization
  Ok( GenericInstruction
  {
    command_name: command_name.to_string(),
    arguments,
  })
}

// Helper function
fn strip_surrounding_quotes( s: &str ) -> String
{
  // Strip outer quotes if both present
  if ( s.starts_with( '"' ) && s.ends_with( '"' ) ) ||
     ( s.starts_with( '\'' ) && s.ends_with( '\'' ) )
  {
    if s.len() >= 2
    {
      return s[ 1..s.len() - 1 ].to_string();
    }
  }
  s.to_string()
}
```

### Key Changes

1. **No string reconstruction:** Don't join tokens back into a string
2. **No re-tokenization:** Build instruction directly from argv
3. **Quote stripping:** Remove surrounding quotes that bash may have left
4. **Value reconstruction:** Still combine subsequent tokens until next `:` or `.`

### Test Cases

```rust
#[test]
fn test_multiword_param_without_shell_quotes()
{
  let parser = UnilangParser::new( Default::default() );

  // Simulate: user types query::"llm rust"
  // Bash outputs: ["query::llm rust"] (one token, quotes removed)
  let result = parser.parse_argv( &[
    ".video.search".to_string(),
    "query::llm rust".to_string()
  ] )?;

  assert_eq!( result.arguments[0].key, "query" );
  assert_eq!( result.arguments[0].value, "llm rust" );
}

#[test]
fn test_multiword_param_with_preserved_quotes()
{
  let parser = UnilangParser::new( Default::default() );

  // Simulate: user types 'query::"llm rust"'
  // Bash outputs: ['query::"llm rust"'] (quotes preserved by outer quotes)
  let result = parser.parse_argv( &[
    ".video.search".to_string(),
    "query::\"llm rust\"".to_string()
  ] )?;

  assert_eq!( result.arguments[0].key, "query" );
  assert_eq!( result.arguments[0].value, "llm rust" );  // Quotes stripped
}

#[test]
fn test_multiword_param_split_across_argv()
{
  let parser = UnilangParser::new( Default::default() );

  // Simulate: user types query::llm rust next::value
  // Bash outputs: ["query::llm", "rust", "next::value"]
  let result = parser.parse_argv( &[
    ".cmd".to_string(),
    "query::llm".to_string(),
    "rust".to_string(),
    "next::value".to_string(),
  ] )?;

  // Should combine "llm" + "rust" until seeing "next::"
  assert_eq!( result.arguments[0].key, "query" );
  assert_eq!( result.arguments[0].value, "llm rust" );
  assert_eq!( result.arguments[1].key, "next" );
  assert_eq!( result.arguments[1].value, "value" );
}
```

## Migration Path

1. **Phase 1:** Implement new `parse_argv` alongside existing code
2. **Phase 2:** Update all callers to use new `parse_argv`
3. **Phase 3:** Run full test suite to verify no regressions
4. **Phase 4:** Remove old `parse_argv_legacy` if all tests pass

## Expected Behavior After Fix

```bash
# All these should work WITHOUT outer quotes:
w3 .crates.for.each cmd::"cargo build"
w3 .crates.for.each cmd::"echo test"
w3 .video.search query::"llm rust"
w3 .command path::"/My Documents/file.txt"
```

## User Benefits

✅ Natural syntax: `param::"value"` just works
✅ No shell quoting gymnastics required
✅ Consistent with user expectations
✅ Impossible to misuse (quotes always work)

## Technical Benefits

✅ Simpler code path (no double parsing)
✅ Better performance (one pass instead of two)
✅ More reliable (respects argv boundaries)
✅ Easier to debug (less string manipulation)

## Acceptance Criteria

- [ ] `cmd param::"multi word"` works without outer quotes
- [ ] `cmd param::single` still works (backward compat)
- [ ] All existing tests pass
- [ ] New tests for multiword params added
- [ ] Documentation updated
- [ ] No performance regression
