# Task 008: Plugin Architecture

**Priority**: 🔌 Medium Impact  
**Phase**: 3 (Advanced Features)  
**Estimated Effort**: 5-6 days  
**Dependencies**: Task 004 (Async Support), Task 007 (Hot Reload System) recommended  

## **Objective**
Implement a comprehensive plugin architecture that allows workspace_tools to be extended with custom functionality, transforming it from a utility library into a platform for workspace management solutions.

## **Technical Requirements**

### **Core Features**
1. **Plugin Discovery and Loading**
   - Dynamic plugin loading from directories
   - Plugin metadata and version management
   - Dependency resolution between plugins
   - Safe plugin sandboxing

2. **Plugin API Framework**
   - Well-defined plugin traits and interfaces
   - Event system for plugin communication
   - Shared state management
   - Plugin lifecycle management

3. **Built-in Plugin Types**
   - File processors (linting, formatting, compilation)
   - Configuration validators
   - Custom command extensions
   - Workspace analyzers

### **New API Surface**
```rust
impl Workspace {
    /// Load and initialize all plugins from plugin directory
    pub fn load_plugins(&mut self) -> Result<PluginRegistry>;
    
    /// Load specific plugin by name or path
    pub fn load_plugin<P: AsRef<Path>>(&mut self, plugin_path: P) -> Result<PluginHandle>;
    
    /// Get loaded plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&PluginHandle>;
    
    /// Execute plugin command
    pub async fn execute_plugin_command(
        &self,
        plugin_name: &str,
        command: &str,
        args: &[String]
    ) -> Result<PluginResult>;
    
    /// Register plugin event listener
    pub fn register_event_listener<F>(&mut self, event_type: &str, listener: F)
    where
        F: Fn(&PluginEvent) -> Result<()> + Send + Sync + 'static;
}

/// Core plugin trait that all plugins must implement
pub trait WorkspacePlugin: Send + Sync {
    fn metadata(&self) -> &PluginMetadata;
    fn initialize(&mut self, context: &PluginContext) -> Result<()>;
    fn execute_command(&self, command: &str, args: &[String]) -> Result<PluginResult>;
    fn handle_event(&self, event: &PluginEvent) -> Result<()> { Ok(()) }
    fn shutdown(&mut self) -> Result<()> { Ok(()) }
}

#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub dependencies: Vec<PluginDependency>,
    pub commands: Vec<PluginCommand>,
    pub event_subscriptions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PluginDependency {
    pub name: String,
    pub version_requirement: String,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct PluginCommand {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub args: Vec<CommandArg>,
}

#[derive(Debug, Clone)]
pub struct CommandArg {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub arg_type: ArgType,
}

#[derive(Debug, Clone)]
pub enum ArgType {
    String,
    Integer,
    Boolean,
    Path,
    Choice(Vec<String>),
}

pub struct PluginRegistry {
    plugins: HashMap<String, PluginHandle>,
    event_bus: EventBus,
    dependency_graph: DependencyGraph,
}

pub struct PluginHandle {
    plugin: Box<dyn WorkspacePlugin>,
    metadata: PluginMetadata,
    state: PluginState,
}

#[derive(Debug, Clone)]
pub enum PluginState {
    Loaded,
    Initialized,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct PluginEvent {
    pub event_type: String,
    pub source: String,
    pub data: serde_json::Value,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug)]
pub enum PluginResult {
    Success(serde_json::Value),
    Error(String),
    Async(Box<dyn std::future::Future<Output = Result<serde_json::Value>>>),
}
```

### **Implementation Steps**

