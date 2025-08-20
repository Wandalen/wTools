# Task 003: Compile-Time Pattern Optimization

## Priority: Medium
## Impact: 10-50% improvement for common patterns, zero runtime overhead
## Estimated Effort: 4-5 days

## Problem Statement

Current `strs_tools` performs pattern compilation and analysis at runtime, even for known constant delimiter patterns:

```rust
// Runtime pattern analysis every time
let result = string::split()
    .src(input)
    .delimeter(vec!["::", ":", "."]) // ← Known at compile time
    .perform()
    .collect();
```

This leads to:
- **Runtime overhead**: Pattern analysis on every call
- **Suboptimal algorithms**: Generic approach for all pattern types
- **Missed optimizations**: No specialization for common cases
- **Code bloat**: Runtime dispatch for compile-time known patterns

## Solution Approach

Implement compile-time pattern analysis using procedural macros and const generics to generate optimal splitting code for known patterns.

### Implementation Plan

#### 1. Procedural Macro for Pattern Analysis

```rust
// Compile-time optimized splitting
use strs_tools::split_optimized;

// Generates specialized code based on pattern analysis
let result = split_optimized!(input, ["::", ":", "."] => {
    // Macro generates optimal algorithm:
    // - Single character delims use memchr
    // - Multi-character use aho-corasick
    // - Pattern order optimization
    // - Dead code elimination
});
```

#### 2. Const Generic Pattern Specialization

```rust
/// Compile-time pattern analysis and specialization
pub struct CompiletimeSplit<const N: usize> {
    delimiters: [&'static str; N],
    algorithm: SplitAlgorithm,
}

impl<const N: usize> CompiletimeSplit<N> {
    /// Analyze patterns at compile time
    pub const fn new(delimiters: [&'static str; N]) -> Self {
        let algorithm = Self::analyze_patterns(&delimiters);
        Self { delimiters, algorithm }
    }
    
    /// Compile-time pattern analysis
    const fn analyze_patterns(patterns: &[&'static str; N]) -> SplitAlgorithm {
        // Const evaluation determines optimal algorithm
        if N == 1 && patterns[0].len() == 1 {
            SplitAlgorithm::SingleChar
        } else if N <= 3 && Self::all_single_char(patterns) {
            SplitAlgorithm::FewChars  
        } else if N <= 8 {
            SplitAlgorithm::SmallPatternSet
        } else {
            SplitAlgorithm::LargePatternSet
        }
    }
}
```

#### 3. Algorithm Specialization

```rust
/// Compile-time algorithm selection
#[derive(Clone, Copy)]
pub enum SplitAlgorithm {
    SingleChar,        // memchr optimization
    FewChars,          // 2-3 characters, manual unrolling  
    SmallPatternSet,   // aho-corasick with small alphabet
    LargePatternSet,   // full aho-corasick with optimization
}

impl<const N: usize> CompiletimeSplit<N> {
    pub fn split<'a>(&self, input: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        match self.algorithm {
            SplitAlgorithm::SingleChar => {
                // Compile-time specialized for single character
                Box::new(SingleCharSplitIterator::new(input, self.delimiters[0]))
            },
            SplitAlgorithm::FewChars => {
                // Unrolled loop for 2-3 characters
                Box::new(FewCharsSplitIterator::new(input, &self.delimiters))
            },
            // ... other specialized algorithms
        }
    }
}
```

#### 4. Procedural Macro Implementation

