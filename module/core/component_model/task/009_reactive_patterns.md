# Task 009: Reactive Patterns and Live Updates

## 🎯 **Objective**

Implement reactive component assignment that automatically updates components when external sources change, enabling live configuration updates, file watching, environment variable monitoring, and real-time data synchronization.

## 📋 **Current State**

Static component assignment with no reactivity:
```rust
let config = AppConfig::default()
  .impute("localhost")
  .impute(8080)
  .load_from_env();  // One-time load

// Config never updates, even if env vars or files change
```

## 🎯 **Target State**

Reactive components that update automatically:
```rust
#[derive(ReactiveAssign)]
struct LiveConfig 
{
  #[component(watch_file = "app.toml")]
  settings: AppSettings,
  
  #[component(watch_env = "DATABASE_URL")]
  database_url: String,
  
  #[component(watch_consul = "app/feature-flags")]
  feature_flags: FeatureFlags,
  
  #[component(watch_api = "https://config.service/live", poll_interval = "30s")]
  live_settings: RemoteConfig,
}

// Configuration updates automatically when sources change
let mut config = LiveConfig::default();
let (config_handle, mut updates) = config.start_watching().await?;

// Listen for updates
while let Some(update) = updates.recv().await {
  match update {
    ComponentUpdate::Settings(new_settings) => {
      println!("Settings updated: {:?}", new_settings);
    },
    ComponentUpdate::DatabaseUrl(new_url) => {
      println!("Database URL changed: {}", new_url);
    },
  }
}
```

## 📝 **Detailed Requirements**

### **Core Reactive Traits**

#### **ReactiveAssign Trait**
```rust
#[async_trait]
pub trait ReactiveAssign<T> {
  type Watcher: ComponentWatcher<T>;
  type UpdateStream: Stream<Item = ComponentUpdate<T>>;
  type Error;
  
  fn start_watching(self) -> Result<(ReactiveHandle, Self::UpdateStream), Self::Error>;
  fn stop_watching(&mut self) -> Result<(), Self::Error>;
  
  async fn get_current_value(&self) -> Result<T, Self::Error>;
  fn add_update_callback<F>(&mut self, callback: F) 
  where
    F: Fn(ComponentUpdate<T>) + Send + Sync + 'static;
}

pub trait ComponentWatcher<T> {
  type Error;
  
  async fn watch(&mut self) -> Result<T, Self::Error>;
  fn should_update(&self, old_value: &T, new_value: &T) -> bool;
}
```

#### **Component Update Types**
```rust
#[derive(Debug, Clone)]
pub enum ComponentUpdate<T> 
{
  Updated { old_value: T, new_value: T },
  Added { value: T },
  Removed,
  Error { error: ComponentError },
}

#[derive(Debug, Clone)]
pub struct ReactiveHandle 
{
  watchers: Vec<Box<dyn WatcherHandle>>,
  cancellation_token: tokio_util::sync::CancellationToken,
}

impl ReactiveHandle 
{
  pub async fn stop(self) 
{
    self.cancellation_token.cancel();
    for watcher in self.watchers {
      watcher.stop().await;
    }
  }
}
```

### **Built-in Watchers**

#### **File System Watcher**
```rust
pub struct FileWatcher<T> 
{
  path: PathBuf,
  parser: Box<dyn Fn(&str) -> Result<T, ParseError>>,
  debounce_duration: Duration,
}

impl<T> FileWatcher<T> {
  pub fn new<P: Into<PathBuf>>(path: P) -> Self 
  where
    T: for<'de> serde::Deserialize<'de>,
  {
    Self {
      path: path.into(),
      parser: Box::new(|content| {
        // Auto-detect format and parse
        if path.extension() == Some("toml") {
          toml::from_str(content)
        } else if path.extension() == Some("yaml") {
          serde_yaml::from_str(content)
        } else {
          serde_json::from_str(content)
        }
      }),
      debounce_duration: Duration::from_millis(100),
    }
  }
}

#[async_trait]
impl<T> ComponentWatcher<T> for FileWatcher<T>
where
  T: Clone + PartialEq + Send + Sync + 'static,
{
  type Error = WatchError;
  
  async fn watch(&mut self) -> Result<T, Self::Error> 
{
    use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event};
    use tokio::sync::mpsc;
    
    let (tx, mut rx) = mpsc::channel(32);
    
    let mut watcher = RecommendedWatcher::new(
      move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
          let _ = tx.try_send(event);
        }
      },
      notify::Config::default(),
    )?;
    
    watcher.watch(&self.path, RecursiveMode::NonRecursive)?;
    
    loop {
      match rx.recv().await {
        Some(event) if event.paths.contains(&self.path) => {
          // Debounce multiple events
          tokio::time::sleep(self.debounce_duration).await;
          
          // Read and parse file
          let content = tokio::fs::read_to_string(&self.path).await?;
          let parsed = (self.parser)(&content)?;
          
          return Ok(parsed);
        },
        Some(_) => continue, // Different file
        None => break, // Channel closed
      }
    }
    
    Err(WatchError::ChannelClosed)
  }
}
```

