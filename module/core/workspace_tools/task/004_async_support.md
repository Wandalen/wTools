# Task 004: Async Support

**Priority**: ⚡ High Impact  
**Phase**: 2 (Ecosystem Integration)  
**Estimated Effort**: 4-5 days  
**Dependencies**: Task 001 (Cargo Integration) recommended  

## **Objective**
Add comprehensive async/await support for modern Rust web services and async applications, including async file operations, configuration loading, and change watching capabilities.

## **Technical Requirements**

### **Core Features**
1. **Async File Operations**
   - Non-blocking file reading and writing
   - Async directory traversal and creation
   - Concurrent resource discovery

2. **Async Configuration Loading**
   - Non-blocking config file parsing
   - Async validation and deserialization
   - Concurrent multi-config loading

3. **File System Watching**
   - Real-time file change notifications
   - Configuration hot-reloading
   - Workspace structure monitoring

### **New API Surface**
```rust
#[cfg(feature = "async")]
impl Workspace {
    /// Async version of find_resources with glob patterns
    pub async fn find_resources_async(&self, pattern: &str) -> Result<Vec<PathBuf>>;
    
    /// Load configuration asynchronously
    pub async fn load_config_async<T>(&self, name: &str) -> Result<T>
    where 
        T: serde::de::DeserializeOwned + Send;
    
    /// Load multiple configurations concurrently
    pub async fn load_configs_async<T>(&self, names: &[&str]) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Send;
    
    /// Watch for file system changes
    pub async fn watch_changes(&self) -> Result<ChangeStream>;
    
    /// Watch specific configuration file for changes
    pub async fn watch_config<T>(&self, name: &str) -> Result<ConfigWatcher<T>>
    where
        T: serde::de::DeserializeOwned + Send + 'static;
    
    /// Async directory creation
    pub async fn create_directories_async(&self, dirs: &[&str]) -> Result<()>;
    
    /// Async file writing with atomic operations
    pub async fn write_file_async<P, C>(&self, path: P, contents: C) -> Result<()>
    where
        P: AsRef<Path> + Send,
        C: AsRef<[u8]> + Send;
}

/// Stream of file system changes
#[cfg(feature = "async")]
pub struct ChangeStream {
    receiver: tokio::sync::mpsc::UnboundedReceiver<WorkspaceChange>,
    _watcher: notify::RecommendedWatcher,
}

/// Configuration watcher for hot-reloading
#[cfg(feature = "async")]
pub struct ConfigWatcher<T> {
    current: T,
    receiver: tokio::sync::watch::Receiver<T>,
}

#[derive(Debug, Clone)]
pub enum WorkspaceChange {
    FileCreated(PathBuf),
    FileModified(PathBuf),
    FileDeleted(PathBuf),
    DirectoryCreated(PathBuf),
    DirectoryDeleted(PathBuf),
}
```

### **Implementation Steps**

