# Task 005: Web Framework Integration

## 🎯 **Objective**

Create specialized derives for seamless integration with popular Rust web frameworks (Axum, Actix-web, Warp) that automatically extract components from HTTP requests into structured data.

## 📋 **Current State**

Manual request extraction with lots of boilerplate:
```rust
// Axum - manual extraction
async fn handler(
  Path(user_id): Path<u64>,
  Query(params): Query<HashMap<String, String>>,
  headers: HeaderMap,
) -> Result<String, StatusCode> {
  let auth = headers.get("authorization")
    .ok_or(StatusCode::UNAUTHORIZED)?;
  
  let page = params.get("page")
    .and_then(|p| p.parse().ok())
    .unwrap_or(1);
    
  // ... manual handling
}
```

## 🎯 **Target State**

Automatic extraction with component model:
```rust
#[derive(WebExtract)]
struct ApiRequest {
  #[extract(path)]
  user_id: u64,
  
  #[extract(query)]
  page: Option<u32>,
  
  #[extract(header = "authorization")]
  auth_token: String,
  
  #[extract(json)]
  body: CreateUserRequest,
}

// Usage - extraction happens automatically
async fn handler(request: ApiRequest) -> impl IntoResponse {
  format!(
    "User {}, Page {}, Auth: {}", 
    request.user_id, 
    request.page.unwrap_or(1),
    request.auth_token
  )
}
```

## 📝 **Detailed Requirements**

### **Framework Support Matrix**

| Framework | Extract From | Status |
|-----------|--------------|---------|
| **Axum** | Path, Query, Headers, JSON, Form | Phase 1 |
| **Actix-web** | Path, Query, Headers, JSON, Form | Phase 2 |
| **Warp** | Path, Query, Headers, JSON | Phase 3 |

### **Extraction Types**

#### **Path Parameters**
```rust
#[derive(WebExtract)]
struct UserRequest {
  #[extract(path)]           // Extracts first path param
  user_id: u64,
  
  #[extract(path = "org_id")] // Extracts named path param
  organization_id: u64,
}

// Route: /users/{user_id}/orgs/{org_id}
```

#### **Query Parameters**
```rust
#[derive(WebExtract)]
struct SearchRequest {
  #[extract(query)]                    // Extracts "q" query param
  q: Option<String>,
  
  #[extract(query = "page")]           // Extracts "page" query param
  page: Option<u32>,
  
  #[extract(query = "limit", default = "20")]  // With default
  limit: u32,
  
  #[extract(query_all)]                // All query params as HashMap
  filters: HashMap<String, String>,
}

// URL: /search?q=rust&page=2&category=web&sort=date
```

#### **Header Extraction**
```rust
#[derive(WebExtract)]
struct AuthenticatedRequest {
  #[extract(header = "authorization")]
  auth_token: String,
  
  #[extract(header = "content-type")]
  content_type: Option<String>,
  
  #[extract(header = "user-agent", default = "unknown")]
  user_agent: String,
}
```

#### **Body Extraction**
```rust
#[derive(WebExtract)]
struct CreateUserRequest {
  #[extract(json)]                     // Extract JSON body
  user_data: UserData,
  
  #[extract(form)]                     // Extract form data
  form_data: FormData,
  
  #[extract(bytes)]                    // Raw bytes
  raw_body: Vec<u8>,
  
  #[extract(text)]                     // Text body
  text_content: String,
}
```

### **Axum Integration**

