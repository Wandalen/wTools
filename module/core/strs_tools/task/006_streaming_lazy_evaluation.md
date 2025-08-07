# Task 006: Streaming and Lazy Evaluation Optimization

## Priority: Medium
## Impact: Memory usage reduction from O(n) to O(1), enables processing of unbounded data
## Estimated Effort: 3-4 days

## Problem Statement

Current `strs_tools` processes entire input strings in memory, making it unsuitable for large files or streaming data:

```rust
// Current approach loads entire file into memory
let large_file_content = std::fs::read_to_string("huge_file.txt")?; // ← 10GB+ in memory
let lines: Vec<String> = string::split()
    .src(&large_file_content)
    .delimeter("\n")
    .perform()
    .collect(); // ← Another copy, 20GB+ total
```

This creates several problems:
- **Memory explosion**: Large files require 2-3x their size in RAM
- **Start-up latency**: Must read entire file before processing begins
- **No streaming**: Cannot process infinite or network streams
- **Poor scalability**: Memory usage grows linearly with input size

## Solution Approach

Implement streaming split iterators with lazy evaluation, enabling constant memory processing of arbitrarily large inputs.

### Implementation Plan

#### 1. Streaming Split Iterator

```rust
use std::io::{BufRead, BufReader, Read};

/// Streaming split iterator for large inputs
pub struct StreamingSplit<R: BufRead> {
    reader: R,
    delimiters: Vec<String>,
    buffer: String,
    buffer_size: usize,
    position: usize,
    finished: bool,
    overlap_size: usize,
}

impl<R: BufRead> StreamingSplit<R> {
    pub fn new(reader: R, delimiters: Vec<String>) -> Self {
        let max_delimiter_len = delimiters.iter().map(|d| d.len()).max().unwrap_or(0);
        
        Self {
            reader,
            delimiters,
            buffer: String::new(),
            buffer_size: 64 * 1024, // 64KB sliding window
            position: 0,
            finished: false,
            overlap_size: max_delimiter_len * 2, // Ensure we don't miss cross-buffer delimiters
        }
    }
    
    /// Fill buffer while preserving overlap for cross-boundary matches
    fn refill_buffer(&mut self) -> std::io::Result<bool> {
        if self.finished {
            return Ok(false);
        }
        
        // Preserve overlap from end of current buffer
        if self.buffer.len() > self.overlap_size {
            let keep_from = self.buffer.len() - self.overlap_size;
            self.buffer.drain(..keep_from);
            self.position = self.position.saturating_sub(keep_from);
        }
        
        // Read more data
        let mut temp_buf = String::with_capacity(self.buffer_size);
        let bytes_read = self.reader.read_line(&mut temp_buf)?;
        
        if bytes_read == 0 {
            self.finished = true;
            return Ok(!self.buffer.is_empty());
        }
        
        self.buffer.push_str(&temp_buf);
        Ok(true)
    }
}

impl<R: BufRead> Iterator for StreamingSplit<R> {
    type Item = Result<String, std::io::Error>;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Look for delimiter in current buffer
            if let Some((start, end, _)) = self.find_next_delimiter() {
                let segment = self.buffer[self.position..start].to_string();
                self.position = end;
                return Some(Ok(segment));
            }
            
            // No delimiter found, need more data
            match self.refill_buffer() {
                Ok(true) => continue, // Got more data, try again
                Ok(false) => {
                    // EOF, return remaining content if any
                    if self.position < self.buffer.len() {
                        let remaining = self.buffer[self.position..].to_string();
                        self.position = self.buffer.len();
                        return Some(Ok(remaining));
                    } else {
                        return None;
                    }
                },
                Err(e) => return Some(Err(e)),
            }
        }
    }
}
```

#### 2. Lazy Evaluation with Generator Pattern

