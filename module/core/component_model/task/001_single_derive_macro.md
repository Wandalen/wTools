# Task 001: Single Derive Macro - ComponentModel

## 🎯 **Objective**

Create a unified `#[derive(ComponentModel)]` macro that combines all existing derives into one convenient annotation, reducing boilerplate and improving developer experience.

## 📋 **Current State**

Users currently need multiple derives:
```rust
#[derive(Default, Assign, ComponentsAssign, FromComponents, ComponentFrom)]
struct Config {
  host: String,
  port: i32,
}
```

## 🎯 **Target State**

Single, comprehensive derive:
```rust
#[derive(ComponentModel)]
struct Config {
  host: String,
  port: i32,
}
```

## 📝 **Detailed Requirements**

### **Core Functionality**
1. **Combine All Existing Derives**
   - `Assign` - Basic component assignment
   - `ComponentsAssign` - Multiple component assignment from tuples
   - `ComponentFrom` - Create objects from single components
   - `FromComponents` - Create objects from multiple components

2. **Automatic Trait Detection**
   - Only generate implementations that make sense for the struct
   - Skip conflicting implementations (e.g., avoid multiple `String` field conflicts)

3. **Backward Compatibility**
   - Existing individual derives must continue to work
   - No breaking changes to current API

### **Implementation Details**

#### **Macro Structure**
```rust
// In component_model_meta/src/lib.rs
#[proc_macro_derive(ComponentModel, attributes(component))]
pub fn derive_component_model(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  
  let assign_impl = generate_assign_impl(&ast);
  let components_assign_impl = generate_components_assign_impl(&ast);
  let component_from_impl = generate_component_from_impl(&ast);
  let from_components_impl = generate_from_components_impl(&ast);
  
  quote! {
    #assign_impl
    #components_assign_impl
    #component_from_impl
    #from_components_impl
  }.into()
}
```

#### **Conflict Resolution**
- **Multiple same-type fields**: Only generate `Assign` if types are unambiguous
- **Tuple assignment**: Only generate if struct has <= 4 fields
- **Component creation**: Generate both `ComponentFrom` and `FromComponents`

### **Testing Strategy**

#### **Unit Tests**
```rust
#[derive(ComponentModel)]
struct TestStruct {
  name: String,
  value: i32,
}

#[test]
fn test_unified_derive() {
  let mut obj = TestStruct::default();
  
  // Test Assign
  obj.assign("test");
  obj.assign(42);
  
  // Test ComponentFrom
  let obj2: TestStruct = ComponentFrom::component_from("hello");
  
  // Test FromComponents
  let obj3: TestStruct = FromComponents::from_components(("world", 100));
  
  assert_eq!(obj.name, "test");
  assert_eq!(obj.value, 42);
}
```

#### **Integration Tests**
- Test with existing code that uses individual derives
- Verify no performance regression
- Test error messages are clear

## 🗂️ **File Changes**

### **New Files**
- `component_model_meta/src/component_model.rs` - Main implementation
- `tests/unified_derive_test.rs` - Comprehensive tests

### **Modified Files**
- `component_model_meta/src/lib.rs` - Export new derive
- `component_model/src/lib.rs` - Re-export derive
- `README.md` - Update examples to use new derive

## ⚡ **Implementation Steps**

### **Phase 1: Core Implementation (Week 1)**
1. Create base macro structure in `component_model_meta`
2. Implement basic `Assign` generation
3. Add conflict detection for same-type fields
4. Create basic test suite

### **Phase 2: Extended Functionality (Week 1-2)**
1. Add `ComponentsAssign` generation
2. Implement `ComponentFrom` and `FromComponents`
3. Add attribute parsing for future extensibility
4. Comprehensive testing

### **Phase 3: Documentation & Polish (Week 2)**
1. Update all examples to use new derive
2. Add migration guide for existing users
3. Performance benchmarking
4. Documentation review

## 🧪 **Testing Checklist**

- [ ] Basic assignment works (`obj.assign(value)`)
- [ ] Fluent assignment works (`obj.impute(value)`)
- [ ] Component creation works (`ComponentFrom::component_from(value)`)
- [ ] Multiple component creation works (`FromComponents::from_components(tuple)`)
- [ ] Backward compatibility maintained
- [ ] Error messages are clear and helpful
- [ ] Performance is equivalent to individual derives
- [ ] Works with generic structs
- [ ] Works with lifetime parameters
- [ ] Handles edge cases (empty structs, single fields, etc.)

## 📊 **Success Metrics**

- [ ] Reduces derive boilerplate from 4+ lines to 1 line
- [ ] Zero performance overhead vs individual derives
- [ ] 100% backward compatibility
- [ ] Clear, actionable error messages
- [ ] Documentation updated with new examples

## 🚧 **Potential Challenges**

1. **Type Ambiguity**: Multiple fields of same type causing conflicts
   - **Solution**: Implement smart conflict detection and clear error messages

2. **Macro Complexity**: Combining multiple derive logic
   - **Solution**: Modular implementation with separate functions for each trait

3. **Error Message Quality**: Complex macros often have poor error messages
   - **Solution**: Custom error types with span information

## 🔄 **Dependencies**

- **Requires**: Current derive implementations working
- **Blocks**: None (additive feature)
- **Related**: All other enhancement tasks will benefit from this foundation

## 📅 **Timeline**

- **Week 1**: Core implementation and basic testing
- **Week 2**: Extended functionality and comprehensive testing  
- **Week 3**: Documentation update and release preparation

## 💡 **Future Enhancements**

Once this is complete, we can add:
- Field-level attributes: `#[component(default = "value")]`
- Validation attributes: `#[component(validate = "function")]`
- Transform attributes: `#[component(transform = "function")]`

This task provides the foundation for all future component model enhancements.