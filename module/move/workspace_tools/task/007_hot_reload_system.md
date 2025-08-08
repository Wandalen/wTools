# Task 007: Hot Reload System

**Priority**: 🔥 Medium Impact  
**Phase**: 3 (Advanced Features)  
**Estimated Effort**: 4-5 days  
**Dependencies**: Task 004 (Async Support), Task 005 (Serde Integration), Task 006 (Environment Management) recommended  

## **Objective**
Implement a comprehensive hot reload system that automatically detects and applies configuration, template, and resource changes without requiring application restarts, enhancing developer experience and reducing deployment friction.

## **Technical Requirements**

### **Core Features**
1. **Configuration Hot Reload**
   - Automatic configuration file monitoring
   - Live configuration updates without restart
   - Validation before applying changes
   - Rollback on invalid configurations

2. **Resource Monitoring**
   - Template file watching and recompilation
   - Static asset change detection
   - Plugin system for custom reload handlers
   - Selective reload based on change types

3. **Change Propagation**
   - Event-driven notification system
   - Graceful service reconfiguration
   - State preservation during reloads
   - Multi-instance coordination

### **New API Surface**
```rust
impl Workspace {
    /// Start hot reload system for configurations
    pub async fn start_hot_reload(&self) -> Result<HotReloadManager>;
    
    /// Start hot reload with custom configuration
    pub async fn start_hot_reload_with_config(
        &self, 
        config: HotReloadConfig
    ) -> Result<HotReloadManager>;
    
    /// Register a configuration for hot reloading
    pub async fn watch_config_changes<T>(&self, config_name: &str) -> Result<ConfigStream<T>>
    where
        T: serde::de::DeserializeOwned + Send + Clone + 'static;
    
    /// Register custom reload handler
    pub fn register_reload_handler<F>(&self, pattern: &str, handler: F) -> Result<()>
    where
        F: Fn(ChangeEvent) -> Result<()> + Send + Sync + 'static;
}

#[derive(Debug, Clone)]
pub struct HotReloadConfig {
    pub watch_patterns: Vec<String>,
    pub debounce_ms: u64,
    pub validate_before_reload: bool,
    pub backup_on_change: bool,
    pub exclude_patterns: Vec<String>,
}

pub struct HotReloadManager {
    config_watchers: HashMap<String, ConfigWatcher<serde_json::Value>>,
    file_watchers: HashMap<String, FileWatcher>,
    event_bus: EventBus,
    _background_tasks: Vec<tokio::task::JoinHandle<()>>,
}

pub struct ConfigStream<T> {
    receiver: tokio::sync::broadcast::Receiver<T>,
    current: T,
}

#[derive(Debug, Clone)]
pub enum ChangeEvent {
    ConfigChanged {
        config_name: String,
        old_value: serde_json::Value,
        new_value: serde_json::Value,
    },
    FileChanged {
        path: PathBuf,
        change_type: ChangeType,
    },
    ValidationFailed {
        config_name: String,
        error: String,
    },
    ReloadCompleted {
        config_name: String,
        duration: std::time::Duration,
    },
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Modified,
    Created,
    Deleted,
    Renamed { from: PathBuf },
}

pub trait ReloadHandler: Send + Sync {
    async fn handle_change(&self, event: ChangeEvent) -> Result<()>;
    fn can_handle(&self, event: &ChangeEvent) -> bool;
}
```

### **Implementation Steps**