#### **Step 1: Async Dependencies and Foundation** (Day 1)
```rust
// Add to Cargo.toml
[features]
default = ["enabled"]
async = [
    "dep:tokio",
    "dep:notify",
    "dep:futures-util",
    "dep:async-trait"
]

[dependencies]
tokio = { version = "1.0", features = ["fs", "sync", "time"], optional = true }
notify = { version = "6.0", optional = true }
futures-util = { version = "0.3", optional = true }
async-trait = { version = "0.1", optional = true }

// Async module foundation
#[cfg(feature = "async")]
pub mod async_ops {
    use tokio::fs;
    use futures_util::stream::{Stream, StreamExt};
    use std::path::{Path, PathBuf};
    use crate::{Workspace, WorkspaceError, Result};
    
    impl Workspace {
        /// Async file reading
        pub async fn read_file_async<P: AsRef<Path>>(&self, path: P) -> Result<String> {
            let full_path = self.join(path);
            fs::read_to_string(full_path).await
                .map_err(|e| WorkspaceError::IoError(e.to_string()))
        }
        
        /// Async file writing
        pub async fn write_file_async<P, C>(&self, path: P, contents: C) -> Result<()>
        where
            P: AsRef<Path> + Send,
            C: AsRef<[u8]> + Send,
        {
            let full_path = self.join(path);
            
            // Ensure parent directory exists
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).await
                    .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            }
            
            // Atomic write: write to temp file, then rename
            let temp_path = full_path.with_extension("tmp");
            fs::write(&temp_path, contents).await
                .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
                
            fs::rename(temp_path, full_path).await
                .map_err(|e| WorkspaceError::IoError(e.to_string()))
        }
        
        /// Async directory creation
        pub async fn create_directories_async(&self, dirs: &[&str]) -> Result<()> {
            let futures: Vec<_> = dirs.iter()
                .map(|dir| {
                    let dir_path = self.join(dir);
                    async move {
                        fs::create_dir_all(dir_path).await
                            .map_err(|e| WorkspaceError::IoError(e.to_string()))
                    }
                })
                .collect();
                
            futures_util::future::try_join_all(futures).await?;
            Ok(())
        }
    }
}
```

#### **Step 2: Async Resource Discovery** (Day 2)
```rust
#[cfg(all(feature = "async", feature = "glob"))]
impl Workspace {
    pub async fn find_resources_async(&self, pattern: &str) -> Result<Vec<PathBuf>> {
        let full_pattern = self.join(pattern);
        let pattern_str = full_pattern.to_string_lossy().to_string();
        
        // Use blocking glob in async task to avoid blocking the runtime
        let result = tokio::task::spawn_blocking(move || -> Result<Vec<PathBuf>> {
            use glob::glob;
            
            let mut results = Vec::new();
            for entry in glob(&pattern_str)
                .map_err(|e| WorkspaceError::GlobError(e.to_string()))? 
            {
                match entry {
                    Ok(path) => results.push(path),
                    Err(e) => return Err(WorkspaceError::GlobError(e.to_string())),
                }
            }
            Ok(results)
        }).await
        .map_err(|e| WorkspaceError::IoError(format!("Task join error: {}", e)))?;
        
        result
    }
    
    /// Concurrent resource discovery with multiple patterns
    pub async fn find_resources_concurrent(&self, patterns: &[&str]) -> Result<Vec<Vec<PathBuf>>> {
        let futures: Vec<_> = patterns.iter()
            .map(|pattern| self.find_resources_async(pattern))
            .collect();
            
        futures_util::future::try_join_all(futures).await
    }
    
    /// Stream-based resource discovery for large workspaces
    pub async fn find_resources_stream(
        &self, 
        pattern: &str
    ) -> Result<impl Stream<Item = Result<PathBuf>>> {
        let full_pattern = self.join(pattern);
        let pattern_str = full_pattern.to_string_lossy().to_string();
        
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        
        tokio::task::spawn_blocking(move || {
            use glob::glob;
            
            if let Ok(entries) = glob(&pattern_str) {
                for entry in entries {
                    match entry {
                        Ok(path) => {
                            if sender.send(Ok(path)).is_err() {
                                break; // Receiver dropped
                            }
                        }
                        Err(e) => {
                            let _ = sender.send(Err(WorkspaceError::GlobError(e.to_string())));
                            break;
                        }
                    }
                }
            }
        });
        
        Ok(tokio_stream::wrappers::UnboundedReceiverStream::new(receiver))
    }
}
```

