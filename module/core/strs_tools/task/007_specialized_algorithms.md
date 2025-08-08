# Task 007: Specialized Algorithm Implementations

## Priority: Medium
## Impact: 2-4x improvement for specific pattern types and use cases  
## Estimated Effort: 4-5 days

## Problem Statement

Current `strs_tools` uses generic algorithms for all splitting scenarios, missing optimization opportunities for specific pattern types:

```rust
// All these use the same generic algorithm:
split().src(text).delimeter(" ").perform();           // ← Single char could use memchr
split().src(text).delimeter("::").perform();          // ← Fixed pattern could use Boyer-Moore  
split().src(csv).delimeter(",").perform();            // ← CSV could use specialized parser
split().src(url).delimeter(["://", "/", "?", "#"]).perform(); // ← URL could use state machine
```

This leads to suboptimal performance:
- **Single character delimiters**: Generic algorithm vs optimized byte search
- **Fixed patterns**: Linear search vs Boyer-Moore/KMP preprocessing
- **CSV/TSV parsing**: Generic split vs specialized CSV handling
- **Structured data**: Pattern matching vs state machine parsing

## Solution Approach

Implement specialized algorithms tailored to common string processing patterns, with automatic algorithm selection based on input characteristics.

### Implementation Plan

#### 1. Single Character Optimization

```rust
/// Highly optimized single character splitting
pub struct SingleCharSplitIterator<'a> {
    input: &'a str,
    delimiter: u8, // ASCII byte for maximum performance
    position: usize,
    preserve_delimiter: bool,
}

impl<'a> SingleCharSplitIterator<'a> {
    pub fn new(input: &'a str, delimiter: char, preserve_delimiter: bool) -> Self {
        assert!(delimiter.is_ascii(), "Single char optimization requires ASCII delimiter");
        
        Self {
            input,
            delimiter: delimiter as u8,
            position: 0,
            preserve_delimiter,
        }
    }
    
    /// Use memchr for ultra-fast single byte search
    fn find_next_delimiter(&self) -> Option<usize> {
        memchr::memchr(self.delimiter, &self.input.as_bytes()[self.position..])
            .map(|pos| self.position + pos)
    }
}

impl<'a> Iterator for SingleCharSplitIterator<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            return None;
        }
        
        match self.find_next_delimiter() {
            Some(delim_pos) => {
                let segment = &self.input[self.position..delim_pos];
                
                if self.preserve_delimiter {
                    // Return segment, delimiter will be next
                    self.position = delim_pos;
                    Some(segment)
                } else {
                    // Skip delimiter
                    self.position = delim_pos + 1;
                    Some(segment)
                }
            },
            None => {
                // Return remaining content
                let remaining = &self.input[self.position..];
                self.position = self.input.len();
                Some(remaining)
            }
        }
    }
}
```

#### 2. Boyer-Moore for Fixed Patterns

```rust
/// Boyer-Moore algorithm for efficient fixed pattern matching
pub struct BoyerMooreSplitIterator<'a> {
    input: &'a str,
    pattern: &'a str,
    bad_char_table: [usize; 256], // ASCII bad character table
    position: usize,
}

impl<'a> BoyerMooreSplitIterator<'a> {
    pub fn new(input: &'a str, pattern: &'a str) -> Self {
        let mut bad_char_table = [pattern.len(); 256];
        
        // Build bad character table
        for (i, &byte) in pattern.as_bytes().iter().enumerate() {
            bad_char_table[byte as usize] = pattern.len() - i - 1;
        }
        
        Self {
            input,
            pattern,
            bad_char_table,
            position: 0,
        }
    }
    
    /// Boyer-Moore pattern search with bad character heuristic
    fn find_next_pattern(&self) -> Option<usize> {
        let text = self.input.as_bytes();
        let pattern = self.pattern.as_bytes();
        let text_len = text.len();
        let pattern_len = pattern.len();
        
        if self.position + pattern_len > text_len {
            return None;
        }
        
        let mut i = self.position + pattern_len - 1; // Start from end of pattern
        
        while i < text_len {
            let mut j = pattern_len - 1;
            
            // Compare from right to left
            while j < pattern_len && text[i] == pattern[j] {
                if j == 0 {
                    return Some(i); // Found complete match
                }
                i -= 1;
                j -= 1;
            }
            
            // Bad character heuristic
            let bad_char_skip = self.bad_char_table[text[i] as usize];
            i += std::cmp::max(1, bad_char_skip);
        }
        
        None
    }
}

impl<'a> Iterator for BoyerMooreSplitIterator<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            return None;
        }
        
        match self.find_next_pattern() {
            Some(match_pos) => {
                let segment = &self.input[self.position..match_pos];
                self.position = match_pos + self.pattern.len();
                Some(segment)
            },
            None => {
                let remaining = &self.input[self.position..];
                self.position = self.input.len();
                Some(remaining)
            }
        }
    }
}
```