#### **Step 1: File Watching Foundation** (Day 1)
```rust
// Add to Cargo.toml
[features]
default = ["enabled", "hot_reload"]
hot_reload = [
    "async",
    "dep:notify",
    "dep:tokio",
    "dep:futures-util",
    "dep:debounce",
    "dep:serde_json",
]

[dependencies]
notify = { version = "6.0", optional = true }
tokio = { version = "1.0", features = ["full"], optional = true }
futures-util = { version = "0.3", optional = true }
debounce = { version = "0.2", optional = true }

#[cfg(feature = "hot_reload")]
mod hot_reload {
    use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
    use tokio::sync::{broadcast, mpsc};
    use std::collections::HashMap;
    use std::time::{Duration, Instant};
    use debounce::EventDebouncer;
    
    pub struct FileWatcher {
        _watcher: RecommendedWatcher,
        event_sender: broadcast::Sender<ChangeEvent>,
        debouncer: EventDebouncer<PathBuf>,
    }
    
    impl FileWatcher {
        pub async fn new(
            watch_paths: Vec<PathBuf>,
            debounce_duration: Duration,
        ) -> Result<Self> {
            let (event_sender, _) = broadcast::channel(1024);
            let sender_clone = event_sender.clone();
            
            // Create debouncer for file events
            let mut debouncer = EventDebouncer::new(debounce_duration, move |paths: Vec<PathBuf>| {
                for path in paths {
                    let change_event = ChangeEvent::FileChanged {
                        path: path.clone(),
                        change_type: ChangeType::Modified, // Simplified for now
                    };
                    let _ = sender_clone.send(change_event);
                }
            });
            
            let mut watcher = notify::recommended_watcher({
                let mut debouncer_clone = debouncer.clone();
                move |result: notify::Result<Event>| {
                    if let Ok(event) = result {
                        for path in event.paths {
                            debouncer_clone.put(path);
                        }
                    }
                }
            })?;
            
            // Start watching all specified paths
            for path in watch_paths {
                watcher.watch(&path, RecursiveMode::Recursive)?;
            }
            
            Ok(Self {
                _watcher: watcher,
                event_sender,
                debouncer,
            })
        }
        
        pub fn subscribe(&self) -> broadcast::Receiver<ChangeEvent> {
            self.event_sender.subscribe()
        }
    }
    
    impl Default for HotReloadConfig {
        fn default() -> Self {
            Self {
                watch_patterns: vec![
                    "config/**/*.toml".to_string(),
                    "config/**/*.yaml".to_string(),
                    "config/**/*.json".to_string(),
                    "templates/**/*".to_string(),
                    "static/**/*".to_string(),
                ],
                debounce_ms: 500,
                validate_before_reload: true,
                backup_on_change: false,
                exclude_patterns: vec![
                    "**/*.tmp".to_string(),
                    "**/*.swp".to_string(),
                    "**/.*".to_string(),
                ],
            }
        }
    }
}
```

#### **Step 2: Configuration Hot Reload** (Day 2)
```rust
#[cfg(feature = "hot_reload")]
impl Workspace {
    pub async fn start_hot_reload(&self) -> Result<HotReloadManager> {
        self.start_hot_reload_with_config(HotReloadConfig::default()).await
    }
    
    pub async fn start_hot_reload_with_config(
        &self,
        config: HotReloadConfig
    ) -> Result<HotReloadManager> {
        let mut manager = HotReloadManager::new();
        
        // Collect all paths to watch
        let mut watch_paths = Vec::new();
        for pattern in &config.watch_patterns {
            let full_pattern = self.join(pattern);
            let matching_paths = glob::glob(&full_pattern.to_string_lossy())?;
            
            for path in matching_paths {
                match path {
                    Ok(p) if p.exists() => {
                        if p.is_dir() {
                            watch_paths.push(p);
                        } else if let Some(parent) = p.parent() {
                            if !watch_paths.contains(&parent.to_path_buf()) {
                                watch_paths.push(parent.to_path_buf());
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
        
        // Add workspace root directories
        watch_paths.extend(vec![
            self.config_dir(),
            self.data_dir(),
        ]);
        
        // Create file watcher
        let file_watcher = FileWatcher::new(
            watch_paths,
            Duration::from_millis(config.debounce_ms)
        ).await?;
        
        let mut change_receiver = file_watcher.subscribe();
        
        // Start background task for handling changes
        let workspace_root = self.root().to_path_buf();
        let validate_before_reload = config.validate_before_reload;
        let backup_on_change = config.backup_on_change;
        let exclude_patterns = config.exclude_patterns.clone();
        
        let background_task = tokio::spawn(async move {
            while let Ok(change_event) = change_receiver.recv().await {
                if let Err(e) = Self::handle_file_change(
                    &workspace_root,
                    change_event,
                    validate_before_reload,
                    backup_on_change,
                    &exclude_patterns,
                ).await {
                    eprintln!("Hot reload error: {}", e);
                }
            }
        });
        
        manager._background_tasks.push(background_task);
        Ok(manager)
    }
    
    async fn handle_file_change(
        workspace_root: &Path,
        event: ChangeEvent,
        validate_before_reload: bool,
        backup_on_change: bool,
        exclude_patterns: &[String],
    ) -> Result<()> {
        match event {
            ChangeEvent::FileChanged { path, change_type } => {
                // Check if file should be excluded
                for pattern in exclude_patterns {
                    if glob::Pattern::new(pattern)?.matches_path(&path) {
                        return Ok(());
                    }
                }
                
                let workspace = Workspace { root: workspace_root.to_path_buf() };
                
                // Handle configuration files
                if Self::is_config_file(&path) {
                    workspace.handle_config_change(&path, validate_before_reload, backup_on_change).await?;
                }
                
                // Handle template files
                else if Self::is_template_file(&path) {
                    workspace.handle_template_change(&path).await?;
                }
                
                // Handle static assets
                else if Self::is_static_asset(&path) {
                    workspace.handle_asset_change(&path).await?;
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn is_config_file(path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            matches!(ext, "toml" | "yaml" | "yml" | "json")
        } else {
            false
        }
    }
    
    fn is_template_file(path: &Path) -> bool {
        path.to_string_lossy().contains("/templates/") ||
        path.extension().and_then(|e| e.to_str()) == Some("hbs")
    }
    
    fn is_static_asset(path: &Path) -> bool {
        path.to_string_lossy().contains("/static/") ||
        path.to_string_lossy().contains("/assets/")
    }
}
```

