# Task: Pretty Error Display & Formatting Enhancement

## Priority: High
## Impact: Significantly improves developer and end-user experience
## Estimated Effort: 3-4 days

## Problem Statement

Based on recent real-world usage, applications using error_tools often display raw debug output instead of user-friendly error messages. For example, in the game CLI project, errors appeared as:

```
Error: Execution(ErrorData { code: "HELP_REQUESTED", message: "Available commands:\n\n  .session.play        \n  .session.status      Display the current session status\n  .turn.end             \n  .version             Show version information\n\nUse '<command> ?' to get detailed help for a specific command.\n", source: None })
```

Instead of the clean, intended output:
```
Available commands:

  .session.play        
  .session.status      Display the current session status
  .turn.end             
  .version             Show version information

Use '<command> ?' to get detailed help for a specific command.
```

## Research Phase Requirements

**IMPORTANT: Research must be conducted before implementation begins.**

### Research Tasks:
1. **Survey existing error formatting libraries**:
   - `color-eyre` (for colored, formatted error display)
   - `miette` (diagnostic-style error reporting)  
   - `anyhow` chain formatting
   - `thiserror` display implementations
   
2. **Analyze error_tools current architecture**:
   - Review current error types (`typed`, `untyped`)
   - Understand feature gate structure
   - Identify integration points for formatting

3. **Define formatting requirements**:
   - Terminal color support detection
   - Structured vs. plain text output
   - Error chain visualization
   - Context information display

4. **Performance analysis**:
   - Measure overhead of formatting features
   - Identify which features need optional compilation
   - Benchmark against baseline error display

## Solution Approach

### Phase 1: Research & Design (1 day)
Complete research tasks above and create detailed design document.

### Phase 2: Core Pretty Display Infrastructure (1-2 days)

#### 1. Add New Cargo Features
```toml
[features]
# Existing features...
pretty_display = ["error_formatted", "dep:owo-colors"]
error_formatted = []  # Basic structured formatting
error_colored = ["error_formatted", "dep:supports-color", "dep:owo-colors"]  # Terminal colors
error_context = ["error_formatted"]  # Rich context display
error_suggestions = ["error_formatted"]  # Error suggestions and hints
```

#### 2. Create Pretty Display Trait
```rust
/// Trait for pretty error display with context and formatting
pub trait PrettyDisplay {
    /// Display error with basic formatting (no colors)
    fn pretty_display(&self) -> String;
    
    /// Display error with colors if terminal supports it
    #[cfg(feature = "error_colored")]
    fn pretty_display_colored(&self) -> String;
    
    /// Display error with suggestions and context
    #[cfg(feature = "error_context")]
    fn pretty_display_with_context(&self) -> String;
}
```

#### 3. Implement for Existing Error Types
```rust
impl PrettyDisplay for crate::error::typed::Error {
    fn pretty_display(&self) -> String {
        // Format structured error without debug wrapper
        format!("{}", self.message)  // Extract clean message
    }
    
    #[cfg(feature = "error_colored")]
    fn pretty_display_colored(&self) -> String {
        use owo_colors::OwoColorize;
        match self.severity {
            ErrorSeverity::Error => format!("❌ {}", self.message.red()),
            ErrorSeverity::Warning => format!("⚠️  {}", self.message.yellow()),
            ErrorSeverity::Info => format!("ℹ️  {}", self.message.blue()),
        }
    }
}
```

### Phase 3: Integration Helpers (1 day)

#### 1. Convenience Macros
```rust
/// Pretty print error to stderr with colors if supported
#[macro_export]
#[cfg(feature = "pretty_display")]
macro_rules! epretty {
    ($err:expr) => {
        #[cfg(feature = "error_colored")]
        {
            if supports_color::on(supports_color::Stream::Stderr).is_some() {
                eprintln!("{}", $err.pretty_display_colored());
            } else {
                eprintln!("{}", $err.pretty_display());
            }
        }
        #[cfg(not(feature = "error_colored"))]
        {
            eprintln!("{}", $err.pretty_display());
        }
    };
}

/// Pretty print error to stdout  
#[macro_export]
#[cfg(feature = "pretty_display")]
macro_rules! pprintln {
    ($err:expr) => {
        #[cfg(feature = "error_colored")]
        {
            if supports_color::on(supports_color::Stream::Stdout).is_some() {
                println!("{}", $err.pretty_display_colored());
            } else {
                println!("{}", $err.pretty_display());
            }
        }
        #[cfg(not(feature = "error_colored"))]
        {
            println!("{}", $err.pretty_display());
        }
    };
}
```