#### 3. Specialized CSV/TSV Parser

```rust
/// High-performance CSV parser with quote handling
pub struct CSVSplitIterator<'a> {
    input: &'a str,
    delimiter: u8, // ',' or '\t'  
    quote_char: u8, // '"'
    escape_char: u8, // '"' (double quote) or '\\' 
    position: usize,
    in_quoted_field: bool,
}

impl<'a> CSVSplitIterator<'a> {
    pub fn new(input: &'a str, delimiter: char) -> Self {
        Self {
            input,
            delimiter: delimiter as u8,
            quote_char: b'"',
            escape_char: b'"', // CSV standard: double quote to escape
            position: 0,
            in_quoted_field: false,
        }
    }
    
    /// Parse next CSV field with proper quote handling
    fn parse_csv_field(&mut self) -> Option<String> {
        let bytes = self.input.as_bytes();
        let mut field = String::new();
        let mut start_pos = self.position;
        
        // Skip leading whitespace (optional)
        while start_pos < bytes.len() && bytes[start_pos] == b' ' {
            start_pos += 1;
        }
        
        if start_pos >= bytes.len() {
            return None;
        }
        
        // Check if field starts with quote
        if bytes[start_pos] == self.quote_char {
            self.in_quoted_field = true;
            start_pos += 1; // Skip opening quote
        }
        
        let mut i = start_pos;
        while i < bytes.len() {
            let current_byte = bytes[i];
            
            if self.in_quoted_field {
                if current_byte == self.quote_char {
                    // Check for escaped quote
                    if i + 1 < bytes.len() && bytes[i + 1] == self.quote_char {
                        field.push('"'); // Add single quote to result
                        i += 2; // Skip both quotes
                    } else {
                        // End of quoted field
                        self.in_quoted_field = false;
                        i += 1; // Skip closing quote
                        break;
                    }
                } else {
                    field.push(current_byte as char);
                    i += 1;
                }
            } else {
                if current_byte == self.delimiter {
                    break; // Found field delimiter
                } else {
                    field.push(current_byte as char);
                    i += 1;
                }
            }
        }
        
        // Skip delimiter if present
        if i < bytes.len() && bytes[i] == self.delimiter {
            i += 1;
        }
        
        self.position = i;
        Some(field)
    }
}

impl<'a> Iterator for CSVSplitIterator<'a> {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.parse_csv_field()
    }
}
```

#### 4. State Machine for Structured Data

