# Specification: proper_tools

**Status**: DRAFT - Placeholder Module Awaiting Definition
**Version**: 0.1.0
**Type**: Alias Module (Re-export Wrapper)
**Authority**: Single source of truth for proper_tools requirements and design

---

## 1. Purpose and Scope

### Current Status

This module is a **PLACEHOLDER** awaiting definition and implementation. It serves as a structural skeleton for a future general-purpose tools collection.

### Intended Purpose (Future)

**Primary Goal:** Provide a collection of general-purpose utility tools that fundamentally extend the Rust language without "spoiling" its design (maintaining idiomatic Rust patterns).

**Design Philosophy:**
- Can be used solely or in conjunction with other utility modules
- Maintains compatibility with `no_std` environments
- Follows wTools alias module pattern (thin re-export wrapper)
- Provides semantic feature flags for granular functionality control

### Explicit Non-Scope

**Out of Scope:**
- Replacement for standard library (complements, doesn't replace)
- Application-specific business logic
- Heavy dependencies or large binary footprint
- Breaking changes to Rust idioms

**Decision Required:**
- What specific problem domains does proper_tools address?
- What distinguishes it from existing wTools modules (fmt_tools, strs_tools, etc.)?
- What is the minimal viable feature set?

---

## 2. Architecture

### Module Pattern

**Type:** Alias module (re-export wrapper)
**Location:** `module/alias/proper_tools`
**Pattern:** Follows wTools alias module convention

**Current Implementation:**
```rust
// src/lib.rs (placeholder)
#[ cfg( feature = "enabled" ) ]
pub fn f1() {}
```

**Target Implementation (Future):**
```rust
// src/lib.rs (when core module exists)
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use proper_tools_core::*;
```

### Dependencies

**Current:**
- Runtime dependencies: NONE
- Dev dependencies: `test_tools` (workspace)

**Planned:**
- `proper_tools_core` (when created) - Core implementation module
- Minimal external dependencies (TBD based on feature set)

**Core Module Status:**
- **Decision Required:** Create `module/core/proper_tools_core` OR repurpose existing module OR implement directly in alias (non-standard)

---

## 3. API Design

### Current API Surface

**Public Functions:**
```rust
/// Placeholder function - no functionality
#[ cfg( feature = "enabled" ) ]
pub fn f1() {}
```

**Status:** Placeholder only, provides no actual utility

### Planned API Surface (TBD)

**Decision Required:** Define specific utility functions/types/traits based on:
1. Problem domain analysis
2. Gap analysis vs existing wTools modules
3. User needs assessment
4. Core module definition

**Constraints:**
- Must be `no_std` compatible where applicable
- Must follow Rust naming conventions
- Must have comprehensive doc comments
- Must have working examples for all public APIs

---

## 4. Feature Flags

### Current Features

```toml
[features]
default = [ "enabled" ]
full = [ "enabled" ]
no_std = []
use_alloc = [ "no_std" ]
enabled = []
```

**Analysis:**
- `default`/`full` currently identical (both enable only `enabled`)
- `no_std`/`use_alloc` configured but unused
- No semantic features (no actual functionality to gate)

### Planned Features (Future)

**Core Features:**
- `default` - Recommended subset for typical use
- `full` - All non-conflicting features enabled
- `enabled` - Master switch for all functionality

**Environment Features:**
- `no_std` - Exclude std library dependencies
- `use_alloc` - Enable alloc-dependent features in no_std

**Semantic Features (TBD):**
```toml
# Example structure (to be defined):
# feature_domain_1 = [ "core_module/feature_1" ]
# feature_domain_2 = [ "core_module/feature_2" ]
```

**Decision Required:** Define semantic features when core functionality determined

---

## 5. Examples

### Current Examples

**Status:** Examples implemented

**Completed:**
- ✅ `examples/proper_tools_trivial.rs` exists and runs successfully
- ✅ `examples/` directory created
- ✅ Placeholder functionality demonstrated
- ✅ Example referenced in readme.md

**Current Examples:**
1. **`examples/proper_tools_trivial.rs`** (32 lines)
   - Demonstrates placeholder f1() function
   - Shows feature flag handling (enabled/disabled states)
   - Includes proper documentation
   - Verified working by tests/examples_exist.rs

### Required Examples (Minimum)

When functionality implemented, MUST include:

1. **`examples/proper_tools_trivial.rs`**
   - Purpose: Minimal working example (referenced in readme)
   - Content: Simplest possible usage demonstrating core value
   - Size: <50 lines preferred

2. **`examples/proper_tools_basic.rs`**
   - Purpose: Common use cases
   - Content: Typical usage patterns for main features
   - Size: <100 lines

3. **`examples/proper_tools_no_std.rs`** (if applicable)
   - Purpose: Demonstrate no_std compatibility
   - Content: Usage without standard library
   - Compilation: `#![ no_std ]`

**Documentation Requirements:**
- Each example MUST have doc comment explaining purpose
- Each example MUST be runnable without errors
- Each example MUST be referenced in readme.md

---

## 6. Testing Strategy

### Current Tests

**Location:** `tests/smoke_test.rs`
**Content:** 2 smoke tests (placeholder)

```rust
#[ test ]
fn local_smoke_test() {
  println!("proper_tools local smoke test passed");
}

#[ test ]
fn published_smoke_test() {
  println!("proper_tools published smoke test passed");
}
```

**Status:** Tests verify compilation only, no functional testing

### Required Tests (When Implemented)

**Functional Tests:**
- Re-export verification (ensure `pub use` works correctly)
- Feature flag tests (each semantic feature tested)
- Integration tests with examples
- No-std compilation tests

**Test Organization:**
- `tests/smoke_test.rs` - Compilation verification (keep existing)
- `tests/re_exports.rs` - Verify alias re-exports work (future)
- `tests/features.rs` - Feature flag validation (future)
- `tests/no_std.rs` - No-std compatibility (future)

**Coverage Requirements:**
- All public API functions tested
- All feature flag combinations verified
- All examples tested in CI

---

## 7. Documentation

### Current Documentation

**readme.md:**
- ✅ Status clearly marked as "EXPERIMENTAL - PLACEHOLDER MODULE"
- ✅ Examples section active (line 45-47)
- ✅ References working example: proper_tools_trivial
- ✅ Current functionality section explains placeholder state

**Cargo.toml:**
- Description matches readme
- ✅ Repository points to correct path: `module/alias/proper_tools`
- ✅ Homepage points to correct path: `module/alias/proper_tools`
- ✅ Documentation URL correct: docs.rs/proper_tools

### Required Documentation Updates

1. **readme.md** - ✅ COMPLETE - Placeholder status documented, examples section active
2. **Cargo.toml** - ✅ COMPLETE - Repository and homepage fields corrected
3. **src/lib.rs** - ✅ COMPLETE - Module-level docs present with readme.md inclusion
4. **This spec.md** - Convert from DRAFT to ACTIVE when design finalized

---

## 8. Implementation Roadmap

### Phase 1: Definition (Current - Pending Decisions)

- [ ] Define specific problem domain and scope
- [ ] Identify or create core module
- [ ] Define minimal viable API surface
- [ ] Define semantic feature organization
- [ ] Specify dependency requirements

### Phase 2: Core Implementation (Future)

- [ ] Create/identify core module (`proper_tools_core`)
- [ ] Implement core functionality
- [ ] Add comprehensive tests to core
- [ ] Document core API

### Phase 3: Alias Integration (Future)

- [ ] Update `src/lib.rs` with `pub use core::*`
- [ ] Add feature flag proxies in Cargo.toml
- [ ] Create working examples
- [ ] Update readme with uncommented examples

### Phase 4: Testing & Documentation (Future)

- [ ] Write functional tests for re-exports
- [ ] Add integration tests
- [ ] Create all required examples
- [ ] Update all documentation
- [ ] Verify docs.rs rendering

### Phase 5: Release (Future)

- [ ] Run full test suite (ctest5 equivalent)
- [ ] Verify all examples work
- [ ] Publish to crates.io (if applicable)
- [ ] Mark spec.md as ACTIVE

---

## 9. Open Questions

**Critical Decisions Required:**

1. **Module Scope:** What specific utilities belong in proper_tools vs other modules?
2. **Core Module:** Create new `proper_tools_core` OR repurpose existing OR implement directly?
3. **Relationship to proper_path_tools:** Historical core module (v0.8-v0.11) was removed - why? Restore or replace?
4. **Feature Set:** What is minimal viable set of utilities to justify module existence?
5. **Naming:** Is "proper_tools" appropriate for intended scope? Consider alternatives?

**Design Questions:**

1. Should proper_tools be aggregating module (re-exporting multiple cores) or focused (single domain)?
2. What compatibility requirements (MSRV, no_std, platforms)?
3. What performance requirements (zero-cost abstractions, compile time)?
4. What dependencies are acceptable (if any)?

---

## 10. Historical Context

### Background

- Original `proper_path_tools` core module existed (v0.8 → v0.11+)
- Module was actively developed and versioned
- Core module subsequently removed/archived
- `proper_tools` alias created but never completed
- Current state: orphaned placeholder awaiting definition

### Git History Evidence

- `827ff974` - Centralized dependencies, proper_tools created
- `46b197c6` - proper_path_tools v0.11.0 release
- `ba1a0663` - proper_path_tools tests fixed
- Earlier versions: v0.9.0, v0.8.0

### Lessons Learned (TBD)

- Why was proper_path_tools removed?
- What should be preserved from original design?
- What should be done differently?

---

## 11. Compliance

### Organizational Principles

- ✅ **Specification Exists:** This document serves as single source of truth
- ✅ **Anti-Duplication:** Spec is sole authority (no duplicate specs)
- ✅ **Unique Responsibility:** Module has clear purpose (general utilities)
- ⚠️ **Appropriate Constraints:** Many aspects marked TBD (preserving freedom until requirements certain)

### Test Organization

- ⚠️ **Testing Mandatory:** Tests exist but only verify compilation (placeholder status acceptable)
- ✅ **Centralized Test Directory:** Tests in `tests/` at crate root
- ⚠️ **Examples Required:** No examples yet (acceptable for placeholder, MUST fix when implementing)

### Codebase Hygiene

- ✅ **No Mocking:** Tests don't use mocks
- ✅ **No Backups:** No backup files present
- ✅ **File Size:** All files under limits
- ⚠️ **Proper Implementation:** Placeholder acceptable temporarily, MUST implement properly

---

## 12. References

### Related Modules

- **wstring_tools** - String utility alias (reference implementation pattern)
- **winterval** - Interval utility alias (reference implementation pattern)
- **strs_tools** - String core module (architectural reference)

### Documentation

- wTools alias pattern: `/home/user1/pro/lib/wTools/doc/rust/LibConventions.md`
- Organizational principles: `organizational_principles.rulebook.md`
- Test organization: `test_organization.rulebook.md`

### Issues

- No tracked issues (no issue tracker reference in project)
- Manual testing revealed 7 issues (documented in testing session notes)

---

## 13. Acceptance Criteria

### Definition Phase Complete When:

- [ ] All "Decision Required" questions answered
- [ ] Core module path/strategy determined
- [ ] Minimal API surface specified with concrete function signatures
- [ ] Semantic features defined
- [ ] Spec status changed from DRAFT to ACTIVE

### Implementation Phase Complete When:

- [ ] Core module exists with functionality
- [ ] Alias module re-exports core correctly
- [ ] All tests passing (ctest3 minimum)
- [ ] All examples exist and work
- [ ] Documentation accurate and complete
- [ ] Zero organizational principle violations
- [ ] Ready for crates.io publication (if applicable)

---

**Document Status:** DRAFT
**Last Updated:** 2026-01-04 (Manual Testing Session)
**Next Review:** When design decisions finalized
