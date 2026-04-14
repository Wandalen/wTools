# Task 003: Validation Framework

## 🎯 **Objective**

Implement a comprehensive validation framework that allows field-level validation during component assignment, providing clear error messages and validation composition.

## 📋 **Current State**

No built-in validation exists - users must implement validation manually:
```rust
impl Config
{
  fn set_port( &mut self, port : u16 )
  {
    if port < 1024
    {
      panic!( "Port must be >= 1024" );
    }
    self.port = port;
  }
}
```

## 🎯 **Target State**

Declarative validation with clear error reporting:
```rust
#[derive(ComponentModel)]
struct Config
{
  #[ component( validate = "is_valid_host" ) ]
  host : String,
  
  #[ component( validate = "is_port_range(1024, 65535)" ) ]
  port : u16,
  
  #[ component( validate = "not_empty" ) ]
  database_name : String,
}

// Usage with validation
let result = Config::default()
  .try_assign( "" ) // Fails validation
  .and_then( | c | c.try_assign( 80u16 ) ) // Fails validation
  .and_then( | c | c.try_assign( "" ) );   // Fails validation

match result
{
  Ok( config ) => println!( "Valid config: {:?}", config ),
  Err( errors ) =>
  {
    for error in errors
    {
      eprintln!( "Validation error: {}", error );
    }
  }
}
```

## 📝 **Detailed Requirements**

### **Core Validation API**

#### **Result-Based Assignment**
```rust
pub trait TryAssign< T, IntoT >
{
  type Error;
  
  fn try_assign( &mut self, component : IntoT ) -> Result< (), Self::Error >;
  fn try_impute( self, component : IntoT ) -> Result< Self, Self::Error >
  where
    Self : Sized;
}
```

#### **Error Types**
```rust
#[ derive( Debug, Clone ) ]
pub struct ValidationError
{
  pub field_name : String,
  pub field_type : String,
  pub provided_value : String,
  pub error_message : String,
  pub suggestion : Option< String >,
}

#[ derive( Debug, Clone ) ]
pub struct ValidationErrors
{
  pub errors : Vec< ValidationError >,
}

impl std::fmt::Display for ValidationErrors
{
  fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result
  {
    for ( i, error ) in self.errors.iter().enumerate()
    {
      if i > 0 { writeln!( f )?; }
      write!( f, "Field '{}': {}", error.field_name, error.error_message )?;
      if let Some( suggestion ) = &error.suggestion
      {
        write!( f, " (try: {})", suggestion )?;
      }
    }
    Ok( () )
  }
}
```

### **Built-in Validators**

#### **String Validators**
```rust
pub fn not_empty( value : &str ) -> Result< (), String >
{
  if value.is_empty()
  {
    Err( "cannot be empty".to_string() )
  }
  else
  {
    Ok( () )
  }
}

pub fn min_length( min : usize ) -> impl Fn( &str ) -> Result< (), String >
{
  move | value |
  {
    if value.len() < min
    {
      Err( format!( "must be at least {} characters", min ) )
    }
    else
    {
      Ok( () )
    }
  }
}

pub fn max_length( max : usize ) -> impl Fn( &str ) -> Result< (), String >
{
  move | value |
  {
    if value.len() > max
    {
      Err( format!( "must be at most {} characters", max ) )
    }
    else
    {
      Ok( () )
    }
  }
}

pub fn matches_regex( pattern : &str ) -> impl Fn( &str ) -> Result< (), String >
{
  let regex = Regex::new( pattern ).expect( "Invalid regex pattern" );
  move | value |
  {
    if regex.is_match( value )
    {
      Ok( () )
    }
    else
    {
      Err( format!( "must match pattern: {}", pattern ) )
    }
  }
}
```

#### **Numeric Validators**
```rust
pub fn min_value< T : PartialOrd + std::fmt::Display >( min : T ) -> impl Fn( &T ) -> Result< (), String >
{
  move | value |
  {
    if value < &min
    {
      Err( format!( "must be at least {}", min ) )
    }
    else
    {
      Ok( () )
    }
  }
}

pub fn max_value< T : PartialOrd + std::fmt::Display >( max : T ) -> impl Fn( &T ) -> Result< (), String >
{
  move | value |
  {
    if value > &max
    {
      Err( format!( "must be at most {}", max ) )
    }
    else
    {
      Ok( () )
    }
  }
}

pub fn range< T : PartialOrd + std::fmt::Display >( min : T, max : T ) -> impl Fn( &T ) -> Result< (), String >
{
  move | value |
  {
    if value < &min || value > &max
    {
      Err( format!( "must be between {} and {}", min, max ) )
    }
    else
    {
      Ok( () )
    }
  }
}
```

### **Attribute Syntax**

#### **Function Reference**
```rust
#[derive(ComponentModel)]
struct Config
{
  #[ component( validate = "not_empty" ) ]
  name : String,
}

fn not_empty( value : &str ) -> Result< (), String >
{
  // validation logic
}
```

#### **Closure Syntax**
```rust
#[derive(ComponentModel)]
struct Config
{
  #[ component( validate = "|v| if v.len() > 0 { Ok(()) } else { Err(\"empty\".to_string()) }" ) ]
  name : String,
}
```

#### **Multiple Validators**
```rust
#[derive(ComponentModel)]
struct Config
{
  #[ component( validate = [ "not_empty", "min_length(3)", "max_length(50)" ] ) ]
  username : String,
}
```

### **Generated Implementation**