#### **Generated Implementation**
```rust
#[derive(WebExtract)]
struct ApiRequest {
  #[extract(path)]
  user_id: u64,
  
  #[extract(query)]
  page: Option<u32>,
}

// Generates:
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for ApiRequest
where
  S: Send + Sync,
{
  type Rejection = ApiRequestRejection;

  async fn from_request_parts(
    parts: &mut axum::http::request::Parts,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    let mut request = Self::default();
    
    // Extract path parameters
    let path = axum::extract::Path::<u64>::from_request_parts(parts, state).await
      .map_err(ApiRequestRejection::PathError)?;
    request.assign(path.0);
    
    // Extract query parameters
    if let Ok(query) = axum::extract::Query::<HashMap<String, String>>::from_request_parts(parts, state).await {
      if let Some(page_str) = query.get("page") {
        if let Ok(page) = page_str.parse::<u32>() {
          request.assign(Some(page));
        }
      }
    }
    
    Ok(request)
  }
}

#[derive(Debug)]
pub enum ApiRequestRejection {
  PathError(axum::extract::rejection::PathRejection),
  QueryError(axum::extract::rejection::QueryRejection),
  HeaderError(String),
  JsonError(axum::extract::rejection::JsonRejection),
}

impl axum::response::IntoResponse for ApiRequestRejection {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::PathError(err) => err.into_response(),
      Self::QueryError(err) => err.into_response(),
      Self::HeaderError(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
      Self::JsonError(err) => err.into_response(),
    }
  }
}
```

### **Actix-web Integration**

#### **Generated Implementation**
```rust
impl actix_web::FromRequest for ApiRequest {
  type Error = ApiRequestError;
  type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(
    req: &actix_web::HttpRequest,
    payload: &mut actix_web::dev::Payload,
  ) -> Self::Future {
    let req = req.clone();
    let mut payload = payload.take();
    
    Box::pin(async move {
      let mut request = Self::default();
      
      // Extract path parameters
      let user_id: u64 = req.match_info().get("user_id")
        .ok_or(ApiRequestError::MissingPathParam("user_id"))?
        .parse()
        .map_err(ApiRequestError::InvalidPathParam)?;
      request.assign(user_id);
      
      // Extract query parameters
      let query = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(ApiRequestError::QueryError)?;
      
      if let Some(page_str) = query.get("page") {
        if let Ok(page) = page_str.parse::<u32>() {
          request.assign(Some(page));
        }
      }
      
      Ok(request)
    })
  }
}
```

### **Advanced Features**

#### **Custom Extractors**
```rust
#[derive(WebExtract)]
struct AdvancedRequest {
  #[extract(custom = "extract_bearer_token")]
  token: BearerToken,
  
  #[extract(custom = "extract_client_ip")]
  client_ip: IpAddr,
}

fn extract_bearer_token(req: &HttpRequest) -> Result<BearerToken, ExtractionError> {
  // Custom extraction logic
}
```

#### **Conditional Extraction**
```rust
#[derive(WebExtract)]
struct ConditionalRequest {
  #[extract(header = "authorization")]
  auth: Option<String>,
  
  #[extract(query, required_if = "auth.is_some()")]
  secure_param: Option<String>,
}
```

#### **Nested Extraction**
```rust
#[derive(WebExtract)]
struct NestedRequest {
  #[extract(json)]
  metadata: RequestMetadata,
  
  #[extract(nested)]
  auth_info: AuthInfo,
}

#[derive(WebExtract)]
struct AuthInfo {
  #[extract(header = "authorization")]
  token: String,
  
  #[extract(header = "x-api-key")]
  api_key: Option<String>,
}
```

## 🗂️ **File Changes**

### **New Files**
- `component_model_web/` - New crate for web framework integration
- `component_model_web/src/lib.rs` - Main web extraction API
- `component_model_web/src/extract_derive.rs` - WebExtract derive implementation
- `component_model_web/src/axum.rs` - Axum-specific implementations
- `component_model_web/src/actix.rs` - Actix-web implementations  
- `component_model_web/src/warp.rs` - Warp implementations
- `component_model_web/src/errors.rs` - Error types and handling
- `examples/web_extract_example.rs` - Web framework examples

### **Modified Files**
- `Cargo.toml` - Add new workspace member
- `component_model/Cargo.toml` - Add web dependency (feature-gated)

## ⚡ **Implementation Steps**