```rust
/// State machine parser for structured formats (URLs, paths, etc.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserState {
    Scheme,    // http, https, ftp, etc.
    Authority, // //domain:port
    Path,      // /path/to/resource
    Query,     // ?param=value
    Fragment,  // #anchor
}

pub struct StateMachineSplitIterator<'a> {
    input: &'a str,
    current_state: ParserState,
    position: usize,
    transitions: &'a [(ParserState, &'a [u8], ParserState)], // (from_state, trigger_bytes, to_state)
}

impl<'a> StateMachineSplitIterator<'a> {
    /// Create URL parser with predefined state transitions
    pub fn new_url_parser(input: &'a str) -> Self {
        const URL_TRANSITIONS: &[(ParserState, &[u8], ParserState)] = &[
            (ParserState::Scheme, b"://", ParserState::Authority),
            (ParserState::Authority, b"/", ParserState::Path), 
            (ParserState::Path, b"?", ParserState::Query),
            (ParserState::Path, b"#", ParserState::Fragment),
            (ParserState::Query, b"#", ParserState::Fragment),
        ];
        
        Self {
            input,
            current_state: ParserState::Scheme,
            position: 0,
            transitions: URL_TRANSITIONS,
        }
    }
    
    /// Find next state transition
    fn find_next_transition(&self) -> Option<(usize, ParserState)> {
        let remaining = &self.input[self.position..];
        
        for &(from_state, trigger_bytes, to_state) in self.transitions {
            if from_state == self.current_state {
                if let Some(pos) = remaining.find(std::str::from_utf8(trigger_bytes).ok()?) {
                    return Some((self.position + pos, to_state));
                }
            }
        }
        
        None
    }
}

impl<'a> Iterator for StateMachineSplitIterator<'a> {
    type Item = (ParserState, &'a str);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            return None;
        }
        
        match self.find_next_transition() {
            Some((transition_pos, next_state)) => {
                let segment = &self.input[self.position..transition_pos];
                let current_state = self.current_state;
                
                // Move past the trigger sequence
                let trigger_len = self.transitions
                    .iter()
                    .find(|(from, _, to)| *from == current_state && *to == next_state)
                    .map(|(_, trigger, _)| trigger.len())
                    .unwrap_or(0);
                
                self.position = transition_pos + trigger_len;
                self.current_state = next_state;
                
                Some((current_state, segment))
            },
            None => {
                // No more transitions, return remaining content
                let remaining = &self.input[self.position..];
                let current_state = self.current_state;
                self.position = self.input.len();
                
                Some((current_state, remaining))
            }
        }
    }
}
```

#### 5. Automatic Algorithm Selection

```rust
/// Analyze input to select optimal algorithm
pub struct AlgorithmSelector;

impl AlgorithmSelector {
    /// Select best algorithm based on delimiter characteristics
    pub fn select_split_algorithm(delimiters: &[&str]) -> SplitAlgorithm {
        if delimiters.len() == 1 {
            let delim = delimiters[0];
            if delim.len() == 1 && delim.chars().next().unwrap().is_ascii() {
                return SplitAlgorithm::SingleChar;
            } else if delim.len() <= 8 && delim.is_ascii() {
                return SplitAlgorithm::BoyerMoore;
            }
        }
        
        if Self::is_csv_pattern(delimiters) {
            return SplitAlgorithm::CSV;
        }
        
        if Self::is_url_pattern(delimiters) {
            return SplitAlgorithm::StateMachine;
        }
        
        if delimiters.len() <= 8 {
            return SplitAlgorithm::AhoCorasick;
        }
        
        SplitAlgorithm::Generic
    }
    
    fn is_csv_pattern(delimiters: &[&str]) -> bool {
        delimiters.len() == 1 && 
        (delimiters[0] == "," || delimiters[0] == "\t" || delimiters[0] == ";")
    }
    
    fn is_url_pattern(delimiters: &[&str]) -> bool {
        let url_delims = ["://", "/", "?", "#"];
        delimiters.iter().all(|d| url_delims.contains(d))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SplitAlgorithm {
    SingleChar,      // memchr optimization
    BoyerMoore,      // Fixed pattern search
    CSV,             // CSV-specific parsing
    StateMachine,    // Structured data parsing
    AhoCorasick,     // Multi-pattern SIMD
    Generic,         // Fallback algorithm
}
```

#### 6. Unified API with Algorithm Selection

```rust
/// Smart split that automatically selects optimal algorithm
pub fn smart_split(input: &str, delimiters: &[&str]) -> Box<dyn Iterator<Item = &str> + '_> {
    let algorithm = AlgorithmSelector::select_split_algorithm(delimiters);
    
    match algorithm {
        SplitAlgorithm::SingleChar => {
            let delim_char = delimiters[0].chars().next().unwrap();
            Box::new(SingleCharSplitIterator::new(input, delim_char, false))
        },
        SplitAlgorithm::BoyerMoore => {
            Box::new(BoyerMooreSplitIterator::new(input, delimiters[0]))
        },
        SplitAlgorithm::CSV => {
            let csv_delim = delimiters[0].chars().next().unwrap();
            // Convert String iterator to &str iterator
            Box::new(CSVSplitIterator::new(input, csv_delim).map(|s| {
                // This is a limitation - CSV needs to return owned strings
                // due to quote processing, but interface expects &str
                // In practice, would need different return types or Cow<str>
                Box::leak(s.into_boxed_str()) as &str
            }))
        },
        SplitAlgorithm::StateMachine => {
            Box::new(StateMachineSplitIterator::new_url_parser(input)
                .map(|(_, segment)| segment))
        },
        SplitAlgorithm::AhoCorasick => {
            // Use existing SIMD implementation
            Box::new(crate::simd::simd_split_cached(input, delimiters)
                .unwrap_or_else(|_| panic!("SIMD split failed"))
                .map(|split| split.string.as_ref()))
        },
        SplitAlgorithm::Generic => {
            // Use existing generic implementation
            Box::new(crate::string::split()
                .src(input)
                .delimeter(delimiters.to_vec())
                .perform()
                .map(|s| Box::leak(s.string.into_owned().into_boxed_str()) as &str))
        },
    }
}
```