#### **Step 3: Async Configuration Loading** (Day 2-3)
```rust
#[cfg(all(feature = "async", feature = "config_validation"))]
impl Workspace {
    pub async fn load_config_async<T>(&self, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Send,
    {
        // Find config file
        let config_path = self.find_config(name)?;
        
        // Read file asynchronously
        let content = self.read_file_async(&config_path).await?;
        
        // Parse in blocking task (CPU-intensive)
        let result = tokio::task::spawn_blocking(move || -> Result<T> {
            // Determine format and parse
            Self::parse_config_content(&content, &config_path)
        }).await
        .map_err(|e| WorkspaceError::IoError(format!("Task join error: {}", e)))?;
        
        result
    }
    
    pub async fn load_configs_async<T>(&self, names: &[&str]) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Send,
    {
        let futures: Vec<_> = names.iter()
            .map(|name| self.load_config_async::<T>(name))
            .collect();
            
        futures_util::future::try_join_all(futures).await
    }
    
    fn parse_config_content<T>(content: &str, path: &Path) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str(content)
                .map_err(|e| WorkspaceError::ConfigurationError(e.to_string())),
            Some("toml") => toml::from_str(content)
                .map_err(|e| WorkspaceError::ConfigurationError(e.to_string())),
            Some("yaml") | Some("yml") => serde_yaml::from_str(content)
                .map_err(|e| WorkspaceError::ConfigurationError(e.to_string())),
            _ => Err(WorkspaceError::ConfigurationError(
                format!("Unsupported config format: {}", path.display())
            )),
        }
    }
}
```

#### **Step 4: File System Watching** (Day 3-4)
```rust
#[cfg(feature = "async")]
impl Workspace {
    pub async fn watch_changes(&self) -> Result<ChangeStream> {
        use notify::{Watcher, RecursiveMode, Event, EventKind};
        
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let workspace_root = self.root().to_path_buf();
        
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            match res {
                Ok(event) => {
                    let changes = event_to_workspace_changes(event, &workspace_root);
                    for change in changes {
                        if tx.send(change).is_err() {
                            break; // Receiver dropped
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Watch error: {:?}", e);
                }
            }
        }).map_err(|e| WorkspaceError::IoError(e.to_string()))?;
        
        watcher.watch(self.root(), RecursiveMode::Recursive)
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            
        Ok(ChangeStream {
            receiver: rx,
            _watcher: watcher,
        })
    }
    
    pub async fn watch_config<T>(&self, name: &str) -> Result<ConfigWatcher<T>>
    where
        T: serde::de::DeserializeOwned + Send + Clone + 'static,
    {
        // Load initial config
        let initial_config = self.load_config_async::<T>(name).await?;
        let config_path = self.find_config(name)?;
        
        let (tx, rx) = tokio::sync::watch::channel(initial_config.clone());
        
        // Start watching the specific config file
        let workspace_root = self.root().to_path_buf();
        let config_file = config_path.clone();
        
        tokio::spawn(async move {
            let mut change_stream = match Self::watch_changes_internal(&workspace_root).await {
                Ok(stream) => stream,
                Err(_) => return,
            };
            
            while let Some(change) = change_stream.receiver.recv().await {
                match change {
                    WorkspaceChange::FileModified(path) if path == config_file => {
                        // Reload configuration
                        let workspace = Workspace { root: workspace_root.clone() };
                        if let Ok(new_config) = workspace.load_config_async::<T>(name).await {
                            let _ = tx.send(new_config);
                        }
                    }
                    _ => {} // Ignore other changes
                }
            }
        });
        
        Ok(ConfigWatcher {
            current: initial_config,
            receiver: rx,
        })
    }
    
    async fn watch_changes_internal(root: &Path) -> Result<ChangeStream> {
        // Internal helper to avoid self reference issues
        let ws = Workspace { root: root.to_path_buf() };
        ws.watch_changes().await
    }
}

fn event_to_workspace_changes(event: notify::Event, workspace_root: &Path) -> Vec<WorkspaceChange> {
    use notify::EventKind;
    
    let mut changes = Vec::new();
    
    for path in event.paths {
        // Only report changes within workspace
        if !path.starts_with(workspace_root) {
            continue;
        }
        
        let change = match event.kind {
            EventKind::Create(notify::CreateKind::File) => 
                WorkspaceChange::FileCreated(path),
            EventKind::Create(notify::CreateKind::Folder) => 
                WorkspaceChange::DirectoryCreated(path),
            EventKind::Modify(_) => 
                WorkspaceChange::FileModified(path),
            EventKind::Remove(notify::RemoveKind::File) => 
                WorkspaceChange::FileDeleted(path),
            EventKind::Remove(notify::RemoveKind::Folder) => 
                WorkspaceChange::DirectoryDeleted(path),
            _ => continue,
        };
        
        changes.push(change);
    }
    
    changes
}

#[cfg(feature = "async")]
impl ChangeStream {
    pub async fn next(&mut self) -> Option<WorkspaceChange> {
        self.receiver.recv().await
    }
    
    /// Convert to a futures Stream
    pub fn into_stream(self) -> impl Stream<Item = WorkspaceChange> {
        tokio_stream::wrappers::UnboundedReceiverStream::new(self.receiver)
    }
}

#[cfg(feature = "async")]
impl<T> ConfigWatcher<T> 
where
    T: Clone
{
    pub fn current(&self) -> &T {
        &self.current
    }
    
    pub async fn wait_for_change(&mut self) -> Result<T> {
        self.receiver.changed().await
            .map_err(|_| WorkspaceError::ConfigurationError("Config watcher closed".to_string()))?;
        
        let new_config = self.receiver.borrow().clone();
        self.current = new_config.clone();
        Ok(new_config)
    }
    
    /// Get a receiver for reactive updates
    pub fn subscribe(&self) -> tokio::sync::watch::Receiver<T> {
        self.receiver.clone()
    }
}
```