#### **Step 1: Plugin Loading Infrastructure** (Day 1)
```rust
// Add to Cargo.toml
[features]
default = ["enabled", "plugins"]
plugins = [
    "dep:libloading",
    "dep:semver",
    "dep:toml",
    "dep:serde_json",
    "dep:async-trait",
]

[dependencies]
libloading = { version = "0.8", optional = true }
semver = { version = "1.0", optional = true }
async-trait = { version = "0.1", optional = true }

#[cfg(feature = "plugins")]
mod plugin_system {
    use libloading::{Library, Symbol};
    use semver::{Version, VersionReq};
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use async_trait::async_trait;
    
    pub struct PluginLoader {
        plugin_directories: Vec<PathBuf>,
        loaded_libraries: Vec<Library>,
    }
    
    impl PluginLoader {
        pub fn new() -> Self {
            Self {
                plugin_directories: Vec::new(),
                loaded_libraries: Vec::new(),
            }
        }
        
        pub fn add_plugin_directory<P: AsRef<Path>>(&mut self, dir: P) {
            self.plugin_directories.push(dir.as_ref().to_path_buf());
        }
        
        pub fn discover_plugins(&self) -> Result<Vec<PluginDiscovery>> {
            let mut plugins = Vec::new();
            
            for plugin_dir in &self.plugin_directories {
                if !plugin_dir.exists() {
                    continue;
                }
                
                for entry in std::fs::read_dir(plugin_dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    // Look for plugin metadata files
                    if path.is_dir() {
                        let metadata_path = path.join("plugin.toml");
                        if metadata_path.exists() {
                            if let Ok(discovery) = self.load_plugin_metadata(&metadata_path) {
                                plugins.push(discovery);
                            }
                        }
                    }
                    
                    // Look for dynamic libraries
                    if path.is_file() && self.is_dynamic_library(&path) {
                        if let Ok(discovery) = self.discover_dynamic_plugin(&path) {
                            plugins.push(discovery);
                        }
                    }
                }
            }
            
            Ok(plugins)
        }
        
        fn load_plugin_metadata(&self, path: &Path) -> Result<PluginDiscovery> {
            let content = std::fs::read_to_string(path)?;
            let metadata: PluginMetadata = toml::from_str(&content)?;
            
            Ok(PluginDiscovery {
                metadata,
                source: PluginSource::Directory(path.parent().unwrap().to_path_buf()),
            })
        }
        
        fn discover_dynamic_plugin(&self, path: &Path) -> Result<PluginDiscovery> {
            // For dynamic libraries, we need to load them to get metadata
            unsafe {
                let lib = Library::new(path)?;
                let get_metadata: Symbol<extern fn() -> PluginMetadata> = 
                    lib.get(b"get_plugin_metadata")?;
                let metadata = get_metadata();
                
                Ok(PluginDiscovery {
                    metadata,
                    source: PluginSource::DynamicLibrary(path.to_path_buf()),
                })
            }
        }
        
        fn is_dynamic_library(&self, path: &Path) -> bool {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                matches!(ext, "so" | "dll" | "dylib")
            } else {
                false
            }
        }
        
        pub unsafe fn load_dynamic_plugin(&mut self, path: &Path) -> Result<Box<dyn WorkspacePlugin>> {
            let lib = Library::new(path)?;
            let create_plugin: Symbol<extern fn() -> Box<dyn WorkspacePlugin>> = 
                lib.get(b"create_plugin")?;
            
            let plugin = create_plugin();
            self.loaded_libraries.push(lib);
            Ok(plugin)
        }
    }
    
    pub struct PluginDiscovery {
        pub metadata: PluginMetadata,
        pub source: PluginSource,
    }
    
    pub enum PluginSource {
        Directory(PathBuf),
        DynamicLibrary(PathBuf),
        Wasm(PathBuf), // Future enhancement
    }
}
```

