# Task 009: Parallel Processing Optimization

## Priority: Medium
## Impact: Near-linear scaling with core count for large inputs (2-16x improvement)
## Estimated Effort: 5-6 days

## Problem Statement

Current `strs_tools` processes strings sequentially, leaving multi-core performance on the table for large inputs:

```rust
// Current sequential processing
let large_input = read_huge_file("10GB_log_file.txt");
let lines: Vec<String> = string::split()
    .src(&large_input)
    .delimeter("\n")
    .perform()
    .collect(); // ← Single-threaded, uses only one core
    
// Processing each line is also sequential
for line in lines {
    expensive_analysis(line); // ← Could be parallelized
}
```

This leads to underutilized hardware:
- **Single-core usage**: Only 1 of 8-16+ cores utilized
- **Memory bandwidth**: Sequential access doesn't saturate memory channels
- **Latency hiding**: No concurrent I/O and computation
- **Poor scaling**: Performance doesn't improve with better hardware

## Solution Approach

Implement parallel string processing with work-stealing, NUMA awareness, and load balancing for optimal multi-core utilization.

### Implementation Plan

#### 1. Parallel Split with Work Distribution

```rust
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Parallel splitting for large inputs with work distribution
pub struct ParallelSplit {
    chunk_size: usize,
    num_threads: Option<usize>,
    load_balance: bool,
}

impl ParallelSplit {
    pub fn new() -> Self {
        Self {
            chunk_size: 1024 * 1024, // 1MB chunks by default
            num_threads: None,        // Use all available cores
            load_balance: true,       // Enable dynamic load balancing
        }
    }
    
    pub fn chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }
    
    pub fn threads(mut self, count: usize) -> Self {
        self.num_threads = Some(count);
        self
    }
    
    /// Split large input across multiple threads
    pub fn split_parallel<'a>(
        &self,
        input: &'a str,
        delimiters: &[&str],
    ) -> ParallelSplitIterator<'a> {
        // Calculate optimal chunk boundaries
        let chunks = self.calculate_chunks(input, delimiters);
        
        ParallelSplitIterator {
            chunks,
            delimiters: delimiters.to_vec(),
            current_chunk: 0,
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Calculate chunk boundaries ensuring no delimiter splits
    fn calculate_chunks(&self, input: &str, delimiters: &[&str]) -> Vec<(usize, usize)> {
        let mut chunks = Vec::new();
        let total_len = input.len();
        let target_chunk_size = self.chunk_size;
        
        let mut start = 0;
        while start < total_len {
            let mut end = std::cmp::min(start + target_chunk_size, total_len);
            
            // Adjust end to not split delimiters
            end = self.find_safe_boundary(input, start, end, delimiters);
            
            chunks.push((start, end));
            start = end;
        }
        
        chunks
    }
    
    fn find_safe_boundary(&self, input: &str, start: usize, proposed_end: usize, delimiters: &[&str]) -> usize {
        if proposed_end >= input.len() {
            return input.len();
        }
        
        // Find the longest delimiter to establish safe zone
        let max_delimiter_len = delimiters.iter().map(|d| d.len()).max().unwrap_or(0);
        let search_start = proposed_end.saturating_sub(max_delimiter_len);
        
        // Look for safe boundary (after a complete delimiter)
        for i in (search_start..proposed_end).rev() {
            for delimiter in delimiters {
                if input[i..].starts_with(delimiter) {
                    return i + delimiter.len(); // Safe boundary after delimiter
                }
            }
        }
        
        // Fallback to character boundary
        while proposed_end > start && !input.is_char_boundary(proposed_end) {
            proposed_end -= 1;
        }
        
        proposed_end
    }
}

/// Iterator for parallel split results
pub struct ParallelSplitIterator<'a> {
    chunks: Vec<(usize, usize)>,
    delimiters: Vec<&'a str>,
    current_chunk: usize,
    results: Arc<Mutex<Vec<Vec<String>>>>,
}
```

#### 2. Work-Stealing Parallel Executor

