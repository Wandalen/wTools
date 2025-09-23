# Task 008: Parser Integration Optimization

## Priority: High
## Impact: 30-60% improvement in parsing pipelines through combined operations
## Estimated Effort: 4-5 days

## Problem Statement

Current parsing workflows require multiple separate passes over input data, creating performance bottlenecks:

```rust
// Current multi-pass approach
let input = "command arg1:value1 arg2:value2 --flag";

// Pass 1: Split into tokens  
let tokens: Vec<String> = string::split()
    .src(input)
    .delimeter(" ")
    .perform()
    .collect();

// Pass 2: Parse each token separately
let mut args = Vec::new();
for token in tokens {
    if token.contains(':') {
        // Pass 3: Split key-value pairs
        let parts: Vec<String> = string::split()
            .src(&token)
            .delimeter(":")
            .perform()  
            .collect();
        args.push((parts[0].clone(), parts[1].clone()));
    }
}
```

This creates multiple inefficiencies:
- **Multiple passes**: Same data processed repeatedly
- **Intermediate allocations**: Temporary vectors and strings
- **Cache misses**: Data accessed multiple times from memory
- **Parsing overhead**: Multiple iterator creation and teardown

## Solution Approach

Implement integrated parsing operations that combine tokenization, validation, and transformation in single passes with parser-aware optimizations.

### Implementation Plan

#### 1. Single-Pass Token Parsing

```rust
/// Combined tokenization and parsing in single pass
pub struct TokenParsingIterator<'a, F, T> 
{
    input: &'a str,
    delimiters: Vec<&'a str>,
    parser_func: F,
    position: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, F, T> TokenParsingIterator<'a, F, T> 
where
    F: Fn(&str) -> Result<T, ParseError>,
{
    pub fn new(input: &'a str, delimiters: Vec<&'a str>, parser: F) -> Self 
{
        Self {
            input,
            delimiters,
            parser_func: parser,
            position: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, F, T> Iterator for TokenParsingIterator<'a, F, T>
where
    F: Fn(&str) -> Result<T, ParseError>,
{
    type Item = Result<T, ParseError>;
    
    fn next(&mut self) -> Option<Self::Item> 
{
        // Find next token using existing split logic
        let token = self.find_next_token()?;
        
        // Parse token immediately without intermediate allocation
        Some((self.parser_func)(token))
    }
}

/// Parse and split in single operation
pub fn parse_and_split<T, F>(
    input: &str,
    delimiters: &[&str],
    parser: F,
) -> TokenParsingIterator<'_, F, T>
where
    F: Fn(&str) -> Result<T, ParseError>,
{
    TokenParsingIterator::new(input, delimiters.to_vec(), parser)
}
```

#### 2. Structured Data Parser with Validation

```rust
/// Parser for structured command-line arguments
#[derive(Debug, Clone)]
pub struct CommandParser<'a> 
{
    input: &'a str,
    token_delimiters: Vec<&'a str>,
    kv_separator: &'a str,
    flag_prefix: &'a str,
}

#[derive(Debug, Clone)]
pub enum ParsedToken<'a> 
{
    Command(&'a str),
    KeyValue { key: &'a str, value: &'a str },
    Flag(&'a str),
    Positional(&'a str),
}

impl<'a> CommandParser<'a> {
    pub fn new(input: &'a str) -> Self 
{
        Self {
            input,
            token_delimiters: vec![" ", "\t"],
            kv_separator: ":",
            flag_prefix: "--",
        }
    }
    
    /// Parse command line in single pass with context awareness
    pub fn parse_structured(self) -> impl Iterator<Item = Result<ParsedToken<'a>, ParseError>> + 'a 

{
        StructuredParsingIterator {
            parser: self,
            position: 0,
            current_context: ParsingContext::Command,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ParsingContext 
{
    Command,     // Expecting command name
    Arguments,   // Expecting arguments or flags
    Value,       // Expecting value after key
}

struct StructuredParsingIterator<'a> 
{
    parser: CommandParser<'a>,
    position: usize,
    current_context: ParsingContext,
}

impl<'a> Iterator for StructuredParsingIterator<'a> {
    type Item = Result<ParsedToken<'a>, ParseError>;
    
    fn next(&mut self) -> Option<Self::Item> 
{
        if self.position >= self.parser.input.len() {
            return None;
        }
        
        // Find next token boundary
        let token = match self.find_next_token() {
            Some(t) => t,
            None => return None,
        };
        
        // Parse based on current context and token characteristics  
        let result = match self.current_context {
            ParsingContext::Command => {
                self.current_context = ParsingContext::Arguments;
                Ok(ParsedToken::Command(token))
            },
            ParsingContext::Arguments => {
                self.parse_argument_token(token)
            },
            ParsingContext::Value => {
                self.current_context = ParsingContext::Arguments;
                Ok(ParsedToken::Positional(token)) // Previous token was expecting this value
            },
        };
        
        Some(result)
    }
}

impl<'a> StructuredParsingIterator<'a> {
    fn parse_argument_token(&mut self, token: &'a str) -> Result<ParsedToken<'a>, ParseError> 
{
        if token.starts_with(self.parser.flag_prefix) {
            // Flag argument
            let flag_name = &token[self.parser.flag_prefix.len()..];
            Ok(ParsedToken::Flag(flag_name))
        } else if token.contains(self.parser.kv_separator) {
            // Key-value pair
            let separator_pos = token.find(self.parser.kv_separator).unwrap();
            let key = &token[..separator_pos];
            let value = &token[separator_pos + self.parser.kv_separator.len()..];
            
            if key.is_empty() || value.is_empty() {
                Err(ParseError::InvalidKeyValuePair(token.to_string()))
            } else {
                Ok(ParsedToken::KeyValue { key, value })
            }
        } else {
            // Positional argument
            Ok(ParsedToken::Positional(token))
        }
    }
}
```

