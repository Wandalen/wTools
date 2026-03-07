# Task 009: Multi-Workspace Support

**Priority**: 🏢 Medium-High Impact  
**Phase**: 3 (Advanced Features)  
**Estimated Effort**: 4-5 days  
**Dependencies**: Task 001 (Cargo Integration), Task 006 (Environment Management) recommended  

## **Objective**
Implement comprehensive multi-workspace support for managing complex projects with multiple related workspaces, enabling workspace_tools to handle enterprise-scale development environments and monorepos effectively.

## **Technical Requirements**

### **Core Features**
1. **Workspace Discovery and Management**
   - Automatic discovery of related workspaces
   - Workspace relationship mapping
   - Hierarchical workspace structures
   - Cross-workspace dependency tracking

2. **Unified Operations**
   - Cross-workspace configuration management
   - Synchronized operations across workspaces
   - Resource sharing between workspaces
   - Global workspace commands

3. **Workspace Orchestration**
   - Build order resolution based on dependencies
   - Parallel workspace operations
   - Workspace-specific environment management
   - Coordination of workspace lifecycles

### **New API Surface**
```rust
impl Workspace {
    /// Discover and create multi-workspace manager
    pub fn discover_multi_workspace(&self) -> Result<MultiWorkspaceManager>;
    
    /// Create multi-workspace from explicit workspace list
    pub fn create_multi_workspace(workspaces: Vec<Workspace>) -> Result<MultiWorkspaceManager>;
    
    /// Find all related workspaces
    pub fn find_related_workspaces(&self) -> Result<Vec<Workspace>>;
    
    /// Get parent workspace if this is a sub-workspace
    pub fn parent_workspace(&self) -> Result<Option<Workspace>>;
    
    /// Get all child workspaces
    pub fn child_workspaces(&self) -> Result<Vec<Workspace>>;
}

pub struct MultiWorkspaceManager {
    workspaces: HashMap<String, Workspace>,
    dependency_graph: WorkspaceDependencyGraph,
    shared_config: SharedConfiguration,
    coordination_mode: CoordinationMode,
}

impl MultiWorkspaceManager {
    /// Get workspace by name
    pub fn get_workspace(&self, name: &str) -> Option<&Workspace>;
    
    /// Execute command across all workspaces
    pub async fn execute_all<F>(&self, operation: F) -> Result<HashMap<String, OperationResult>>
    where
        F: Fn(&Workspace) -> Result<OperationResult> + Send + Sync;
    
    /// Execute command across workspaces in dependency order
    pub async fn execute_ordered<F>(&self, operation: F) -> Result<HashMap<String, OperationResult>>
    where
        F: Fn(&Workspace) -> Result<OperationResult> + Send + Sync;
    
    /// Get build/operation order based on dependencies
    pub fn get_execution_order(&self) -> Result<Vec<String>>;
    
    /// Load shared configuration across all workspaces
    pub fn load_shared_config<T>(&self, config_name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
    
    /// Set shared configuration for all workspaces
    pub fn set_shared_config<T>(&self, config_name: &str, config: &T) -> Result<()>
    where
        T: serde::Serialize;
    
    /// Synchronize configurations across workspaces
    pub fn sync_configurations(&self) -> Result<()>;
    
    /// Watch for changes across all workspaces
    pub async fn watch_all_changes(&self) -> Result<MultiWorkspaceChangeStream>;
}

#[derive(Debug, Clone)]
pub struct WorkspaceRelation {
    pub workspace_name: String,
    pub relation_type: RelationType,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone)]
pub enum RelationType {
    Parent,
    Child,
    Sibling,
    Dependency,
    Dependent,
}

#[derive(Debug, Clone)]
pub enum DependencyType {
    Build,      // Build-time dependency
    Runtime,    // Runtime dependency  
    Data,       // Shared data dependency
    Config,     // Configuration dependency
}

#[derive(Debug, Clone)]
pub enum CoordinationMode {
    Centralized,   // Single coordinator
    Distributed,   // Peer-to-peer coordination
    Hierarchical,  // Tree-based coordination
}

pub struct SharedConfiguration {
    global_config: HashMap<String, serde_json::Value>,
    workspace_overrides: HashMap<String, HashMap<String, serde_json::Value>>,
}

pub struct WorkspaceDependencyGraph {
    workspaces: HashMap<String, WorkspaceNode>,
    dependencies: HashMap<String, Vec<WorkspaceDependency>>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceDependency {
    pub target: String,
    pub dependency_type: DependencyType,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct OperationResult {
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
    pub duration: std::time::Duration,
}

pub struct MultiWorkspaceChangeStream {
    receiver: tokio::sync::mpsc::UnboundedReceiver<WorkspaceChange>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceChange {
    pub workspace_name: String,
    pub change_type: ChangeType,
    pub path: PathBuf,
    pub timestamp: std::time::SystemTime,
}
```

