# willbe Specification Index

**Version:** 2.0
**Status:** Draft (Implementation Pending)
**Last Updated:** 2025-11-08

---

## Overview

This specification defines the enhanced publishing algorithm for `willbe`, a multi-crate workspace publishing tool. The enhancement adds dependency staleness detection and cascade publishing to fix critical version conflict issues.

---

## Document Structure

### 1. [Publishing Algorithm](./publishing_algorithm.md)

**Purpose:** Formal algorithmic specification

**Contents:**
- High-level algorithm flow (5 phases)
- Detailed phase specifications with pseudocode
- Data structure definitions
- Performance characteristics
- Edge case handling
- Acceptance criteria

**Audience:** Developers, reviewers, architects

### 2. [Implementation Plan](./implementation_plan.md)

**Purpose:** Step-by-step implementation roadmap

**Contents:**
- 9 increments broken down by task
- TDD approach with test-first development
- File-by-file implementation guide
- Verification steps for each increment
- Timeline estimates (25 days total)

**Audience:** Implementers, project managers

### 3. [Test Scenarios](./test_scenarios.md)

**Purpose:** Comprehensive test matrix

**Contents:**
- 40+ test scenarios across 5 categories
- Test fixtures and utilities
- Automation commands
- Coverage targets
- Success criteria

**Audience:** QA engineers, test developers

---

## Quick Navigation

### By Role

**For Architects:**
1. Read [Publishing Algorithm](./publishing_algorithm.md) sections 1-3
2. Review data structures (section 4)
3. Check performance characteristics (section 6)

**For Implementers:**
1. Start with [Implementation Plan](./implementation_plan.md)
2. Reference [Publishing Algorithm](./publishing_algorithm.md) for details
3. Use [Test Scenarios](./test_scenarios.md) for TDD

**For Testers:**
1. Begin with [Test Scenarios](./test_scenarios.md)
2. Reference [Publishing Algorithm](./publishing_algorithm.md) section 5 (edge cases)
3. Follow test organization matrix

**For Reviewers:**
1. Read [Publishing Algorithm](./publishing_algorithm.md) sections 1-2
2. Check [Test Scenarios](./test_scenarios.md) category 3 (bug reproduction)
3. Verify [Implementation Plan](./implementation_plan.md) increment 7 (documentation)

### By Topic

