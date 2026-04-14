# Task 010: Standalone Constructors

## 📋 **Overview**
Introduce body( struct/enum ) attribute `standalone_constructors` which create stand-alone, top-level constructors for struct/enum.

## 🎯 **Objectives**
- Add `standalone_constructors` attribute for struct/enum bodies
- For struct: create single constructor function
- For enum: create as many functions as enum has variants
- If no `arg_for_constructor` then constructors expect exactly zero arguments
- Start from implementations without respect of attribute `arg_for_constructor`
- By default `standalone_constructors` is false

## 🔧 **Technical Details**

### Struct Constructor
- Create stand-alone, top-level constructor function
- Name: same as struct but snake_case (e.g., `MyStruct` → `my_struct()`)
- Single function per struct

### Enum Constructor  
- Create separate constructor function for each variant
- Name: same as variant but snake_case (e.g., `MyVariant` → `my_variant()`)
- Multiple functions per enum (one per variant)

### Default Behavior
- `standalone_constructors` defaults to `false`
- Only generate constructors when explicitly enabled

## 📍 **Source Location**
File: `/home/user1/pro/lib/wTools/module/core/component_model/src/lib.rs`
Line: 11

## 🏷️ **Labels**
- **Type**: Feature Enhancement  
- **Priority**: Medium
- **Difficulty**: 🟡 Medium
- **Value**: 🟠 Medium
- **Status**: 📋 Planned

## 📦 **Dependencies**
- Component model core functionality
- Macro generation system

## 🧪 **Acceptance Criteria**
- [ ] Add `standalone_constructors` attribute parsing
- [ ] Generate standalone constructor for structs
- [ ] Generate multiple constructors for enum variants
- [ ] Use snake_case naming convention
- [ ] Handle zero-argument constructors by default
- [ ] Add comprehensive tests
- [ ] Update documentation with examples