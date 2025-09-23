# Task 005: Unicode Optimization

## Priority: Low-Medium
## Impact: 3-8x improvement for Unicode-heavy text processing
## Estimated Effort: 5-6 days

## Problem Statement

Current `strs_tools` SIMD optimizations primarily benefit ASCII text, with Unicode text falling back to slower scalar implementations:

```rust
// SIMD works well for ASCII
let ascii_result = split().src("field1,field2,field3").delimeter(",").perform();

// Falls back to slow scalar processing
let unicode_result = split().src("поле1,поле2,поле3").delimeter(",").perform(); // ← Slow
let emoji_result = split().src("😀🎉😎").delimeter("🎉").perform(); // ← Very slow
```

This creates performance disparities:
- **ASCII bias**: 6x SIMD speedup for ASCII, 1x for Unicode
- **UTF-8 boundaries**: Char boundary checks add overhead
- **Grapheme clusters**: Visual characters may span multiple bytes
- **Normalization**: Different Unicode representations of same text

## Solution Approach

Implement Unicode-aware SIMD optimizations with UTF-8 boundary handling, grapheme cluster support, and Unicode normalization caching.

### Implementation Plan

#### 1. UTF-8 Boundary-Aware SIMD

```rust
use std::arch::x86_64::*;

/// UTF-8 boundary-aware SIMD operations
pub struct UnicodeSIMD;

impl UnicodeSIMD 
{
    /// Find Unicode delimiter with boundary checking
    pub fn find_unicode_delimiter(haystack: &str, needle: &str) -> Option<usize> 
{
        // Use SIMD to find byte patterns, then validate UTF-8 boundaries
        let haystack_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        
        // SIMD search for byte pattern
        let mut candidate_pos = 0;
        while let Some(pos) = Self::simd_find_bytes(
            &haystack_bytes[candidate_pos..], 
            needle_bytes
        ) {
            let absolute_pos = candidate_pos + pos;
            
            // Validate UTF-8 boundaries
            if Self::is_char_boundary(haystack, absolute_pos) &&
               Self::is_char_boundary(haystack, absolute_pos + needle_bytes.len()) {
                return Some(absolute_pos);
            }
            
            candidate_pos = absolute_pos + 1;
        }
        
        None
    }
    
    /// SIMD byte pattern search with UTF-8 awareness
    unsafe fn simd_find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> 
{
        if haystack.len() < 16 || needle.is_empty() {
            return Self::scalar_find(haystack, needle);
        }
        
        let first_byte = needle[0];
        let first_vec = _mm_set1_epi8(first_byte as i8);
        
        let mut i = 0;
        while i + 16 <= haystack.len() {
            let chunk = _mm_loadu_si128(haystack.as_ptr().add(i) as *const __m128i);
            let comparison = _mm_cmpeq_epi8(chunk, first_vec);
            let mask = _mm_movemask_epi8(comparison);
            
            if mask != 0 {
                // Found potential match, check full needle
                for bit in 0..16 {
                    if (mask & (1 << bit)) != 0 {
                        let pos = i + bit;
                        if pos + needle.len() <= haystack.len() &&
                           haystack[pos..pos + needle.len()] == *needle {
                            return Some(pos);
                        }
                    }
                }
            }
            
            i += 16;
        }
        
        // Handle remaining bytes
        Self::scalar_find(&haystack[i..], needle).map(|pos| i + pos)
    }
    
    /// Check if position is on UTF-8 character boundary
    fn is_char_boundary(s: &str, index: usize) -> bool 
{
        if index == 0 || index >= s.len() {
            return true;
        }
        
        let byte = s.as_bytes()[index];
        // UTF-8 boundary: not a continuation byte (0b10xxxxxx)
        (byte & 0b11000000) != 0b10000000
    }
}
```

#### 2. Grapheme Cluster Support