#### **Step 5: Testing and Integration** (Day 5)
```rust
#[cfg(test)]
#[cfg(feature = "async")]
mod async_tests {
    use super::*;
    use crate::testing::create_test_workspace_with_structure;
    use tokio::time::{timeout, Duration};
    
    #[tokio::test]
    async fn test_async_file_operations() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        // Test async file writing
        let content = "async test content";
        ws.write_file_async("data/async_test.txt", content).await.unwrap();
        
        // Test async file reading
        let read_content = ws.read_file_async("data/async_test.txt").await.unwrap();
        assert_eq!(read_content, content);
    }
    
    #[tokio::test]
    #[cfg(feature = "glob")]
    async fn test_async_resource_discovery() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        // Create test files
        ws.write_file_async("src/main.rs", "fn main() {}").await.unwrap();
        ws.write_file_async("src/lib.rs", "// lib").await.unwrap();
        ws.write_file_async("tests/test1.rs", "// test").await.unwrap();
        
        // Test async resource discovery
        let rust_files = ws.find_resources_async("**/*.rs").await.unwrap();
        assert_eq!(rust_files.len(), 3);
    }
    
    #[tokio::test]
    #[cfg(feature = "config_validation")]
    async fn test_async_config_loading() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct TestConfig {
            name: String,
            port: u16,
        }
        
        let config_content = r#"
name = "async_test"
port = 8080
"#;
        
        ws.write_file_async("config/test.toml", config_content).await.unwrap();
        
        let config: TestConfig = ws.load_config_async("test").await.unwrap();
        assert_eq!(config.name, "async_test");
        assert_eq!(config.port, 8080);
    }
    
    #[tokio::test]
    async fn test_file_watching() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        let mut change_stream = ws.watch_changes().await.unwrap();
        
        // Create a file in another task
        let ws_clone = ws.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            ws_clone.write_file_async("data/watched_file.txt", "content").await.unwrap();
        });
        
        // Wait for change notification
        let change = timeout(Duration::from_secs(5), change_stream.next())
            .await
            .expect("Timeout waiting for file change")
            .expect("Stream closed unexpectedly");
            
        match change {
            WorkspaceChange::FileCreated(path) => {
                assert!(path.to_string_lossy().contains("watched_file.txt"));
            }
            _ => panic!("Expected FileCreated event, got {:?}", change),
        }
    }
    
    #[tokio::test]
    #[cfg(feature = "config_validation")]
    async fn test_config_watching() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        #[derive(serde::Deserialize, Debug, Clone, PartialEq)]
        struct WatchConfig {
            value: String,
        }
        
        // Write initial config
        let initial_content = r#"value = "initial""#;
        ws.write_file_async("config/watch_test.toml", initial_content).await.unwrap();
        
        let mut config_watcher = ws.watch_config::<WatchConfig>("watch_test").await.unwrap();
        assert_eq!(config_watcher.current().value, "initial");
        
        // Modify config file
        tokio::spawn({
            let ws = ws.clone();
            async move {
                tokio::time::sleep(Duration::from_millis(100)).await;
                let new_content = r#"value = "updated""#;
                ws.write_file_async("config/watch_test.toml", new_content).await.unwrap();
            }
        });
        
        // Wait for config reload
        let updated_config = timeout(
            Duration::from_secs(5),
            config_watcher.wait_for_change()
        ).await
        .expect("Timeout waiting for config change")
        .expect("Config watcher error");
        
        assert_eq!(updated_config.value, "updated");
    }
}
```

