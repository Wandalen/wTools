# Task 002: Improve MarkdownUpdater API Design to Prevent Misuse

## Priority: MEDIUM  
## Status: Open
## Severity: Medium - API design improvement

## Problem Summary

The current `MarkdownUpdater` API makes it easy to create section name conflicts that trigger the substring matching bug (Task 001). Users naturally choose descriptive section names that often share common words, leading to unintended interactions.

## Current API Issues

### 1. No Validation of Section Names
```rust
// These names will conflict due to shared "Performance" substring
MarkdownUpdater::new("readme.md", "Performance Benchmarks");
MarkdownUpdater::new("readme.md", "Language Operations Performance");
MarkdownUpdater::new("readme.md", "Realistic Scenarios Performance");
```

### 2. No Guidance for Safe Section Naming
Users receive no warnings about potentially conflicting section names.

### 3. Silent Failures
When section conflicts occur, the API doesn't report the issue - it just creates duplicate content.

## Proposed API Improvements

### 1. Section Name Validation

Add validation to catch potential conflicts:

```rust
impl MarkdownUpdater {
    /// Create new markdown updater with section name validation
    pub fn new(file_path: impl AsRef<Path>, section_name: &str) -> Result<Self, MarkdownError> {
        // Validate section name doesn't contain problematic characters
        Self::validate_section_name(section_name)?;
        
        Ok(Self {
            file_path: file_path.as_ref().to_path_buf(),
            section_marker: format!("## {section_name}"),
        })
    }
    
    fn validate_section_name(section_name: &str) -> Result<(), MarkdownError> {
        if section_name.trim().is_empty() {
            return Err(MarkdownError::EmptySectionName);
        }
        
        if section_name.len() > 100 {
            return Err(MarkdownError::SectionNameTooLong);
        }
        
        // Warn about potentially problematic patterns
        if section_name.contains('\n') || section_name.contains('\r') {
            return Err(MarkdownError::InvalidCharacters);
        }
        
        Ok(())
    }
}
```

### 2. Conflict Detection

Add method to detect potential section name conflicts:

```rust
impl MarkdownUpdater {
    /// Check if this section name might conflict with existing sections
    pub fn check_conflicts(&self) -> Result<Vec<String>, std::io::Error> {
        if !self.file_path.exists() {
            return Ok(vec![]);
        }
        
        let content = std::fs::read_to_string(&self.file_path)?;
        let existing_sections = Self::extract_section_names(&content);
        
        let target_words: HashSet<_> = self.section_marker
            .trim_start_matches("## ")
            .split_whitespace()
            .collect();
            
        let conflicts: Vec<String> = existing_sections
            .into_iter()
            .filter(|section| {
                let section_words: HashSet<_> = section.split_whitespace().collect();
                // Check for shared words that could cause substring conflicts
                !target_words.is_disjoint(&section_words) && section != &self.section_marker
            })
            .collect();
            
        Ok(conflicts)
    }
    
    fn extract_section_names(content: &str) -> Vec<String> {
        content.lines()
            .filter(|line| line.trim_start().starts_with("## "))
            .map(|line| line.trim().to_string())
            .collect()
    }
}
```

### 3. Safe Builder Pattern

Provide a builder that guides users to safe section names:

```rust
pub struct MarkdownUpdaterBuilder {
    file_path: PathBuf,
    category: Option<String>,
    subcategory: Option<String>,
    suffix: Option<String>,
}

impl MarkdownUpdaterBuilder {
    pub fn new(file_path: impl AsRef<Path>) -> Self {
        Self {
            file_path: file_path.as_ref().to_path_buf(),
            category: None,
            subcategory: None,
            suffix: None,
        }
    }
    
    /// Set the main category (e.g., "Benchmarks", "Results", "Analysis")
    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }
    
    /// Set subcategory (e.g., "Performance", "Memory", "Accuracy")
    pub fn subcategory(mut self, subcategory: &str) -> Self {
        self.subcategory = Some(subcategory.to_string());
        self
    }
    
    /// Set suffix for uniqueness (e.g., "v1", "detailed", "summary")
    pub fn suffix(mut self, suffix: &str) -> Self {
        self.suffix = Some(suffix.to_string());
        self
    }
    
    /// Build the updater with a guaranteed unique section name
    pub fn build(self) -> Result<MarkdownUpdater, MarkdownError> {
        let section_name = match (self.category, self.subcategory, self.suffix) {
            (Some(cat), Some(sub), Some(suf)) => format!("{cat} {sub} {suf}"),
            (Some(cat), Some(sub), None) => format!("{cat} {sub}"),
            (Some(cat), None, Some(suf)) => format!("{cat} {suf}"),
            (Some(cat), None, None) => cat,
            _ => return Err(MarkdownError::InsufficientSectionInfo),
        };
        
        MarkdownUpdater::new(&self.file_path, &section_name)
    }
}
```