**Dependency Staleness:**
- [Publishing Algorithm § 3.3](./publishing_algorithm.md#33-phase-2-dependency-staleness-detection)
- [Implementation Plan § Increment 4](./implementation_plan.md#increment-4-dependency-staleness-detection-4-days)
- [Test Scenarios § Category 1](./test_scenarios.md#category-1-basic-staleness-detection-10-scenarios)

**Cascade Publishing:**
- [Publishing Algorithm § 3.4](./publishing_algorithm.md#34-phase-3-transitive-closure)
- [Implementation Plan § Increment 5](./implementation_plan.md#increment-5-transitive-closure-computation-4-days)
- [Test Scenarios § Category 2](./test_scenarios.md#category-2-cascade-publishing-10-scenarios)

**Original Bug:**
- [Publishing Algorithm § 5.4](./publishing_algorithm.md#54-incompatible-version-bumps)
- [Implementation Plan § Increment 4 Step 4.3](./implementation_plan.md#step-43-write-staleness-detection-tests)
- [Test Scenarios § Category 3](./test_scenarios.md#category-3-original-bug-scenario-5-scenarios)

---

## Project Context

### Problem Statement

The willbe publishing algorithm fails to detect when workspace packages need republishing due to stale dependencies. This causes version conflicts during `cargo publish` when a dependency has been bumped but dependents still reference old versions.

**Example Failure:**
```
willbe depends on wca ~0.36.0, former ~2.37.0
wca 0.36.0 depends on former ~2.36.0
former bumped 2.36.0 → 2.37.0

Result: cargo fails to resolve former version (conflict between ~2.36.0 and ~2.37.0)
```

### Solution Summary

Enhance the publishing algorithm with three new capabilities:

1. **Dependency Staleness Detection:** Check if workspace dependency versions satisfy package requirements
2. **Cascade Effect Tracking:** Identify all packages affected by version bumps
3. **Transitive Closure Computation:** Include all transitively affected packages in publish plan

### Expected Outcome

- Zero version conflicts during publishing
- Automatic detection of all affected packages
- Clear display of why each package needs publishing
- Maintains topological ordering for correct publish sequence

---

## Implementation Status

| Increment | Status | Progress |
|-----------|--------|----------|
| 1. Setup & Spec | 🟢 Complete | 100% (This document) |
| 2. Data Structures | 🔴 Not Started | 0% |
| 3. Semver Utils | 🔴 Not Started | 0% |
| 4. Staleness Detection | 🔴 Not Started | 0% |
| 5. Closure Computation | 🔴 Not Started | 0% |
| 6. Integration | 🔴 Not Started | 0% |
| 7. Documentation | 🔴 Not Started | 0% |
| 8. Performance | 🔴 Not Started | 0% |
| 9. Verification | 🔴 Not Started | 0% |

**Overall: 11% (1/9 increments)**

---

## Key Decisions

### Decision 1: Dependency Types to Check

**Chosen:** Check all three types (normal, dev, build)

**Rationale:** Dev and build dependencies can also cause publishing failures if stale

**Alternative:** Only check normal dependencies (rejected - incomplete coverage)

### Decision 2: External vs Workspace Dependencies

**Chosen:** Only check workspace dependencies for staleness

**Rationale:** External dependencies managed by crates.io, outside our control

**Alternative:** Check all dependencies (rejected - can't republish external crates)

### Decision 3: Version Bumping Strategy

**Chosen:** Default to patch bump (0.1.0 → 0.1.1)

**Rationale:** Most conservative, minimal API impact

**Alternative:** Smart bumping based on changes (rejected - complex to implement)

### Decision 4: Cascade Trigger

**Chosen:** Cascade on both incompatible versions AND dependencies being published

**Rationale:** Ensures fresh versions in publish batch resolve correctly

**Alternative:** Only cascade on incompatible versions (rejected - misses batch conflicts)

### Decision 5: Circular Dependency Handling

**Chosen:** Detect and error

**Rationale:** Circular dependencies cannot be published sequentially

**Alternative:** Try to break cycles (rejected - undefined behavior)

---

## References

### External Documentation

- [Cargo Version Requirements](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [Semantic Versioning Specification](https://semver.org/)
- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)

### Internal Documentation

- [wTools Repository](https://github.com/Wandalen/wTools)
- [willbe Crate](https://github.com/Wandalen/wTools/tree/master/module/core/willbe)

### Related Issues

- Issue #willbe-staleness-001: Original bug report (version conflicts during publish)

---

## Change History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2024-XX-XX | Initial algorithm (no staleness detection) | - |
| 2.0 | 2025-11-08 | Add dependency staleness detection specification | Development Team |

---

## Appendix: Glossary

**Dependency Staleness:** State where a package's dependency requirement doesn't match the workspace version

**Cascade Effect:** Transitive publishing requirement where package A must be republished because dependency B is being published

**Transitive Closure:** Complete set of packages that must be published, including all direct and indirect dependencies

**Workspace Version:** Version of a package as specified in workspace Cargo.toml

**Published Version:** Version currently available on crates.io

**Semver:** Semantic Versioning (major.minor.patch)

**Version Requirement:** Specification of acceptable versions (e.g., ~1.0.0, ^1.0.0)

**Topological Sort:** Ordering of packages respecting dependency relationships (dependencies before dependents)

**MRE:** Minimum Reproducible Example (test that reproduces a specific bug)

---

**Next Steps:**
1. Review and approve this specification
2. Begin [Increment 2: Data Structures](./implementation_plan.md#increment-2-core-data-structures-3-days)
3. Follow TDD approach with test-first development