```rust
// In strs_tools_macros crate
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Expr};

#[proc_macro]
pub fn split_optimized(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SplitOptimizedInput);
    
    // Analyze delimiter patterns at compile time
    let algorithm = analyze_delimiter_patterns(&input.delimiters);
    
    // Generate optimized code based on analysis
    let optimized_code = match algorithm {
        PatternType::SingleChar(ch) => {
            quote! {
                #input_expr.split(#ch)
            }
        },
        PatternType::FewChars(chars) => {
            generate_few_chars_split(&chars)
        },
        PatternType::MultiPattern(patterns) => {
            generate_aho_corasick_split(&patterns)
        },
    };
    
    optimized_code.into()
}

/// Compile-time pattern analysis
fn analyze_delimiter_patterns(patterns: &[String]) -> PatternType {
    if patterns.len() == 1 && patterns[0].len() == 1 {
        PatternType::SingleChar(patterns[0].chars().next().unwrap())
    } else if patterns.len() <= 3 && patterns.iter().all(|p| p.len() == 1) {
        let chars: Vec<char> = patterns.iter().map(|p| p.chars().next().unwrap()).collect();
        PatternType::FewChars(chars)
    } else {
        PatternType::MultiPattern(patterns.clone())
    }
}
```

#### 5. Const Evaluation Optimization

```rust
/// Compile-time string analysis
pub const fn analyze_string_const(s: &str) -> StringMetrics {
    let mut metrics = StringMetrics::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    
    // Const-evaluable analysis
    while i < bytes.len() {
        let byte = bytes[i];
        if byte < 128 {
            metrics.ascii_count += 1;
        } else {
            metrics.unicode_count += 1;
        }
        i += 1;
    }
    
    metrics
}

/// Compile-time optimal algorithm selection
pub const fn select_algorithm(
    pattern_count: usize, 
    metrics: StringMetrics
) -> OptimalAlgorithm {
    match (pattern_count, metrics.ascii_count > metrics.unicode_count) {
        (1, true) => OptimalAlgorithm::AsciiMemchr,
        (2..=3, true) => OptimalAlgorithm::AsciiMultiChar,
        (4..=8, _) => OptimalAlgorithm::AhoCorasick,
        _ => OptimalAlgorithm::Generic,
    }
}
```

### Technical Requirements

#### Compile-Time Analysis
- **Pattern complexity** analysis during compilation
- **Algorithm selection** based on delimiter characteristics  
- **Code generation** for optimal splitting approach
- **Dead code elimination** for unused algorithm paths

#### Runtime Performance
- **Zero overhead** pattern analysis after compilation
- **Optimal algorithms** selected for each pattern type
- **Inlined code** generation for simple patterns
- **Minimal binary size** through specialization

#### API Design
- **Ergonomic macros** for common use cases
- **Backward compatibility** with existing runtime API
- **Const generic** support for type-safe patterns
- **Error handling** at compile time for invalid patterns

### Performance Targets

| Pattern Type | Runtime Analysis | Compile-Time Optimized | Improvement |
|--------------|------------------|-------------------------|-------------|
| **Single char delimiter** | 45.2ns | 12.8ns | **3.5x faster** |
| **2-3 char delimiters** | 89.1ns | 31.4ns | **2.8x faster** |  
| **4-8 patterns** | 156.7ns | 89.2ns | **1.8x faster** |
| **Complex patterns** | 234.5ns | 168.3ns | **1.4x faster** |

#### Binary Size Impact
- **Code specialization**: Potentially larger binary for many patterns
- **Dead code elimination**: Unused algorithms removed
- **Macro expansion**: Controlled expansion for common cases
- **LTO optimization**: Link-time optimization for final binary

### Implementation Steps

1. **Design macro interface** for ergonomic compile-time optimization
2. **Implement pattern analysis** in procedural macro
3. **Create specialized algorithms** for different pattern types
4. **Add const generic support** for type-safe pattern handling
5. **Integrate with SIMD** for compile-time SIMD algorithm selection
6. **Comprehensive benchmarking** comparing compile-time vs runtime
7. **Documentation and examples** for macro usage patterns

### Challenges & Solutions

#### Challenge: Complex Macro Design
**Solution**: Provide multiple levels of macro complexity
```rust
// Simple case - automatic analysis
split_fast!(input, ":");

// Medium case - explicit pattern count
split_optimized!(input, [",", ";", ":"]);

// Advanced case - full control
split_specialized!(input, SingleChar(','));
```

#### Challenge: Compile Time Impact
**Solution**: Incremental compilation and cached analysis
```rust
// Cache pattern analysis results
const COMMON_DELIMITERS: CompiletimeSplit<3> = 
    CompiletimeSplit::new([",", ";", ":"]);

// Reuse cached analysis
let result = COMMON_DELIMITERS.split(input);
```