### Technical Requirements

#### Algorithm Selection
- **Automatic detection** of optimal algorithm based on input patterns
- **Performance profiling** for algorithm switching thresholds
- **Fallback mechanisms** when specialized algorithms fail
- **Runtime adaptation** based on observed performance characteristics

#### Performance Characteristics  
- **Single character**: 5-10x improvement using memchr
- **Fixed patterns**: 2-4x improvement using Boyer-Moore  
- **CSV parsing**: 3-6x improvement with specialized parser
- **Structured data**: 2-3x improvement with state machines

#### Correctness Guarantees
- **Algorithm equivalence** - all algorithms produce identical results
- **Edge case handling** - proper behavior for empty inputs, edge cases
- **Memory safety** - no buffer overruns or undefined behavior
- **Unicode compatibility** where applicable

### Performance Targets

| Pattern Type | Generic Algorithm | Specialized Algorithm | Improvement |
|--------------|-------------------|----------------------|-------------|
| **Single char delimiter** | 89.2ns | 18.4ns | **4.8x faster** |
| **Fixed pattern (2-8 chars)** | 145.6ns | 52.3ns | **2.8x faster** |
| **CSV with quotes** | 234.7ns | 78.9ns | **3.0x faster** |
| **URL parsing** | 298.1ns | 134.5ns | **2.2x faster** |
| **Multi-pattern (2-8)** | 456.2ns | 198.7ns | **2.3x faster** |

#### Algorithm Selection Overhead
- **Pattern analysis**: < 1μs for typical delimiter sets
- **Algorithm dispatch**: < 10ns runtime overhead
- **Memory footprint**: < 1KB additional for specialized algorithms
- **Compilation impact**: Acceptable binary size increase

### Implementation Steps

1. **Implement single character** optimization using memchr
2. **Add Boyer-Moore algorithm** for fixed pattern matching
3. **Create specialized CSV parser** with proper quote handling
4. **Implement state machine parser** for structured data formats
5. **Build algorithm selection logic** with automatic detection
6. **Integrate with existing APIs** maintaining backward compatibility
7. **Comprehensive benchmarking** comparing all algorithm variants

### Challenges & Solutions

#### Challenge: Algorithm Selection Complexity
**Solution**: Hierarchical decision tree with performance profiling
```rust
impl AlgorithmSelector {
    fn select_with_profiling(delimiters: &[&str], input_size: usize) -> SplitAlgorithm {
        // Use input size to influence algorithm selection
        match (delimiters.len(), input_size) {
            (1, _) if Self::is_single_ascii_char(delimiters[0]) => SplitAlgorithm::SingleChar,
            (1, 0..=1024) => SplitAlgorithm::Generic, // Small inputs don't benefit from Boyer-Moore
            (1, _) => SplitAlgorithm::BoyerMoore,
            (2..=8, 10000..) => SplitAlgorithm::AhoCorasick, // Large inputs benefit from SIMD
            _ => SplitAlgorithm::Generic,
        }
    }
}
```

#### Challenge: Return Type Consistency
**Solution**: Unified return types using Cow<str> or trait objects
```rust
pub enum SplitResult<'a> {
    Borrowed(&'a str),
    Owned(String),
}

impl<'a> AsRef<str> for SplitResult<'a> {
    fn as_ref(&self) -> &str {
        match self {
            SplitResult::Borrowed(s) => s,
            SplitResult::Owned(s) => s.as_str(),
        }
    }
}
```