#### **Step 2: Plugin Registry and Management** (Day 2)
```rust
#[cfg(feature = "plugins")]
impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            event_bus: EventBus::new(),
            dependency_graph: DependencyGraph::new(),
        }
    }
    
    pub fn register_plugin(&mut self, plugin: Box<dyn WorkspacePlugin>) -> Result<()> {
        let metadata = plugin.metadata().clone();
        
        // Check for name conflicts
        if self.plugins.contains_key(&metadata.name) {
            return Err(WorkspaceError::ConfigurationError(
                format!("Plugin '{}' is already registered", metadata.name)
            ));
        }
        
        // Add to dependency graph
        self.dependency_graph.add_plugin(&metadata)?;
        
        // Create plugin handle
        let handle = PluginHandle {
            plugin,
            metadata: metadata.clone(),
            state: PluginState::Loaded,
        };
        
        self.plugins.insert(metadata.name, handle);
        Ok(())
    }
    
    pub fn initialize_plugins(&mut self, workspace: &Workspace) -> Result<()> {
        // Get plugins in dependency order
        let initialization_order = self.dependency_graph.get_initialization_order()?;
        
        for plugin_name in initialization_order {
            if let Some(handle) = self.plugins.get_mut(&plugin_name) {
                let context = PluginContext::new(workspace, &self.plugins);
                
                match handle.plugin.initialize(&context) {
                    Ok(()) => {
                        handle.state = PluginState::Initialized;
                        println!("✅ Plugin '{}' initialized successfully", plugin_name);
                    }
                    Err(e) => {
                        handle.state = PluginState::Error(e.to_string());
                        eprintln!("❌ Plugin '{}' initialization failed: {}", plugin_name, e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn execute_command(
        &self,
        plugin_name: &str,
        command: &str,
        args: &[String]
    ) -> Result<PluginResult> {
        let handle = self.plugins.get(plugin_name)
            .ok_or_else(|| WorkspaceError::ConfigurationError(
                format!("Plugin '{}' not found", plugin_name)
            ))?;
        
        match handle.state {
            PluginState::Initialized => {
                handle.plugin.execute_command(command, args)
            }
            PluginState::Loaded => {
                Err(WorkspaceError::ConfigurationError(
                    format!("Plugin '{}' not initialized", plugin_name)
                ))
            }
            PluginState::Error(ref error) => {
                Err(WorkspaceError::ConfigurationError(
                    format!("Plugin '{}' is in error state: {}", plugin_name, error)
                ))
            }
        }
    }
    
    pub fn broadcast_event(&self, event: &PluginEvent) -> Result<()> {
        for (name, handle) in &self.plugins {
            if handle.metadata.event_subscriptions.contains(&event.event_type) {
                if let Err(e) = handle.plugin.handle_event(event) {
                    eprintln!("Plugin '{}' event handler error: {}", name, e);
                }
            }
        }
        Ok(())
    }
    
    pub fn shutdown(&mut self) -> Result<()> {
        for (name, handle) in &mut self.plugins {
            if let Err(e) = handle.plugin.shutdown() {
                eprintln!("Plugin '{}' shutdown error: {}", name, e);
            }
        }
        self.plugins.clear();
        Ok(())
    }
    
    pub fn list_plugins(&self) -> Vec<&PluginMetadata> {
        self.plugins.values().map(|h| &h.metadata).collect()
    }
    
    pub fn list_commands(&self) -> Vec<(String, &PluginCommand)> {
        let mut commands = Vec::new();
        for (plugin_name, handle) in &self.plugins {
            for command in &handle.metadata.commands {
                commands.push((plugin_name.clone(), command));
            }
        }
        commands
    }
}

pub struct DependencyGraph {
    plugins: HashMap<String, PluginMetadata>,
    dependencies: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    pub fn add_plugin(&mut self, metadata: &PluginMetadata) -> Result<()> {
        let name = metadata.name.clone();
        
        // Validate dependencies exist
        for dep in &metadata.dependencies {
            if !dep.optional && !self.plugins.contains_key(&dep.name) {
                return Err(WorkspaceError::ConfigurationError(
                    format!("Plugin '{}' depends on '{}' which is not available", 
                            name, dep.name)
                ));
            }
            
            // Check version compatibility
            if let Some(existing) = self.plugins.get(&dep.name) {
                let existing_version = Version::parse(&existing.version)?;
                let required_version = VersionReq::parse(&dep.version_requirement)?;
                
                if !required_version.matches(&existing_version) {
                    return Err(WorkspaceError::ConfigurationError(
                        format!("Plugin '{}' requires '{}' version '{}', but '{}' is available",
                                name, dep.name, dep.version_requirement, existing.version)
                    ));
                }
            }
        }
        
        // Add to graph
        let deps: Vec<String> = metadata.dependencies
            .iter()
            .filter(|d| !d.optional)
            .map(|d| d.name.clone())
            .collect();
        
        self.dependencies.insert(name.clone(), deps);
        self.plugins.insert(name, metadata.clone());
        
        Ok(())
    }
    
    pub fn get_initialization_order(&self) -> Result<Vec<String>> {
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        let mut order = Vec::new();
        
        for plugin_name in self.plugins.keys() {
            if !visited.contains(plugin_name) {
                self.dfs_visit(plugin_name, &mut visited, &mut temp_visited, &mut order)?;
            }
        }
        
        Ok(order)
    }
    
    fn dfs_visit(
        &self,
        plugin: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<()> {
        if temp_visited.contains(plugin) {
            return Err(WorkspaceError::ConfigurationError(
                format!("Circular dependency detected involving plugin '{}'", plugin)
            ));
        }
        
        if visited.contains(plugin) {
            return Ok(());
        }
        
        temp_visited.insert(plugin.to_string());
        
        if let Some(deps) = self.dependencies.get(plugin) {
            for dep in deps {
                self.dfs_visit(dep, visited, temp_visited, order)?;
            }
        }
        
        temp_visited.remove(plugin);
        visited.insert(plugin.to_string());
        order.push(plugin.to_string());
        
        Ok(())
    }
}
```