### **Implementation Steps**

#### **Step 1: Workspace Discovery** (Day 1)
```rust
// Add to Cargo.toml
[features]
default = ["enabled", "multi_workspace"]
multi_workspace = [
    "async",
    "dep:walkdir",
    "dep:petgraph",
    "dep:futures-util",
]

[dependencies]
walkdir = { version = "2.0", optional = true }
petgraph = { version = "0.6", optional = true }

#[cfg(feature = "multi_workspace")]
mod multi_workspace {
    use walkdir::WalkDir;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    
    impl Workspace {
        pub fn discover_multi_workspace(&self) -> Result<MultiWorkspaceManager> {
            let mut discovered_workspaces = HashMap::new();
            
            // Start from current workspace
            discovered_workspaces.insert(
                self.workspace_name(),
                self.clone()
            );
            
            // Discover related workspaces
            let related = self.find_related_workspaces()?;
            for workspace in related {
                discovered_workspaces.insert(
                    workspace.workspace_name(),
                    workspace
                );
            }
            
            // Build dependency graph
            let dependency_graph = self.build_dependency_graph(&discovered_workspaces)?;
            
            Ok(MultiWorkspaceManager {
                workspaces: discovered_workspaces,
                dependency_graph,
                shared_config: SharedConfiguration::new(),
                coordination_mode: CoordinationMode::Centralized,
            })
        }
        
        pub fn find_related_workspaces(&self) -> Result<Vec<Workspace>> {
            let mut workspaces = Vec::new();
            let current_root = self.root();
            
            // Search upward for parent workspaces
            if let Some(parent) = self.find_parent_workspace()? {
                workspaces.push(parent);
            }
            
            // Search downward for child workspaces
            workspaces.extend(self.find_child_workspaces()?);
            
            // Search sibling directories
            if let Some(parent_dir) = current_root.parent() {
                workspaces.extend(self.find_sibling_workspaces(parent_dir)?);
            }
            
            // Search for workspaces mentioned in configuration
            workspaces.extend(self.find_configured_workspaces()?);
            
            Ok(workspaces)
        }
        
        fn find_parent_workspace(&self) -> Result<Option<Workspace>> {
            let mut current_path = self.root();
            
            while let Some(parent) = current_path.parent() {
                // Check if parent directory contains workspace markers
                if self.is_workspace_root(parent) && parent != self.root() {
                    return Ok(Some(Workspace::new(parent)?));
                }
                current_path = parent;
            }
            
            Ok(None)
        }
        
        fn find_child_workspaces(&self) -> Result<Vec<Workspace>> {
            let mut workspaces = Vec::new();
            
            for entry in WalkDir::new(self.root())
                .max_depth(3) // Don't go too deep
                .into_iter()
                .filter_entry(|e| !self.should_skip_directory(e.path()))
            {
                let entry = entry.map_err(|e| WorkspaceError::IoError(e.to_string()))?;
                let path = entry.path();
                
                if path != self.root() && self.is_workspace_root(path) {
                    workspaces.push(Workspace::new(path)?);
                }
            }
            
            Ok(workspaces)
        }
        
        fn find_sibling_workspaces(&self, parent_dir: &Path) -> Result<Vec<Workspace>> {
            let mut workspaces = Vec::new();
            
            if let Ok(entries) = std::fs::read_dir(parent_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_dir() && 
                       path != self.root() && 
                       self.is_workspace_root(&path) {
                        workspaces.push(Workspace::new(path)?);
                    }
                }
            }
            
            Ok(workspaces)
        }
        
        fn find_configured_workspaces(&self) -> Result<Vec<Workspace>> {
            let mut workspaces = Vec::new();
            
            // Check for workspace configuration file
            let workspace_config_path = self.config_dir().join("workspaces.toml");
            if workspace_config_path.exists() {
                let config_content = std::fs::read_to_string(&workspace_config_path)?;
                let config: WorkspaceConfig = toml::from_str(&config_content)?;
                
                for workspace_path in config.workspaces {
                    let full_path = if Path::new(&workspace_path).is_absolute() {
                        PathBuf::from(workspace_path)
                    } else {
                        self.root().join(workspace_path)
                    };
                    
                    if full_path.exists() && self.is_workspace_root(&full_path) {
                        workspaces.push(Workspace::new(full_path)?);
                    }
                }
            }
            
            Ok(workspaces)
        }
        
        fn is_workspace_root(&self, path: &Path) -> bool {
            // Check for common workspace markers
            let markers = [
                "Cargo.toml",
                "package.json", 
                "workspace_tools.toml",
                ".workspace",
                "pyproject.toml",
            ];
            
            markers.iter().any(|marker| path.join(marker).exists())
        }
        
        fn should_skip_directory(&self, path: &Path) -> bool {
            let skip_dirs = [
                "target", "node_modules", ".git", "dist", "build", 
                "__pycache__", ".pytest_cache", "venv", ".venv"
            ];
            
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                skip_dirs.contains(&dir_name) || dir_name.starts_with('.')
            } else {
                false
            }
        }
        
        fn workspace_name(&self) -> String {
            self.root()
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string()
        }
    }
    
    #[derive(serde::Deserialize)]
    struct WorkspaceConfig {
        workspaces: Vec<String>,
    }
}
```