```rust
use unicode_segmentation::{UnicodeSegmentation, GraphemeIndices};

/// Grapheme cluster-aware splitting
pub struct GraphemeSplitIterator<'a> 
{
    input: &'a str,
    delimiters: Vec<&'a str>,
    grapheme_indices: std::vec::IntoIter<(usize, &'a str)>,
    position: usize,
}

impl<'a> GraphemeSplitIterator<'a> {
    pub fn new(input: &'a str, delimiters: Vec<&'a str>) -> Self 
{
        let grapheme_indices: Vec<(usize, &str)> = input
            .grapheme_indices(true) // Extended grapheme clusters
            .collect();
        
        Self {
            input,
            delimiters,
            grapheme_indices: grapheme_indices.into_iter(),
            position: 0,
        }
    }
    
    /// Find delimiter respecting grapheme boundaries
    fn find_grapheme_delimiter(&mut self) -> Option<(usize, usize, &'a str)> 
{
        let mut grapheme_buffer = String::new();
        let mut start_pos = self.position;
        
        while let Some((pos, grapheme)) = self.grapheme_indices.next() {
            grapheme_buffer.push_str(grapheme);
            
            // Check if buffer contains any delimiter
            for delimiter in &self.delimiters {
                if let Some(delim_pos) = grapheme_buffer.find(delimiter) {
                    let absolute_start = start_pos + delim_pos;
                    let absolute_end = absolute_start + delimiter.len();
                    return Some((absolute_start, absolute_end, delimiter));
                }
            }
            
            // Sliding window approach for long text
            if grapheme_buffer.len() > 1024 {
                let keep_size = 512;
                grapheme_buffer.drain(..keep_size);
                start_pos += keep_size;
            }
        }
        
        None
    }
}
```

#### 3. Unicode Normalization Caching

```rust
use unicode_normalization::{UnicodeNormalization, IsNormalized};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Cache for normalized Unicode strings  
pub struct NormalizationCache 
{
    nfc_cache: RwLock<HashMap<String, String>>,
    nfd_cache: RwLock<HashMap<String, String>>,
    cache_size_limit: usize,
}

impl NormalizationCache 
{
    pub fn new(size_limit: usize) -> Self 
{
        Self {
            nfc_cache: RwLock::new(HashMap::new()),
            nfd_cache: RwLock::new(HashMap::new()),
            cache_size_limit: size_limit,
        }
    }
    
    /// Get NFC normalized string with caching
    pub fn nfc_normalize(&self, input: &str) -> String 
{
        // Quick check if already normalized
        if input.is_nfc() {
            return input.to_string();
        }
        
        // Check cache first
        {
            let cache = self.nfc_cache.read().unwrap();
            if let Some(normalized) = cache.get(input) {
                return normalized.clone();
            }
        }
        
        // Normalize and cache result
        let normalized: String = input.nfc().collect();
        
        {
            let mut cache = self.nfc_cache.write().unwrap();
            if cache.len() >= self.cache_size_limit {
                cache.clear(); // Simple eviction policy
            }
            cache.insert(input.to_string(), normalized.clone());
        }
        
        normalized
    }
    
    /// Compare strings with normalization
    pub fn normalized_equals(&self, a: &str, b: &str) -> bool 
{
        if a == b {
            return true; // Fast path for identical strings
        }
        
        let norm_a = self.nfc_normalize(a);
        let norm_b = self.nfc_normalize(b);
        norm_a == norm_b
    }
}
```

#### 4. Unicode-Aware Split Implementation

```rust
/// Unicode-optimized split operations
pub struct UnicodeSplit<'a> 
{
    src: &'a str,
    delimiters: Vec<&'a str>,
    normalization_cache: Option<&'a NormalizationCache>,
    grapheme_aware: bool,
}

impl<'a> UnicodeSplit<'a> {
    pub fn new(src: &'a str) -> Self 
{
        Self {
            src,
            delimiters: Vec::new(),
            normalization_cache: None,
            grapheme_aware: false,
        }
    }
    
    pub fn delimeter(mut self, delim: &'a str) -> Self 
{
        self.delimiters.push(delim);
        self
    }
    
    pub fn with_normalization(mut self, cache: &'a NormalizationCache) -> Self 
{
        self.normalization_cache = Some(cache);
        self
    }
    
    pub fn grapheme_aware(mut self) -> Self 
{
        self.grapheme_aware = true;
        self
    }
    
    pub fn perform(self) -> Box<dyn Iterator<Item = &'a str> + 'a> 
{
        if self.grapheme_aware {
            Box::new(GraphemeSplitIterator::new(self.src, self.delimiters))
        } else if self.has_unicode_delimiters() {
            Box::new(UnicodeSplitIterator::new(self.src, self.delimiters, self.normalization_cache))
        } else {
            // Fall back to ASCII-optimized SIMD
            Box::new(ASCIISplitIterator::new(self.src, self.delimiters))
        }
    }
    
    fn has_unicode_delimiters(&self) -> bool 
{
        self.delimiters.iter().any(|delim| !delim.is_ascii())
    }
}
```

#### 5. Optimized Unicode Character Classification