#### **Environment Variable Watcher**
```rust
pub struct EnvWatcher 
{
  var_name: String,
  poll_interval: Duration,
  last_value: Option<String>,
}

#[async_trait]
impl ComponentWatcher<String> for EnvWatcher 
{
  type Error = WatchError;
  
  async fn watch(&mut self) -> Result<String, Self::Error> 
{
    let mut interval = tokio::time::interval(self.poll_interval);
    
    loop {
      interval.tick().await;
      
      let current_value = std::env::var(&self.var_name).ok();
      
      if current_value != self.last_value {
        if let Some(value) = current_value {
          self.last_value = Some(value.clone());
          return Ok(value);
        } else if self.last_value.is_some() {
          self.last_value = None;
          return Err(WatchError::VariableRemoved(self.var_name.clone()));
        }
      }
    }
  }
}
```

#### **HTTP API Watcher**
```rust
pub struct ApiWatcher<T> 
{
  url: String,
  client: reqwest::Client,
  poll_interval: Duration,
  last_etag: Option<String>,
}

#[async_trait]
impl<T> ComponentWatcher<T> for ApiWatcher<T>
where
  T: serde::de::DeserializeOwned + Send + Sync + 'static,
{
  type Error = WatchError;
  
  async fn watch(&mut self) -> Result<T, Self::Error> 
{
    let mut interval = tokio::time::interval(self.poll_interval);
    
    loop {
      interval.tick().await;
      
      let mut request = self.client.get(&self.url);
      
      // Use ETag for efficient polling
      if let Some(etag) = &self.last_etag {
        request = request.header("If-None-Match", etag);
      }
      
      let response = request.send().await?;
      
      if response.status() == 304 {
        continue; // No changes
      }
      
      // Update ETag
      if let Some(etag) = response.headers().get("etag") {
        self.last_etag = Some(etag.to_str()?.to_string());
      }
      
      let data: T = response.json().await?;
      return Ok(data);
    }
  }
}
```

#### **Consul KV Watcher**
```rust
pub struct ConsulWatcher<T> 
{
  client: consul::Client,
  key: String,
  last_index: Option<u64>,
}

#[async_trait]
impl<T> ComponentWatcher<T> for ConsulWatcher<T>
where
  T: serde::de::DeserializeOwned + Send + Sync + 'static,
{
  type Error = WatchError;
  
  async fn watch(&mut self) -> Result<T, Self::Error> 
{
    loop {
      let query = consul::kv::GetOptions::new()
        .with_index(self.last_index.unwrap_or(0))
        .with_wait(Duration::from_secs(30)); // Long polling
        
      let response = self.client.get_kv_with_options(&self.key, &query).await?;
      
      if let Some((value, meta)) = response {
        if Some(meta.modify_index) != self.last_index {
          self.last_index = Some(meta.modify_index);
          let parsed: T = serde_json::from_str(&value)?;
          return Ok(parsed);
        }
      }
    }
  }
}
```

### **Reactive Derive Implementation**

#### **ReactiveAssign Derive**
```rust
#[derive(ReactiveAssign)]
struct LiveConfig 
{
  #[component(watch_file = "config.toml", debounce = "200ms")]
  file_config: FileConfig,
  
  #[component(watch_env = "DATABASE_URL")]
  database_url: String,
  
  #[component(watch_consul = "app/flags", long_poll = "true")]
  feature_flags: FeatureFlags,
}

// Generates:
impl ReactiveAssign<FileConfig> for LiveConfig 
{
  type Watcher = FileWatcher<FileConfig>;
  type UpdateStream = tokio::sync::mpsc::Receiver<ComponentUpdate<FileConfig>>;
  type Error = ReactiveError;
  
  fn start_watching(mut self) -> Result<(ReactiveHandle, Self::UpdateStream), Self::Error> 
{
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let mut watchers = Vec::new();
    
    // File watcher
    let file_watcher = FileWatcher::new("config.toml")
      .with_debounce(Duration::from_millis(200));
    
    let file_tx = tx.clone();
    let file_handle = tokio::spawn(async move {
      let mut watcher = file_watcher;
      loop {
        match watcher.watch().await {
          Ok(new_config) => {
            let update = ComponentUpdate::Updated {
              old_value: self.file_config.clone(),
              new_value: new_config.clone(),
            };
            
            self.file_config = new_config;
            
            if file_tx.send(update).await.is_err() {
              break; // Receiver dropped
            }
          },
          Err(e) => {
            let _ = file_tx.send(ComponentUpdate::Error { 
              error: e.into() 
            }).await;
          }
        }
      }
    });
    
    watchers.push(Box::new(file_handle));
    
    // Environment variable watcher
    let env_watcher = EnvWatcher::new("DATABASE_URL");
    let env_tx = tx.clone();
    let env_handle = tokio::spawn(async move {
      // Similar implementation...
    });
    
    watchers.push(Box::new(env_handle));
    
    let handle = ReactiveHandle::new(watchers);
    Ok((handle, rx))
  }
}
```