```rust
use crossbeam::deque::{Injector, Stealer, Worker};
use crossbeam::utils::Backoff;
use std::thread;

/// Work-stealing executor for string processing tasks
pub struct WorkStealingExecutor {
    workers: Vec<Worker<StringTask>>,
    stealers: Vec<Stealer<StringTask>>,
    injector: Injector<StringTask>,
    num_workers: usize,
}

#[derive(Debug)]
enum StringTask {
    Split { 
        input: String, 
        delimiters: Vec<String>,
        start: usize,
        end: usize,
        result_sender: std::sync::mpsc::Sender<Vec<String>>,
    },
    Process {
        tokens: Vec<String>,
        processor: fn(&str) -> String,
        result_sender: std::sync::mpsc::Sender<Vec<String>>,
    },
}

impl WorkStealingExecutor {
    pub fn new(num_workers: usize) -> Self {
        let mut workers = Vec::new();
        let mut stealers = Vec::new();
        
        for _ in 0..num_workers {
            let worker = Worker::new_fifo();
            stealers.push(worker.stealer());
            workers.push(worker);
        }
        
        Self {
            workers,
            stealers,
            injector: Injector::new(),
            num_workers,
        }
    }
    
    /// Execute string processing tasks with work stealing
    pub fn execute_parallel<F, R>(&self, tasks: Vec<StringTask>) -> Vec<R>
    where
        F: Fn(&str) -> R + Send + Sync,
        R: Send,
    {
        // Inject initial tasks
        for task in tasks {
            self.injector.push(task);
        }
        
        let mut handles = Vec::new();
        
        // Spawn worker threads
        for (worker_id, worker) in self.workers.iter().enumerate() {
            let worker = worker.clone();
            let stealers = self.stealers.clone();
            let injector = self.injector.clone();
            
            let handle = thread::spawn(move || {
                let mut backoff = Backoff::new();
                
                loop {
                    // Try to get task from local queue
                    if let Some(task) = worker.pop() {
                        Self::execute_task(task);
                        backoff.reset();
                        continue;
                    }
                    
                    // Try to steal from global injector
                    if let Some(task) = injector.steal().success() {
                        Self::execute_task(task);
                        backoff.reset();
                        continue;
                    }
                    
                    // Try to steal from other workers
                    let mut found_work = false;
                    for (stealer_id, stealer) in stealers.iter().enumerate() {
                        if stealer_id != worker_id {
                            if let Some(task) = stealer.steal().success() {
                                Self::execute_task(task);
                                found_work = true;
                                backoff.reset();
                                break;
                            }
                        }
                    }
                    
                    if !found_work {
                        backoff.snooze();
                        
                        if backoff.is_completed() {
                            break; // No more work available
                        }
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all workers to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Collect results (implementation depends on result collection strategy)
        Vec::new() // Placeholder
    }
    
    fn execute_task(task: StringTask) {
        match task {
            StringTask::Split { input, delimiters, start, end, result_sender } => {
                let chunk = &input[start..end];
                let delim_refs: Vec<&str> = delimiters.iter().map(|s| s.as_str()).collect();
                
                let results: Vec<String> = crate::string::split()
                    .src(chunk)
                    .delimeter(delim_refs)
                    .perform()
                    .map(|s| s.string.into_owned())
                    .collect();
                    
                let _ = result_sender.send(results);
            },
            StringTask::Process { tokens, processor, result_sender } => {
                let results: Vec<String> = tokens
                    .into_iter()
                    .map(|token| processor(&token))
                    .collect();
                    
                let _ = result_sender.send(results);
            },
        }
    }
}
```

#### 3. NUMA-Aware Memory Management