#### Challenge: Memory Management Complexity
**Solution**: Algorithm-specific memory pools and RAII cleanup
```rust
pub struct SpecializedSplitIterator<'a> {
    algorithm: SplitAlgorithm,
    iterator: Box<dyn Iterator<Item = SplitResult<'a>> + 'a>,
    cleanup: Option<Box<dyn FnOnce() + 'a>>, // Algorithm-specific cleanup
}

impl<'a> Drop for SpecializedSplitIterator<'a> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}
```

### Success Criteria

- [ ] **5x improvement** for single character delimiters using memchr
- [ ] **3x improvement** for fixed patterns using Boyer-Moore
- [ ] **3x improvement** for CSV parsing with specialized parser
- [ ] **2x improvement** for structured data using state machines
- [ ] **Automatic algorithm selection** with < 1μs overhead
- [ ] **100% correctness** - all algorithms produce identical results

### Benchmarking Strategy

#### Algorithm Comparison Benchmarks
```rust
#[bench]
fn bench_single_char_generic(b: &mut Bencher) {
    let input = "word1 word2 word3 word4".repeat(1000);
    b.iter(|| {
        let result: Vec<_> = generic_split(&input, &[" "]).collect();
        black_box(result)
    });
}

#[bench]  
fn bench_single_char_specialized(b: &mut Bencher) {
    let input = "word1 word2 word3 word4".repeat(1000);
    b.iter(|| {
        let result: Vec<_> = SingleCharSplitIterator::new(&input, ' ', false).collect();
        black_box(result)
    });
}

#[bench]
fn bench_boyer_moore_vs_generic(b: &mut Bencher) {
    let input = "field1::field2::field3::field4".repeat(1000);
    
    // Test both algorithms for comparison
    b.iter(|| {
        let generic_result: Vec<_> = generic_split(&input, &["::"]).collect(); 
        let bm_result: Vec<_> = BoyerMooreSplitIterator::new(&input, "::").collect();
        
        assert_eq!(generic_result, bm_result); // Correctness check
        black_box((generic_result, bm_result))
    });
}
```

#### Algorithm Selection Accuracy
- **Selection overhead** measurement with high-precision timers
- **Accuracy validation** - verify optimal algorithm chosen for different inputs
- **Fallback behavior** testing when specialized algorithms fail
- **Performance regression** detection across algorithm boundaries

### Integration Points

#### SIMD Compatibility
- Specialized algorithms can use SIMD internally (e.g., Boyer-Moore with SIMD)
- Algorithm selection considers SIMD availability  
- Hybrid approaches combining specialization with SIMD acceleration

#### Zero-Copy Integration
- All specialized algorithms support zero-copy where possible
- Lifetime management for borrowed vs owned results
- Memory pool integration for owned string results

### Usage Examples

#### Automatic Algorithm Selection
```rust
use strs_tools::smart_split;

// Automatically uses SingleChar algorithm (memchr)  
let words: Vec<&str> = smart_split("word1 word2 word3", &[" "]).collect();

// Automatically uses Boyer-Moore algorithm
let parts: Vec<&str> = smart_split("a::b::c::d", &["::"]).collect();

// Automatically uses CSV algorithm  
let fields: Vec<&str> = smart_split("name,\"value, with comma\",123", &[","]).collect();

// Automatically uses StateMachine algorithm
let url_parts: Vec<&str> = smart_split("https://example.com/path?query=value#anchor", 
                                       &["://", "/", "?", "#"]).collect();
```

#### Manual Algorithm Control
```rust
use strs_tools::{SingleCharSplitIterator, BoyerMooreSplitIterator, CSVSplitIterator};

// Force specific algorithm for performance-critical code
let fast_split = SingleCharSplitIterator::new(input, ',', false);
let boyer_moore = BoyerMooreSplitIterator::new(input, "::");
let csv_parser = CSVSplitIterator::new(csv_input, ',');
```

### Documentation Requirements

Update documentation with:
- **Algorithm selection guide** explaining when each algorithm is optimal
- **Performance characteristics** for different algorithm and input combinations  
- **Manual algorithm control** for performance-critical applications
- **Correctness guarantees** and equivalence testing between algorithms

### Related Tasks

- Task 001: SIMD optimization (hybrid SIMD + specialized algorithm approaches)
- Task 002: Zero-copy optimization (zero-copy support in specialized algorithms)
- Task 003: Compile-time optimization (compile-time algorithm selection)
- Task 006: Streaming evaluation (specialized algorithms for streaming inputs)