### **Advanced Reactive Patterns**

#### **Dependency-Based Updates**
```rust
#[derive(ReactiveAssign)]
struct DependentConfig 
{
  #[component(watch_file = "base.toml")]
  base_config: BaseConfig,
  
  #[component(
    watch_file = "derived.toml", 
    depends_on = ["base_config"],
    update_fn = "merge_configs"
  )]
  derived_config: DerivedConfig,
}

impl DependentConfig 
{
  fn merge_configs(&mut self, new_derived: DerivedConfig) 
{
    // Custom merge logic that considers base_config
    self.derived_config = new_derived.merge_with(&self.base_config);
  }
}
```

#### **Conditional Watching**
```rust
#[derive(ReactiveAssign)]
struct ConditionalConfig 
{
  #[component(watch_env = "APP_MODE")]
  mode: AppMode,
  
  #[component(
    watch_file = "dev.toml",
    condition = "mode == AppMode::Development"
  )]
  dev_settings: Option<DevSettings>,
  
  #[component(
    watch_consul = "prod/settings",
    condition = "mode == AppMode::Production"
  )]
  prod_settings: Option<ProdSettings>,
}
```

#### **Throttling and Rate Limiting**
```rust
#[derive(ReactiveAssign)]
struct ThrottledConfig 
{
  #[component(
    watch_api = "https://config.service/live",
    throttle = "5s",  // Max one update per 5 seconds
    burst_limit = "3"  // Allow burst of 3 updates
  )]
  live_settings: LiveSettings,
}
```

## 🗂️ **File Changes**

### **New Files**
- `component_model_reactive/` - New crate for reactive patterns
- `component_model_reactive/src/lib.rs` - Main reactive API
- `component_model_reactive/src/reactive_derive.rs` - ReactiveAssign derive
- `component_model_reactive/src/watchers/` - Built-in watchers
- `component_model_reactive/src/watchers/file.rs` - File system watcher
- `component_model_reactive/src/watchers/env.rs` - Environment variable watcher
- `component_model_reactive/src/watchers/http.rs` - HTTP API watcher
- `component_model_reactive/src/watchers/consul.rs` - Consul integration
- `component_model_reactive/src/watchers/vault.rs` - Vault integration
- `component_model_reactive/src/stream.rs` - Update stream utilities
- `component_model_reactive/src/handle.rs` - Reactive handle management
- `examples/reactive_config_example.rs` - Live configuration example
- `examples/reactive_web_app.rs` - Web app with live updates

### **Modified Files**
- `Cargo.toml` - Add new workspace member
- `component_model/Cargo.toml` - Add reactive dependency (feature-gated)

## ⚡ **Implementation Steps**

### **Phase 1: Core Infrastructure (Week 1-2)**
1. Define reactive traits and update types
2. Implement basic file watcher with notify crate
3. Create environment variable polling watcher
4. Basic reactive derive macro with file watching

### **Phase 2: Advanced Watchers (Week 2-3)**
1. HTTP API watcher with efficient polling (ETag support)
2. Consul KV watcher with long polling
3. Vault secret watcher
4. Error handling and retry logic