```rust
use std::collections::HashMap;

/// NUMA-aware parallel string processor
pub struct NUMAStringProcessor {
    numa_nodes: Vec<NUMANode>,
    thread_affinity: HashMap<usize, usize>, // thread_id -> numa_node
}

#[derive(Debug)]
struct NUMANode {
    id: usize,
    memory_pool: crate::memory_pool::StringArena,
    worker_threads: Vec<usize>,
}

impl NUMAStringProcessor {
    pub fn new() -> Self {
        let numa_topology = Self::detect_numa_topology();
        let numa_nodes = Self::initialize_numa_nodes(numa_topology);
        
        Self {
            numa_nodes,
            thread_affinity: HashMap::new(),
        }
    }
    
    /// Process string data with NUMA locality optimization
    pub fn process_parallel<F, R>(
        &mut self,
        input: &str,
        chunk_size: usize,
        processor: F,
    ) -> Vec<R>
    where
        F: Fn(&str) -> R + Send + Sync + Clone,
        R: Send,
    {
        // Divide input into NUMA-aware chunks
        let chunks = self.create_numa_aware_chunks(input, chunk_size);
        
        // Process chunks on appropriate NUMA nodes
        let mut results = Vec::new();
        let mut handles = Vec::new();
        
        for (chunk_data, numa_node_id) in chunks {
            let processor = processor.clone();
            let numa_node = &mut self.numa_nodes[numa_node_id];
            
            // Allocate processing buffer on correct NUMA node
            let local_buffer = numa_node.memory_pool.alloc_str(&chunk_data);
            
            let handle = std::thread::spawn(move || {
                // Set thread affinity to NUMA node
                Self::set_thread_affinity(numa_node_id);
                
                // Process data with local memory access
                processor(local_buffer)
            });
            
            handles.push(handle);
        }
        
        // Collect results
        for handle in handles {
            results.push(handle.join().unwrap());
        }
        
        results
    }
    
    fn detect_numa_topology() -> Vec<usize> {
        // Platform-specific NUMA detection
        // This is a simplified version - real implementation would use
        // libnuma on Linux, GetNumaHighestNodeNumber on Windows, etc.
        
        #[cfg(target_os = "linux")]
        {
            // Read from /sys/devices/system/node/
            std::fs::read_dir("/sys/devices/system/node/")
                .map(|entries| {
                    entries
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let name = entry.file_name().to_string_lossy().into_owned();
                            if name.starts_with("node") {
                                name[4..].parse::<usize>().ok()
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .unwrap_or_else(|_| vec![0]) // Fallback to single node
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            vec![0] // Single NUMA node fallback
        }
    }
}
```

#### 4. Parallel Streaming with Backpressure

```rust
use tokio::sync::mpsc;
use tokio::stream::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

/// Parallel streaming processor with configurable parallelism
pub struct ParallelStreamProcessor<T> {
    input_stream: Pin<Box<dyn Stream<Item = String> + Send>>,
    processor: Box<dyn Fn(String) -> T + Send + Sync>,
    parallelism: usize,
    buffer_size: usize,
}

impl<T> ParallelStreamProcessor<T>
where
    T: Send + 'static,
{
    pub fn new<S, F>(input: S, processor: F, parallelism: usize) -> Self
    where
        S: Stream<Item = String> + Send + 'static,
        F: Fn(String) -> T + Send + Sync + 'static,
    {
        Self {
            input_stream: Box::pin(input),
            processor: Box::new(processor),
            parallelism,
            buffer_size: parallelism * 2, // Buffer to keep workers busy
        }
    }
    
    /// Process stream in parallel with backpressure
    pub fn process(self) -> impl Stream<Item = T> {
        ParallelStreamOutput::new(
            self.input_stream,
            self.processor,
            self.parallelism,
            self.buffer_size,
        )
    }
}

struct ParallelStreamOutput<T> {
    input_stream: Pin<Box<dyn Stream<Item = String> + Send>>,
    processor: Arc<dyn Fn(String) -> T + Send + Sync>,
    sender: mpsc::UnboundedSender<String>,
    receiver: mpsc::UnboundedReceiver<T>,
    active_tasks: usize,
    max_parallelism: usize,
}

impl<T> ParallelStreamOutput<T>
where
    T: Send + 'static,
{
    fn new(
        input_stream: Pin<Box<dyn Stream<Item = String> + Send>>,
        processor: Box<dyn Fn(String) -> T + Send + Sync>,
        parallelism: usize,
        buffer_size: usize,
    ) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        
        Self {
            input_stream,
            processor: Arc::from(processor),
            sender: tx,
            receiver: rx,
            active_tasks: 0,
            max_parallelism: parallelism,
        }
    }
    
    fn spawn_processing_task(&mut self, input: String) {
        if self.active_tasks >= self.max_parallelism {
            return; // Backpressure - don't spawn more tasks
        }
        
        let processor = Arc::clone(&self.processor);
        let sender = self.sender.clone();
        
        tokio::spawn(async move {
            let result = processor(input);
            let _ = sender.send(result); // Send result back
        });
        
        self.active_tasks += 1;
    }
}

impl<T> Stream for ParallelStreamOutput<T>
where
    T: Send + 'static,
{
    type Item = T;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Try to get results first
        match self.receiver.poll_recv(cx) {
            Poll::Ready(Some(result)) => {
                self.active_tasks -= 1;
                return Poll::Ready(Some(result));
            },
            Poll::Ready(None) => return Poll::Ready(None), // Stream ended
            Poll::Pending => {},
        }
        
        // Try to spawn more tasks if we have capacity
        if self.active_tasks < self.max_parallelism {
            match self.input_stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(input)) => {
                    self.spawn_processing_task(input);
                    // Continue polling for results
                    self.poll_next(cx)
                },
                Poll::Ready(None) => {
                    // Input stream ended, wait for remaining tasks
                    if self.active_tasks == 0 {
                        Poll::Ready(None)
                    } else {
                        Poll::Pending
                    }
                },
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Pending // Wait for tasks to complete
        }
    }
}
```