```rust
/// Fast Unicode character classification using lookup tables
pub struct UnicodeClassifier 
{
    // Pre-computed lookup tables for common ranges
    ascii_table: [CharClass; 128],
    latin1_table: [CharClass; 256],
    // Fallback for full Unicode range
}

#[derive(Copy, Clone, PartialEq)]
enum CharClass 
{
    Whitespace,
    Punctuation, 
    Letter,
    Digit,
    Symbol,
    Other,
}

impl UnicodeClassifier 
{
    /// Classify character with optimized lookup
    pub fn classify_char(&self, ch: char) -> CharClass 
{
        let code_point = ch as u32;
        
        match code_point {
            0..=127 => self.ascii_table[code_point as usize],
            128..=255 => self.latin1_table[code_point as usize],
            _ => self.classify_full_unicode(ch), // Slower fallback
        }
    }
    
    /// SIMD-optimized whitespace detection for Unicode
    pub fn is_unicode_whitespace_simd(text: &str) -> Vec<bool> 
{
        let mut results = Vec::with_capacity(text.chars().count());
        
        // Process ASCII characters with SIMD
        let mut byte_pos = 0;
        for ch in text.chars() {
            if ch.is_ascii() {
                // Use SIMD for ASCII whitespace detection
                results.push(Self::simd_is_ascii_whitespace(ch as u8));
            } else {
                // Unicode whitespace check
                results.push(ch.is_whitespace());
            }
            byte_pos += ch.len_utf8();
        }
        
        results
    }
}
```

### Technical Requirements

#### Unicode Compliance
- **UTF-8 boundary** detection and validation
- **Grapheme cluster** awareness for visual character integrity  
- **Normalization** support (NFC, NFD, NFKC, NFKD)
- **Case folding** for case-insensitive operations

#### Performance Optimization
- **Selective SIMD** usage based on text content analysis
- **Lookup table** optimization for common Unicode ranges
- **Caching strategies** for expensive Unicode operations
- **Streaming processing** to handle large Unicode documents

#### Correctness Guarantees
- **Boundary safety** - no splitting within multi-byte characters
- **Normalization consistency** - handle equivalent representations
- **Grapheme integrity** - respect visual character boundaries
- **Locale awareness** for culture-specific text handling

### Performance Targets

| Text Type | Current Performance | Unicode Optimized | Improvement |
|-----------|-------------------|------------------|-------------|
| **ASCII text** | 742.5 MiB/s | 750+ MiB/s | **1.1x faster** |
| **Latin-1 text** | 45.2 MiB/s | 180.5 MiB/s | **4x faster** |
| **Mixed Unicode** | 12.3 MiB/s | 89.7 MiB/s | **7.3x faster** |
| **CJK text** | 8.1 MiB/s | 65.4 MiB/s | **8.1x faster** |
| **Emoji/symbols** | 3.2 MiB/s | 24.8 MiB/s | **7.8x faster** |

#### Unicode-Specific Metrics
- **Boundary violations**: Zero tolerance for char boundary splits
- **Normalization accuracy**: 100% correctness for equivalent forms
- **Grapheme preservation**: No visual character fragmentation  
- **Memory overhead**: < 20% increase for Unicode support

### Implementation Steps

1. **Implement UTF-8 boundary-aware** SIMD operations
2. **Create Unicode character** classification lookup tables
3. **Add normalization caching** for expensive Unicode operations
4. **Implement grapheme cluster** support for visual integrity
5. **Optimize common Unicode ranges** (Latin-1, CJK) with specialized algorithms
6. **Comprehensive Unicode testing** across different scripts and languages
7. **Performance benchmarking** for various Unicode content types

### Challenges & Solutions

#### Challenge: Complex UTF-8 Validation
**Solution**: SIMD-accelerated UTF-8 validation with lookup tables
```rust
/// Fast UTF-8 validation using SIMD
unsafe fn validate_utf8_simd(bytes: &[u8]) -> bool 
{
    // Use SIMD instructions to validate UTF-8 sequences
    // Based on algorithms from simdjson and similar libraries
    let mut i = 0;
    while i + 16 <= bytes.len() {
        let chunk = _mm_loadu_si128(bytes.as_ptr().add(i) as *const __m128i);
        if !Self::validate_utf8_chunk(chunk) {
            return false;
        }
        i += 16;
    }
    
    // Validate remaining bytes with scalar code
    Self::validate_utf8_scalar(&bytes[i..])
}
```