### **Phase 3: Advanced Patterns (Week 3-4)**
1. Dependency-based updates and conditional watching  
2. Throttling, rate limiting, and debouncing
3. Update stream filtering and transformation
4. Performance optimization and comprehensive testing

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::TempDir;
  
  #[tokio::test]
  async fn test_file_watcher() 
{
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");
    
    // Write initial config
    tokio::fs::write(&config_file, r#"value = "initial""#).await.unwrap();
    
    let mut watcher = FileWatcher::<TestConfig>::new(&config_file);
    
    // Start watching in background
    let watch_task = tokio::spawn(async move {
      watcher.watch().await
    });
    
    // Update file
    tokio::time::sleep(Duration::from_millis(100)).await;
    tokio::fs::write(&config_file, r#"value = "updated""#).await.unwrap();
    
    // Should detect change
    let result = tokio::time::timeout(Duration::from_secs(5), watch_task).await;
    assert!(result.is_ok());
    
    let config = result.unwrap().unwrap();
    assert_eq!(config.value, "updated");
  }
  
  #[tokio::test]
  async fn test_env_watcher() 
{
    std::env::set_var("TEST_VAR", "initial");
    
    let mut watcher = EnvWatcher::new("TEST_VAR")
      .with_poll_interval(Duration::from_millis(50));
    
    let watch_task = tokio::spawn(async move {
      watcher.watch().await
    });
    
    // Change environment variable
    tokio::time::sleep(Duration::from_millis(100)).await;
    std::env::set_var("TEST_VAR", "updated");
    
    let result = tokio::time::timeout(Duration::from_secs(5), watch_task).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), "updated");
    
    std::env::remove_var("TEST_VAR");
  }
}
```

### **Integration Tests**
```rust
// tests/reactive_integration.rs
#[tokio::test]
async fn test_full_reactive_config() 
{
  #[derive(ReactiveAssign, Clone)]
  struct TestConfig 
{
    #[component(watch_file = "test_config.toml")]
    settings: AppSettings,
    
    #[component(watch_env = "TEST_DATABASE_URL")]
    database_url: String,
  }
  
  // Setup test files and environment
  tokio::fs::write("test_config.toml", r#"
    debug = true
    port = 8080
  "#).await.unwrap();
  
  std::env::set_var("TEST_DATABASE_URL", "postgres://localhost/test");
  
  // Start reactive config
  let config = TestConfig::default();
  let (handle, mut updates) = config.start_watching().await.unwrap();
  
  // Collect initial updates
  let mut received_updates = Vec::new();
  
  // Update file
  tokio::fs::write("test_config.toml", r#"
    debug = false
    port = 9090
  "#).await.unwrap();
  
  // Update environment
  std::env::set_var("TEST_DATABASE_URL", "postgres://localhost/updated");
  
  // Collect updates with timeout
  let collect_task = tokio::spawn(async move {
    let mut updates = Vec::new();
    let mut timeout = tokio::time::interval(Duration::from_secs(1));
    
    loop {
      tokio::select! {
        update = updates.recv() => {
          match update {
            Some(u) => updates.push(u),
            None => break,
          }
        }
        _ = timeout.tick() => {
          if updates.len() >= 2 { // Expect file + env update
            break;
          }
        }
      }
    }
    
    updates
  });
  
  let updates = tokio::time::timeout(Duration::from_secs(10), collect_task)
    .await
    .unwrap()
    .unwrap();
  
  assert!(updates.len() >= 2);
  // Verify updates contain expected changes
  
  handle.stop().await;
  
  // Cleanup
  std::env::remove_var("TEST_DATABASE_URL");
  let _ = std::fs::remove_file("test_config.toml");
}
```

## 📊 **Success Metrics**

- [ ] Support for 5+ reactive data sources (file, env, HTTP, Consul, Vault)
- [ ] Sub-second update latency for file and environment changes
- [ ] Efficient polling with minimal resource usage
- [ ] Proper error handling and recovery from watcher failures
- [ ] Clean shutdown and resource cleanup
- [ ] Comprehensive update filtering and transformation

## 🚧 **Potential Challenges**

1. **Resource Management**: File watchers and polling can be resource-intensive
   - **Solution**: Efficient polling, proper cleanup, resource limits

2. **Error Handling**: Network failures, file permission issues, etc.
   - **Solution**: Comprehensive error types, retry logic, graceful degradation

3. **Update Ordering**: Multiple sources updating simultaneously
   - **Solution**: Update ordering guarantees, dependency resolution

4. **Memory Usage**: Keeping old values for comparison
   - **Solution**: Smart diffing, configurable history limits

## 🔄 **Dependencies**

- **Requires**: 
  - Task 001 (Single Derive Macro) for attribute parsing
  - Task 006 (Async Support) for async watchers
- **Blocks**: None
- **Related**: All configuration tasks benefit from reactive updates

## 📅 **Timeline**

- **Week 1-2**: Core infrastructure and basic watchers
- **Week 2-3**: Advanced watchers and HTTP/Consul integration
- **Week 3-4**: Advanced patterns, optimization, and testing

## 💡 **Future Enhancements**

- **WebSocket Integration**: Real-time updates via WebSocket connections
- **Database Change Streams**: React to database table changes  
- **Message Queue Integration**: Updates via Redis pub/sub, Kafka, etc.
- **Distributed Coordination**: Coordinate updates across multiple instances
- **Update History**: Track and rollback configuration changes
- **Hot Code Reloading**: Update component logic without restart