# Task 005: Universal Extraction Framework

## 🎯 **Objective**

Create a generic, framework-agnostic extraction system that works with any web framework, database, configuration source, or custom data source through a unified component model interface.

## 📋 **Current State**

Manual extraction with framework-specific boilerplate:
```rust
// Different boilerplate for each framework
// Axum
async fn axum_handler(
  Path( user_id ) : Path< u64 >,
  Query( params ) : Query< HashMap< String, String > >,
  headers : HeaderMap,
) -> Result< String, StatusCode > { /* ... */ }

// Actix-web  
async fn actix_handler(
  path : web::Path< u64 >,
  query : web::Query< HashMap< String, String > >,
  req : HttpRequest,
) -> Result< String, ActixError > { /* ... */ }

// Custom framework - completely different API
async fn custom_handler( request : CustomRequest ) -> CustomResponse
{
  let user_id = request.get_path_param( "user_id" )?;
  let page = request.get_query( "page" )?;
  // ... different extraction logic
}
```

## 🎯 **Target State**

Universal extraction that works with any framework:
```rust
#[ derive( Extract ) ]
struct ApiRequest
{
  #[ extract( path ) ]
  user_id : u64,
  
  #[ extract( query ) ]
  page : Option< u32 >,
  
  #[ extract( header = "authorization" ) ]
  auth_token : String,
  
  #[ extract( json ) ]
  body : CreateUserRequest,
  
  #[extract(custom = "extract_user_from_jwt")]
  current_user: User,
}

// Works with ANY framework through adapters
async fn axum_handler(
  Extract(AxumExtractor, request): Extract<AxumExtractor, ApiRequest>
) -> impl IntoResponse { /* ... */ }

async fn actix_handler(
  Extract(ActixExtractor, request): Extract<ActixExtractor, ApiRequest>  
) -> impl Responder { /* ... */ }

async fn custom_handler(
  Extract(MyFrameworkExtractor, request): Extract<MyFrameworkExtractor, ApiRequest>
) -> CustomResponse { /* ... */ }

// Even works with non-web sources
async fn config_handler(
  Extract(ConfigExtractor, settings): Extract<ConfigExtractor, ApiRequest>
) { /* Extract from config files, env vars, etc. */ }
```

## 📝 **Detailed Requirements**

### **Core Generic Traits**

#### **ExtractSource Trait**
```rust
pub trait ExtractSource
{
  type Context;
  type Error : std::error::Error;
  
  fn extract< T >( &self, context : &Self::Context, spec : &ExtractSpec ) -> Result< T, Self::Error >
  where 
    T : FromExtract< Self >;
    
  fn supports_extraction( &self, spec : &ExtractSpec ) -> bool;
}

pub trait FromExtract< E : ExtractSource >
{
  fn from_extract( source : &E, context : &E::Context, spec : &ExtractSpec ) -> Result< Self, E::Error >
  where 
    Self : Sized;
}
```