### 4. Usage Examples

#### Safe API Usage
```rust
// Recommended approach - explicit and unique
let updater = MarkdownUpdaterBuilder::new("readme.md")
    .category("Benchmarks")
    .subcategory("Performance")  
    .suffix("Line Counting")
    .build()?;
    
// Alternative safe approach
let updater = MarkdownUpdater::new("readme.md", "Line Counting Benchmarks")?;

// Check for conflicts before updating
if let Ok(conflicts) = updater.check_conflicts() {
    if !conflicts.is_empty() {
        println!("Warning: Potential conflicts detected: {:?}", conflicts);
    }
}
```

### 5. Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum MarkdownError {
    #[error("Section name cannot be empty")]
    EmptySectionName,
    
    #[error("Section name is too long (max 100 characters)")]
    SectionNameTooLong,
    
    #[error("Section name contains invalid characters")]
    InvalidCharacters,
    
    #[error("Insufficient information to create unique section name")]
    InsufficientSectionInfo,
    
    #[error("Potential section name conflict detected: {conflicts:?}")]
    SectionConflict { conflicts: Vec<String> },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## Implementation Plan

### Phase 1: Basic Validation (Immediate)
- [ ] Add `MarkdownError` enum
- [ ] Add section name validation to `MarkdownUpdater::new()`  
- [ ] Update existing tests to handle new error cases

### Phase 2: Conflict Detection (Next)
- [ ] Implement `check_conflicts()` method
- [ ] Add tests for conflict detection
- [ ] Update documentation with conflict examples

### Phase 3: Builder Pattern (Future)
- [ ] Implement `MarkdownUpdaterBuilder`
- [ ] Add comprehensive examples
- [ ] Migration guide for existing code

## Testing Requirements

### Unit Tests
```rust
#[test]
fn test_section_name_validation() {
    assert!(MarkdownUpdater::new("test.md", "").is_err());
    assert!(MarkdownUpdater::new("test.md", "Valid Section").is_ok());
    assert!(MarkdownUpdater::new("test.md", "Line\nBreak").is_err());
}

#[test]
fn test_conflict_detection() {
    // Create file with existing sections
    let content = "## Performance Benchmarks\ndata\n## Language Performance\ndata";
    std::fs::write("test.md", content).unwrap();
    
    let updater = MarkdownUpdater::new("test.md", "Performance Results").unwrap();
    let conflicts = updater.check_conflicts().unwrap();
    
    assert!(!conflicts.is_empty());
    assert!(conflicts.iter().any(|c| c.contains("Performance")));
}
```

## Benefits

1. **Prevents Task 001 bug**: Users guided away from conflicting names
2. **Better UX**: Clear errors instead of silent failures  
3. **Future-proof**: Builder pattern enables schema evolution
4. **Backwards compatible**: Existing code continues to work
5. **Self-documenting**: API guides users to best practices

## Migration Strategy

1. **Phase 1**: Add validation as opt-in (feature flag)
2. **Phase 2**: Make validation default, with escape hatch
3. **Phase 3**: Remove escape hatch in next major version

This ensures existing code doesn't break while encouraging safer patterns.

---
**Created by:** wflow development team  
**Date:** Current  
**Assignee:** TBD  
**Labels:** enhancement, api-design, documentation, user-experience