#### 5. High-Level Parallel API Integration

```rust
/// High-level parallel string processing API
pub trait ParallelStringExt {
    /// Split string in parallel across multiple threads
    fn par_split(&self, delimiters: &[&str]) -> ParallelSplitIterator<'_>;
    
    /// Process string chunks in parallel
    fn par_process<F, R>(&self, chunk_size: usize, processor: F) -> Vec<R>
    where
        F: Fn(&str) -> R + Send + Sync,
        R: Send;
    
    /// Parallel search with work distribution
    fn par_find_all(&self, patterns: &[&str]) -> Vec<(usize, String)>;
    
    /// Map over split results in parallel
    fn par_split_map<F, R>(&self, delimiters: &[&str], mapper: F) -> Vec<R>
    where
        F: Fn(&str) -> R + Send + Sync,
        R: Send;
}

impl ParallelStringExt for str {
    fn par_split(&self, delimiters: &[&str]) -> ParallelSplitIterator<'_> {
        ParallelSplit::new()
            .split_parallel(self, delimiters)
    }
    
    fn par_process<F, R>(&self, chunk_size: usize, processor: F) -> Vec<R>
    where
        F: Fn(&str) -> R + Send + Sync,
        R: Send,
    {
        self.par_chunks(chunk_size)
            .map(processor)
            .collect()
    }
    
    fn par_find_all(&self, patterns: &[&str]) -> Vec<(usize, String)> {
        use rayon::prelude::*;
        
        // Parallel search across patterns
        patterns
            .par_iter()
            .flat_map(|pattern| {
                // Parallel search within string for each pattern
                self.match_indices(pattern)
                    .par_bridge()
                    .map(|(pos, matched)| (pos, matched.to_string()))
            })
            .collect()
    }
    
    fn par_split_map<F, R>(&self, delimiters: &[&str], mapper: F) -> Vec<R>
    where
        F: Fn(&str) -> R + Send + Sync,
        R: Send,
    {
        self.par_split(delimiters)
            .flat_map(|chunk_results| {
                chunk_results.into_par_iter().map(&mapper)
            })
            .collect()
    }
}
```

### Technical Requirements

#### Scalability
- **Linear scaling** with core count for embarrassingly parallel operations
- **Load balancing** to handle uneven work distribution
- **Work stealing** to maximize CPU utilization
- **NUMA awareness** for optimal memory locality on multi-socket systems