#### **Generic Extraction Specification**
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractSpec 
{
  pub source_type: SourceType,
  pub key: Option<String>,
  pub default_value: Option<String>,
  pub required: bool,
  pub transform: Option<String>,
  pub condition: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceType 
{
  Path(Option<String>),        // Path parameter by position or name
  Query(Option<String>),       // Query parameter by name or all
  Header(String),              // HTTP header by name
  Body(BodyType),             // Request body in various formats
  Custom(String),             // Custom extraction function
  Environment(String),        // Environment variable
  Config(String),            // Configuration key
  Database(String),          // Database query
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyType 
{
  Json,
  Form,
  Text,
  Bytes,
  Multipart,
}
```

#### **Framework Adapters**

Framework adapters implement `ExtractSource` to bridge the generic system with specific frameworks:

```rust
// Axum adapter
pub struct AxumExtractor;

impl ExtractSource for AxumExtractor 
{
  type Context = (axum::http::request::Parts, Option<axum::extract::State<AppState>>);
  type Error = AxumExtractionError;
  
  fn extract<T>(&self, context: &Self::Context, spec: &ExtractSpec) -> Result<T, Self::Error>
  where 
    T: FromExtract<Self> + std::str::FromStr,
    T::Err: std::fmt::Display,
  {
    let (parts, state) = context;
    
    match &spec.source_type {
      SourceType::Path(key) => {
        // Extract from Axum path parameters
        extract_from_axum_path(parts, key, spec)
      },
      SourceType::Query(key) => {
        // Extract from Axum query parameters  
        extract_from_axum_query(parts, key, spec)
      },
      SourceType::Header(name) => {
        // Extract from HTTP headers
        extract_from_headers(&parts.headers, name, spec)
      },
      SourceType::Custom(func_name) => {
        // Call custom extraction function
        call_custom_extractor(func_name, parts, state, spec)
      },
      _ => Err(AxumExtractionError::UnsupportedSource(spec.source_type.clone())),
    }
  }
  
  fn supports_extraction(&self, spec: &ExtractSpec) -> bool 
{
    matches!(spec.source_type, 
      SourceType::Path(_) | 
      SourceType::Query(_) | 
      SourceType::Header(_) |
      SourceType::Body(_) |
      SourceType::Custom(_)
    )
  }
}

// Actix-web adapter
pub struct ActixExtractor;

impl ExtractSource for ActixExtractor 
{
  type Context = (actix_web::HttpRequest, Option<&mut actix_web::dev::Payload>);
  type Error = ActixExtractionError;
  
  fn extract<T>(&self, context: &Self::Context, spec: &ExtractSpec) -> Result<T, Self::Error>
  where 
    T: FromExtract<Self>,
  {
    let (req, payload) = context;
    
    match &spec.source_type {
      SourceType::Path(key) => {
        // Extract from Actix path parameters using match_info
        extract_from_actix_path(req, key, spec)
      },
      SourceType::Query(key) => {
        // Extract from Actix query string
        extract_from_actix_query(req, key, spec)
      },
      SourceType::Header(name) => {
        // Extract from HTTP headers
        extract_from_actix_headers(req, name, spec)
      },
      _ => Err(ActixExtractionError::UnsupportedSource(spec.source_type.clone())),
    }
  }
}

// Generic config extractor (non-web)
pub struct ConfigExtractor 
{
  config: std::collections::HashMap<String, String>,
}

impl ExtractSource for ConfigExtractor 
{
  type Context = ();
  type Error = ConfigExtractionError;
  
  fn extract<T>(&self, _context: &Self::Context, spec: &ExtractSpec) -> Result<T, Self::Error>
  where 
    T: FromExtract<Self>,
  {
    match &spec.source_type {
      SourceType::Config(key) => {
        if let Some(value) = self.config.get(key) {
          value.parse().map_err(|_| ConfigExtractionError::ParseError)
        } else if let Some(default) = &spec.default_value {
          default.parse().map_err(|_| ConfigExtractionError::ParseError)
        } else if spec.required {
          Err(ConfigExtractionError::MissingRequired(key.clone()))
        } else {
          Err(ConfigExtractionError::MissingOptional)
        }
      },
      SourceType::Environment(var_name) => {
        std::env::var(var_name)
          .map(|v| v.parse())
          .map_err(|_| ConfigExtractionError::MissingEnvironment(var_name.clone()))?
          .map_err(|_| ConfigExtractionError::ParseError)
      },
      _ => Err(ConfigExtractionError::UnsupportedSource),
    }
  }
}
```

### **Universal Usage Patterns**

#### **Basic Extraction**
```rust
#[derive(Extract)]
struct ApiRequest 
{
  #[extract(path)]                    // Extract first path parameter
  user_id: u64,
  
  #[extract(query = "page")]          // Extract specific query parameter
  page: Option<u32>,
  
  #[extract(header = "authorization")] // Extract HTTP header
  auth_token: String,
  
  #[extract(json)]                    // Extract JSON body
  body: CreateUserRequest,
}
```

#### **Cross-Platform Extraction**
```rust
#[derive(Extract)]
struct UniversalConfig 
{
  #[extract(config = "database.url")]     // From config files
  database_url: String,
  
  #[extract(environment = "API_KEY")]     // From environment variables  
  api_key: String,
  
  #[extract(query = "override")]          // From web requests
  config_override: Option<String>,
  
  #[extract(custom = "get_user_preferences")] // Custom logic
  user_prefs: UserPreferences,
}

// Works with web frameworks
async fn web_handler(
  Extract(AxumExtractor, config): Extract<AxumExtractor, UniversalConfig>
) -> impl IntoResponse { /* ... */ }

// Works with config systems  
fn load_app_config(
  Extract(ConfigExtractor::from_file("app.toml"), config): Extract<ConfigExtractor, UniversalConfig>
) { /* ... */ }
```

### **Advanced Features**

#### **Custom Extractors**
```rust
#[derive(Extract)]
struct AdvancedRequest 
{
  #[extract(custom = "extract_bearer_token")]
  token: BearerToken,
  
  #[extract(custom = "extract_client_ip")]
  client_ip: IpAddr,
  
  #[extract(custom = "extract_user_from_jwt")]
  current_user: User,
}

// Custom extractor functions are framework-agnostic
fn extract_bearer_token<E: ExtractSource>(
  source: &E, 
  context: &E::Context, 
  _spec: &ExtractSpec
) -> Result<BearerToken, E::Error> {
  // Generic bearer token extraction logic
  // Works with any framework that provides headers
}

fn extract_user_from_jwt<E: ExtractSource>(
  source: &E,
  context: &E::Context,
  _spec: &ExtractSpec
) -> Result<User, E::Error> {
  // Extract JWT from authorization header, decode, return user
  // Same logic works across all frameworks
}
```

#### **Conditional and Contextual Extraction**
```rust
#[derive(Extract)]
struct ConditionalRequest 
{
  #[extract(header = "authorization")]
  auth: Option<String>,
  
  #[extract(query = "admin_param", condition = "auth.is_some()")]
  admin_param: Option<String>,
  
  #[extract(environment = "DEBUG_MODE", default = "false")]
  debug_enabled: bool,
  
  #[extract(config = "feature_flags", transform = "parse_feature_flags")]
  features: Vec<FeatureFlag>,
}
```

#### **Nested and Composite Extraction** 
```rust
#[derive(Extract)]
struct CompositeRequest 
{
  #[extract(nested)]
  auth_info: AuthInfo,
  
  #[extract(nested)]  
  request_metadata: RequestMetadata,
  
  #[extract(json)]
  payload: BusinessData,
}

#[derive(Extract)]
struct AuthInfo 
{
  #[extract(header = "authorization")]
  token: String,
  
  #[extract(custom = "extract_user_permissions")]
  permissions: UserPermissions,
}

#[derive(Extract)]  
struct RequestMetadata 
{
  #[extract(header = "user-agent")]
  user_agent: String,
  
  #[extract(custom = "extract_request_id")]
  request_id: Uuid,
  
  #[extract(query = "trace")]
  trace_enabled: Option<bool>,
}
```

### **Derive Implementation**

#### **Generated Extract Implementation**
```rust
#[derive(Extract)]
struct ApiRequest 
{
  #[extract(path)]
  user_id: u64,
  
  #[extract(query = "page")]  
  page: Option<u32>,
}

// Generates:
impl<E: ExtractSource> FromExtract<E> for ApiRequest {
  fn from_extract(
    source: &E, 
    context: &E::Context, 
    _spec: &ExtractSpec
  ) -> Result<Self, E::Error> {
    let mut request = Self {
      user_id: 0,
      page: None,
    };
    
    // Extract user_id from path
    let user_id_spec = ExtractSpec {
      source_type: SourceType::Path(None),
      key: None,
      default_value: None,
      required: true,
      transform: None,
      condition: None,
    };
    request.assign(source.extract::<u64>(context, &user_id_spec)?);
    
    // Extract page from query
    let page_spec = ExtractSpec {
      source_type: SourceType::Query(Some("page".to_string())),
      key: Some("page".to_string()),
      default_value: None,
      required: false,
      transform: None,
      condition: None,
    };
    
    if let Ok(page_val) = source.extract::<u32>(context, &page_spec) {
      request.assign(Some(page_val));
    }
    
    Ok(request)
  }
}

// Generic extraction wrapper for any framework
pub struct Extract<E: ExtractSource, T: FromExtract<E>>(pub E, pub T);

// Framework-specific implementations
#[axum::async_trait]
impl<S, T> axum::extract::FromRequestParts<S> for Extract<AxumExtractor, T>
where
  S: Send + Sync,
  T: FromExtract<AxumExtractor> + Send,
{
  type Rejection = T::Error;

  async fn from_request_parts(
    parts: &mut axum::http::request::Parts,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    let extractor = AxumExtractor;
    let context = (parts.clone(), Some(axum::extract::State(state)));
    let extracted = T::from_extract(&extractor, &context, &ExtractSpec::default())?;
    
    Ok(Extract(extractor, extracted))
  }
}
```

## 🗂️ **File Changes**

### **New Files**
- `component_model_extract/` - New crate for universal extraction
- `component_model_extract/src/lib.rs` - Core extraction traits and types
- `component_model_extract/src/extract_derive.rs` - Extract derive implementation
- `component_model_extract/src/spec.rs` - ExtractSpec and SourceType definitions
- `component_model_extract/src/adapters/` - Framework adapter implementations
- `component_model_extract/src/adapters/axum.rs` - Axum ExtractSource adapter
- `component_model_extract/src/adapters/actix.rs` - Actix-web adapter
- `component_model_extract/src/adapters/warp.rs` - Warp adapter
- `component_model_extract/src/adapters/config.rs` - Configuration file adapter
- `component_model_extract/src/adapters/database.rs` - Database query adapter
- `component_model_extract/src/errors.rs` - Universal error types
- `component_model_extract/src/custom.rs` - Custom extractor utilities
- `examples/universal_extract_example.rs` - Cross-platform extraction examples
- `examples/web_framework_examples/` - Specific framework examples

### **Modified Files**
- `Cargo.toml` - Add new workspace member
- `component_model/Cargo.toml` - Add extract dependency (feature-gated)

## ⚡ **Implementation Steps**

### **Phase 1: Core Generic System (Week 1-2)**
1. Create `component_model_extract` crate with generic traits
2. Implement `ExtractSource`, `FromExtract`, and `ExtractSpec` 
3. Create basic `Extract` derive macro with attribute parsing
4. Implement simple Axum adapter as proof of concept
5. Basic testing infrastructure for generic system

### **Phase 2: Multiple Framework Adapters (Week 2-3)**
1. Implement Actix-web and Warp adapters
2. Add non-web adapters (Config, Environment, Database)
3. Create custom extractor function support
4. Cross-adapter compatibility testing

### **Phase 3: Advanced Universal Features (Week 3-4)**
1. Implement conditional and nested extraction
2. Add transformation and validation hooks
3. Performance optimization across all adapters
4. Comprehensive documentation and examples
5. Framework-specific integration helpers

## 🧪 **Testing Strategy**

### **Generic Trait Tests**
```rust
#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_generic_extraction() 
{
    #[derive(Extract, Debug, PartialEq)]
    struct TestRequest 
{
      #[extract(config = "app.name")]
      name: String,
      
      #[extract(environment = "PORT")]  
      port: Option<u16>,
    }
    
    let config = ConfigExtractor::from_map([
      ("app.name", "test-app"),
    ]);
    
    std::env::set_var("PORT", "8080");
    
    let result = TestRequest::from_extract(&config, &(), &ExtractSpec::default());
    assert!(result.is_ok());
    
    let request = result.unwrap();
    assert_eq!(request.name, "test-app");
    assert_eq!(request.port, Some(8080));
  }
  
  #[test]
  fn test_custom_extractor() 
{
    #[derive(Extract)]
    struct TestRequest 
{
      #[extract(custom = "extract_test_value")]
      value: TestValue,
    }
    
    fn extract_test_value<E: ExtractSource>(
      _source: &E,
      _context: &E::Context,
      _spec: &ExtractSpec
    ) -> Result<TestValue, E::Error> {
      Ok(TestValue { data: "custom".to_string() })
    }
    
    // Test works with any ExtractSource implementation
  }
  
  #[test] 
  fn test_conditional_extraction() 
{
    #[derive(Extract)]
    struct TestRequest 
{
      #[extract(config = "debug")]
      debug: bool,
      
      #[extract(config = "debug_level", condition = "debug")]  
      debug_level: Option<String>,
    }
    
    // Test conditional logic
  }
}

### **Cross-Framework Integration Tests**
```rust
// tests/universal_integration.rs
use axum::{routing::get, Router};
use actix_web::{web, App, HttpServer};
use tower::ServiceExt;

#[derive(Extract, Clone)]
struct UniversalRequest {
  #[extract(path)]
  user_id: u64,
  
  #[extract(query = "page")]
  page: Option<u32>,
  
  #[extract(header = "authorization")]
  auth: Option<String>,
}

// Same struct works with Axum
async fn axum_handler(
  Extract(AxumExtractor, request): Extract<AxumExtractor, UniversalRequest>
) -> String {
  format!("Axum - User: {}, Page: {:?}", request.user_id, request.page)
}

// And with Actix-web
async fn actix_handler(
  Extract(ActixExtractor, request): Extract<ActixExtractor, UniversalRequest>
) -> String {
  format!("Actix - User: {}, Page: {:?}", request.user_id, request.page)
}

// And with config files
fn config_handler(
  Extract(ConfigExtractor::from_file("test.toml"), config): Extract<ConfigExtractor, UniversalRequest>
) {
  println!("Config - User: {}", config.user_id);
}

#[tokio::test]
async fn test_axum_integration() {
  let app = Router::new().route("/users/:user_id", get(axum_handler));
  
  let response = app
    .oneshot(
      axum::http::Request::builder()
        .uri("/users/123?page=5")
        .body(axum::body::Body::empty())
        .unwrap()
    )
    .await
    .unwrap();
    
  let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
  assert_eq!(&body[..], b"Axum - User: 123, Page: Some(5)");
}

#[tokio::test] 
async fn test_actix_integration() {
  // Similar test but with Actix-web setup
  // Same extraction struct, different framework
}

#[test]
fn test_config_integration() {
  // Test the same struct works with config extraction
  let config_data = r#"
    user_id = 456
    page = 2
  "#;
  
  let config = ConfigExtractor::from_toml(config_data);
  let result = UniversalRequest::from_extract(&config, &(), &ExtractSpec::default()).unwrap();
  
  assert_eq!(result.user_id, 456);
  assert_eq!(result.page, Some(2));
}
```

## 📊 **Success Metrics**

- [ ] **Universal Compatibility**: Works with ANY framework through adapter pattern
- [ ] **Framework Agnostic**: Same extraction struct works across web, config, database sources
- [ ] **Extensible**: Easy to add new frameworks/sources without changing core system  
- [ ] **Zero Lock-in**: Not tied to specific framework versions or implementations
- [ ] **95% Boilerplate Reduction**: Minimal extraction code needed
- [ ] **Type Safety**: Compile-time validation of extraction specifications
- [ ] **Performance**: Zero-cost abstractions, optimal generated code

## 🚧 **Potential Challenges**

1. **Generic Complexity**: Complex trait bounds and generic constraints
   - **Solution**: Incremental implementation, clear trait design, extensive testing

2. **Framework Integration**: Each framework has unique request/context types
   - **Solution**: Adapter pattern isolates framework-specific logic

3. **Error Handling**: Unified error reporting across different source types
   - **Solution**: Hierarchical error types with source-specific context

4. **Performance**: Additional abstraction layer overhead
   - **Solution**: Generate optimal code per adapter, benchmark extensively

5. **Ecosystem Adoption**: Convincing framework authors to integrate adapters
   - **Solution**: Make adapters external, show clear benefits, provide migration guides

## 🔄 **Dependencies**

- **Requires**: 
  - Task 001 (Single Derive Macro) for attribute parsing infrastructure
  - Task 003 (Validation) for extraction validation hooks
- **Blocks**: None
- **Related**: 
  - Benefits from Task 002 (Popular Types) for automatic type conversions
  - Synergy with Task 004 (Config Support) for non-web sources
  - Works with Task 006 (Async Support) for async extraction

## 📅 **Timeline**

- **Week 1-2**: Core generic traits and basic Axum adapter
- **Week 2-3**: Multiple framework adapters and non-web sources
- **Week 3-4**: Advanced features, optimization, and comprehensive testing

## 💡 **Future Enhancements**

- **Automatic Adapter Generation**: Generate adapters from framework trait definitions
- **OpenAPI Integration**: Generate API specs from extraction structs universally
- **GraphQL Support**: Extract from any GraphQL server implementation
- **Protocol Buffers**: Extract from protobuf messages and gRPC contexts
- **Message Queues**: Extract from Kafka, RabbitMQ, Redis streams
- **IoT Protocols**: Extract from MQTT, CoAP, LoRaWAN messages
- **Blockchain Integration**: Extract from smart contract calls and transactions