#### **Step 3: Configuration Change Handling** (Day 2-3)
```rust
#[cfg(feature = "hot_reload")]
impl Workspace {
    async fn handle_config_change(
        &self,
        path: &Path,
        validate_before_reload: bool,
        backup_on_change: bool,
    ) -> Result<()> {
        println!("🔄 Configuration change detected: {}", path.display());
        
        // Create backup if requested
        if backup_on_change {
            self.create_config_backup(path).await?;
        }
        
        // Determine config name from path
        let config_name = self.extract_config_name(path)?;
        
        // Validate new configuration if requested
        if validate_before_reload {
            if let Err(e) = self.validate_config_file(path) {
                println!("❌ Configuration validation failed: {}", e);
                return Ok(()); // Don't reload invalid config
            }
        }
        
        // Read new configuration
        let new_config_value: serde_json::Value = self.load_config_as_json(path).await?;
        
        // Notify all listeners
        self.notify_config_change(&config_name, new_config_value).await?;
        
        println!("✅ Configuration reloaded: {}", config_name);
        Ok(())
    }
    
    async fn create_config_backup(&self, path: &Path) -> Result<()> {
        let backup_dir = self.data_dir().join("backups").join("configs");
        std::fs::create_dir_all(&backup_dir)?;
        
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("{}_{}",
            timestamp,
            path.file_name().unwrap().to_string_lossy()
        );
        let backup_path = backup_dir.join(backup_name);
        
        tokio::fs::copy(path, backup_path).await?;
        Ok(())
    }
    
    fn extract_config_name(&self, path: &Path) -> Result<String> {
        // Extract config name from file path
        // Example: config/app.toml -> "app"
        // Example: config/database.production.yaml -> "database"
        
        if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
            // Remove environment suffix if present
            let config_name = file_name.split('.').next().unwrap_or(file_name);
            Ok(config_name.to_string())
        } else {
            Err(WorkspaceError::ConfigurationError(
                format!("Unable to extract config name from path: {}", path.display())
            ))
        }
    }
    
    async fn load_config_as_json(&self, path: &Path) -> Result<serde_json::Value> {
        let content = tokio::fs::read_to_string(path).await?;
        
        match path.extension().and_then(|e| e.to_str()) {
            Some("json") => {
                serde_json::from_str(&content)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
            }
            Some("toml") => {
                let toml_value: toml::Value = toml::from_str(&content)?;
                serde_json::to_value(toml_value)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
            }
            Some("yaml") | Some("yml") => {
                let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;
                serde_json::to_value(yaml_value)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
            }
            _ => Err(WorkspaceError::ConfigurationError(
                format!("Unsupported config format: {}", path.display())
            ))
        }
    }
    
    async fn notify_config_change(
        &self,
        config_name: &str,
        new_value: serde_json::Value,
    ) -> Result<()> {
        // In a real implementation, this would notify all registered listeners
        // For now, we'll just log the change
        println!("📢 Notifying config change for '{}': {:?}", config_name, new_value);
        Ok(())
    }
}
```