#### Synchronization
- **Lock-free algorithms** where possible to avoid contention
- **Minimal synchronization** overhead for task coordination
- **Backpressure mechanisms** to prevent memory exhaustion
- **Graceful degradation** when thread pool is exhausted

#### Memory Management
- **Thread-local memory** pools to avoid allocation contention
- **NUMA-aware allocation** for optimal memory access patterns  
- **Bounded memory usage** even with unlimited input streams
- **Cache-friendly** data structures and access patterns

### Performance Targets

| Operation | Single Thread | Parallel (8 cores) | Improvement |
|-----------|---------------|-------------------|-------------|
| **Large file splitting** | 2.4 GB/s | 15.8 GB/s | **6.6x faster** |
| **Pattern search** | 890 MB/s | 6.2 GB/s | **7.0x faster** |
| **Text processing** | 445 MB/s | 3.1 GB/s | **7.0x faster** |
| **CSV parsing** | 234 MB/s | 1.6 GB/s | **6.8x faster** |

#### Scalability Characteristics
- **2 cores**: 1.8-1.9x speedup (90-95% efficiency)
- **4 cores**: 3.5-3.8x speedup (87-95% efficiency)  
- **8 cores**: 6.6-7.0x speedup (82-87% efficiency)
- **16 cores**: 11.2-13.4x speedup (70-84% efficiency)

### Implementation Steps

1. **Implement basic parallel split** with chunk boundary handling
2. **Add work-stealing executor** for dynamic load balancing
3. **Create NUMA-aware processing** for multi-socket systems
4. **Implement parallel streaming** with backpressure control
5. **Build high-level parallel APIs** integrating with existing interfaces
6. **Add comprehensive benchmarking** across different core counts
7. **Performance tuning** and optimization for various workload patterns

### Challenges & Solutions

#### Challenge: Chunk Boundary Management
**Solution**: Overlap regions and delimiter-aware boundary detection
```rust
fn find_safe_chunk_boundary(input: &str, proposed_end: usize, delimiters: &[&str]) -> usize {
    // Create overlap region to handle cross-boundary delimiters
    let max_delim_len = delimiters.iter().map(|d| d.len()).max().unwrap_or(0);
    let overlap_start = proposed_end.saturating_sub(max_delim_len * 2);
    
    // Search backwards for complete delimiter
    for i in (overlap_start..proposed_end).rev() {
        for delimiter in delimiters {
            if input[i..].starts_with(delimiter) {
                return i + delimiter.len(); // Safe boundary after complete delimiter
            }
        }
    }
    
    // Fallback to UTF-8 character boundary
    while !input.is_char_boundary(proposed_end) {
        proposed_end -= 1;
    }
    proposed_end
}
```

#### Challenge: Load Balancing for Uneven Work
**Solution**: Dynamic work stealing with fine-grained tasks  
```rust
impl WorkStealingExecutor {
    fn subdivide_large_task(&self, task: StringTask) -> Vec<StringTask> {
        match task {
            StringTask::Split { input, delimiters, start, end, .. } => {
                let size = end - start;
                if size > self.max_task_size {
                    // Subdivide into smaller tasks
                    let mid = start + size / 2;
                    let safe_mid = self.find_safe_boundary(&input, mid, &delimiters);
                    
                    vec![
                        StringTask::Split { /* first half */ },
                        StringTask::Split { /* second half */ },
                    ]
                } else {
                    vec![task] // Keep as single task
                }
            },
        }
    }
}
```

#### Challenge: Memory Scaling with Thread Count
**Solution**: Adaptive memory pool sizing based on available memory
```rust
impl ParallelMemoryManager {
    fn calculate_optimal_memory_per_thread(&self) -> usize {
        let total_memory = Self::get_available_memory();
        let num_threads = self.thread_count;
        let memory_per_thread = total_memory / (num_threads * 4); // Reserve 75% for other uses
        
        // Clamp to reasonable bounds
        memory_per_thread.clamp(64 * 1024, 128 * 1024 * 1024) // 64KB - 128MB per thread
    }
}
```