#### 2. Helper Functions
```rust
#[cfg(feature = "pretty_display")]
pub fn display_error_pretty(error: &dyn std::error::Error) -> String {
    // Smart error chain formatting
}

#[cfg(feature = "error_context")]
pub fn display_error_with_context(error: &dyn std::error::Error, context: &str) -> String {
    // Error with additional context
}
```

### Phase 4: Advanced Features (1 day)

#### 1. Error Chain Visualization
```rust
#[cfg(feature = "error_context")]
impl ErrorChainDisplay for Error {
    fn display_chain(&self) -> String {
        // Visual error chain like:
        // ┌─ Main Error: Command failed
        // ├─ Caused by: Network timeout  
        // └─ Root cause: Connection refused
    }
}
```

#### 2. Suggestion System
```rust
#[cfg(feature = "error_suggestions")]  
pub trait ErrorSuggestions {
    fn suggestions(&self) -> Vec<String>;
    fn display_with_suggestions(&self) -> String;
}
```

## Technical Requirements

### Dependencies (All Optional)
```toml
[dependencies]
# Existing dependencies...

# Pretty display features
owo-colors = { version = "4.0", optional = true }  # Terminal colors
supports-color = { version = "3.0", optional = true }  # Color support detection
```

### Performance Constraints
- **Zero overhead when features disabled**: No runtime cost for basic error handling
- **Lazy formatting**: Only format when explicitly requested
- **Minimal allocations**: Reuse buffers where possible
- **Feature-gated dependencies**: Heavy dependencies only when needed

### Compatibility Requirements
- **Maintain existing API**: All current functionality preserved
- **Feature flag isolation**: Each feature can be enabled/disabled independently
- **no_std compatibility**: Core functionality works in no_std environments
- **Backward compatibility**: Existing error types unchanged

## Testing Strategy

### Unit Tests
1. **Feature flag combinations**: Test all valid feature combinations
2. **Formatting correctness**: Verify clean message extraction
3. **Color detection**: Test terminal color support detection
4. **Performance regression**: Ensure no overhead when features disabled

### Integration Tests
1. **Real error scenarios**: Test with actual application errors
2. **Terminal compatibility**: Test across different terminal types
3. **Chain formatting**: Test complex error chains
4. **Memory usage**: Validate no memory leaks in formatting

### Example Usage Tests
```rust
#[test]
#[cfg(feature = "pretty_display")]
fn test_pretty_display_basic() {
    let error = create_test_error();
    let pretty = error.pretty_display();
    assert!(!pretty.contains("ErrorData {"));  // No debug wrapper
    assert!(!pretty.contains("source: None"));  // No debug fields
}

#[test] 
#[cfg(feature = "error_colored")]
fn test_colored_output() {
    let error = create_test_error();
    let colored = error.pretty_display_colored();
    assert!(colored.contains("\x1b["));  // ANSI color codes present
}
```

## Success Criteria

- [x] **Clean message extraction**: Errors display intended content, not debug wrappers
- [x] **Zero performance overhead**: No impact when features disabled  
- [x] **Optional dependencies**: Heavy deps only loaded when needed
- [x] **Terminal compatibility**: Works across different terminal environments
- [x] **Backward compatibility**: Existing code unchanged
- [x] **Feature modularity**: Each feature independently toggleable

## Integration Examples

### Before (Current State)
```rust
// Raw debug output - not user friendly
eprintln!("Error: {:?}", error);
// Output: Error: Execution(ErrorData { code: "HELP_REQUESTED", message: "...", source: None })
```

### After (With Pretty Display)
```rust  
// Clean, user-friendly output
use error_tools::prelude::*;

epretty!(error);  // Macro handles color detection
// Output: Available commands: ...

// Or explicit control:
println!("{}", error.pretty_display());
```

## Deliverables

1. **Research document** with library survey and requirements analysis
2. **Core PrettyDisplay trait** and implementations
3. **Feature-gated formatting** infrastructure  
4. **Convenience macros** for common usage patterns
5. **Comprehensive test suite** covering all feature combinations
6. **Documentation and examples** for new functionality
7. **Performance benchmarks** validating zero overhead requirement

## Dependencies on Other Work

- **None**: This is a pure enhancement to existing error_tools functionality
- **Synergistic with**: Applications using error_tools (unilang, game projects, etc.)

## Risk Mitigation

- **Feature flags**: Heavy functionality optional to prevent bloat
- **Research phase**: Understand ecosystem before implementation  
- **Incremental delivery**: Core functionality first, advanced features later
- **Performance testing**: Validate no regression in error handling performance