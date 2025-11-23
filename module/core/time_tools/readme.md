<!-- {{# generate.module_header{} #}} -->

# Module :: time_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_time_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_time_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/time_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/time_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ftime_tools%2Fexamples%2Ftime_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Ftime_tools%2Fexamples%2Ftime_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose time tools.

A minimal, zero-dependency crate providing simple functions for obtaining current time from UNIX epoch in multiple units (seconds, milliseconds, nanoseconds). Wraps `std::time::SystemTime` with ergonomic module-based APIs for different time granularities.

## Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
use time_tools ::*;

// Get milliseconds since UNIX epoch (default)
let now_ms = now();
println!( "Current time: {} ms since epoch", now_ms );

// Get time in different units
let now_s = s ::now();   // Seconds
let now_ns = ns ::now(); // Nanoseconds

// Unit conversions are consistent
assert_eq!( now_ms / 1000, now_s );
```

## API Reference

### Time Retrieval Functions

All functions return `i64` representing time since UNIX epoch (1970-01-01 00:00:00 UTC).

#### `time_tools::now() -> i64`

Returns current time in **milliseconds** since UNIX epoch.

This is the default and most commonly used function.

```rust
use time_tools ::now;

let timestamp_ms = now();
println!( "Current time: {} ms since epoch", timestamp_ms );
```

**Panics:** If system time is before UNIX epoch (1970-01-01). This indicates a misconfigured system clock.

**Performance:** Typically <100ns per call, zero heap allocations.

---

#### `time_tools::ms::now() -> i64`

Returns current time in **milliseconds** since UNIX epoch.

Explicit milliseconds module, functionally identical to `now()`.

```rust
use time_tools ::ms;

let timestamp_ms = ms ::now();
println!( "Current time: {} milliseconds", timestamp_ms );
```

**Panics:** If system time is before UNIX epoch (1970-01-01).

**Performance:** Typically <100ns per call, zero heap allocations.

---

#### `time_tools::s::now() -> i64`

Returns current time in **seconds** since UNIX epoch.

Use for coarse-grained timing where second precision is sufficient.

```rust
use time_tools ::s;

let timestamp_s = s ::now();
println!( "Current time: {} seconds since epoch", timestamp_s );
```

**Panics:** If system time is before UNIX epoch (1970-01-01).

**Performance:** Typically <100ns per call, zero heap allocations.

---

#### `time_tools::ns::now() -> i64`

Returns current time in **nanoseconds** since UNIX epoch.

Use for high-precision timing within a single process. Note that this is still wall-clock time, not monotonic.

```rust
use time_tools ::ns;

let timestamp_ns = ns ::now();
println!( "Current time: {} nanoseconds", timestamp_ns );
```

**Panics:** If system time is before UNIX epoch (1970-01-01).

**Performance:** Typically <100ns per call, zero heap allocations.

**Note:** `i64` nanoseconds will overflow after approximately 292 years from epoch (year 2262).

## Usage Examples

### Basic Time Retrieval

```rust
use time_tools ::*;

// Default: milliseconds (most common use case)
let now_ms = now();

// Explicit unit selection
let seconds = s ::now();
let milliseconds = ms ::now();
let nanoseconds = ns ::now();
```

### Cross-Unit Conversion Verification

```rust
use time_tools ::*;

let now_ms = now();
let now_s = s ::now();
let now_ns = ns ::now();

// Verify unit relationships
assert_eq!( now_ms / 1000, now_s );
assert_eq!( now_ns / 1_000_000, now_ms );
```

### Measuring Elapsed Time

```rust
use time_tools ::*;

let start = now();

// Do some work...
std ::thread ::sleep( std ::time ::Duration ::from_millis( 100 ) );

let end = now();
let elapsed_ms = end - start;

println!( "Operation took {} ms", elapsed_ms );
```

### Integration with std::time::Duration

```rust
use time_tools ::*;
use std ::time ::Duration;

// Create a duration from time_tools timestamp difference
let start = now();
std ::thread ::sleep( Duration ::from_millis( 50 ) );
let elapsed_ms = now() - start;

// Convert to Duration if needed
let duration = Duration ::from_millis( elapsed_ms as u64 );
println!( "Elapsed: {:?}", duration );
```

### High-Precision Timing

```rust
use time_tools ::ns;

let start_ns = ns ::now();

// Precise operation...

let end_ns = ns ::now();
let elapsed_ns = end_ns - start_ns;

println!( "Operation took {} nanoseconds", elapsed_ns );
```

### Simple Logging with Timestamps

```rust
use time_tools ::now;

fn log_event( message : &str )
{
  let timestamp = now();
  println!( "[{}] {}", timestamp, message );
}

log_event( "Application started" );
log_event( "Processing complete" );
```

## Performance Characteristics

| Characteristic | Value |
|----------------|-------|
| **Call Latency** | <100ns typical |
| **Heap Allocations** | Zero |
| **Thread Safety** | Safe (no shared state) |
| **Clock Type** | Wall clock (SystemTime) |

### Important Notes

- **Not Monotonic:** Uses `SystemTime`, which can jump backwards due to NTP adjustments or manual clock changes. For benchmarking or measuring elapsed time where backwards jumps are unacceptable, use `std::time::Instant` instead.

- **Zero Dependencies:** No external crates required in production. Uses only `std::time`.

- **no_std Support:** The crate supports `#![no_std]` via feature flag, but time functions require `std::time` so they are disabled in no_std mode.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | Yes | Master switch for the crate |
| `time_now` | Yes | Enable time retrieval functions |
| `no_std` | No | Enable no_std compatibility (disables time functions) |
| `use_alloc` | No | Enable allocation in no_std mode |
| `chrono` | No | Reserved for future chrono integration |

## When to Use time_tools

**Good for:**
- Simple timestamp logging
- Basic elapsed time measurement
- Rate limiting implementations
- Timeout checking
- Cache expiration tracking

**Use alternatives for:**
- High-precision benchmarking (`std::time::Instant`)
- Date arithmetic (`chrono` crate)
- Timezone conversions (`chrono` crate)
- Formatted date display (`chrono` crate)

## To add to your project

```sh
cargo add time_tools
```

## Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/time_tools_trivial
cargo run
```