#### 3. Context-Aware CSV Parser

```rust
/// Advanced CSV parser with context-aware field processing  
pub struct ContextAwareCSVParser<'a, F> 
{
    input: &'a str,
    field_processors: Vec<F>, // One processor per column
    current_row: usize,
    current_col: usize,
    position: usize,
}

impl<'a, F> ContextAwareCSVParser<'a, F>
where
    F: Fn(&str, usize, usize) -> Result<String, ParseError>, // (field, row, col) -> processed_value
{
    pub fn new(input: &'a str, field_processors: Vec<F>) -> Self 
{
        Self {
            input,
            field_processors,
            current_row: 0,
            current_col: 0,
            position: 0,
        }
    }
    
    /// Parse CSV with column-specific processing
    pub fn parse_with_context(mut self) -> impl Iterator<Item = Result<Vec<String>, ParseError>> + 'a 

{
        std::iter::from_fn(move || {
            if self.position >= self.input.len() {
                return None;
            }
            
            let mut row = Vec::new();
            self.current_col = 0;
            
            // Parse entire row
            while let Some(field) = self.parse_csv_field() {
                // Apply column-specific processing
                let processed_field = if self.current_col < self.field_processors.len() {
                    match (self.field_processors[self.current_col])(field, self.current_row, self.current_col) {
                        Ok(processed) => processed,
                        Err(e) => return Some(Err(e)),
                    }
                } else {
                    field.to_string() // No processor for this column
                };
                
                row.push(processed_field);
                self.current_col += 1;
                
                // Check for end of row
                if self.at_end_of_row() {
                    break;
                }
            }
            
            self.current_row += 1;
            Some(Ok(row))
        })
    }
}
```

#### 4. Streaming Parser with Lookahead

```rust
use std::collections::VecDeque;

/// Streaming parser with configurable lookahead for context-sensitive parsing
pub struct StreamingParserWithLookahead<R: BufRead> 
{
    reader: R,
    lookahead_buffer: VecDeque<String>,
    lookahead_size: usize,
    delimiters: Vec<String>,
    position: usize,
}

impl<R: BufRead> StreamingParserWithLookahead<R> {
    pub fn new(reader: R, delimiters: Vec<String>, lookahead_size: usize) -> Self 
{
        Self {
            reader,
            lookahead_buffer: VecDeque::new(),
            lookahead_size,
            delimiters,
            position: 0,
        }
    }
    
    /// Fill lookahead buffer to enable context-aware parsing
    fn ensure_lookahead(&mut self) -> std::io::Result<()> 
{
        while self.lookahead_buffer.len() < self.lookahead_size {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;
            
            if bytes_read == 0 {
                break; // EOF
            }
            
            // Split line into tokens and add to lookahead
            let tokens: Vec<String> = line.split_whitespace()
                .map(|s| s.to_string())
                .collect();
            
            for token in tokens {
                self.lookahead_buffer.push_back(token);
            }
        }
        
        Ok(())
    }
    
    /// Parse with context from lookahead
    pub fn parse_with_context<T, F>(&mut self, parser: F) -> Result<Option<T>, ParseError>
    where
        F: Fn(&str, &[String]) -> Result<T, ParseError>, // (current_token, lookahead_context)
    {
        self.ensure_lookahead().map_err(ParseError::IoError)?;
        
        if let Some(current_token) = self.lookahead_buffer.pop_front() {
            // Provide lookahead context to parser
            let context: Vec<String> = self.lookahead_buffer.iter().cloned().collect();
            
            match parser(&current_token, &context) {
                Ok(result) => Ok(Some(result)),
                Err(e) => Err(e),
            }
        } else {
            Ok(None) // EOF
        }
    }
}
```

