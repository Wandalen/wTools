# Task 006: Async/Concurrent Support

## 🎯 **Objective**

Extend component model with async capabilities for fetching components from external sources like databases, APIs, configuration servers, and other async operations.

## 📋 **Current State**

All component assignment is synchronous:
```rust
let config = AppConfig::default()
  .impute("localhost")
  .impute(8080)
  .impute("production");
```

## 🎯 **Target State**

Async component resolution and assignment:
```rust
#[derive(AsyncAssign)]
struct AppConfig {
  #[component(fetch_from = "database")]
  database_url: String,
  
  #[component(fetch_from = "consul", key = "app/port")]
  port: u16,
  
  #[component(fetch_from = "vault", secret = "app/api-key")]
  api_key: String,
  
  #[component(fetch_from = "redis", ttl = "3600")]
  cached_config: CachedSettings,
}

// Async component resolution
let config = AppConfig::default()
  .async_assign(fetch_database_url().await)
  .async_assign(load_api_key_from_vault().await)
  .async_assign(get_cached_settings().await)
  .build()
  .await?;

// Or fetch all components concurrently
let config = AppConfig::fetch_all_components().await?;
```

## 📝 **Detailed Requirements**

### **Core Async Traits**

#### **AsyncAssign Trait**
```rust
#[async_trait]
pub trait AsyncAssign<T, IntoT> {
  type Error;
  
  async fn async_assign(&mut self, component: IntoT) -> Result<(), Self::Error>;
  async fn async_impute(self, component: IntoT) -> Result<Self, Self::Error>
  where 
    Self: Sized;
}

// Future-based version for better composability
pub trait FutureAssign<T, IntoT> {
  type Future: Future<Output = Result<(), Self::Error>>;
  type Error;
  
  fn future_assign(&mut self, component: IntoT) -> Self::Future;
  fn future_impute(self, component: IntoT) -> impl Future<Output = Result<Self, Self::Error>>
  where 
    Self: Sized;
}
```

#### **ComponentFetcher Trait**
```rust
#[async_trait]
pub trait ComponentFetcher<T> {
  type Error;
  
  async fn fetch_component(&self) -> Result<T, Self::Error>;
}

// Built-in fetchers
pub struct DatabaseFetcher {
  query: String,
  connection: DatabaseConnection,
}

pub struct ConsulFetcher {
  key: String,
  client: ConsulClient,
}

pub struct VaultFetcher {
  secret_path: String,
  client: VaultClient,
}
```

### **Async Derive Implementation**

#### **AsyncAssign Derive**
```rust
#[derive(AsyncAssign)]
struct AppConfig {
  #[component(fetch_from = "database", query = "SELECT value FROM config WHERE key = 'db_url'")]
  database_url: String,
  
  #[component(fetch_from = "env", fallback = "localhost")]
  host: String,
  
  #[component(fetch_from = "consul", key = "app/port")]
  port: u16,
}

// Generates:
impl AsyncAssign<String, DatabaseFetcher> for AppConfig {
  type Error = ComponentError;
  
  async fn async_assign(&mut self, fetcher: DatabaseFetcher) -> Result<(), Self::Error> {
    let value = fetcher.fetch_component().await?;
    self.database_url = value;
    Ok(())
  }
}

impl AppConfig {
  // Fetch all components concurrently
  async fn fetch_all_components() -> Result<Self, Vec<ComponentError>> {
    let mut config = Self::default();
    let mut errors = Vec::new();
    
    // Create all fetchers
    let db_fetcher = DatabaseFetcher::new("SELECT value FROM config WHERE key = 'db_url'");
    let consul_fetcher = ConsulFetcher::new("app/port");
    
    // Fetch concurrently
    let (db_result, consul_result) = tokio::join!(
      db_fetcher.fetch_component(),
      consul_fetcher.fetch_component()
    );
    
    // Assign results
    match db_result {
      Ok(url) => config.assign(url),
      Err(e) => errors.push(e.into()),
    }
    
    match consul_result {
      Ok(port) => config.assign(port),
      Err(e) => errors.push(e.into()),
    }
    
    if errors.is_empty() {
      Ok(config)
    } else {
      Err(errors)
    }
  }
  
  // Fetch with retry and timeout
  async fn fetch_with_resilience() -> Result<Self, ComponentError> {
    use tokio::time::{timeout, Duration};
    
    timeout(Duration::from_secs(30), Self::fetch_all_components())
      .await
      .map_err(|_| ComponentError::Timeout)?
      .map_err(ComponentError::Multiple)
  }
}
```

### **Built-in Async Fetchers**