#### **Step 3: Plugin Context and Communication** (Day 3)
```rust
#[cfg(feature = "plugins")]
pub struct PluginContext<'a> {
    workspace: &'a Workspace,
    plugins: &'a HashMap<String, PluginHandle>,
    shared_state: HashMap<String, serde_json::Value>,
}

impl<'a> PluginContext<'a> {
    pub fn new(workspace: &'a Workspace, plugins: &'a HashMap<String, PluginHandle>) -> Self {
        Self {
            workspace,
            plugins,
            shared_state: HashMap::new(),
        }
    }
    
    pub fn workspace(&self) -> &Workspace {
        self.workspace
    }
    
    pub fn get_plugin(&self, name: &str) -> Option<&PluginHandle> {
        self.plugins.get(name)
    }
    
    pub fn set_shared_data(&mut self, key: String, value: serde_json::Value) {
        self.shared_state.insert(key, value);
    }
    
    pub fn get_shared_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.shared_state.get(key)
    }
    
    pub fn list_available_plugins(&self) -> Vec<&String> {
        self.plugins.keys().collect()
    }
}

pub struct EventBus {
    listeners: HashMap<String, Vec<Box<dyn Fn(&PluginEvent) -> Result<()> + Send + Sync>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }
    
    pub fn subscribe<F>(&mut self, event_type: String, listener: F)
    where
        F: Fn(&PluginEvent) -> Result<()> + Send + Sync + 'static,
    {
        self.listeners
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(Box::new(listener));
    }
    
    pub fn emit(&self, event: &PluginEvent) -> Result<()> {
        if let Some(listeners) = self.listeners.get(&event.event_type) {
            for listener in listeners {
                if let Err(e) = listener(event) {
                    eprintln!("Event listener error: {}", e);
                }
            }
        }
        Ok(())
    }
}
```