#### 5. High-Level Parsing Combinators

```rust
/// Parser combinator interface for complex parsing scenarios
pub struct ParseCombinator<'a> 
{
    input: &'a str,
    position: usize,
}

impl<'a> ParseCombinator<'a> {
    pub fn new(input: &'a str) -> Self 
{
        Self { input, position: 0 }
    }
    
    /// Parse sequence of tokens with different parsers
    pub fn sequence<T1, T2, F1, F2>(
        mut self,
        delim: &str,
        parser1: F1,
        parser2: F2,
    ) -> Result<(T1, T2), ParseError>
    where
        F1: Fn(&str) -> Result<T1, ParseError>,
        F2: Fn(&str) -> Result<T2, ParseError>,
    {
        let first_token = self.consume_until(delim)?;
        let second_token = self.consume_remaining();
        
        let first_result = parser1(first_token)?;
        let second_result = parser2(second_token)?;
        
        Ok((first_result, second_result))
    }
    
    /// Parse optional token with fallback
    pub fn optional<T, F>(
        mut self,
        delim: &str,
        parser: F,
        default: T,
    ) -> Result<T, ParseError>
    where
        F: Fn(&str) -> Result<T, ParseError>,
    {
        if let Ok(token) = self.consume_until(delim) {
            parser(token)
        } else {
            Ok(default)
        }
    }
    
    /// Parse repeated pattern
    pub fn repeat<T, F>(
        mut self,
        delim: &str,
        parser: F,
    ) -> Result<Vec<T>, ParseError>
    where
        F: Fn(&str) -> Result<T, ParseError>,
    {
        let mut results = Vec::new();
        
        while !self.at_end() {
            let token = self.consume_until(delim)?;
            results.push(parser(token)?);
        }
        
        Ok(results)
    }
}
```

#### 6. Integration with Existing Split Operations

```rust
/// Extension trait adding parser integration to existing split operations
pub trait ParserIntegrationExt {
    /// Parse tokens while splitting
    fn split_and_parse<T, F>(
        &self,
        delimiters: &[&str],
        parser: F,
    ) -> impl Iterator<Item = Result<T, ParseError>>
    where
        F: Fn(&str) -> Result<T, ParseError>;
    
    /// Split with validation
    fn split_with_validation<F>(
        &self,
        delimiters: &[&str],
        validator: F,
    ) -> impl Iterator<Item = Result<&str, ParseError>>
    where
        F: Fn(&str) -> bool;
    
    /// Parse structured command line
    fn parse_command_line(&self) -> impl Iterator<Item = Result<ParsedToken, ParseError>>;
}

impl ParserIntegrationExt for str 
{
    fn split_and_parse<T, F>(
        &self,
        delimiters: &[&str],
        parser: F,
    ) -> impl Iterator<Item = Result<T, ParseError>>
    where
        F: Fn(&str) -> Result<T, ParseError>,
    {
        parse_and_split(self, delimiters, parser)
    }
    
    fn split_with_validation<F>(
        &self,
        delimiters: &[&str],
        validator: F,
    ) -> impl Iterator<Item = Result<&str, ParseError>>
    where
        F: Fn(&str) -> bool,
    {
        string::split()
            .src(self)
            .delimeter(delimiters.to_vec())
            .perform()
            .map(move |token| {
                let token_str = token.string.as_ref();
                if validator(token_str) {
                    Ok(token_str)
                } else {
                    Err(ParseError::ValidationFailed(token_str.to_string()))
                }
            })
    }
    
    fn parse_command_line(&self) -> impl Iterator<Item = Result<ParsedToken, ParseError>> 

{
        CommandParser::new(self).parse_structured()
    }
}
```

