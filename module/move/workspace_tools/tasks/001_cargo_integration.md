# Task 001: Cargo Integration

**Priority**: 🎯 Highest Impact  
**Phase**: 1 (Immediate)  
**Estimated Effort**: 3-4 days  
**Dependencies**: None  

## **Objective**
Implement automatic Cargo workspace detection to eliminate the need for manual `.cargo/config.toml` setup, making workspace_tools adoption frictionless.

## **Technical Requirements**

### **Core Features**
1. **Automatic Workspace Detection**
   - Traverse up directory tree looking for `Cargo.toml` with `[workspace]` section
   - Support both workspace roots and workspace members
   - Handle virtual workspaces (workspace without root package)

2. **Cargo Metadata Integration** 
   - Parse `Cargo.toml` workspace configuration
   - Access workspace member information
   - Integrate with `cargo metadata` command output

3. **Fallback Strategy**
   - Primary: Auto-detect from Cargo workspace
   - Secondary: `WORKSPACE_PATH` environment variable  
   - Tertiary: Current directory/git root

### **New API Surface**
```rust
impl Workspace {
    /// Create workspace from Cargo workspace root (auto-detected)
    pub fn from_cargo_workspace() -> Result<Self>;
    
    /// Create workspace from specific Cargo.toml path
    pub fn from_cargo_manifest<P: AsRef<Path>>(manifest_path: P) -> Result<Self>;
    
    /// Get cargo metadata for this workspace
    pub fn cargo_metadata(&self) -> Result<CargoMetadata>;
    
    /// Check if this workspace is a Cargo workspace
    pub fn is_cargo_workspace(&self) -> bool;
    
    /// Get workspace members (if Cargo workspace)
    pub fn workspace_members(&self) -> Result<Vec<PathBuf>>;
}

#[derive(Debug, Clone)]
pub struct CargoMetadata {
    pub workspace_root: PathBuf,
    pub members: Vec<CargoPackage>,
    pub workspace_dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone)]  
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    pub package_root: PathBuf,
}
```

### **Implementation Steps**

#### **Step 1: Cargo.toml Parsing** (Day 1)
```rust
// Add to Cargo.toml dependencies
[dependencies]
cargo_metadata = "0.18"
toml = "0.8"

// Implementation in src/lib.rs
fn find_cargo_workspace() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    
    loop {
        let manifest = current.join("Cargo.toml");
        if manifest.exists() {
            let content = std::fs::read_to_string(&manifest)?;
            let parsed: toml::Value = toml::from_str(&content)?;
            
            if parsed.get("workspace").is_some() {
                return Ok(current);
            }
            
            // Check if this is a workspace member
            if let Some(package) = parsed.get("package") {
                if let Some(workspace_deps) = package.get("workspace") {
                    // Continue searching upward
                }
            }
        }
        
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => return Err(WorkspaceError::PathNotFound(current)),
        }
    }
}
```

#### **Step 2: Metadata Integration** (Day 2)
```rust
impl Workspace {
    pub fn cargo_metadata(&self) -> Result<CargoMetadata> {
        let output = std::process::Command::new("cargo")
            .args(&["metadata", "--format-version", "1"])
            .current_dir(&self.root)
            .output()
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            
        if !output.status.success() {
            return Err(WorkspaceError::ConfigurationError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        let metadata: cargo_metadata::Metadata = serde_json::from_slice(&output.stdout)
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
            
        Ok(CargoMetadata {
            workspace_root: metadata.workspace_root.into_std_path_buf(),
            members: metadata.workspace_members.into_iter()
                .map(|id| CargoPackage {
                    name: id.name,
                    version: id.version.to_string(),
                    manifest_path: metadata.packages.iter()
                        .find(|p| p.id == id)
                        .map(|p| p.manifest_path.clone().into_std_path_buf())
                        .unwrap_or_default(),
                    package_root: metadata.packages.iter()
                        .find(|p| p.id == id)
                        .map(|p| p.manifest_path.parent().unwrap().into_std_path_buf())
                        .unwrap_or_default(),
                })
                .collect(),
            workspace_dependencies: HashMap::new(), // TODO: Extract from metadata
        })
    }
}
```