#### **Step 4: Built-in Plugin Types** (Day 4)
```rust
// File processor plugin example
#[cfg(feature = "plugins")]
pub struct FileProcessorPlugin {
    metadata: PluginMetadata,
    processors: HashMap<String, Box<dyn FileProcessor>>,
}

pub trait FileProcessor: Send + Sync {
    fn can_process(&self, path: &Path) -> bool;
    fn process_file(&self, path: &Path, content: &str) -> Result<String>;
}

struct RustFormatterProcessor;

impl FileProcessor for RustFormatterProcessor {
    fn can_process(&self, path: &Path) -> bool {
        path.extension().and_then(|e| e.to_str()) == Some("rs")
    }
    
    fn process_file(&self, _path: &Path, content: &str) -> Result<String> {
        // Simple formatting example (real implementation would use rustfmt)
        let formatted = content
            .lines()
            .map(|line| line.trim_start())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(formatted)
    }
}

impl WorkspacePlugin for FileProcessorPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn initialize(&mut self, _context: &PluginContext) -> Result<()> {
        // Register built-in processors
        self.processors.insert(
            "rust_formatter".to_string(),
            Box::new(RustFormatterProcessor)
        );
        Ok(())
    }
    
    fn execute_command(&self, command: &str, args: &[String]) -> Result<PluginResult> {
        match command {
            "format" => {
                if args.is_empty() {
                    return Ok(PluginResult::Error("Path argument required".to_string()));
                }
                
                let path = Path::new(&args[0]);
                if !path.exists() {
                    return Ok(PluginResult::Error("File does not exist".to_string()));
                }
                
                let content = std::fs::read_to_string(path)?;
                
                for processor in self.processors.values() {
                    if processor.can_process(path) {
                        let formatted = processor.process_file(path, &content)?;
                        std::fs::write(path, formatted)?;
                        return Ok(PluginResult::Success(
                            serde_json::json!({"status": "formatted", "file": path})
                        ));
                    }
                }
                
                Ok(PluginResult::Error("No suitable processor found".to_string()))
            }
            "list_processors" => {
                let processors: Vec<&String> = self.processors.keys().collect();
                Ok(PluginResult::Success(serde_json::json!(processors)))
            }
            _ => Ok(PluginResult::Error(format!("Unknown command: {}", command)))
        }
    }
}

// Workspace analyzer plugin
pub struct WorkspaceAnalyzerPlugin {
    metadata: PluginMetadata,
}

impl WorkspacePlugin for WorkspaceAnalyzerPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn initialize(&mut self, _context: &PluginContext) -> Result<()> {
        Ok(())
    }
    
    fn execute_command(&self, command: &str, args: &[String]) -> Result<PluginResult> {
        match command {
            "analyze" => {
                // Analyze workspace structure
                let workspace_path = args.get(0)
                    .map(|s| Path::new(s))
                    .unwrap_or_else(|| Path::new("."));
                
                let analysis = self.analyze_workspace(workspace_path)?;
                Ok(PluginResult::Success(analysis))
            }
            "report" => {
                // Generate analysis report
                let format = args.get(0).unwrap_or(&"json".to_string()).clone();
                let report = self.generate_report(&format)?;
                Ok(PluginResult::Success(report))
            }
            _ => Ok(PluginResult::Error(format!("Unknown command: {}", command)))
        }
    }
}

impl WorkspaceAnalyzerPlugin {
    fn analyze_workspace(&self, path: &Path) -> Result<serde_json::Value> {
        let mut file_count = 0;
        let mut dir_count = 0;
        let mut file_types = HashMap::new();
        
        if path.is_dir() {
            for entry in walkdir::WalkDir::new(path) {
                let entry = entry.map_err(|e| WorkspaceError::IoError(e.to_string()))?;
                
                if entry.file_type().is_file() {
                    file_count += 1;
                    
                    if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                        *file_types.entry(ext.to_string()).or_insert(0) += 1;
                    }
                } else if entry.file_type().is_dir() {
                    dir_count += 1;
                }
            }
        }
        
        Ok(serde_json::json!({
            "workspace_path": path,
            "total_files": file_count,
            "total_directories": dir_count,
            "file_types": file_types,
            "analyzed_at": chrono::Utc::now().to_rfc3339()
        }))
    }
    
    fn generate_report(&self, format: &str) -> Result<serde_json::Value> {
        match format {
            "json" => Ok(serde_json::json!({
                "format": "json",
                "generated_at": chrono::Utc::now().to_rfc3339()
            })),
            "markdown" => Ok(serde_json::json!({
                "format": "markdown",
                "content": "# Workspace Analysis Report\n\nGenerated by workspace_tools analyzer plugin."
            })),
            _ => Err(WorkspaceError::ConfigurationError(
                format!("Unsupported report format: {}", format)
            ))
        }
    }
}
```