### Technical Requirements

#### Parser Integration
- **Single-pass processing** combining tokenization and parsing
- **Context awareness** using lookahead and state tracking
- **Error propagation** with detailed error information
- **Memory efficiency** avoiding intermediate allocations

#### Performance Optimization
- **Cache-friendly access** patterns with sequential processing  
- **Minimal allocations** through in-place parsing where possible
- **SIMD integration** for pattern matching within parsers
- **Streaming support** for large input processing

#### API Design
- **Combinator interface** for complex parsing scenarios
- **Type safety** with compile-time parser validation
- **Error handling** with detailed parse error information
- **Backward compatibility** with existing string operations

### Performance Targets

| Parsing Scenario | Multi-Pass Approach | Integrated Parsing | Improvement |
|------------------|---------------------|-------------------|-------------|
| **Command line parsing** | 1.2μs | 0.45μs | **2.7x faster** |
| **CSV with validation** | 2.8μs/row | 1.1μs/row | **2.5x faster** |
| **Key-value extraction** | 890ns | 340ns | **2.6x faster** |
| **Structured data parsing** | 3.4μs | 1.3μs | **2.6x faster** |

#### Memory Usage Improvement
- **Intermediate allocations**: 80% reduction through single-pass processing
- **Peak memory**: 40-60% reduction by avoiding temporary collections
- **Cache misses**: 50% reduction through sequential data access
- **Parser state**: Minimal memory overhead for context tracking

### Implementation Steps

1. **Implement single-pass token parsing** with generic parser functions
2. **Create structured command-line parser** with context awareness  
3. **Add CSV parser with column-specific processing** and validation
4. **Implement streaming parser** with configurable lookahead
5. **Build parser combinator interface** for complex scenarios
6. **Integrate with existing split APIs** maintaining compatibility
7. **Comprehensive testing and benchmarking** across parsing scenarios

### Challenges & Solutions

#### Challenge: Context Management Complexity
**Solution**: State machine approach with clear context transitions
```rust
#[derive(Debug, Clone, Copy)]
enum ParserState 
{
    Initial,
    ExpectingValue(usize), // Parameter: expected value type ID
    InQuotedString,
    EscapeSequence,
}

impl ParserStateMachine 
{
    fn transition(&mut self, token: &str) -> Result<ParserState, ParseError> 
{
        match (self.current_state, token) {
            (ParserState::Initial, token) if token.starts_with('"') => {
                Ok(ParserState::InQuotedString)
            },
            (ParserState::ExpectingValue(type_id), token) => {
                self.validate_value(token, type_id)?;
                Ok(ParserState::Initial)
            },
            // ... other transitions
        }
    }
}
```

#### Challenge: Error Propagation in Single Pass
**Solution**: Detailed error types with position information
```rust
#[derive(Debug, Clone)]
pub enum ParseError 
{
    InvalidToken { token: String, position: usize, expected: String },
    ValidationFailed { token: String, position: usize, reason: String },
    UnexpectedEof { position: usize, expected: String },
    IoError(std::io::Error),
}

impl ParseError 
{
    pub fn with_position(mut self, pos: usize) -> Self 
{
        match &mut self {
            ParseError::InvalidToken { position, .. } => *position = pos,
            ParseError::ValidationFailed { position, .. } => *position = pos,
            ParseError::UnexpectedEof { position, .. } => *position = pos,
            _ => {},
        }
        self
    }
}
```

#### Challenge: Type Safety with Generic Parsers
**Solution**: Parser trait with associated types and compile-time validation
```rust
pub trait TokenParser<'a> {
    type Output;
    type Error;
    
    fn parse(&self, token: &'a str, context: &ParserContext) -> Result<Self::Output, Self::Error>;
    
    /// Validate parser at compile time
    fn validate_parser() -> Result<(), &'static str> 
{
        // Compile-time validation logic
        Ok(())
    }
}

// Usage with compile-time validation
struct IntParser;
impl<'a> TokenParser<'a> for IntParser {
    type Output = i32;
    type Error = ParseError;
    
    fn parse(&self, token: &'a str, _: &ParserContext) -> Result<i32, ParseError> 
{
        token.parse().map_err(|_| ParseError::InvalidToken {
            token: token.to_string(),
            position: 0,
            expected: "integer".to_string(),
        })
    }
}
```