#### **Step 3: Updated Constructor Logic** (Day 3)
```rust
impl Workspace {
    pub fn from_cargo_workspace() -> Result<Self> {
        let workspace_root = find_cargo_workspace()?;
        Ok(Self { root: workspace_root })
    }
    
    // Update existing resolve() to try Cargo first
    pub fn resolve() -> Result<Self> {
        // Try Cargo workspace detection first
        if let Ok(ws) = Self::from_cargo_workspace() {
            return Ok(ws);
        }
        
        // Fall back to environment variable
        if let Ok(root) = Self::get_env_path("WORKSPACE_PATH") {
            if root.exists() {
                return Ok(Self { root });
            }
        }
        
        // Other fallback strategies...
        Self::from_current_dir()
    }
}

// Update convenience function
pub fn workspace() -> Result<Workspace> {
    Workspace::resolve()
}
```

#### **Step 4: Testing & Documentation** (Day 4)
```rust
#[cfg(test)]
mod cargo_integration_tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_cargo_workspace_detection() {
        let (_temp_dir, test_ws) = create_test_workspace_with_structure();
        
        // Create fake Cargo.toml with workspace
        let cargo_toml = r#"[workspace]
members = ["member1", "member2"]

[workspace.dependencies]
serde = "1.0"
"#;
        fs::write(test_ws.join("Cargo.toml"), cargo_toml).unwrap();
        
        let ws = Workspace::from_cargo_workspace().unwrap();
        assert_eq!(ws.root(), test_ws.root());
        assert!(ws.is_cargo_workspace());
    }
    
    #[test]
    fn test_cargo_metadata_parsing() {
        // Test cargo metadata integration
        // Requires actual cargo workspace for testing
    }
    
    #[test] 
    fn test_workspace_member_detection() {
        // Test detection from within workspace member directory
    }
}
```

### **Documentation Updates**

#### **README.md Changes**
```markdown
## ⚡ quick start

### 1. add dependency
```toml
[dependencies]
workspace_tools = "0.2"  # No configuration needed!
```

### 2. use in your code  
```rust
use workspace_tools::workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Automatically detects Cargo workspace - no setup required!
    let ws = workspace()?;
    
    // Access workspace members
    for member in ws.workspace_members()? {
        println!("Member: {}", member.display());
    }
    
    Ok(())
}
```

**Note**: No `.cargo/config.toml` setup required when using Cargo workspaces!
```

#### **New Example: cargo_integration.rs**
```rust
//! Cargo workspace integration example
use workspace_tools::{workspace, Workspace};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Automatic detection - no configuration needed
    let ws = workspace()?;
    
    println!("🦀 Cargo Workspace Integration");
    println!("Workspace root: {}", ws.root().display());
    
    // Check if this is a Cargo workspace
    if ws.is_cargo_workspace() {
        println!("✅ Detected Cargo workspace");
        
        // Get metadata
        let metadata = ws.cargo_metadata()?;
        println!("📦 Workspace members:");
        
        for member in metadata.members {
            println!("  {} v{} at {}", 
                member.name, 
                member.version, 
                member.package_root.display()
            );
        }
    } else {
        println!("ℹ️  Standard workspace (non-Cargo)");
    }
    
    Ok(())
}
```

### **Breaking Changes & Migration**

**Breaking Changes**: None - this is purely additive functionality.

**Migration Path**: 
- Existing code continues to work unchanged
- New code can omit `.cargo/config.toml` setup
- Gradual migration to new constructor methods

### **Success Criteria**
- [ ] Auto-detects Cargo workspaces without configuration
- [ ] Provides access to workspace member information
- [ ] Maintains backward compatibility with existing API
- [ ] Comprehensive test coverage (>90%)
- [ ] Updated documentation and examples
- [ ] Performance: Detection completes in <10ms
- [ ] Works with both workspace roots and members

### **Future Enhancements**
- Integration with `cargo metadata` caching
- Support for multiple workspace formats (future Cargo features)
- Workspace dependency graph analysis
- Integration with cargo commands

### **Testing Strategy**
1. **Unit Tests**: Cargo.toml parsing, metadata extraction
2. **Integration Tests**: Real Cargo workspace detection
3. **Property Tests**: Various workspace configurations
4. **Performance Tests**: Detection speed benchmarks
5. **Compatibility Tests**: Different Cargo versions

This task transforms workspace_tools from requiring configuration to being zero-configuration for the majority of Rust projects using Cargo workspaces.