#### **Database Fetcher**
```rust
pub struct DatabaseFetcher {
  pool: sqlx::PgPool,
  query: String,
}

impl DatabaseFetcher {
  pub fn new(pool: sqlx::PgPool, query: impl Into<String>) -> Self {
    Self {
      pool,
      query: query.into(),
    }
  }
  
  pub fn from_url(url: &str, query: impl Into<String>) -> Result<Self, sqlx::Error> {
    let pool = sqlx::PgPool::connect(url).await?;
    Ok(Self::new(pool, query))
  }
}

#[async_trait]
impl<T> ComponentFetcher<T> for DatabaseFetcher 
where
  T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
  type Error = sqlx::Error;
  
  async fn fetch_component(&self) -> Result<T, Self::Error> {
    sqlx::query_as(&self.query)
      .fetch_one(&self.pool)
      .await
  }
}
```

#### **HTTP API Fetcher**
```rust
pub struct ApiFetcher {
  client: reqwest::Client,
  url: String,
  headers: HeaderMap,
}

impl ApiFetcher {
  pub fn new(url: impl Into<String>) -> Self {
    Self {
      client: reqwest::Client::new(),
      url: url.into(),
      headers: HeaderMap::new(),
    }
  }
  
  pub fn with_auth_header(mut self, token: &str) -> Self {
    self.headers.insert(
      "Authorization",
      format!("Bearer {}", token).parse().unwrap()
    );
    self
  }
}

#[async_trait]
impl<T> ComponentFetcher<T> for ApiFetcher
where
  T: serde::de::DeserializeOwned + Send,
{
  type Error = reqwest::Error;
  
  async fn fetch_component(&self) -> Result<T, Self::Error> {
    self.client
      .get(&self.url)
      .headers(self.headers.clone())
      .send()
      .await?
      .json::<T>()
      .await
  }
}
```

#### **Configuration Service Fetchers**
```rust
// Consul KV fetcher
pub struct ConsulFetcher {
  client: consul::Client,
  key: String,
}

#[async_trait]
impl ComponentFetcher<String> for ConsulFetcher {
  type Error = consul::Error;
  
  async fn fetch_component(&self) -> Result<String, Self::Error> {
    self.client.get_kv(&self.key).await
  }
}

// Vault secret fetcher  
pub struct VaultFetcher {
  client: vault::Client,
  secret_path: String,
  field: Option<String>,
}

#[async_trait]
impl<T> ComponentFetcher<T> for VaultFetcher
where
  T: serde::de::DeserializeOwned,
{
  type Error = vault::Error;
  
  async fn fetch_component(&self) -> Result<T, Self::Error> {
    let secret = self.client.read_secret(&self.secret_path).await?;
    
    if let Some(field) = &self.field {
      serde_json::from_value(secret.data[field].clone())
        .map_err(|e| vault::Error::Json(e))
    } else {
      serde_json::from_value(serde_json::to_value(secret.data)?)
        .map_err(|e| vault::Error::Json(e))
    }
  }
}
```

### **Advanced Async Patterns**

#### **Streaming Components**
```rust
#[derive(AsyncAssign)]
struct StreamingConfig {
  #[component(stream_from = "kafka", topic = "config-updates")]
  live_settings: Settings,
  
  #[component(stream_from = "websocket", url = "ws://config.service")]
  realtime_flags: FeatureFlags,
}

impl StreamingConfig {
  async fn watch_for_updates(&mut self) -> impl Stream<Item = ConfigUpdate> {
    // Return stream of configuration updates
  }
}
```

#### **Cached Async Components**
```rust
#[derive(AsyncAssign)]
struct CachedConfig {
  #[component(
    fetch_from = "api", 
    cache_for = "3600",  // Cache for 1 hour
    fallback = "default_value"
  )]
  expensive_setting: ExpensiveData,
}

// Generates caching logic
impl CachedConfig {
  async fn fetch_with_cache() -> Result<Self, ComponentError> {
    // Check cache first, fetch if expired, update cache
  }
}
```

#### **Retry and Circuit Breaker**
```rust
#[derive(AsyncAssign)]
struct ResilientConfig {
  #[component(
    fetch_from = "remote_api",
    retry_attempts = "3",
    circuit_breaker = "true",
    fallback_to = "local_cache"
  )]
  critical_setting: CriticalData,
}
```

## 🗂️ **File Changes**

### **New Files**
- `component_model_async/` - New crate for async support
- `component_model_async/src/lib.rs` - Main async API
- `component_model_async/src/async_derive.rs` - AsyncAssign derive
- `component_model_async/src/fetchers/` - Built-in fetchers
- `component_model_async/src/fetchers/database.rs` - Database fetchers
- `component_model_async/src/fetchers/http.rs` - HTTP API fetchers
- `component_model_async/src/fetchers/consul.rs` - Consul integration
- `component_model_async/src/fetchers/vault.rs` - Vault integration
- `component_model_async/src/cache.rs` - Caching support
- `component_model_async/src/resilience.rs` - Retry/circuit breaker
- `examples/async_config_example.rs` - Async configuration examples