#### **Step 2: Dependency Graph Construction** (Day 2)
```rust
#[cfg(feature = "multi_workspace")]
impl Workspace {
    fn build_dependency_graph(
        &self,
        workspaces: &HashMap<String, Workspace>
    ) -> Result<WorkspaceDependencyGraph> {
        use petgraph::{Graph, Directed};
        use petgraph::graph::NodeIndex;
        
        let mut graph = WorkspaceDependencyGraph::new();
        let mut node_indices = HashMap::new();
        
        // Add all workspaces as nodes
        for (name, workspace) in workspaces {
            graph.add_workspace_node(name.clone(), workspace.clone());
        }
        
        // Discover dependencies between workspaces
        for (name, workspace) in workspaces {
            let dependencies = self.discover_workspace_dependencies(workspace, workspaces)?;
            
            for dep in dependencies {
                graph.add_dependency(name.clone(), dep)?;
            }
        }
        
        Ok(graph)
    }
    
    fn discover_workspace_dependencies(
        &self,
        workspace: &Workspace,
        all_workspaces: &HashMap<String, Workspace>
    ) -> Result<Vec<WorkspaceDependency>> {
        let mut dependencies = Vec::new();
        
        // Check Cargo.toml dependencies (for Rust workspaces)
        dependencies.extend(self.discover_cargo_dependencies(workspace, all_workspaces)?);
        
        // Check package.json dependencies (for Node.js workspaces)
        dependencies.extend(self.discover_npm_dependencies(workspace, all_workspaces)?);
        
        // Check workspace configuration dependencies
        dependencies.extend(self.discover_config_dependencies(workspace, all_workspaces)?);
        
        // Check data dependencies (shared resources)
        dependencies.extend(self.discover_data_dependencies(workspace, all_workspaces)?);
        
        Ok(dependencies)
    }
    
    fn discover_cargo_dependencies(
        &self,
        workspace: &Workspace,
        all_workspaces: &HashMap<String, Workspace>
    ) -> Result<Vec<WorkspaceDependency>> {
        let mut dependencies = Vec::new();
        let cargo_toml_path = workspace.root().join("Cargo.toml");
        
        if !cargo_toml_path.exists() {
            return Ok(dependencies);
        }
        
        let content = std::fs::read_to_string(&cargo_toml_path)?;
        let cargo_toml: CargoToml = toml::from_str(&content)?;
        
        // Check workspace members
        if let Some(workspace_config) = &cargo_toml.workspace {
            for member in &workspace_config.members {
                let member_path = workspace.root().join(member);
                
                // Find matching workspace
                for (ws_name, ws) in all_workspaces {
                    if ws.root().starts_with(&member_path) || member_path.starts_with(ws.root()) {
                        dependencies.push(WorkspaceDependency {
                            target: ws_name.clone(),
                            dependency_type: DependencyType::Build,
                            required: true,
                        });
                    }
                }
            }
        }
        
        // Check path dependencies
        if let Some(deps) = &cargo_toml.dependencies {
            for (_, dep) in deps {
                if let Some(path) = self.extract_dependency_path(dep) {
                    let dep_path = workspace.root().join(&path);
                    
                    for (ws_name, ws) in all_workspaces {
                        if ws.root() == dep_path || dep_path.starts_with(ws.root()) {
                            dependencies.push(WorkspaceDependency {
                                target: ws_name.clone(),
                                dependency_type: DependencyType::Build,
                                required: true,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(dependencies)
    }
    
    fn discover_npm_dependencies(
        &self,
        workspace: &Workspace,
        all_workspaces: &HashMap<String, Workspace>
    ) -> Result<Vec<WorkspaceDependency>> {
        let mut dependencies = Vec::new();
        let package_json_path = workspace.root().join("package.json");
        
        if !package_json_path.exists() {
            return Ok(dependencies);
        }
        
        let content = std::fs::read_to_string(&package_json_path)?;
        let package_json: PackageJson = serde_json::from_str(&content)?;
        
        // Check workspaces field
        if let Some(workspaces_config) = &package_json.workspaces {
            for workspace_pattern in workspaces_config {
                // Expand glob patterns to find actual workspace directories
                let pattern_path = workspace.root().join(workspace_pattern);
                
                if let Ok(glob_iter) = glob::glob(&pattern_path.to_string_lossy()) {
                    for glob_result in glob_iter {
                        if let Ok(ws_path) = glob_result {
                            for (ws_name, ws) in all_workspaces {
                                if ws.root() == ws_path {
                                    dependencies.push(WorkspaceDependency {
                                        target: ws_name.clone(),
                                        dependency_type: DependencyType::Build,
                                        required: true,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(dependencies)
    }
    
    fn discover_config_dependencies(
        &self,
        workspace: &Workspace,
        all_workspaces: &HashMap<String, Workspace>
    ) -> Result<Vec<WorkspaceDependency>> {
        let mut dependencies = Vec::new();
        
        // Check workspace configuration for explicit dependencies
        let ws_config_path = workspace.config_dir().join("workspace_deps.toml");
        if ws_config_path.exists() {
            let content = std::fs::read_to_string(&ws_config_path)?;
            let config: WorkspaceDepsConfig = toml::from_str(&content)?;
            
            for dep in config.dependencies {
                if all_workspaces.contains_key(&dep.name) {
                    dependencies.push(WorkspaceDependency {
                        target: dep.name,
                        dependency_type: match dep.dep_type.as_str() {
                            "build" => DependencyType::Build,
                            "runtime" => DependencyType::Runtime,
                            "data" => DependencyType::Data,
                            "config" => DependencyType::Config,
                            _ => DependencyType::Build,
                        },
                        required: dep.required,
                    });
                }
            }
        }
        
        Ok(dependencies)
    }
    
    fn discover_data_dependencies(
        &self,
        workspace: &Workspace,
        all_workspaces: &HashMap<String, Workspace>
    ) -> Result<Vec<WorkspaceDependency>> {
        let mut dependencies = Vec::new();
        
        // Check for shared data directories
        let shared_data_config = workspace.data_dir().join("shared_sources.toml");
        if shared_data_config.exists() {
            let content = std::fs::read_to_string(&shared_data_config)?;
            let config: SharedDataConfig = toml::from_str(&content)?;
            
            for shared_path in config.shared_paths {
                let full_path = Path::new(&shared_path);
                
                // Find which workspace owns this shared data
                for (ws_name, ws) in all_workspaces {
                    if full_path.starts_with(ws.root()) {
                        dependencies.push(WorkspaceDependency {
                            target: ws_name.clone(),
                            dependency_type: DependencyType::Data,
                            required: false,
                        });
                    }
                }
            }
        }
        
        Ok(dependencies)
    }
}

#[derive(serde::Deserialize)]
struct CargoToml {
    workspace: Option<CargoWorkspace>,
    dependencies: Option<HashMap<String, toml::Value>>,
}

#[derive(serde::Deserialize)]
struct CargoWorkspace {
    members: Vec<String>,
}

#[derive(serde::Deserialize)]
struct PackageJson {
    workspaces: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct WorkspaceDepsConfig {
    dependencies: Vec<WorkspaceDep>,
}

#[derive(serde::Deserialize)]
struct WorkspaceDep {
    name: String,
    dep_type: String,
    required: bool,
}

#[derive(serde::Deserialize)]
struct SharedDataConfig {
    shared_paths: Vec<String>,
}
```

