# Task 011: Argument for Constructor Attribute

## 📋 **Overview**
Introduce field attribute `arg_for_constructor` to mark fields as arguments for constructing functions.

## 🎯 **Objectives**
- Add `arg_for_constructor` field attribute
- Mark fields that should be used in constructing functions
- Support both standalone constructors and associated constructors
- Handle enum field restrictions properly
- By default `arg_for_constructor` is false

## 🔧 **Technical Details**

### Field Marking
- Mark fields with `arg_for_constructor` attribute
- Fields marked as constructor arguments
- Works with both structs and enums

### Enum Restrictions
- `arg_for_constructor` attachable only to fields of variant
- **Error**: Attempting to attach to variant itself must throw understandable error
- Only variant fields can be constructor arguments

### Constructor Naming
- **Struct**: snake_case version of struct name
- **Enum**: snake_case version of variant name

### Default Behavior
- `arg_for_constructor` defaults to `false`
- Only marked fields become constructor arguments

## 📍 **Source Location**
File: `/home/user1/pro/lib/wTools/module/core/component_model/src/lib.rs`
Line: 12

## 🏷️ **Labels**
- **Type**: Feature Enhancement  
- **Priority**: Medium
- **Difficulty**: 🟡 Medium
- **Value**: 🟠 Medium
- **Status**: 📋 Planned

## 📦 **Dependencies**
- Task 010: Standalone Constructors
- Component model core functionality

## 🧪 **Acceptance Criteria**
- [ ] Add `arg_for_constructor` field attribute parsing
- [ ] Support constructor arguments for struct fields
- [ ] Support constructor arguments for enum variant fields
- [ ] Validate enum usage (fields only, not variants)
- [ ] Generate constructors with proper arguments
- [ ] Provide clear error messages for invalid usage
- [ ] Add comprehensive tests
- [ ] Update documentation with examples