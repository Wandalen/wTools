# Improve Secrets API UX and Error Handling

## Description

Improve the secrets API user experience by addressing critical usability pitfalls and enhancing error handling to prevent common developer mistakes. The current API has several issues that lead to confusion and debugging difficulties:

1. **Misleading parameter naming**: `filename` parameter in `load_secrets_from_file()` is actually treated as a path component
2. **Silent failure**: Missing files return empty HashMap instead of errors
3. **Poor error context**: Error messages don't explain path resolution logic
4. **Inadequate documentation**: Examples don't clarify filename vs. path distinction

This task focuses on improving developer experience through better API design, explicit error handling, comprehensive documentation, and helpful debugging tools. The improvements maintain full backward compatibility while adding new methods and enhancing existing ones.

Based on real-world usage analysis from `api_huggingface` project where developers attempted `load_secrets_from_file("lib/llm_tools/secret/-secrets.sh")` expecting it to work as a path, but the API treated it as a filename, resulting in silent failure and debugging confusion.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

### Phase 1: Enhanced Error Handling and Validation

-   [ ] **Explicit file existence errors**: Replace silent empty HashMap returns with explicit `WorkspaceError::ConfigurationError` when files don't exist
-   [ ] **Path validation warnings**: Detect when `filename` parameter contains path separators (`/` or `\`) and emit helpful warnings
-   [ ] **Enhanced error context**: Error messages must include both original parameter and resolved absolute path
-   [ ] **Available files suggestions**: When a file is not found, suggest available files in the secrets directory

### Phase 2: API Method Improvements

-   [ ] **Parameter renaming**: Rename `filename` parameter to `secret_file_name` in `load_secrets_from_file()` with deprecation warning
-   [ ] **New path-aware methods**: Add `load_secrets_from_path()` for workspace-relative paths and `load_secrets_from_absolute_path()` for absolute paths
-   [ ] **Debug helper methods**: Add `secrets_file_exists()`, `resolve_secrets_path()`, and `list_secrets_files()`
-   [ ] **Validation method**: Add `load_secrets_with_debug()` that provides verbose path resolution and validation information

### Phase 3: Documentation and Examples Enhancement

-   [ ] **Comprehensive API documentation**: Update all secrets-related method documentation with clear examples showing correct vs incorrect usage
-   [ ] **Path resolution explanation**: Document how each method resolves paths with explicit examples
-   [ ] **Migration guide**: Create guide for common mistakes and how to fix them
-   [ ] **Example updates**: Update existing examples to demonstrate best practices and common pitfalls

### Phase 4: Testing and Validation

-   [ ] **Pitfall prevention tests**: Add tests that verify error cases (missing files, path-like filenames) produce helpful error messages
-   [ ] **API consistency tests**: Ensure new methods integrate seamlessly with existing functionality
-   [ ] **Documentation tests**: All code examples in documentation must compile and run successfully
-   [ ] **Backward compatibility tests**: Existing code using old API must continue working with deprecation warnings only

## Implementation Plan

### Step 1: Enhanced Error Handling
```rust
// Current (silent failure)
if !secret_file.exists() {
  return Ok( HashMap::new() );
}

// New (explicit error)
if !secret_file.exists() {
  let available = self.list_secrets_files().unwrap_or_default();
  let suggestion = if !available.is_empty() {
    format!("\nAvailable files: {}", available.join(", "))
  } else {
    String::new()
  };
  
  return Err( WorkspaceError::ConfigurationError(
    format!(
      "Secrets file '{}' not found at {}{}", 
      secret_file_name, 
      secret_file.display(),
      suggestion
    )
  ) );
}
```

### Step 2: Parameter Validation
```rust
pub fn load_secrets_from_file( &self, secret_file_name : &str ) -> Result< HashMap< String, String > > 
{
  // Validate parameter doesn't look like a path
  if secret_file_name.contains('/') || secret_file_name.contains('\\') {
    eprintln!(
      "⚠️  Warning: '{}' contains path separators. Use load_secrets_from_path() for paths.",
      secret_file_name
    );
  }
  
  // Rest of implementation
}
```

### Step 3: New API Methods
```rust
/// Load secrets from workspace-relative path
pub fn load_secrets_from_path( &self, relative_path : &str ) -> Result< HashMap< String, String > >

/// Load secrets from absolute path  
pub fn load_secrets_from_absolute_path( &self, absolute_path : &Path ) -> Result< HashMap< String, String > >

/// List available secrets files
pub fn list_secrets_files( &self ) -> Result< Vec< String > >

/// Check if secrets file exists
pub fn secrets_file_exists( &self, secret_file_name : &str ) -> bool

/// Get resolved path for debugging
pub fn resolve_secrets_path( &self, secret_file_name : &str ) -> PathBuf
```

### Step 4: Documentation Template
```rust
/// Load secrets from a file in the workspace secrets directory
///
/// # Path Resolution
/// 
/// Files are resolved as: `workspace_root/secret/{secret_file_name}`
///
/// # Examples
/// 
/// ```rust
/// // ✅ Correct usage - simple filenames
/// let secrets = ws.load_secrets_from_file("-secrets.sh")?;      // -> secret/-secrets.sh
/// let dev = ws.load_secrets_from_file("dev.env")?;             // -> secret/dev.env
/// 
/// // ❌ Common mistake - using paths
/// // let secrets = ws.load_secrets_from_file("config/secrets.sh")?; // DON'T DO THIS
/// 
/// // ✅ For paths, use the path-specific method
/// let secrets = ws.load_secrets_from_path("config/secrets.sh")?;  // -> workspace/config/secrets.sh
/// ```
```

## Success Metrics

-   **Zero silent failures**: All missing file cases produce explicit errors
-   **Clear error messages**: All errors include both input parameter and resolved path
-   **Intuitive API**: Developers can distinguish between filename and path parameters
-   **Comprehensive documentation**: Examples cover both correct usage and common mistakes
-   **Backward compatibility**: Existing code works with deprecation warnings only

## Migration Strategy

1. **Phase 1**: Add new methods alongside existing ones
2. **Phase 2**: Add deprecation warnings to methods with confusing parameters  
3. **Phase 3**: Update all documentation and examples
4. **Phase 4**: Plan future version with renamed parameters as defaults

## Related Issues

This task addresses developer experience issues discovered in:
- `api_huggingface` project secret loading confusion
- General workspace_tools API usability feedback
- Need for better debugging tools for path resolution

## Priority: High

**Value**: 9/10 - Critical UX improvement preventing common developer mistakes
**Easiness**: 7/10 - Mostly additive changes with clear implementation path  
**Safety**: 9/10 - Maintains backward compatibility while improving safety
**Advisability**: 10/10 - Essential for developer productivity and API adoption