#### **Step 3: Multi-Workspace Operations** (Day 3)
```rust
#[cfg(feature = "multi_workspace")]
impl MultiWorkspaceManager {
    pub fn new(workspaces: HashMap<String, Workspace>) -> Self {
        Self {
            workspaces,
            dependency_graph: WorkspaceDependencyGraph::new(),
            shared_config: SharedConfiguration::new(),
            coordination_mode: CoordinationMode::Centralized,
        }
    }
    
    pub fn get_workspace(&self, name: &str) -> Option<&Workspace> {
        self.workspaces.get(name)
    }
    
    pub async fn execute_all<F>(&self, operation: F) -> Result<HashMap<String, OperationResult>>
    where
        F: Fn(&Workspace) -> Result<OperationResult> + Send + Sync + Clone,
    {
        use futures_util::stream::{FuturesUnordered, StreamExt};
        
        let mut futures = FuturesUnordered::new();
        
        for (name, workspace) in &self.workspaces {
            let op = operation.clone();
            let ws = workspace.clone();
            let name = name.clone();
            
            futures.push(tokio::task::spawn_blocking(move || {
                let start = std::time::Instant::now();
                let result = op(&ws);
                let duration = start.elapsed();
                
                let op_result = match result {
                    Ok(mut op_res) => {
                        op_res.duration = duration;
                        op_res
                    }
                    Err(e) => OperationResult {
                        success: false,
                        output: None,
                        error: Some(e.to_string()),
                        duration,
                    }
                };
                
                (name, op_result)
            }));
        }
        
        let mut results = HashMap::new();
        
        while let Some(result) = futures.next().await {
            match result {
                Ok((name, op_result)) => {
                    results.insert(name, op_result);
                }
                Err(e) => {
                    eprintln!("Task execution error: {}", e);
                }
            }
        }
        
        Ok(results)
    }
    
    pub async fn execute_ordered<F>(&self, operation: F) -> Result<HashMap<String, OperationResult>>
    where
        F: Fn(&Workspace) -> Result<OperationResult> + Send + Sync,
    {
        let execution_order = self.get_execution_order()?;
        let mut results = HashMap::new();
        
        for workspace_name in execution_order {
            if let Some(workspace) = self.workspaces.get(&workspace_name) {
                println!("🔄 Executing operation on workspace: {}", workspace_name);
                
                let start = std::time::Instant::now();
                let result = operation(workspace);
                let duration = start.elapsed();
                
                let op_result = match result {
                    Ok(mut op_res) => {
                        op_res.duration = duration;
                        println!("✅ Completed: {} ({:.2}s)", workspace_name, duration.as_secs_f64());
                        op_res
                    }
                    Err(e) => {
                        println!("❌ Failed: {} - {}", workspace_name, e);
                        OperationResult {
                            success: false,
                            output: None,
                            error: Some(e.to_string()),
                            duration,
                        }
                    }
                };
                
                results.insert(workspace_name, op_result);
            }
        }
        
        Ok(results)
    }
    
    pub fn get_execution_order(&self) -> Result<Vec<String>> {
        self.dependency_graph.topological_sort()
    }
    
    pub fn load_shared_config<T>(&self, config_name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        if let Some(global_value) = self.shared_config.global_config.get(config_name) {
            serde_json::from_value(global_value.clone())
                .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
        } else {
            // Try loading from first workspace that has the config
            for workspace in self.workspaces.values() {
                if let Ok(config) = workspace.load_config::<T>(config_name) {
                    return Ok(config);
                }
            }
            
            Err(WorkspaceError::ConfigurationError(
                format!("Shared config '{}' not found", config_name)
            ))
        }
    }
    
    pub fn set_shared_config<T>(&mut self, config_name: &str, config: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let json_value = serde_json::to_value(config)
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
        
        self.shared_config.global_config.insert(config_name.to_string(), json_value);
        Ok(())
    }
    
    pub fn sync_configurations(&self) -> Result<()> {
        println!("🔄 Synchronizing configurations across workspaces...");
        
        for (config_name, global_value) in &self.shared_config.global_config {
            for (ws_name, workspace) in &self.workspaces {
                // Apply workspace-specific overrides
                let final_value = if let Some(overrides) = self.shared_config.workspace_overrides.get(ws_name) {
                    if let Some(override_value) = overrides.get(config_name) {
                        self.merge_config_values(global_value, override_value)?
                    } else {
                        global_value.clone()
                    }
                } else {
                    global_value.clone()
                };
                
                // Write configuration to workspace
                let config_path = workspace.config_dir().join(format!("{}.json", config_name));
                let config_content = serde_json::to_string_pretty(&final_value)?;
                std::fs::write(&config_path, config_content)?;
                
                println!("  ✅ Synced {} to {}", config_name, ws_name);
            }
        }
        
        Ok(())
    }
    
    fn merge_config_values(
        &self,
        base: &serde_json::Value,
        override_val: &serde_json::Value
    ) -> Result<serde_json::Value> {
        // Simple merge - override values take precedence
        // In a real implementation, this would be more sophisticated
        match (base, override_val) {
            (serde_json::Value::Object(base_obj), serde_json::Value::Object(override_obj)) => {
                let mut result = base_obj.clone();
                for (key, value) in override_obj {
                    result.insert(key.clone(), value.clone());
                }
                Ok(serde_json::Value::Object(result))
            }
            _ => Ok(override_val.clone())
        }
    }
}

impl WorkspaceDependencyGraph {
    pub fn new() -> Self {
        Self {
            workspaces: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    pub fn add_workspace_node(&mut self, name: String, workspace: Workspace) {
        self.workspaces.insert(name.clone(), WorkspaceNode {
            name: name.clone(),
            workspace,
        });
        self.dependencies.entry(name).or_insert_with(Vec::new);
    }
    
    pub fn add_dependency(&mut self, from: String, dependency: WorkspaceDependency) -> Result<()> {
        self.dependencies
            .entry(from)
            .or_insert_with(Vec::new)
            .push(dependency);
        Ok(())
    }
    
    pub fn topological_sort(&self) -> Result<Vec<String>> {
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        let mut result = Vec::new();
        
        for workspace_name in self.workspaces.keys() {
            if !visited.contains(workspace_name) {
                self.visit(workspace_name, &mut visited, &mut temp_visited, &mut result)?;
            }
        }
        
        Ok(result)
    }
    
    fn visit(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<()> {
        if temp_visited.contains(node) {
            return Err(WorkspaceError::ConfigurationError(
                format!("Circular dependency detected involving workspace '{}'", node)
            ));
        }
        
        if visited.contains(node) {
            return Ok(());
        }
        
        temp_visited.insert(node.to_string());
        
        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if dep.required {
                    self.visit(&dep.target, visited, temp_visited, result)?;
                }
            }
        }
        
        temp_visited.remove(node);
        visited.insert(node.to_string());
        result.push(node.to_string());
        
        Ok(())
    }
}

#[derive(Debug)]
struct WorkspaceNode {
    name: String,
    workspace: Workspace,
}

impl SharedConfiguration {
    pub fn new() -> Self {
        Self {
            global_config: HashMap::new(),
            workspace_overrides: HashMap::new(),
        }
    }
}
```

