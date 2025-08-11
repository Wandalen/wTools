# Task: Improve API Consistency for CommandResult and Error Handling

**Task ID:** 019  
**Priority:** Medium  
**Status:** Not Started  
**Responsible:** @maintainers  
**Created:** 2025-01-10  

## Problem Statement

During CLI integration work, several API inconsistencies were discovered in unilang's command processing and error handling that create confusion and require workarounds:

1. **CommandResult Structure Inconsistency**: The `CommandResult` returned by `pipeline.process_command()` has unclear success/failure semantics
2. **Error Message Format Variations**: Different error types return inconsistent message formats
3. **Missing Helper Methods**: Common operations require verbose code patterns
4. **Undocumented Error Codes**: Special error codes like `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` are not well documented

## Current API Issues

### 1. CommandResult Success Detection

Current usage requires checking both `error` field and `success` boolean:

```rust
// Current - unclear which is authoritative
let result = pipeline.process_command(input, context);
if result.success && result.error.is_none() {
    // Handle success
} else {
    // Handle error - but which field to trust?
}
```

### 2. Error Message Parsing

Special error handling requires string matching:

```rust
// Current - fragile string matching
if error.contains("UNILANG_ARGUMENT_INTERACTIVE_REQUIRED") {
    handle_interactive_prompt();
} else if error.contains("Available commands:") {
    show_help_from_error();
}
```

### 3. Static Command Limitations

Error message reveals internal limitation:

```rust
// Current - exposes implementation details
"The .version command is a static command without an executable routine"
```

## Requested Improvements

### 1. CommandResult API Enhancement

```rust
impl CommandResult {
    /// Returns true if command executed successfully
    pub fn is_success(&self) -> bool {
        self.error.is_none() && self.success
    }
    
    /// Returns true if command failed
    pub fn is_error(&self) -> bool {
        !self.is_success()
    }
    
    /// Returns error message if any
    pub fn error_message(&self) -> Option<&str> {
        self.error.as_ref().map(|e| e.as_str())
    }
    
    /// Returns outputs if command succeeded
    pub fn outputs_or_empty(&self) -> &[OutputData] {
        if self.is_success() {
            &self.outputs
        } else {
            &[]
        }
    }
}
```

### 2. Structured Error Types

Replace string matching with typed errors:

```rust
#[derive(Debug, Clone)]
pub enum UnilangError {
    CommandNotFound { command: String, suggestions: Vec<String> },
    InteractiveArgumentRequired { argument: String, command: String },
    StaticCommandNoRoutine { command: String },
    InvalidArguments { message: String },
    ExecutionFailure { message: String },
    HelpRequest { commands: Vec<String> }, // When user types '.'
}

impl CommandResult {
    pub fn error_type(&self) -> Option<UnilangError> {
        // Parse error string into structured type
    }
}
```

### 3. Interactive Argument Detection

```rust
impl CommandResult {
    /// Returns true if error indicates interactive input is required
    pub fn requires_interactive_input(&self) -> bool {
        matches!(self.error_type(), Some(UnilangError::InteractiveArgumentRequired { .. }))
    }
    
    /// Returns argument name that requires interactive input
    pub fn interactive_argument(&self) -> Option<&str> {
        if let Some(UnilangError::InteractiveArgumentRequired { argument, .. }) = self.error_type() {
            Some(&argument)
        } else {
            None
        }
    }
}
```

### 4. Help System Integration

```rust
impl CommandResult {
    /// Returns true if error contains help information
    pub fn is_help_response(&self) -> bool {
        matches!(self.error_type(), Some(UnilangError::HelpRequest { .. }))
    }
    
    /// Extracts formatted help content from error
    pub fn help_content(&self) -> Option<String> {
        if let Some(UnilangError::HelpRequest { commands }) = self.error_type() {
            Some(format_help_content(&commands))
        } else {
            None
        }
    }
}
```

## Implementation Plan

### Phase 1: Backward Compatible Additions
1. Add helper methods to `CommandResult` without breaking existing API
2. Implement `UnilangError` enum with parsing from existing error strings
3. Add comprehensive tests for new API methods

### Phase 2: Documentation Updates
1. Update API documentation with new helper methods
2. Add examples showing improved error handling patterns
3. Document error codes and their meanings

### Phase 3: Example Modernization
1. Update examples to use new helper methods
2. Show best practices for error handling
3. Demonstrate interactive argument handling

### Phase 4: Deprecation (Future)
1. Consider deprecating direct field access in favor of helper methods
2. Plan migration path for major version update

## Success Criteria

1. **Error Handling Clarity**: Developers can handle errors without string matching
2. **API Consistency**: All command processing follows same patterns
3. **Reduced Boilerplate**: Common operations require less code
4. **Better IDE Support**: Structured errors enable better autocomplete and documentation

## Example Usage After Implementation

```rust
let result = pipeline.process_command(input, context);

match result.error_type() {
    None => {
        // Command succeeded
        for output in result.outputs_or_empty() {
            println!("{}", output.content);
        }
    }
    Some(UnilangError::InteractiveArgumentRequired { argument, .. }) => {
        let secure_input = prompt_secure_input(&argument);
        retry_with_argument(input, &argument, &secure_input);
    }
    Some(UnilangError::HelpRequest { .. }) => {
        println!("{}", result.help_content().unwrap());
    }
    Some(UnilangError::CommandNotFound { suggestions, .. }) => {
        println!("Command not found. Did you mean: {}", suggestions.join(", "));
    }
    Some(error) => {
        println!("Error: {}", result.error_message().unwrap_or("Unknown error"));
    }
}
```

## Related Issues

This addresses usability issues discovered during:
- tilemap_renderer CLI integration
- Example development and testing
- Developer experience feedback

## Testing Requirements

1. Unit tests for all new helper methods
2. Integration tests showing error handling patterns
3. Backward compatibility tests ensuring existing code continues working
4. Performance tests ensuring no regression in command processing speed