### Success Criteria

- [ ] **6x speedup** on 8-core systems for large input processing
- [ ] **Linear scaling** up to available core count with 80%+ efficiency
- [ ] **NUMA awareness** showing performance benefits on multi-socket systems
- [ ] **Memory usage scaling** that doesn't exceed 2x single-threaded usage
- [ ] **Graceful degradation** when system resources are constrained
- [ ] **Backward compatibility** with existing single-threaded APIs

### Benchmarking Strategy

#### Scalability Benchmarks
```rust
#[bench]
fn bench_parallel_scaling(b: &mut Bencher) {
    let input = generate_large_test_input(100 * 1024 * 1024); // 100MB
    let thread_counts = [1, 2, 4, 8, 16];
    
    for thread_count in thread_counts {
        b.iter_with_setup(
            || rayon::ThreadPoolBuilder::new().num_threads(thread_count).build().unwrap(),
            |pool| {
                pool.install(|| {
                    let results: Vec<_> = input
                        .par_split(&["\n"])
                        .flat_map(|chunk| chunk.into_par_iter())
                        .collect();
                    black_box(results.len())
                })
            }
        );
    }
}

#[bench]
fn bench_numa_awareness(b: &mut Bencher) {
    let input = generate_numa_test_data();
    
    b.iter(|| {
        let mut numa_processor = NUMAStringProcessor::new();
        let results = numa_processor.process_parallel(&input, 1024 * 1024, |chunk| {
            // Simulate processing
            chunk.len()
        });
        black_box(results)
    });
}
```

#### Memory Usage Analysis
- **Memory scaling** with thread count measurement
- **NUMA locality** validation using hardware performance counters
- **Cache performance** analysis across different parallelization strategies
- **Allocation overhead** comparison between parallel and serial approaches

### Integration Points

#### SIMD Compatibility
- Parallel SIMD processing with thread-local SIMD state
- Work distribution strategies that maintain SIMD alignment
- Hybrid CPU + SIMD parallelization for maximum throughput

#### Zero-Copy Integration
- Thread-safe zero-copy sharing using Arc and lifetime management
- Parallel processing with minimal data copying between threads
- NUMA-aware zero-copy allocation strategies

### Usage Examples

#### Basic Parallel Processing
```rust
use strs_tools::parallel::ParallelStringExt;

// Parallel split for large inputs
let large_log = read_huge_file("access.log");
let entries: Vec<_> = large_log
    .par_split(&["\n"])
    .flat_map(|chunk| chunk.into_iter())
    .collect();

// Parallel processing with custom logic
let processed: Vec<_> = large_text
    .par_process(64 * 1024, |chunk| {
        expensive_analysis(chunk)
    });

// Parallel search across multiple patterns  
let matches = document
    .par_find_all(&["error", "warning", "critical"])
    .into_iter()
    .collect();
```

#### Advanced Parallel Streaming
```rust
use strs_tools::parallel::ParallelStreamProcessor;
use tokio_util::codec::{FramedRead, LinesCodec};

// Parallel processing of incoming stream
let file_stream = FramedRead::new(file, LinesCodec::new());
let processed_stream = ParallelStreamProcessor::new(
    file_stream,
    |line| expensive_line_processing(line),
    8, // 8-way parallelism
).process();

// Consume results as they become available
while let Some(result) = processed_stream.next().await {
    handle_processed_result(result);
}
```

### Documentation Requirements

Update documentation with:
- **Parallel processing guide** with performance tuning recommendations
- **Scalability characteristics** for different workload types
- **NUMA optimization** guidance for multi-socket systems  
- **Memory usage patterns** and optimization strategies

### Related Tasks

- Task 001: SIMD optimization (parallel SIMD processing strategies)
- Task 004: Memory pool allocation (thread-local memory pool management)
- Task 006: Streaming evaluation (parallel streaming with backpressure)
- Task 008: Parser integration (parallel parsing pipeline optimization)