#### **Step 4: Configuration Streams and Reactive Updates** (Day 3-4)
```rust
#[cfg(feature = "hot_reload")]
impl Workspace {
    pub async fn watch_config_changes<T>(&self, config_name: &str) -> Result<ConfigStream<T>>
    where
        T: serde::de::DeserializeOwned + Send + Clone + 'static,
    {
        // Load initial configuration
        let initial_config: T = self.load_config(config_name)?;
        
        // Create broadcast channel for updates
        let (sender, receiver) = tokio::sync::broadcast::channel(16);
        
        // Start monitoring the configuration file
        let config_path = self.find_config(config_name)?;
        let watch_paths = vec![
            config_path.parent().unwrap_or_else(|| self.config_dir()).to_path_buf()
        ];
        
        let file_watcher = FileWatcher::new(watch_paths, Duration::from_millis(500)).await?;
        let mut change_receiver = file_watcher.subscribe();
        
        // Start background task to monitor changes
        let workspace_clone = self.clone();
        let config_name_clone = config_name.to_string();
        let sender_clone = sender.clone();
        
        tokio::spawn(async move {
            while let Ok(change_event) = change_receiver.recv().await {
                if let ChangeEvent::FileChanged { path, .. } = change_event {
                    // Check if this change affects our config
                    if workspace_clone.extract_config_name(&path)
                        .map(|name| name == config_name_clone)
                        .unwrap_or(false)
                    {
                        // Reload configuration
                        match workspace_clone.load_config::<T>(&config_name_clone) {
                            Ok(new_config) => {
                                let _ = sender_clone.send(new_config);
                            }
                            Err(e) => {
                                eprintln!("Failed to reload config '{}': {}", config_name_clone, e);
                            }
                        }
                    }
                }
            }
        });
        
        Ok(ConfigStream {
            receiver,
            current: initial_config,
        })
    }
}

#[cfg(feature = "hot_reload")]
impl<T> ConfigStream<T>
where
    T: Clone,
{
    pub fn current(&self) -> &T {
        &self.current
    }
    
    pub async fn next(&mut self) -> Option<T> {
        match self.receiver.recv().await {
            Ok(new_config) => {
                self.current = new_config.clone();
                Some(new_config)
            }
            Err(_) => None, // Channel closed
        }
    }
    
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<T> {
        self.receiver.resubscribe()
    }
}

#[cfg(feature = "hot_reload")]
impl HotReloadManager {
    pub fn new() -> Self {
        Self {
            config_watchers: HashMap::new(),
            file_watchers: HashMap::new(),
            event_bus: EventBus::new(),
            _background_tasks: Vec::new(),
        }
    }
    
    pub async fn shutdown(self) -> Result<()> {
        // Wait for all background tasks to complete
        for task in self._background_tasks {
            let _ = task.await;
        }
        Ok(())
    }
    
    pub fn register_handler<H>(&mut self, handler: H)
    where
        H: ReloadHandler + 'static,
    {
        self.event_bus.register(Box::new(handler));
    }
}

struct EventBus {
    handlers: Vec<Box<dyn ReloadHandler>>,
}

impl EventBus {
    fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }
    
    fn register(&mut self, handler: Box<dyn ReloadHandler>) {
        self.handlers.push(handler);
    }
    
    async fn emit(&self, event: ChangeEvent) -> Result<()> {
        for handler in &self.handlers {
            if handler.can_handle(&event) {
                if let Err(e) = handler.handle_change(event.clone()).await {
                    eprintln!("Handler error: {}", e);
                }
            }
        }
        Ok(())
    }
}
```