#### **Step 4: Change Watching and Coordination** (Day 4)
```rust
#[cfg(feature = "multi_workspace")]
impl MultiWorkspaceManager {
    pub async fn watch_all_changes(&self) -> Result<MultiWorkspaceChangeStream> {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        
        for (ws_name, workspace) in &self.workspaces {
            let change_sender = sender.clone();
            let ws_name = ws_name.clone();
            let ws_root = workspace.root().to_path_buf();
            
            // Start file watcher for this workspace
            tokio::spawn(async move {
                if let Ok(mut watcher) = workspace.watch_changes().await {
                    while let Some(change) = watcher.next().await {
                        let ws_change = WorkspaceChange {
                            workspace_name: ws_name.clone(),
                            change_type: match change {
                                workspace_tools::WorkspaceChange::FileModified(path) => 
                                    ChangeType::FileModified,
                                workspace_tools::WorkspaceChange::FileCreated(path) => 
                                    ChangeType::FileCreated,
                                workspace_tools::WorkspaceChange::FileDeleted(path) => 
                                    ChangeType::FileDeleted,
                                _ => ChangeType::FileModified,
                            },
                            path: match change {
                                workspace_tools::WorkspaceChange::FileModified(path) |
                                workspace_tools::WorkspaceChange::FileCreated(path) |
                                workspace_tools::WorkspaceChange::FileDeleted(path) => path,
                                _ => ws_root.clone(),
                            },
                            timestamp: std::time::SystemTime::now(),
                        };
                        
                        if sender.send(ws_change).is_err() {
                            break; // Receiver dropped
                        }
                    }
                }
            });
        }
        
        Ok(MultiWorkspaceChangeStream { receiver })
    }
    
    /// Coordinate a build across all workspaces
    pub async fn coordinate_build(&self) -> Result<HashMap<String, OperationResult>> {
        println!("🏗️  Starting coordinated build across all workspaces...");
        
        self.execute_ordered(|workspace| {
            println!("Building workspace: {}", workspace.root().display());
            
            // Try different build systems
            if workspace.root().join("Cargo.toml").exists() {
                self.run_cargo_build(workspace)
            } else if workspace.root().join("package.json").exists() {
                self.run_npm_build(workspace)
            } else if workspace.root().join("Makefile").exists() {
                self.run_make_build(workspace)
            } else {
                Ok(OperationResult {
                    success: true,
                    output: Some("No build system detected, skipping".to_string()),
                    error: None,
                    duration: std::time::Duration::from_millis(0),
                })
            }
        }).await
    }
    
    fn run_cargo_build(&self, workspace: &Workspace) -> Result<OperationResult> {
        let output = std::process::Command::new("cargo")
            .arg("build")
            .current_dir(workspace.root())
            .output()?;
        
        Ok(OperationResult {
            success: output.status.success(),
            output: Some(String::from_utf8_lossy(&output.stdout).to_string()),
            error: if output.status.success() {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
            duration: std::time::Duration::from_millis(0), // Will be set by caller
        })
    }
    
    fn run_npm_build(&self, workspace: &Workspace) -> Result<OperationResult> {
        let output = std::process::Command::new("npm")
            .arg("run")
            .arg("build")
            .current_dir(workspace.root())
            .output()?;
        
        Ok(OperationResult {
            success: output.status.success(),
            output: Some(String::from_utf8_lossy(&output.stdout).to_string()),
            error: if output.status.success() {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
            duration: std::time::Duration::from_millis(0),
        })
    }
    
    fn run_make_build(&self, workspace: &Workspace) -> Result<OperationResult> {
        let output = std::process::Command::new("make")
            .current_dir(workspace.root())
            .output()?;
        
        Ok(OperationResult {
            success: output.status.success(),
            output: Some(String::from_utf8_lossy(&output.stdout).to_string()),
            error: if output.status.success() {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
            duration: std::time::Duration::from_millis(0),
        })
    }
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    FileModified,
    FileCreated,
    FileDeleted,
    DirectoryCreated,
    DirectoryDeleted,
}

impl MultiWorkspaceChangeStream {
    pub async fn next(&mut self) -> Option<WorkspaceChange> {
        self.receiver.recv().await
    }
    
    pub fn into_stream(self) -> impl futures_util::Stream<Item = WorkspaceChange> {
        tokio_stream::wrappers::UnboundedReceiverStream::new(self.receiver)
    }
}
```