#### Challenge: Normalization Performance
**Solution**: Lazy normalization with content analysis
```rust
/// Analyze text to determine if normalization is needed
fn needs_normalization(&self, text: &str) -> bool 
{
    // Quick heuristic checks before expensive normalization
    if text.is_ascii() {
        return false; // ASCII is always normalized
    }
    
    // Check for combining characters, compatibility characters
    text.chars().any(|ch| {
        unicode_normalization::char::is_combining_mark(ch) ||
        unicode_normalization::char::needs_nfc_normalization(ch)
    })
}
```

#### Challenge: Memory Usage for Large Unicode
**Solution**: Streaming processing with bounded buffers
```rust
/// Process large Unicode text in streaming fashion
pub fn split_unicode_streaming(
    input: impl Iterator<Item = char>,
    delimiters: &[&str],
) -> impl Iterator<Item = String> 
{
    UnicodeStreamSplitter::new(input, delimiters, 64 * 1024) // 64KB buffer
}
```

### Success Criteria

- [ ] **5x improvement** for Latin-1 text processing
- [ ] **8x improvement** for CJK text processing  
- [ ] **Zero boundary violations** in all Unicode splitting operations
- [ ] **100% normalization correctness** for equivalent Unicode forms
- [ ] **Grapheme cluster integrity** preserved in all operations
- [ ] **< 20% memory overhead** compared to ASCII-only implementation

### Benchmarking Strategy

#### Unicode Content Benchmarks
```rust
#[bench]
fn bench_unicode_split_latin1(b: &mut Bencher) 
{
    let input = "café,naïve,résumé,piñata".repeat(1000); // Latin-1 with diacritics
    b.iter(|| {
        let result: Vec<_> = UnicodeSplit::new(&input)
            .delimeter(",")
            .perform()
            .collect();
        black_box(result)
    });
}

#[bench]
fn bench_unicode_split_cjk(b: &mut Bencher) {  
    let input = "你好,世界,测试,文本".repeat(1000); // Chinese text
    b.iter(|| {
        let result: Vec<_> = UnicodeSplit::new(&input)
            .delimeter(",")
            .perform()
            .collect();
        black_box(result)
    });
}

#[bench]
fn bench_unicode_split_emoji(b: &mut Bencher) 
{
    let input = "😀🎉😎🚀🎯".repeat(200); // Emoji grapheme clusters
    b.iter(|| {
        let result: Vec<_> = UnicodeSplit::new(&input)
            .delimeter("🎉")
            .grapheme_aware()
            .perform()
            .collect();
        black_box(result)
    });
}
```

#### Correctness Validation
- **Boundary violation** detection with comprehensive test suites
- **Normalization correctness** testing across Unicode forms
- **Grapheme cluster** integrity verification
- **Cross-platform consistency** testing

### Integration Points

#### SIMD Synergy
- Unicode detection enables optimal SIMD algorithm selection  
- ASCII fast-path maintains existing SIMD performance
- Hybrid processing for mixed ASCII/Unicode content

#### Zero-Copy Compatibility
- Unicode-aware zero-copy operations with boundary validation
- Normalization caching reduces copy-on-write overhead
- Grapheme cluster slicing with lifetime management

### Usage Examples

#### Basic Unicode Support
```rust
use strs_tools::unicode::UnicodeSplit;

// Automatic Unicode handling
let parts: Vec<_> = UnicodeSplit::new("café,naïve,résumé")
    .delimeter(",")
    .perform()
    .collect();

// Grapheme cluster awareness for emoji
let emoji_parts: Vec<_> = UnicodeSplit::new("👨‍👩‍👧‍👦🎉👨‍👩‍👧‍👦")
    .delimeter("🎉")
    .grapheme_aware()
    .perform()
    .collect();
```

#### Advanced Unicode Features
```rust
use strs_tools::unicode::{UnicodeSplit, NormalizationCache};

// With normalization for equivalent forms
let cache = NormalizationCache::new(1024);
let normalized_parts: Vec<_> = UnicodeSplit::new("café vs cafe\u{0301}") // Different representations
    .delimeter("vs")
    .with_normalization(&cache)
    .perform()
    .collect();
```

### Documentation Requirements

Update documentation with:
- **Unicode support guide** explaining UTF-8, normalization, and grapheme clusters
- **Performance characteristics** for different script types and content
- **Best practices** for Unicode text processing
- **Migration guide** from ASCII-only to Unicode-aware operations

### Related Tasks

- Task 001: SIMD optimization (Unicode-aware SIMD algorithm selection)
- Task 002: Zero-copy optimization (Unicode boundary-aware zero-copy)
- Task 006: Specialized algorithms (Unicode-specific algorithm implementations)
- Task 007: Parser integration (Unicode-aware parsing optimizations)