### **Modified Files**
- `Cargo.toml` - Add new workspace member
- `component_model/Cargo.toml` - Add async dependency (feature-gated)

## ⚡ **Implementation Steps**

### **Phase 1: Core Async Traits (Week 1)**
1. Define `AsyncAssign` and `ComponentFetcher` traits
2. Create basic `AsyncAssign` derive macro
3. Implement simple async assignment patterns
4. Basic testing infrastructure

### **Phase 2: Built-in Fetchers (Week 2)**
1. Implement database fetcher with sqlx
2. Add HTTP API fetcher with reqwest
3. Create environment variable fetcher
4. Basic error handling and resilience

### **Phase 3: Advanced Features (Week 3-4)**
1. Add Consul and Vault fetchers
2. Implement caching layer
3. Add retry logic and circuit breakers
4. Streaming/watch capabilities
5. Comprehensive testing and documentation

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
  use super::*;
  
  #[tokio::test]
  async fn test_async_assignment() {
    #[derive(AsyncAssign, Default)]
    struct TestConfig {
      value: String,
    }
    
    let mut config = TestConfig::default();
    config.async_assign("test_value").await.unwrap();
    
    assert_eq!(config.value, "test_value");
  }
  
  #[tokio::test]
  async fn test_concurrent_fetching() {
    #[derive(AsyncAssign)]
    struct TestConfig {
      #[component(fetch_from = "mock_api")]
      api_value: String,
      
      #[component(fetch_from = "mock_db")]
      db_value: i32,
    }
    
    // Mock fetchers return predictable values
    let config = TestConfig::fetch_all_components().await.unwrap();
    
    assert_eq!(config.api_value, "api_result");
    assert_eq!(config.db_value, 42);
  }
}
```

### **Integration Tests**
```rust
// tests/async_integration.rs
#[tokio::test]
async fn test_database_fetcher() {
  // Setup test database
  let pool = sqlx::PgPool::connect("postgresql://test:test@localhost/test")
    .await
    .unwrap();
    
  sqlx::query("INSERT INTO config (key, value) VALUES ('test_key', 'test_value')")
    .execute(&pool)
    .await
    .unwrap();
  
  let fetcher = DatabaseFetcher::new(pool, "SELECT value FROM config WHERE key = 'test_key'");
  let result: String = fetcher.fetch_component().await.unwrap();
  
  assert_eq!(result, "test_value");
}

#[tokio::test]
async fn test_api_fetcher() {
  use wiremock::{Mock, MockServer, ResponseTemplate};
  
  let mock_server = MockServer::start().await;
  Mock::given(wiremock::matchers::method("GET"))
    .and(wiremock::matchers::path("/config"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
      "setting": "value"
    })))
    .mount(&mock_server)
    .await;
    
  let fetcher = ApiFetcher::new(format!("{}/config", mock_server.uri()));
  let result: serde_json::Value = fetcher.fetch_component().await.unwrap();
  
  assert_eq!(result["setting"], "value");
}
```

## 📊 **Success Metrics**

- [ ] Support for 5+ async data sources
- [ ] Concurrent component fetching with proper error handling
- [ ] Built-in caching and retry mechanisms
- [ ] Zero runtime overhead when async features not used
- [ ] Comprehensive error reporting and fallback strategies

## 🚧 **Potential Challenges**

1. **Error Handling Complexity**: Multiple async operations can fail
   - **Solution**: Structured error types with context and partial success handling

2. **Performance**: Async overhead and coordination costs
   - **Solution**: Benchmarking, optimization, and concurrent fetching

3. **Testing**: Async code is harder to test reliably
   - **Solution**: Mock services, deterministic testing, timeout handling

4. **Dependency Management**: Many optional async dependencies
   - **Solution**: Feature flags and careful dependency organization

## 🔄 **Dependencies**

- **Requires**: 
  - Task 001 (Single Derive Macro) for attribute parsing
  - Task 003 (Validation) for async validation
- **Blocks**: None
- **Related**: Task 004 (Config Support) benefits from async config loading

## 📅 **Timeline**

- **Week 1**: Core async traits and basic derive
- **Week 2**: Built-in fetchers (DB, HTTP, env)  
- **Week 3**: Advanced fetchers (Consul, Vault)
- **Week 4**: Caching, resilience, and streaming features

## 💡 **Future Enhancements**

- **Event-Driven Updates**: Components that update based on external events
- **Dependency Resolution**: Components that depend on other async components
- **Async Validation**: Validation that requires async operations (DB uniqueness checks)
- **Distributed Configuration**: Multi-node configuration synchronization
- **Configuration Versioning**: Track and rollback configuration changes