#### **Step 5: Workspace Plugin Integration** (Day 5)
```rust
#[cfg(feature = "plugins")]
impl Workspace {
    pub fn load_plugins(&mut self) -> Result<PluginRegistry> {
        let mut registry = PluginRegistry::new();
        let mut loader = PluginLoader::new();
        
        // Add default plugin directories
        loader.add_plugin_directory(self.plugins_dir());
        loader.add_plugin_directory(self.join(".plugins"));
        
        // Add system-wide plugin directory if it exists
        if let Some(home_dir) = dirs::home_dir() {
            loader.add_plugin_directory(home_dir.join(".workspace_tools/plugins"));
        }
        
        // Discover and load plugins
        let discovered_plugins = loader.discover_plugins()?;
        
        for discovery in discovered_plugins {
            match self.load_plugin_from_discovery(discovery, &mut loader) {
                Ok(plugin) => {
                    if let Err(e) = registry.register_plugin(plugin) {
                        eprintln!("Failed to register plugin: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load plugin: {}", e);
                }
            }
        }
        
        // Initialize all plugins
        registry.initialize_plugins(self)?;
        
        Ok(registry)
    }
    
    fn load_plugin_from_discovery(
        &self,
        discovery: PluginDiscovery,
        loader: &mut PluginLoader,
    ) -> Result<Box<dyn WorkspacePlugin>> {
        match discovery.source {
            PluginSource::Directory(path) => {
                // Load Rust source plugin (compile and load)
                self.load_source_plugin(&path, &discovery.metadata)
            }
            PluginSource::DynamicLibrary(path) => {
                // Load compiled plugin
                unsafe { loader.load_dynamic_plugin(&path) }
            }
            PluginSource::Wasm(_) => {
                // Future enhancement
                Err(WorkspaceError::ConfigurationError(
                    "WASM plugins not yet supported".to_string()
                ))
            }
        }
    }
    
    fn load_source_plugin(
        &self,
        path: &Path,
        metadata: &PluginMetadata,
    ) -> Result<Box<dyn WorkspacePlugin>> {
        // For source plugins, we need to compile them first
        // This is a simplified example - real implementation would be more complex
        
        let plugin_main = path.join("src").join("main.rs");
        if !plugin_main.exists() {
            return Err(WorkspaceError::ConfigurationError(
                "Plugin main.rs not found".to_string()
            ));
        }
        
        // For now, return built-in plugins based on metadata
        match metadata.name.as_str() {
            "file_processor" => Ok(Box::new(FileProcessorPlugin {
                metadata: metadata.clone(),
                processors: HashMap::new(),
            })),
            "workspace_analyzer" => Ok(Box::new(WorkspaceAnalyzerPlugin {
                metadata: metadata.clone(),
            })),
            _ => Err(WorkspaceError::ConfigurationError(
                format!("Unknown plugin type: {}", metadata.name)
            ))
        }
    }
    
    /// Get plugins directory
    pub fn plugins_dir(&self) -> PathBuf {
        self.root().join("plugins")
    }
    
    pub async fn execute_plugin_command(
        &self,
        plugin_name: &str,
        command: &str,
        args: &[String]
    ) -> Result<PluginResult> {
        // This would typically be stored as instance state
        let registry = self.load_plugins()?;
        registry.execute_command(plugin_name, command, args)
    }
}
```