### Success Criteria

- [ ] **50% improvement** in command-line parsing performance
- [ ] **40% improvement** in CSV processing with validation
- [ ] **30% reduction** in memory usage for parsing pipelines
- [ ] **Single-pass processing** for all common parsing scenarios
- [ ] **Detailed error reporting** with position and context information
- [ ] **Backward compatibility** with existing parsing code

### Benchmarking Strategy

#### Parser Integration Benchmarks
```rust
#[bench]
fn bench_multipass_command_parsing(b: &mut Bencher) 
{
    let input = "command arg1:value1 arg2:value2 --flag positional";
    
    b.iter(|| {
        // Traditional multi-pass approach
        let tokens: Vec<String> = split().src(input).delimeter(" ").perform().collect();
        let mut results = Vec::new();
        
        for token in tokens {
            if token.starts_with("--") {
                results.push(ParsedToken::Flag(&token[2..]));
            } else if token.contains(':') {
                let parts: Vec<_> = token.split(':').collect();
                results.push(ParsedToken::KeyValue { 
                    key: parts[0], 
                    value: parts[1] 
                });
            } else {
                results.push(ParsedToken::Positional(token.as_str()));
            }
        }
        
        black_box(results)
    });
}

#[bench]
fn bench_integrated_command_parsing(b: &mut Bencher) 
{
    let input = "command arg1:value1 arg2:value2 --flag positional";
    
    b.iter(|| {
        let results: Result<Vec<_>, _> = input
            .parse_command_line()
            .collect();
        black_box(results)
    });
}
```

#### Memory Allocation Tracking
- **Allocation count** comparison between multi-pass and single-pass
- **Peak memory usage** measurement during parsing operations
- **Cache performance** analysis using hardware performance counters
- **Throughput scaling** with input size and complexity

### Integration Points

#### SIMD Compatibility
- Parser-aware SIMD pattern matching for delimiter detection
- Bulk validation operations using SIMD instructions
- Optimized character classification for parsing operations

#### Zero-Copy Integration  
- Zero-copy token extraction with lifetime management
- In-place parsing for compatible data types
- Copy-on-write for parsed results requiring ownership

### Usage Examples

#### Basic Parser Integration
```rust
use strs_tools::parser::ParserIntegrationExt;

// Parse integers while splitting
let numbers: Result<Vec<i32>, _> = "1,2,3,4,5"
    .split_and_parse(&[","], |token| token.parse())
    .collect();

// Parse command line arguments
let parsed_args: Result<Vec<ParsedToken>, _> = "app --verbose input.txt output.txt"
    .parse_command_line()
    .collect();

// CSV with column validation
let csv_data = "name,age,email\nJohn,25,john@example.com\nJane,30,jane@example.com";
let validated_rows: Result<Vec<Vec<String>>, _> = csv_data
    .split_and_parse(&["\n"], |line| {
        line.split_and_parse(&[","], |field| {
            // Validate each field based on column
            Ok(field.trim().to_string())
        }).collect()
    })
    .collect();
```

#### Advanced Parser Combinators
```rust  
use strs_tools::parser::ParseCombinator;

// Parse key-value pairs with optional defaults
let config_parser = ParseCombinator::new("timeout:30,retries:3,debug");
let (timeout, retries, debug) = config_parser
    .sequence(":", |k| k.parse(), |v| v.parse::<i32>())
    .and_then(|(k, v)| match k {
        "timeout" => Ok(v),
        _ => Err(ParseError::UnknownKey(k.to_string())),
    })?;
```

### Documentation Requirements

Update documentation with:
- **Parser integration guide** showing single-pass vs multi-pass patterns
- **Error handling strategies** for parsing operations
- **Performance optimization tips** for different parsing scenarios
- **Migration guide** from traditional parsing approaches

### Related Tasks

- Task 001: SIMD optimization (parser-aware SIMD pattern matching)
- Task 002: Zero-copy optimization (zero-copy parsing with lifetime management)
- Task 006: Streaming evaluation (streaming parser integration)
- Task 007: Specialized algorithms (parsing-specific algorithm selection)