#### **Step 5: Template and Asset Hot Reload** (Day 4-5)
```rust
#[cfg(feature = "hot_reload")]
impl Workspace {
    async fn handle_template_change(&self, path: &Path) -> Result<()> {
        println!("🎨 Template change detected: {}", path.display());
        
        // For template changes, we might want to:
        // 1. Recompile templates if using a template engine
        // 2. Clear template cache
        // 3. Notify web servers to reload templates
        
        let change_event = ChangeEvent::FileChanged {
            path: path.to_path_buf(),
            change_type: ChangeType::Modified,
        };
        
        // Emit event to registered handlers
        // In a real implementation, this would notify template engines
        println!("📢 Template change event emitted for: {}", path.display());
        
        Ok(())
    }
    
    async fn handle_asset_change(&self, path: &Path) -> Result<()> {
        println!("🖼️  Asset change detected: {}", path.display());
        
        // For asset changes, we might want to:
        // 1. Process assets (minification, compression)
        // 2. Update asset manifests
        // 3. Notify CDNs or reverse proxies
        // 4. Trigger browser cache invalidation
        
        let change_event = ChangeEvent::FileChanged {
            path: path.to_path_buf(),
            change_type: ChangeType::Modified,
        };
        
        println!("📢 Asset change event emitted for: {}", path.display());
        
        Ok(())
    }
    
    /// Register a custom reload handler for specific file patterns
    pub fn register_reload_handler<F>(&self, pattern: &str, handler: F) -> Result<()>
    where
        F: Fn(ChangeEvent) -> Result<()> + Send + Sync + 'static,
    {
        // Store the handler with its pattern
        // In a real implementation, this would be stored in the hot reload manager
        println!("Registered reload handler for pattern: {}", pattern);
        Ok(())
    }
}

// Example custom reload handler
struct WebServerReloadHandler {
    server_url: String,
}

#[cfg(feature = "hot_reload")]
#[async_trait::async_trait]
impl ReloadHandler for WebServerReloadHandler {
    async fn handle_change(&self, event: ChangeEvent) -> Result<()> {
        match event {
            ChangeEvent::ConfigChanged { config_name, .. } => {
                // Notify web server to reload configuration
                println!("🌐 Notifying web server to reload config: {}", config_name);
                // HTTP request to server reload endpoint
                // reqwest::get(&format!("{}/reload", self.server_url)).await?;
            }
            ChangeEvent::FileChanged { path, .. } if path.to_string_lossy().contains("static") => {
                // Notify web server about asset changes
                println!("🌐 Notifying web server about asset change: {}", path.display());
            }
            _ => {}
        }
        Ok(())
    }
    
    fn can_handle(&self, event: &ChangeEvent) -> bool {
        matches!(
            event,
            ChangeEvent::ConfigChanged { .. } | 
            ChangeEvent::FileChanged { .. }
        )
    }
}
```

#### **Step 6: Testing and Integration** (Day 5)
```rust
#[cfg(test)]
#[cfg(feature = "hot_reload")]
mod hot_reload_tests {
    use super::*;
    use crate::testing::create_test_workspace_with_structure;
    use tokio::time::{sleep, Duration};
    
    #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
    struct TestConfig {
        name: String,
        value: i32,
    }
    
    #[tokio::test]
    async fn test_config_hot_reload() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        // Create initial config
        let initial_config = TestConfig {
            name: "initial".to_string(),
            value: 42,
        };
        
        let config_path = ws.config_dir().join("test.json");
        let config_content = serde_json::to_string_pretty(&initial_config).unwrap();
        tokio::fs::write(&config_path, config_content).await.unwrap();
        
        // Start watching config changes
        let mut config_stream = ws.watch_config_changes::<TestConfig>("test").await.unwrap();
        assert_eq!(config_stream.current().name, "initial");
        assert_eq!(config_stream.current().value, 42);
        
        // Modify config file
        let updated_config = TestConfig {
            name: "updated".to_string(),
            value: 100,
        };
        
        tokio::spawn({
            let config_path = config_path.clone();
            async move {
                sleep(Duration::from_millis(100)).await;
                let updated_content = serde_json::to_string_pretty(&updated_config).unwrap();
                tokio::fs::write(&config_path, updated_content).await.unwrap();
            }
        });
        
        // Wait for configuration update
        let new_config = tokio::time::timeout(
            Duration::from_secs(5),
            config_stream.next()
        ).await
        .expect("Timeout waiting for config update")
        .expect("Config stream closed");
        
        assert_eq!(new_config.name, "updated");
        assert_eq!(new_config.value, 100);
    }
    
    #[tokio::test]
    async fn test_hot_reload_manager() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        let hot_reload_config = HotReloadConfig {
            watch_patterns: vec!["config/**/*.json".to_string()],
            debounce_ms: 100,
            validate_before_reload: false,
            backup_on_change: false,
            exclude_patterns: vec!["**/*.tmp".to_string()],
        };
        
        let _manager = ws.start_hot_reload_with_config(hot_reload_config).await.unwrap();
        
        // Create and modify a config file
        let config_path = ws.config_dir().join("app.json");
        let config_content = r#"{"name": "test_app", "version": "1.0.0"}"#;
        tokio::fs::write(&config_path, config_content).await.unwrap();
        
        // Give some time for the file watcher to detect the change
        sleep(Duration::from_millis(200)).await;
        
        // Modify the file
        let updated_content = r#"{"name": "test_app", "version": "2.0.0"}"#;
        tokio::fs::write(&config_path, updated_content).await.unwrap();
        
        // Give some time for the change to be processed
        sleep(Duration::from_millis(300)).await;
        
        // Test passed if no panics occurred
    }
    
    #[tokio::test]
    async fn test_config_backup() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        // Create initial config
        let config_path = ws.config_dir().join("backup_test.toml");
        let config_content = r#"name = "backup_test""#;
        tokio::fs::write(&config_path, config_content).await.unwrap();
        
        // Create backup
        ws.create_config_backup(&config_path).await.unwrap();
        
        // Check that backup was created
        let backup_dir = ws.data_dir().join("backups").join("configs");
        assert!(backup_dir.exists());
        
        let backup_files: Vec<_> = std::fs::read_dir(backup_dir).unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_name().to_string_lossy().contains("backup_test.toml")
            })
            .collect();
        
        assert!(!backup_files.is_empty(), "Backup file should have been created");
    }
}
```