### **Documentation Updates**

#### **README.md Addition**
```markdown
## ⚡ async support

workspace_tools provides full async/await support for modern applications:

```rust
use workspace_tools::workspace;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    // Async resource discovery
    let rust_files = ws.find_resources_async("src/**/*.rs").await?;
    
    // Async configuration loading
    let config: AppConfig = ws.load_config_async("app").await?;
    
    // Watch for changes
    let mut changes = ws.watch_changes().await?;
    while let Some(change) = changes.next().await {
        println!("Change detected: {:?}", change);
    }
    
    Ok(())
}
```

**Async Features:**
- Non-blocking file operations  
- Concurrent resource discovery
- Configuration hot-reloading
- Real-time file system watching
```

#### **New Example: async_web_service.rs**
```rust
//! Async web service example with hot-reloading

use workspace_tools::workspace;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    println!("🚀 Async Web Service Example");
    
    // Load initial configuration
    let mut config_watcher = ws.watch_config::<ServerConfig>("server").await?;
    println!("Initial config: {:?}", config_watcher.current());
    
    // Start background task to watch for config changes
    let mut config_rx = config_watcher.subscribe();
    tokio::spawn(async move {
        while config_rx.changed().await.is_ok() {
            let new_config = config_rx.borrow();
            println!("🔄 Configuration reloaded: {:?}", *new_config);
        }
    });
    
    // Watch for general file changes
    let mut change_stream = ws.watch_changes().await?;
    tokio::spawn(async move {
        while let Some(change) = change_stream.next().await {
            println!("📁 File system change: {:?}", change);
        }
    });
    
    // Simulate server running
    println!("✅ Server started, watching for changes...");
    println!("   Try modifying config/server.toml to see hot-reloading");
    
    // Run for demo purposes
    for i in 0..30 {
        sleep(Duration::from_secs(1)).await;
        
        // Demonstrate async file operations
        if i % 10 == 0 {
            let log_content = format!("Server running for {} seconds\n", i);
            ws.write_file_async("logs/server.log", log_content).await?;
        }
    }
    
    Ok(())
}
```

### **Success Criteria**
- [ ] Complete async/await API coverage
- [ ] Non-blocking file operations with tokio::fs
- [ ] Real-time file system watching with notify
- [ ] Configuration hot-reloading capabilities
- [ ] Concurrent resource discovery
- [ ] Stream-based APIs for large workspaces
- [ ] Comprehensive async test suite
- [ ] Performance: Async operations don't block runtime

### **Future Enhancements**
- WebSocket integration for real-time workspace updates
- Database connection pooling with async workspace configs
- Integration with async HTTP clients for remote configs
- Distributed workspace synchronization
- Advanced change filtering and debouncing

### **Breaking Changes**
None - async support is purely additive with feature flag.

This task positions workspace_tools as the go-to solution for modern async Rust applications, particularly web services that need configuration hot-reloading and real-time file monitoring.