#### **Step 6: Testing and Examples** (Day 6)
```rust
#[cfg(test)]
#[cfg(feature = "plugins")]
mod plugin_tests {
    use super::*;
    use crate::testing::create_test_workspace_with_structure;
    
    struct TestPlugin {
        metadata: PluginMetadata,
        initialized: bool,
    }
    
    impl WorkspacePlugin for TestPlugin {
        fn metadata(&self) -> &PluginMetadata {
            &self.metadata
        }
        
        fn initialize(&mut self, _context: &PluginContext) -> Result<()> {
            self.initialized = true;
            Ok(())
        }
        
        fn execute_command(&self, command: &str, args: &[String]) -> Result<PluginResult> {
            match command {
                "test" => Ok(PluginResult::Success(
                    serde_json::json!({"command": "test", "args": args})
                )),
                "error" => Ok(PluginResult::Error("Test error".to_string())),
                _ => Ok(PluginResult::Error(format!("Unknown command: {}", command)))
            }
        }
    }
    
    #[test]
    fn test_plugin_registry() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        let mut registry = PluginRegistry::new();
        
        let test_plugin = TestPlugin {
            metadata: PluginMetadata {
                name: "test_plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "Test plugin".to_string(),
                author: "Test Author".to_string(),
                dependencies: Vec::new(),
                commands: vec![
                    PluginCommand {
                        name: "test".to_string(),
                        description: "Test command".to_string(),
                        usage: "test [args...]".to_string(),
                        args: Vec::new(),
                    }
                ],
                event_subscriptions: Vec::new(),
            },
            initialized: false,
        };
        
        registry.register_plugin(Box::new(test_plugin)).unwrap();
        registry.initialize_plugins(&ws).unwrap();
        
        let result = registry.execute_command("test_plugin", "test", &["arg1".to_string()]).unwrap();
        
        match result {
            PluginResult::Success(value) => {
                assert_eq!(value["command"], "test");
                assert_eq!(value["args"][0], "arg1");
            }
            _ => panic!("Expected success result"),
        }
    }
    
    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        
        let plugin_a = PluginMetadata {
            name: "plugin_a".to_string(),
            version: "1.0.0".to_string(),
            description: "Plugin A".to_string(),
            author: "Test".to_string(),
            dependencies: Vec::new(),
            commands: Vec::new(),
            event_subscriptions: Vec::new(),
        };
        
        let plugin_b = PluginMetadata {
            name: "plugin_b".to_string(),
            version: "1.0.0".to_string(),
            description: "Plugin B".to_string(),
            author: "Test".to_string(),
            dependencies: vec![PluginDependency {
                name: "plugin_a".to_string(),
                version_requirement: "^1.0".to_string(),
                optional: false,
            }],
            commands: Vec::new(),
            event_subscriptions: Vec::new(),
        };
        
        graph.add_plugin(&plugin_a).unwrap();
        graph.add_plugin(&plugin_b).unwrap();
        
        let order = graph.get_initialization_order().unwrap();
        assert_eq!(order, vec!["plugin_a".to_string(), "plugin_b".to_string()]);
    }
}
```

### **Documentation Updates**

#### **README.md Addition**
```markdown
## 🔌 plugin architecture

workspace_tools supports a comprehensive plugin system for extending functionality:

```rust
use workspace_tools::workspace;

let mut ws = workspace()?;

// Load all plugins from plugin directories
let mut registry = ws.load_plugins()?;

// Execute plugin commands
let result = ws.execute_plugin_command("file_processor", "format", &["src/main.rs"]).await?;

// List available plugins and commands
for plugin in registry.list_plugins() {
    println!("Plugin: {} v{}", plugin.name, plugin.version);
    for command in &plugin.commands {
        println!("  Command: {} - {}", command.name, command.description);
    }
}
```

**Plugin Types:**
- File processors (formatting, linting, compilation)
- Workspace analyzers and reporters
- Custom command extensions
- Configuration validators
- Template engines
```