```rust
/// Lazy string processing with generator-like interface
pub struct LazyStringSplit<'a> {
    source: &'a str,
    delimiters: Vec<&'a str>,
    current_pos: usize,
    chunk_size: usize,
}

impl<'a> LazyStringSplit<'a> {
    pub fn new(source: &'a str, delimiters: Vec<&'a str>) -> Self {
        Self {
            source,
            delimiters,
            current_pos: 0,
            chunk_size: 4096, // Process in 4KB chunks
        }
    }
    
    /// Process next chunk lazily
    pub fn process_chunk<F, R>(&mut self, mut processor: F) -> Option<R>
    where
        F: FnMut(&str) -> R,
    {
        if self.current_pos >= self.source.len() {
            return None;
        }
        
        let end_pos = std::cmp::min(
            self.current_pos + self.chunk_size,
            self.source.len()
        );
        
        // Adjust end to avoid splitting mid-delimiter
        let chunk_end = self.adjust_chunk_boundary(end_pos);
        let chunk = &self.source[self.current_pos..chunk_end];
        
        let result = processor(chunk);
        self.current_pos = chunk_end;
        
        Some(result)
    }
    
    /// Ensure chunk boundaries don't split delimiters
    fn adjust_chunk_boundary(&self, proposed_end: usize) -> usize {
        if proposed_end >= self.source.len() {
            return self.source.len();
        }
        
        // Look backwards from proposed end to find safe boundary
        for i in (self.current_pos..proposed_end).rev() {
            if self.is_safe_boundary(i) {
                return i;
            }
        }
        
        // Fallback to proposed end if no safe boundary found
        proposed_end
    }
    
    fn is_safe_boundary(&self, pos: usize) -> bool {
        // Check if position would split any delimiter
        for delimiter in &self.delimiters {
            let delim_len = delimiter.len();
            if pos >= delim_len {
                let start_check = pos - delim_len + 1;
                let end_check = std::cmp::min(pos + delim_len, self.source.len());
                let window = &self.source[start_check..end_check];
                if window.contains(delimiter) {
                    return false; // Would split this delimiter
                }
            }
        }
        true
    }
}
```

#### 3. Memory-Bounded Streaming with Backpressure

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

/// Streaming split with bounded memory and backpressure
pub struct BoundedStreamingSplit<R: BufRead> {
    inner: StreamingSplit<R>,
    buffer_queue: Arc<Mutex<VecDeque<String>>>,
    max_buffered_items: usize,
    buffer_not_full: Arc<Condvar>,
    buffer_not_empty: Arc<Condvar>,
}

impl<R: BufRead> BoundedStreamingSplit<R> {
    pub fn new(reader: R, delimiters: Vec<String>, max_buffer_size: usize) -> Self {
        Self {
            inner: StreamingSplit::new(reader, delimiters),
            buffer_queue: Arc::new(Mutex::new(VecDeque::new())),
            max_buffered_items: max_buffer_size,
            buffer_not_full: Arc::new(Condvar::new()),
            buffer_not_empty: Arc::new(Condvar::new()),
        }
    }
    
    /// Start background processing thread
    pub fn start_background_processing(&mut self) -> std::thread::JoinHandle<()> {
        let buffer_queue = Arc::clone(&self.buffer_queue);
        let buffer_not_full = Arc::clone(&self.buffer_not_full);
        let buffer_not_empty = Arc::clone(&self.buffer_not_empty);
        let max_items = self.max_buffered_items;
        
        std::thread::spawn(move || {
            while let Some(item) = self.inner.next() {
                match item {
                    Ok(segment) => {
                        // Wait if buffer is full (backpressure)
                        let mut queue = buffer_queue.lock().unwrap();
                        while queue.len() >= max_items {
                            queue = self.buffer_not_full.wait(queue).unwrap();
                        }
                        
                        queue.push_back(segment);
                        self.buffer_not_empty.notify_one();
                    },
                    Err(_) => break, // Handle error by stopping processing
                }
            }
        })
    }
    
    /// Get next item with blocking
    pub fn next_blocking(&self) -> Option<String> {
        let mut queue = self.buffer_queue.lock().unwrap();
        
        // Wait for item if queue is empty
        while queue.is_empty() {
            queue = self.buffer_not_empty.wait(queue).unwrap();
        }
        
        let item = queue.pop_front();
        if queue.len() < self.max_buffered_items {
            self.buffer_not_full.notify_one();
        }
        
        item
    }
}
```

#### 4. Async/Await Streaming Support

```rust
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_core::Stream;
use tokio::io::{AsyncBufReadExt, BufReader};

/// Async streaming split iterator
pub struct AsyncStreamingSplit<R> {
    reader: BufReader<R>,
    delimiters: Vec<String>,
    buffer: String,
    position: usize,
    finished: bool,
}

impl<R: tokio::io::AsyncRead + Unpin> AsyncStreamingSplit<R> {
    pub fn new(reader: R, delimiters: Vec<String>) -> Self {
        Self {
            reader: BufReader::new(reader),
            delimiters,
            buffer: String::new(),
            position: 0,
            finished: false,
        }
    }
}

impl<R: tokio::io::AsyncRead + Unpin> Stream for AsyncStreamingSplit<R> {
    type Item = Result<String, std::io::Error>;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.finished && self.position >= self.buffer.len() {
            return Poll::Ready(None);
        }
        
        // Try to find delimiter in current buffer
        if let Some((start, end, _)) = self.find_next_delimiter() {
            let segment = self.buffer[self.position..start].to_string();
            self.position = end;
            return Poll::Ready(Some(Ok(segment)));
        }
        