### **Phase 1: Axum Integration (Week 1-2)**
1. Create `component_model_web` crate with Axum focus
2. Implement `WebExtract` derive macro
3. Add path, query, and header extraction
4. Create comprehensive error handling
5. Basic testing and examples

### **Phase 2: Actix-web Integration (Week 2-3)**
1. Add Actix-web support to existing derive
2. Implement Actix-specific extraction patterns
3. Handle Actix's unique features (middleware integration)
4. Cross-framework testing

### **Phase 3: Advanced Features (Week 3-4)**
1. Add Warp support
2. Implement custom extractors
3. Add nested and conditional extraction
4. Performance optimization and benchmarking

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
  use super::*;
  use axum::http::{HeaderMap, StatusCode};
  
  #[test]
  fn test_path_extraction() {
    #[derive(WebExtract, Debug, PartialEq)]
    struct TestRequest {
      #[extract(path)]
      id: u64,
    }
    
    // Mock Axum request parts
    let mut parts = axum::http::request::Parts::default();
    // ... setup mock data
    
    let result = TestRequest::from_request_parts(&mut parts, &()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, 123);
  }
  
  #[test]
  fn test_query_extraction() {
    #[derive(WebExtract)]
    struct TestRequest {
      #[extract(query)]
      page: Option<u32>,
    }
    
    // Test with query parameter
    // ... setup and test
    
    // Test without query parameter
    // ... setup and test
  }
}
```

### **Integration Tests**
```rust
// tests/axum_integration.rs
use axum::{extract::Path, routing::get, Router};
use tower::ServiceExt;

#[tokio::test]
async fn test_axum_integration() {
  #[derive(WebExtract)]
  struct UserRequest {
    #[extract(path)]
    user_id: u64,
    
    #[extract(query)]
    include_posts: Option<bool>,
  }
  
  async fn handler(request: UserRequest) -> String {
    format!("User: {}, Posts: {}", 
      request.user_id, 
      request.include_posts.unwrap_or(false)
    )
  }
  
  let app = Router::new().route("/users/:user_id", get(handler));
  
  let response = app
    .oneshot(
      axum::http::Request::builder()
        .uri("/users/123?include_posts=true")
        .body(axum::body::Body::empty())
        .unwrap()
    )
    .await
    .unwrap();
    
  assert_eq!(response.status(), StatusCode::OK);
  
  let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
  assert_eq!(&body[..], b"User: 123, Posts: true");
}
```

## 📊 **Success Metrics**

- [ ] Support for 3+ major web frameworks
- [ ] 90% reduction in extraction boilerplate
- [ ] Clear, framework-specific error messages
- [ ] Zero performance overhead vs manual extraction
- [ ] Comprehensive documentation and examples

## 🚧 **Potential Challenges**

1. **Framework Differences**: Each framework has different extraction APIs
   - **Solution**: Abstract common patterns, framework-specific implementations

2. **Error Handling**: Unified errors across different frameworks
   - **Solution**: Framework-agnostic error types with conversion traits

3. **Performance**: Additional abstraction layers
   - **Solution**: Generate optimal code for each framework, benchmarking

4. **Type Safety**: Maintaining compile-time guarantees
   - **Solution**: Extensive type-level validation in derive macro

## 🔄 **Dependencies**

- **Requires**: 
  - Task 001 (Single Derive Macro) for attribute infrastructure
  - Task 003 (Validation) for request validation
- **Blocks**: None
- **Related**: Benefits from Task 002 (Popular Types) for type conversions

## 📅 **Timeline**

- **Week 1-2**: Axum integration and core framework
- **Week 2-3**: Actix-web support and advanced features  
- **Week 3-4**: Warp support, optimization, and documentation

## 💡 **Future Enhancements**

- **OpenAPI Integration**: Generate OpenAPI specs from extraction structs
- **Request Validation**: Integration with validation framework
- **Middleware Integration**: Custom middleware for pre-processing
- **Response Generation**: Complement extraction with response building
- **GraphQL Support**: Extract from GraphQL contexts and resolvers