#### **New Example: plugin_system.rs**
```rust
//! Plugin system demonstration

use workspace_tools::{workspace, WorkspacePlugin, PluginMetadata, PluginContext, PluginResult, PluginCommand, CommandArg, ArgType};

struct CustomAnalyzerPlugin {
    metadata: PluginMetadata,
}

impl CustomAnalyzerPlugin {
    fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "custom_analyzer".to_string(),
                version: "1.0.0".to_string(),
                description: "Custom workspace analyzer".to_string(),
                author: "Example Developer".to_string(),
                dependencies: Vec::new(),
                commands: vec![
                    PluginCommand {
                        name: "analyze".to_string(),
                        description: "Analyze workspace structure".to_string(),
                        usage: "analyze [directory]".to_string(),
                        args: vec![
                            CommandArg {
                                name: "directory".to_string(),
                                description: "Directory to analyze".to_string(),
                                required: false,
                                arg_type: ArgType::Path,
                            }
                        ],
                    }
                ],
                event_subscriptions: Vec::new(),
            }
        }
    }
}

impl WorkspacePlugin for CustomAnalyzerPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn initialize(&mut self, context: &PluginContext) -> workspace_tools::Result<()> {
        println!("🔌 Initializing custom analyzer plugin");
        println!("   Workspace root: {}", context.workspace().root().display());
        Ok(())
    }
    
    fn execute_command(&self, command: &str, args: &[String]) -> workspace_tools::Result<PluginResult> {
        match command {
            "analyze" => {
                let target_dir = args.get(0)
                    .map(|s| std::path::Path::new(s))
                    .unwrap_or_else(|| std::path::Path::new("."));
                
                println!("🔍 Analyzing directory: {}", target_dir.display());
                
                let mut file_count = 0;
                let mut rust_files = 0;
                
                if let Ok(entries) = std::fs::read_dir(target_dir) {
                    for entry in entries.flatten() {
                        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                            file_count += 1;
                            
                            if entry.path().extension()
                                .and_then(|ext| ext.to_str()) == Some("rs") {
                                rust_files += 1;
                            }
                        }
                    }
                }
                
                let result = serde_json::json!({
                    "directory": target_dir,
                    "total_files": file_count,
                    "rust_files": rust_files,
                    "analysis_date": chrono::Utc::now().to_rfc3339()
                });
                
                Ok(PluginResult::Success(result))
            }
            _ => Ok(PluginResult::Error(format!("Unknown command: {}", command)))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = workspace()?;
    
    println!("🔌 Plugin System Demo");
    
    // Manually register our custom plugin (normally loaded from plugin directory)
    let mut registry = workspace_tools::PluginRegistry::new();
    let custom_plugin = CustomAnalyzerPlugin::new();
    
    registry.register_plugin(Box::new(custom_plugin))?;
    registry.initialize_plugins(&ws)?;
    
    // List available plugins
    println!("\n📋 Available plugins:");
    for plugin in registry.list_plugins() {
        println!("  {} v{}: {}", plugin.name, plugin.version, plugin.description);
    }
    
    // List available commands
    println!("\n⚡ Available commands:");
    for (plugin_name, command) in registry.list_commands() {
        println!("  {}.{}: {}", plugin_name, command.name, command.description);
    }
    
    // Execute plugin command
    println!("\n🚀 Executing plugin command...");
    match registry.execute_command("custom_analyzer", "analyze", &["src".to_string()]) {
        Ok(PluginResult::Success(result)) => {
            println!("✅ Command executed successfully:");
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Ok(PluginResult::Error(error)) => {
            println!("❌ Command failed: {}", error);
        }
        Err(e) => {
            println!("❌ Execution error: {}", e);
        }
    }
    
    Ok(())
}
```

### **Success Criteria**
- [ ] Dynamic plugin discovery and loading
- [ ] Plugin dependency resolution and initialization ordering
- [ ] Safe plugin sandboxing and error isolation
- [ ] Extensible plugin API with well-defined interfaces
- [ ] Built-in plugin types for common use cases
- [ ] Event system for plugin communication
- [ ] Plugin metadata and version management
- [ ] Comprehensive test coverage

### **Future Enhancements**
- WASM plugin support for language-agnostic plugins
- Plugin marketplace and distribution system
- Hot-swappable plugin reloading
- Plugin security and permission system
- Visual plugin management interface
- Plugin testing and validation framework
- Cross-platform plugin compilation

### **Breaking Changes**
None - this is purely additive functionality with feature flag.

This task transforms workspace_tools from a utility library into a comprehensive platform for workspace management, enabling unlimited extensibility through the plugin ecosystem.