#### **Step 5: Testing and Examples** (Day 5)
```rust
#[cfg(test)]
#[cfg(feature = "multi_workspace")]
mod multi_workspace_tests {
    use super::*;
    use crate::testing::create_test_workspace;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_multi_workspace_discovery() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();
        
        // Create multiple workspace directories
        let ws1_path = base_path.join("workspace1");
        let ws2_path = base_path.join("workspace2");
        let ws3_path = base_path.join("workspace3");
        
        std::fs::create_dir_all(&ws1_path).unwrap();
        std::fs::create_dir_all(&ws2_path).unwrap();
        std::fs::create_dir_all(&ws3_path).unwrap();
        
        // Create workspace markers
        std::fs::write(ws1_path.join("Cargo.toml"), "[package]\nname = \"ws1\"").unwrap();
        std::fs::write(ws2_path.join("package.json"), "{\"name\": \"ws2\"}").unwrap();
        std::fs::write(ws3_path.join(".workspace"), "").unwrap();
        
        let main_workspace = Workspace::new(&ws1_path).unwrap();
        let multi_ws = main_workspace.discover_multi_workspace().unwrap();
        
        assert!(multi_ws.workspaces.len() >= 1);
        assert!(multi_ws.get_workspace("workspace1").is_some());
    }
    
    #[tokio::test]
    async fn test_coordinated_execution() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();
        
        // Create two workspaces
        let ws1 = Workspace::new(base_path.join("ws1")).unwrap();
        let ws2 = Workspace::new(base_path.join("ws2")).unwrap();
        
        let mut workspaces = HashMap::new();
        workspaces.insert("ws1".to_string(), ws1);
        workspaces.insert("ws2".to_string(), ws2);
        
        let multi_ws = MultiWorkspaceManager::new(workspaces);
        
        let results = multi_ws.execute_all(|workspace| {
            // Simple test operation
            Ok(OperationResult {
                success: true,
                output: Some(format!("Processed: {}", workspace.root().display())),
                error: None,
                duration: std::time::Duration::from_millis(100),
            })
        }).await.unwrap();
        
        assert_eq!(results.len(), 2);
        assert!(results.get("ws1").unwrap().success);
        assert!(results.get("ws2").unwrap().success);
    }
    
    #[test]
    fn test_dependency_graph() {
        let mut graph = WorkspaceDependencyGraph::new();
        
        let ws1 = Workspace::new("/tmp/ws1").unwrap();
        let ws2 = Workspace::new("/tmp/ws2").unwrap();
        
        graph.add_workspace_node("ws1".to_string(), ws1);
        graph.add_workspace_node("ws2".to_string(), ws2);
        
        // ws2 depends on ws1
        graph.add_dependency("ws2".to_string(), WorkspaceDependency {
            target: "ws1".to_string(),
            dependency_type: DependencyType::Build,
            required: true,
        }).unwrap();
        
        let order = graph.topological_sort().unwrap();
        assert_eq!(order, vec!["ws1".to_string(), "ws2".to_string()]);
    }
}
```