### **Documentation Updates**

#### **README.md Addition**
```markdown
## 🔥 hot reload system

workspace_tools provides automatic hot reloading for configurations, templates, and assets:

```rust
use workspace_tools::workspace;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    // Start hot reload system
    let _manager = ws.start_hot_reload().await?;
    
    // Watch configuration changes
    let mut config_stream = ws.watch_config_changes::<AppConfig>("app").await?;
    
    while let Some(new_config) = config_stream.next().await {
        println!("Configuration updated: {:?}", new_config);
        // Apply new configuration to your application
    }
    
    Ok(())
}
```

**Features:**
- Automatic configuration file monitoring
- Live updates without application restart
- Template and asset change detection
- Validation before applying changes
- Configurable debouncing and filtering
```

#### **New Example: hot_reload_server.rs**
```rust
//! Hot reload web server example

use workspace_tools::workspace;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: usize,
    debug: bool,
}

impl workspace_tools::ConfigMerge for ServerConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            host: other.host,
            port: other.port,
            max_connections: other.max_connections,
            debug: other.debug,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    println!("🔥 Hot Reload Server Demo");
    
    // Start hot reload system
    let _manager = ws.start_hot_reload().await?;
    println!("✅ Hot reload system started");
    
    // Watch server configuration changes
    let mut config_stream = ws.watch_config_changes::<ServerConfig>("server").await?;
    println!("👀 Watching server configuration for changes...");
    println!("   Current config: {:?}", config_stream.current());
    
    // Simulate server running with config updates
    let mut server_task = None;
    
    loop {
        tokio::select! {
            // Check for configuration updates
            new_config = config_stream.next() => {
                if let Some(config) = new_config {
                    println!("🔄 Configuration updated: {:?}", config);
                    
                    // Gracefully restart server with new config
                    if let Some(handle) = server_task.take() {
                        handle.abort();
                        println!("   🛑 Stopped old server");
                    }
                    
                    server_task = Some(tokio::spawn(run_server(config)));
                    println!("   🚀 Started server with new configuration");
                }
            }
            
            // Simulate other work
            _ = sleep(Duration::from_secs(1)) => {
                if server_task.is_some() {
                    print!(".");
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();
                }
            }
        }
    }
}

async fn run_server(config: ServerConfig) {
    println!("   🌐 Server running on {}:{}", config.host, config.port);
    println!("   📊 Max connections: {}", config.max_connections);
    println!("   🐛 Debug mode: {}", config.debug);
    
    // Simulate server work
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
```

### **Success Criteria**
- [ ] Automatic configuration file monitoring with debouncing
- [ ] Live configuration updates without restart
- [ ] Template and asset change detection
- [ ] Validation before applying changes
- [ ] Configurable watch patterns and exclusions
- [ ] Graceful error handling for invalid configs
- [ ] Background task management
- [ ] Comprehensive test coverage

### **Future Enhancements**
- WebSocket notifications for browser hot-reloading
- Integration with popular web frameworks (Axum, Warp, Actix)
- Remote configuration synchronization
- A/B testing support with configuration switching
- Performance monitoring during reloads
- Distributed hot-reload coordination

### **Breaking Changes**
None - this is purely additive functionality with feature flag.

This task transforms workspace_tools into a comprehensive development experience enhancer, eliminating the friction of manual restarts during development and deployment.