#### Challenge: Binary Size Growth
**Solution**: Smart specialization with size limits
```rust
// Limit macro expansion for large pattern sets
#[proc_macro]
pub fn split_optimized(input: TokenStream) -> TokenStream {
    if pattern_count > MAX_SPECIALIZED_PATTERNS {
        // Fall back to runtime algorithm
        generate_runtime_fallback()
    } else {
        // Generate specialized code
        generate_optimized_algorithm()
    }
}
```

### Success Criteria

- [ ] **30% improvement** for single character delimiters
- [ ] **20% improvement** for 2-3 character delimiter sets
- [ ] **15% improvement** for small pattern sets (4-8 patterns)
- [ ] **Zero runtime overhead** for pattern analysis after compilation
- [ ] **Backward compatibility** maintained with existing API
- [ ] **Reasonable binary size** growth (< 20% for typical usage)

### Benchmarking Strategy

#### Compile-Time vs Runtime Comparison
```rust
#[bench]
fn bench_runtime_pattern_analysis(b: &mut Bencher) {
    let input = "field1:value1,field2:value2;field3:value3";
    b.iter(|| {
        // Runtime analysis every iteration
        let result: Vec<_> = split()
            .src(input)
            .delimeter(vec![":", ",", ";"])
            .perform()
            .collect();
        black_box(result)
    });
}

#[bench]  
fn bench_compiletime_specialized(b: &mut Bencher) {
    let input = "field1:value1,field2:value2;field3:value3";
    
    // Pattern analysis done at compile time
    const PATTERNS: CompiletimeSplit<3> = CompiletimeSplit::new([":", ",", ";"]);
    
    b.iter(|| {
        let result: Vec<_> = PATTERNS.split(input).collect();
        black_box(result)
    });
}
```

#### Binary Size Analysis
- **Specialized code size** measurement for different pattern counts
- **Dead code elimination** verification  
- **LTO impact** on final binary optimization
- **Cache-friendly specialization** balance analysis

### Integration Points

#### SIMD Compatibility
- Compile-time SIMD algorithm selection based on pattern analysis
- Automatic fallback selection for non-SIMD platforms
- Pattern caching integration with compile-time decisions

#### Zero-Copy Integration  
- Compile-time lifetime analysis for optimal zero-copy patterns
- Specialized iterators for compile-time known pattern lifetimes
- Memory layout optimization based on pattern characteristics

### Usage Examples

#### Basic Macro Usage
```rust
use strs_tools::split_optimized;

// Automatic optimization for common patterns
let parts: Vec<&str> = split_optimized!("a:b,c;d", ["::", ":", ",", "."]);

// Single character optimization (compiles to memchr)
let words: Vec<&str> = split_optimized!("word1 word2 word3", [" "]);

// Few characters (compiles to unrolled loop)
let fields: Vec<&str> = split_optimized!("a,b;c", [",", ";"]);
```

#### Advanced Const Generic Usage
```rust
// Type-safe compile-time patterns
const DELIMS: CompiletimeSplit<2> = CompiletimeSplit::new([",", ";"]);

fn process_csv_line(line: &str) -> Vec<&str> {
    DELIMS.split(line).collect()
}

// Pattern reuse across multiple calls
const URL_DELIMS: CompiletimeSplit<4> = CompiletimeSplit::new(["://", "/", "?", "#"]);
```

### Documentation Requirements

Update documentation with:
- **Macro usage guide** with examples for different pattern types
- **Performance characteristics** for each specialization
- **Compile-time vs runtime** trade-offs analysis  
- **Binary size impact** guidance and mitigation strategies

### Related Tasks

- Task 001: SIMD optimization (compile-time SIMD algorithm selection)
- Task 002: Zero-copy optimization (compile-time lifetime specialization)
- Task 006: Specialized algorithms (compile-time algorithm selection)
- Task 007: Parser integration (compile-time parser-specific optimizations)