        // Need to read more data
        let mut line = String::new();
        match Pin::new(&mut self.reader).poll_read_line(cx, &mut line) {
            Poll::Ready(Ok(0)) => {
                // EOF
                self.finished = true;
                if self.position < self.buffer.len() {
                    let remaining = self.buffer[self.position..].to_string();
                    self.position = self.buffer.len();
                    Poll::Ready(Some(Ok(remaining)))
                } else {
                    Poll::Ready(None)
                }
            },
            Poll::Ready(Ok(_)) => {
                self.buffer.push_str(&line);
                // Recursively poll for delimiter
                self.poll_next(cx)
            },
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}
```

#### 5. Integration with Existing APIs

```rust
/// Extension trait for streaming operations
pub trait StreamingStringExt {
    /// Create streaming split from Read source
    fn streaming_split<R: BufRead>(
        reader: R, 
        delimiters: Vec<String>
    ) -> StreamingSplit<R>;
    
    /// Create async streaming split  
    fn async_streaming_split<R: tokio::io::AsyncRead + Unpin>(
        reader: R,
        delimiters: Vec<String>
    ) -> AsyncStreamingSplit<R>;
    
    /// Process large string in chunks
    fn lazy_process<F, R>(&self, chunk_size: usize, processor: F) -> LazyProcessor<'_, F, R>
    where
        F: FnMut(&str) -> R;
}

impl StreamingStringExt for str {
    fn streaming_split<R: BufRead>(
        reader: R, 
        delimiters: Vec<String>
    ) -> StreamingSplit<R> {
        StreamingSplit::new(reader, delimiters)
    }
    
    fn async_streaming_split<R: tokio::io::AsyncRead + Unpin>(
        reader: R,
        delimiters: Vec<String>
    ) -> AsyncStreamingSplit<R> {
        AsyncStreamingSplit::new(reader, delimiters)
    }
    
    fn lazy_process<F, R>(&self, chunk_size: usize, processor: F) -> LazyProcessor<'_, F, R>
    where
        F: FnMut(&str) -> R,
    {
        LazyProcessor::new(self, chunk_size, processor)
    }
}
```

### Technical Requirements

#### Memory Management
- **Constant memory** usage regardless of input size
- **Bounded buffering** with configurable limits
- **Overlap handling** to prevent missing cross-boundary delimiters
- **Backpressure** mechanisms for flow control

#### Performance Characteristics
- **Streaming latency**: Process results as soon as available  
- **Throughput**: Maintain high throughput for continuous streams
- **Memory predictability**: Bounded memory usage guarantees
- **CPU efficiency**: Minimize copying and allocation in hot paths

#### Compatibility
- **Sync and async** versions for different use cases
- **Integration** with existing split APIs
- **Error handling** for I/O operations and malformed input
- **Cross-platform** support for different I/O mechanisms

### Performance Targets

| Input Size | Memory Usage (Current) | Memory Usage (Streaming) | Improvement |
|------------|----------------------|-------------------------|-------------|
| **1MB file** | ~3MB (3x overhead) | ~64KB (constant) | **47x less memory** |
| **100MB file** | ~300MB (3x overhead) | ~64KB (constant) | **4,688x less memory** |
| **1GB file** | ~3GB (3x overhead) | ~64KB (constant) | **46,875x less memory** |
| **Infinite stream** | Impossible | ~64KB (constant) | **Enables previously impossible** |

#### Streaming Performance Metrics
- **Time to first result**: < 1ms for typical inputs
- **Sustained throughput**: 500+ MB/s for streaming processing
- **Memory overhead**: < 100KB regardless of input size
- **Latency**: Results available as soon as delimiters found

### Implementation Steps

1. **Implement basic streaming split** iterator with sliding window
2. **Add overlap handling** to prevent cross-boundary delimiter misses
3. **Create async version** using tokio/futures for async compatibility
4. **Add backpressure mechanisms** for memory-bounded processing
5. **Integrate with SIMD** optimizations for streaming pattern matching
6. **Comprehensive testing** with large files and streaming sources
7. **Performance benchmarking** comparing memory usage and throughput

### Challenges & Solutions

#### Challenge: Cross-Boundary Delimiter Detection
**Solution**: Overlap buffer with maximum delimiter length
```rust
fn ensure_delimiter_visibility(&mut self) {
    let max_delim_len = self.delimiters.iter().map(|d| d.len()).max().unwrap_or(0);
    let overlap_size = max_delim_len * 2; // Safety margin
    
    // Always preserve overlap when sliding window
    if self.buffer.len() > self.buffer_size + overlap_size {
        let keep_from = self.buffer.len() - overlap_size;
        self.buffer.drain(..keep_from);
    }
}
```

#### Challenge: Memory Pressure from Large Segments
**Solution**: Segment size limits with progressive fallback
```rust
const MAX_SEGMENT_SIZE: usize = 1024 * 1024; // 1MB limit

fn handle_large_segment(&mut self, start: usize) -> Option<String> {
    let segment_size = self.position - start;
    if segment_size > MAX_SEGMENT_SIZE {
        // Split large segment into smaller chunks
        return self.split_large_segment(start, MAX_SEGMENT_SIZE);
    }
    
    Some(self.buffer[start..self.position].to_string())
}
```

#### Challenge: I/O Error Handling
**Solution**: Graceful error propagation with partial results
```rust
impl<R: BufRead> Iterator for StreamingSplit<R> {
    type Item = Result<String, StreamingError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(segment)) => Some(Ok(segment)),
            Ok(None) => None,
            Err(StreamingError::IoError(e)) => {
                // Return partial results if available
                if self.has_partial_data() {
                    Some(Ok(self.consume_partial_data()))
                } else {
                    Some(Err(StreamingError::IoError(e)))
                }
            },
            Err(e) => Some(Err(e)),
        }
    }
}
```

### Success Criteria

- [ ] **Constant memory usage** (< 100KB) for arbitrarily large inputs
- [ ] **< 1ms time to first result** for streaming inputs
- [ ] **500+ MB/s sustained throughput** for continuous processing
- [ ] **Async/sync compatibility** with both blocking and non-blocking I/O
- [ ] **Zero data loss** at buffer boundaries with overlap handling  
- [ ] **Graceful error handling** with partial result recovery

### Benchmarking Strategy

#### Memory Usage Comparison
```rust
#[bench]
fn bench_memory_usage_large_file(b: &mut Bencher) {
    let large_content = generate_large_test_content(100 * 1024 * 1024); // 100MB
    
    // Current approach - loads everything into memory
    b.iter(|| {
        let parts: Vec<String> = string::split()
            .src(&large_content)
            .delimeter("\n")
            .perform()
            .collect();
        black_box(parts.len()) // Just count, don't keep in memory
    });
}