The derive macro generates:
```rust
impl TryAssign< String, &str > for Config
{
  type Error = ValidationErrors;
  
  fn try_assign( &mut self, component : &str ) -> Result< (), Self::Error >
  {
    let mut errors = Vec::new();
    
    // Run validation
    if let Err( msg ) = not_empty( component )
    {
      errors.push
      (
        ValidationError
        {
          field_name : "name".to_string(),
          field_type : "String".to_string(),
          provided_value : component.to_string(),
          error_message : msg,
          suggestion : Some( "provide a non-empty string".to_string() ),
        }
      );
    }
    
    if !errors.is_empty()
    {
      return Err( ValidationErrors { errors } );
    }
    
    // If validation passes, assign
    self.name = component.to_string();
    Ok( () )
  }
}
```

## 🗂️ **File Changes**

### **New Files**
- `component_model_types/src/validation/mod.rs` - Core validation types
- `component_model_types/src/validation/validators.rs` - Built-in validators
- `component_model_types/src/validation/error.rs` - Error types
- `component_model_meta/src/validation.rs` - Validation macro logic
- `examples/validation_example.rs` - Comprehensive example

### **Modified Files**
- `component_model_types/src/lib.rs` - Export validation module
- `component_model_meta/src/lib.rs` - Add validation to derives
- `component_model/src/lib.rs` - Re-export validation types

## ⚡ **Implementation Steps**

### **Phase 1: Core Framework (Week 1)**
1. Define `TryAssign` trait and error types
2. Implement basic string validators (`not_empty`, `min_length`, etc.)
3. Create validation attribute parsing in derive macro
4. Generate basic validation code

### **Phase 2: Advanced Validators (Week 2)**
1. Add numeric validators (`min_value`, `max_value`, `range`)
2. Implement custom validator support
3. Add validator composition (multiple validators per field)
4. Error message improvement and suggestions

### **Phase 3: Integration & Polish (Week 2-3)**
1. Integration with existing `Assign` trait (fallback behavior)
2. Performance optimization for validation chains  
3. Comprehensive documentation and examples
4. Error message localization support

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[ cfg( test ) ]
mod tests
{
  use super::*;
  
  #[ test ]
  fn test_validation_success()
  {
    #[ derive( ComponentModel ) ]
    struct Config
    {
      #[ component( validate = "not_empty" ) ]
      name : String,
    }
    
    let mut config = Config::default();
    assert!( config.try_assign( "test" ).is_ok() );
    assert_eq!( config.name, "test" );
  }
  
  #[ test ]
  fn test_validation_failure()
  {
    #[ derive( ComponentModel ) ]
    struct Config
    {
      #[ component( validate = "not_empty" ) ]
      name : String,
    }
    
    let mut config = Config::default();
    let result = config.try_assign( "" );
    
    assert!( result.is_err() );
    let errors = result.unwrap_err();
    assert_eq!( errors.errors.len(), 1 );
    assert_eq!( errors.errors[ 0 ].field_name, "name" );
  }
  
  #[ test ]
  fn test_multiple_validators()
  {
    #[ derive( ComponentModel ) ]
    struct Config
    {
      #[ component( validate = [ "not_empty", "min_length(3)" ] ) ]
      username : String,
    }
    
    let mut config = Config::default();
    
    // Should fail both validations
    let result = config.try_assign( "" );
    assert!( result.is_err() );
    
    // Should fail min_length
    let result = config.try_assign( "ab" );
    assert!( result.is_err() );
    
    // Should succeed
    let result = config.try_assign( "abc" );
    assert!( result.is_ok() );
  }
}
```

### **Integration Tests**
```rust
#[ test ]
fn test_real_world_validation()
{
  #[ derive( ComponentModel ) ]
  struct ServerConfig
  {
    #[ component( validate = "not_empty" ) ]
    host : String,
    
    #[ component( validate = "range(1024, 65535)" ) ]
    port : u16,
    
    #[ component( validate = "min_value(1)" ) ]
    worker_count : usize,
  }
  
  // Test valid configuration
  let config = ServerConfig::default()
    .try_impute( "localhost" )
    .and_then( | c | c.try_impute( 8080u16 ) )
    .and_then( | c | c.try_impute( 4usize ) );
    
  assert!( config.is_ok() );
  
  // Test invalid configuration
  let result = ServerConfig::default()
    .try_impute( "" )  // Empty host
    .and_then( | c | c.try_impute( 80u16 ) )  // Invalid port
    .and_then( | c | c.try_impute( 0usize ) ); // Invalid worker count
    
  assert!( result.is_err() );
  let errors = result.unwrap_err();
  assert_eq!( errors.errors.len(), 3 );
}
```

## 📊 **Success Metrics**

- [ ] Support for 10+ built-in validators
- [ ] Clear, actionable error messages
- [ ] Zero performance overhead when validation disabled
- [ ] Composable validation (multiple validators per field)
- [ ] Integration with existing assignment patterns

## 🚧 **Potential Challenges**

1. **Performance Impact**: Validation adds overhead
   - **Solution**: Compile-time optimization and benchmarking

2. **Error Message Quality**: Generic errors aren't helpful
   - **Solution**: Context-aware error generation with suggestions

3. **Validator Composition**: Complex attribute parsing
   - **Solution**: Robust parser with clear error messages

## 🔄 **Dependencies**

- **Requires**: Task 001 (Single Derive Macro) for attribute parsing
- **Blocks**: None
- **Related**: Task 002 benefits from validation for type conversion

## 📅 **Timeline**

- **Week 1**: Core validation framework and basic validators
- **Week 2**: Advanced validators and composition
- **Week 3**: Integration, optimization, and documentation

## 💡 **Future Enhancements**

- **Async Validation**: For database uniqueness checks, etc.
- **Custom Error Types**: Allow users to define their own error types
- **Conditional Validation**: Validators that depend on other field values
- **Validation Groups**: Different validation rules for different contexts