### **Documentation Updates**

#### **README.md Addition**
```markdown
## 🏢 multi-workspace support

workspace_tools can manage complex projects with multiple related workspaces:

```rust
use workspace_tools::workspace;

let ws = workspace()?;

// Discover all related workspaces
let multi_ws = ws.discover_multi_workspace()?;

// Execute operations across all workspaces
let results = multi_ws.execute_all(|workspace| {
    println!("Processing: {}", workspace.root().display());
    // Your operation here
    Ok(OperationResult { success: true, .. })
}).await?;

// Execute in dependency order (build dependencies first)
let build_results = multi_ws.coordinate_build().await?;

// Watch changes across all workspaces
let mut changes = multi_ws.watch_all_changes().await?;
while let Some(change) = changes.next().await {
    println!("Change in {}: {:?}", change.workspace_name, change.path);
}
```

**Features:**
- Automatic workspace discovery and relationship mapping
- Dependency-ordered execution across workspaces
- Shared configuration management
- Cross-workspace change monitoring
- Support for Cargo, npm, and custom workspace types
```

#### **New Example: multi_workspace_manager.rs**
```rust
//! Multi-workspace management example

use workspace_tools::{workspace, MultiWorkspaceManager, OperationResult};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    println!("🏢 Multi-Workspace Management Demo");
    
    // Discover related workspaces
    println!("🔍 Discovering related workspaces...");
    let multi_ws = ws.discover_multi_workspace()?;
    
    println!("Found {} workspaces:", multi_ws.workspaces.len());
    for (name, workspace) in &multi_ws.workspaces {
        println!("  📁 {}: {}", name, workspace.root().display());
    }
    
    // Show execution order
    if let Ok(order) = multi_ws.get_execution_order() {
        println!("\n📋 Execution order (based on dependencies):");
        for (i, ws_name) in order.iter().enumerate() {
            println!("  {}. {}", i + 1, ws_name);
        }
    }
    
    // Execute a simple operation across all workspaces
    println!("\n⚙️ Running analysis across all workspaces...");
    let analysis_results = multi_ws.execute_all(|workspace| {
        println!("  🔍 Analyzing: {}", workspace.root().display());
        
        let mut file_count = 0;
        let mut dir_count = 0;
        
        if let Ok(entries) = std::fs::read_dir(workspace.root()) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    file_count += 1;
                } else if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    dir_count += 1;
                }
            }
        }
        
        Ok(OperationResult {
            success: true,
            output: Some(format!("Files: {}, Dirs: {}", file_count, dir_count)),
            error: None,
            duration: std::time::Duration::from_millis(0), // Will be set by framework
        })
    }).await?;
    
    println!("\n📊 Analysis Results:");
    for (ws_name, result) in &analysis_results {
        if result.success {
            println!("  ✅ {}: {} ({:.2}s)", 
                ws_name, 
                result.output.as_ref().unwrap_or(&"No output".to_string()),
                result.duration.as_secs_f64()
            );
        } else {
            println!("  ❌ {}: {}", 
                ws_name, 
                result.error.as_ref().unwrap_or(&"Unknown error".to_string())
            );
        }
    }
    
    // Demonstrate coordinated build
    println!("\n🏗️ Attempting coordinated build...");
    match multi_ws.coordinate_build().await {
        Ok(build_results) => {
            println!("Build completed for {} workspaces:", build_results.len());
            for (ws_name, result) in &build_results {
                if result.success {
                    println!("  ✅ {}: Build succeeded", ws_name);
                } else {
                    println!("  ❌ {}: Build failed", ws_name);
                }
            }
        }
        Err(e) => {
            println!("❌ Coordinated build failed: {}", e);
        }
    }
    
    // Start change monitoring (run for a short time)
    println!("\n👀 Starting change monitoring (5 seconds)...");
    if let Ok(mut changes) = multi_ws.watch_all_changes().await {
        let timeout = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            while let Some(change) = changes.next().await {
                println!("  📁 Change in {}: {} ({:?})",
                    change.workspace_name,
                    change.path.display(),
                    change.change_type
                );
            }
        });
        
        match timeout.await {
            Ok(_) => println!("Change monitoring completed"),
            Err(_) => println!("Change monitoring timed out (no changes detected)"),
        }
    }
    
    Ok(())
}
```

### **Success Criteria**
- [ ] Automatic discovery of related workspaces
- [ ] Dependency graph construction and validation
- [ ] Topological ordering for execution
- [ ] Parallel and sequential workspace operations
- [ ] Shared configuration management
- [ ] Cross-workspace change monitoring
- [ ] Support for multiple workspace types (Cargo, npm, custom)
- [ ] Comprehensive test coverage

### **Future Enhancements**
- Remote workspace support (Git submodules, network mounts)
- Workspace templates and cloning
- Advanced dependency resolution with version constraints
- Distributed build coordination
- Workspace synchronization and mirroring
- Integration with CI/CD systems
- Visual workspace relationship mapping

### **Breaking Changes**
None - this is purely additive functionality with feature flag.

This task enables workspace_tools to handle enterprise-scale development environments and complex monorepos, making it the go-to solution for organizations with sophisticated workspace management needs.