#[bench]
fn bench_streaming_memory_usage(b: &mut Bencher) {
    let reader = create_large_test_reader(100 * 1024 * 1024); // 100MB
    
    // Streaming approach - constant memory
    b.iter(|| {
        let mut count = 0;
        let streaming_split = StreamingSplit::new(reader, vec!["\n".to_string()]);
        
        for result in streaming_split {
            if result.is_ok() {
                count += 1;
            }
        }
        black_box(count)
    });
}
```

#### Latency and Throughput Testing  
- **Time to first result** measurement with high-precision timers
- **Sustained throughput** testing with large continuous streams
- **Memory allocation** patterns with custom allocator tracking
- **Backpressure behavior** under different consumer speeds

### Integration Points

#### SIMD Compatibility
- Streaming buffers aligned for SIMD operations
- Pattern matching optimizations in sliding window
- Bulk processing of buffered segments with SIMD

#### Zero-Copy Integration
- Zero-copy segment extraction from streaming buffers
- Lifetime management for streaming string slices
- Copy-on-write only when segments cross buffer boundaries

### Usage Examples

#### Basic File Streaming
```rust
use std::fs::File;
use std::io::BufReader;
use strs_tools::streaming::StreamingStringExt;

// Process large file with constant memory
let file = File::open("huge_log_file.txt")?;
let reader = BufReader::new(file);
let streaming_split = reader.streaming_split(vec!["\n".to_string()]);

for line_result in streaming_split {
    let line = line_result?;
    process_log_line(&line); // Process immediately, no accumulation
}
```

#### Async Network Streaming  
```rust
use tokio::net::TcpStream;
use strs_tools::streaming::StreamingStringExt;

// Process network stream asynchronously
let stream = TcpStream::connect("log-server:8080").await?;
let mut async_split = stream.async_streaming_split(vec!["\n".to_string()]);

while let Some(line_result) = async_split.next().await {
    let line = line_result?;
    handle_network_data(&line).await;
}
```

#### Bounded Memory Processing
```rust
use strs_tools::streaming::BoundedStreamingSplit;

// Process with memory limits and backpressure
let reader = BufReader::new(huge_file);
let mut bounded_split = BoundedStreamingSplit::new(
    reader, 
    vec![",".to_string()], 
    1000 // Max 1000 buffered segments
);

let processor_thread = bounded_split.start_background_processing();

// Consumer controls processing rate
while let Some(segment) = bounded_split.next_blocking() {
    expensive_processing(&segment); // Backpressure automatically applied
}
```

### Documentation Requirements

Update documentation with:
- **Streaming processing guide** with memory usage patterns
- **Async integration examples** for tokio and other async runtimes
- **Error handling strategies** for I/O failures and partial results
- **Performance tuning** recommendations for different streaming scenarios

### Related Tasks

- Task 002: Zero-copy optimization (streaming zero-copy segment extraction)
- Task 004: Memory pool allocation (streaming-aware pool management)
- Task 008: Parallel processing (parallel streaming with work distribution)
- Task 001: SIMD optimization (streaming SIMD pattern matching)