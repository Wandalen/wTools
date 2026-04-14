# Test Scenarios: Dependency Staleness Detection

**Purpose:** Comprehensive test matrix for publishing algorithm enhancement
**Coverage:** 40+ scenarios across all phases
**Organization:** By category and complexity

---

## Category 1: Basic Staleness Detection (10 scenarios)

### S1.1: Simple Stale Dependency

```yaml
Given:
  - Package A v1.0.0 depends on B "~1.0.0"
  - Package B v1.0.0 (workspace)
  - B is being published (in publish set)
When:
  - detect_stale_dependencies(workspace, {B})
Then:
  - A is detected as stale
  - Reason: BeingPublished
  - stale_deps: [B]
```

### S1.2: Incompatible Version (Tilde Requirement)

```yaml
Given:
  - Package A v1.0.0 depends on B "~1.0.0"
  - Package B v1.1.0 (workspace)
  - B not being published
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is detected as stale
  - Reason: IncompatibleVersion
  - Required: ~1.0.0, Workspace: 1.1.0
```

### S1.3: Compatible Version (Caret Requirement)

```yaml
Given:
  - Package A v1.0.0 depends on B "^1.0.0"
  - Package B v1.0.1 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is NOT detected as stale
  - Reason: ^1.0.0 matches 1.0.1
```

### S1.4: Multiple Stale Dependencies

```yaml
Given:
  - Package A depends on B "~1.0.0", C "~2.0.0"
  - B v1.1.0, C v2.1.0 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is detected as stale
  - stale_deps: [B, C]
  - Both IncompatibleVersion
```

### S1.5: Dev Dependency Staleness

```yaml
Given:
  - Package A has dev-dependency B "~1.0.0"
  - Package B v1.1.0 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is detected as stale
  - Reason: Dev dependency also checked
```

### S1.6: Build Dependency Staleness

```yaml
Given:
  - Package A has build-dependency B "~1.0.0"
  - Package B v1.1.0 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is detected as stale
  - Reason: Build dependency also checked
```

### S1.7: External Dependency (Not Stale)

```yaml
Given:
  - Package A depends on external_crate "~1.0.0"
  - external_crate v1.1.0 (crates.io, not in workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is NOT detected as stale
  - Reason: External dependencies ignored
```

### S1.8: Exact Version Match (Compatible)

```yaml
Given:
  - Package A depends on B "= 1.0.0"
  - Package B v1.0.0 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is NOT stale
```

### S1.9: Exact Version Mismatch (Incompatible)

```yaml
Given:
  - Package A depends on B "= 1.0.0"
  - Package B v1.0.1 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is stale
  - Reason: IncompatibleVersion
```

### S1.10: Wildcard Requirement

```yaml
Given:
  - Package A depends on B "1.*"
  - Package B v1.5.0 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A is NOT stale
  - Reason: 1.* matches 1.5.0
```

---

## Category 2: Cascade Publishing (10 scenarios)

### S2.1: Linear Cascade

```yaml
Given:
  - A depends on B
  - B depends on C
  - C depends on D
  - D has local changes
When:
  - compute_publish_closure(workspace, {D})
Then:
  - Publish set: [D, C, B, A]
  - D: LocalChanges
  - C: CascadeEffect(D)
  - B: CascadeEffect(C)
  - A: CascadeEffect(B)
```

### S2.2: Diamond Dependency

```yaml
Given:
  - A depends on B, C
  - B depends on D
  - C depends on D
  - D has local changes
When:
  - compute_publish_closure(workspace, {D})
Then:
  - Publish set: [D, B, C, A]
  - Order: D first, then B and C (either order), then A
```

### S2.3: Wide Cascade

```yaml
Given:
  - A, B, C all depend on D
  - D has local changes
When:
  - compute_publish_closure(workspace, {D})
Then:
  - Publish set: [D, A, B, C]
  - D: LocalChanges
  - A, B, C: CascadeEffect(D)
```

### S2.4: Deep Cascade (5 levels)

```yaml
Given:
  - E depends on D depends on C depends on B depends on A
  - A has local changes
When:
  - compute_publish_closure(workspace, {A})
Then:
  - Publish set: [A, B, C, D, E]
  - All cascade from A
```

### S2.5: Partial Cascade (Compatible Version)

```yaml
Given:
  - A depends on B "^1.0.0"
  - B v1.0.0 → v1.0.1 (patch bump)
When:
  - compute_publish_closure(workspace, {B})
Then:
  - Publish set: [B] only
  - A NOT included (compatible)
```

### S2.6: Full Cascade (Incompatible Version)

```yaml
Given:
  - A depends on B "~1.0.0"
  - B v1.0.0 → v1.1.0 (minor bump)
When:
  - compute_publish_closure(workspace, {B})
Then:
  - Publish set: [B, A]
  - B: LocalChanges
  - A: StaleDependencies(B)
```

### S2.7: Multiple Triggers

```yaml
Given:
  - C depends on A, B
  - A has local changes
  - B has local changes
When:
  - compute_publish_closure(workspace, {A, B})
Then:
  - Publish set: [A, B, C]
  - C: CascadeEffect([A, B])
```

### S2.8: No Cascade (Isolated Change)

```yaml
Given:
  - A, B, C are independent (no dependencies)
  - A has local changes
When:
  - compute_publish_closure(workspace, {A})
Then:
  - Publish set: [A] only
  - B, C not included
```

### S2.9: Convergence Test (Fixed Point)

```yaml
Given:
  - Complex dependency graph (20 packages)
  - 1 package has local changes
When:
  - compute_publish_closure(workspace, {changed_pkg})
Then:
  - Algorithm converges in <2×20 iterations
  - No infinite loop
```

### S2.10: Max Iteration Guard

```yaml
Given:
  - Artificially broken graph (simulated)
When:
  - compute_publish_closure(workspace, {pkg})
  - Iterations exceed max (2 × package_count)
Then:
  - Error: "Closure computation did not converge"
```

---

## Category 3: Original Bug Scenario (5 scenarios)

### S3.1: willbe-wca-former Conflict (Exact Reproduction)

```yaml
Given:
  - willbe v0.28.0 depends on wca "~0.36.0", former "~2.37.0"
  - wca v0.36.0 depends on former "~2.36.0"
  - former v2.36.0 (initial state)
When:
  - former bumped 2.36.0 → 2.37.0
  - Build publish plan
Then:
  - WITHOUT FIX: Version conflict (cargo fails)
  - WITH FIX: Publish set [former, wca, willbe]
  - wca: StaleDependencies(former)
  - willbe: CascadeEffect(wca)
```

### S3.2: Tilde Requirement Breaking Change

```yaml
Given:
  - Package A depends on B "~X.Y.0"
  - B version X.Y.0
When:
  - B bumped to X.(Y+1).0 (minor bump)
Then:
  - A detected as stale
  - Reason: ~X.Y.0 doesn't match X.(Y+1).0
```

### S3.3: Caret Requirement Preserving Compatibility

```yaml
Given:
  - Package A depends on B "^X.Y.0"
  - B version X.Y.0
When:
  - B bumped to X.(Y+1).0 (minor bump)
Then:
  - A NOT stale
  - Reason: ^X.Y.0 matches X.(Y+1).0
```

### S3.4: Workspace-Only Detection

```yaml
Given:
  - Package A depends on workspace_pkg "~1.0.0", external_pkg "~1.0.0"
  - workspace_pkg v1.1.0, external_pkg v1.1.0 (crates.io)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A stale due to workspace_pkg only
  - external_pkg ignored
```

### S3.5: Publish Batch Staleness

```yaml
Given:
  - A depends on B "~1.0.0"
  - B v1.0.0 (workspace and crates.io match)
  - B being published (in current batch)
When:
  - detect_stale_dependencies(workspace, {B})
Then:
  - A detected as stale
  - Reason: BeingPublished (even though version matches)
```

---

## Category 4: Edge Cases (10 scenarios)

### S4.1: Circular Dependency (2 packages)

```yaml
Given:
  - A depends on B
  - B depends on A
When:
  - topological_sort(workspace, {A, B})
Then:
  - Error: "Circular dependency detected"
  - Publishing aborts
```

### S4.2: Circular Dependency (3 packages)

```yaml
Given:
  - A depends on B
  - B depends on C
  - C depends on A
When:
  - topological_sort(workspace, {A, B, C})
Then:
  - Error: "Circular dependency detected"
```

### S4.3: Self-Dependency

```yaml
Given:
  - Package A depends on A (self)
When:
  - Build publish plan
Then:
  - Error or ignore (implementation choice)
  - Document behavior in spec
```

### S4.4: Empty Workspace

```yaml
Given:
  - Workspace with 0 packages
When:
  - Build publish plan
Then:
  - Empty plan
  - "Nothing to publish"
```

### S4.5: All Packages Already Published

```yaml
Given:
  - 10 packages in workspace
  - All versions match crates.io
  - No local changes
When:
  - Build publish plan
Then:
  - Empty plan
  - "Nothing to publish"
```

### S4.6: Package with publish = false

```yaml
Given:
  - Package A has publish = false in Cargo.toml
  - A has local changes
When:
  - Build publish plan
Then:
  - A NOT included
  - Respects publish = false
```

### S4.7: Mixed Dependency Types

```yaml
Given:
  - A has normal dep on B "~1.0.0"
  - A has dev dep on C "~2.0.0"
  - A has build dep on D "~3.0.0"
  - B, C, D all bumped (incompatible)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A stale with all three listed
```

### S4.8: Version Requirement Range

```yaml
Given:
  - A depends on B ">=1.0.0, <2.0.0"
  - B v1.5.0 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - A NOT stale (1.5.0 in range)
```

### S4.9: Pre-release Versions

```yaml
Given:
  - A depends on B "~1.0.0"
  - B v1.0.0-alpha.1 (workspace)
When:
  - detect_stale_dependencies(workspace, {})
Then:
  - Implementation-defined behavior
  - Document in spec
```

### S4.10: Huge Workspace (Performance)

```yaml
Given:
  - 100 packages in workspace
  - Complex dependency graph
  - 1 package has local changes
When:
  - compute_publish_closure(workspace, {changed})
  - Measure time
Then:
  - Completes in <10 seconds
  - Memory usage reasonable
```

---

## Category 5: Integration Scenarios (5 scenarios)

### S5.1: Full Publishing Flow

```yaml
Given:
  - Complete workspace with staleness
When:
  - Run `will .publish dry:1`
Then:
  - Display shows all affected packages
  - Reasons displayed for each
  - Correct topological order
  - Dry run succeeds
```

### S5.2: Actual Publishing (Not Dry)

```yaml
Given:
  - Complete workspace ready for publish
When:
  - Run `will .publish dry:0`
Then:
  - Packages published in correct order
  - Git commits created
  - Versions bumped
  - No cargo errors
```

### S5.3: Publish Failure Midway

```yaml
Given:
  - 5 packages to publish
  - Package 3 fails (simulated)
When:
  - Run publish command
Then:
  - Packages 1-2 published
  - Package 3 fails
  - Packages 4-5 NOT published
  - State consistent
```

### S5.4: Display Output Verification

```yaml
Given:
  - Packages with various reasons
When:
  - Build and display publish plan
Then:
  - Output shows:
    - Tree view of dependencies
    - Package list with versions
    - Reasons for each package
    - Clear formatting
```

### S5.5: Manual Testing Checklist

```
Workspace: wTools
Steps:
1. Create test scenario:
   - Bump former version
   - Do NOT bump wca
   - Check willbe
2. Run: will .publish dry:1
3. Verify:
   - [ ] wca detected for publishing
   - [ ] Reason shown: "Stale dependencies: former"
   - [ ] willbe detected for publishing
   - [ ] Reason shown: "Cascade from: wca"
   - [ ] Order: former → wca → willbe
4. Run: will .publish dry:0
5. Verify:
   - [ ] All publish successfully
   - [ ] No version conflicts
   - [ ] Git commits created
   - [ ] crates.io updated
```

---

## Test Organization Matrix

### By Phase

| Phase | Scenarios | Purpose |
|-------|-----------|---------|
| Phase 2 (Staleness) | S1.1-S1.10 | Basic detection |
| Phase 3 (Closure) | S2.1-S2.10 | Cascade effects |
| Integration | S5.1-S5.5 | End-to-end |
| Edge Cases | S4.1-S4.10 | Error handling |
| Bug Reproduction | S3.1-S3.5 | Original issue |

### By Priority

| Priority | Scenarios | When to Implement |
|----------|-----------|-------------------|
| P0 (Critical) | S3.1, S1.2, S2.6 | Increment 4 |
| P1 (High) | S2.1-S2.4, S1.1, S1.3 | Increment 5 |
| P2 (Medium) | S1.4-S1.10, S2.5-S2.10 | Increment 6 |
| P3 (Low) | S4.1-S4.10 | Increment 7 |
| P4 (Integration) | S5.1-S5.5 | Increment 9 |

### By Complexity

| Complexity | Scenarios | Effort |
|------------|-----------|--------|
| Simple | S1.1-S1.3, S1.8, S1.9 | 1 hour each |
| Medium | S1.4-S1.7, S2.1-S2.4 | 2 hours each |
| Complex | S2.5-S2.10, S3.1, S4.10 | 4 hours each |
| Very Complex | S4.1-S4.3, S5.2 | 8 hours each |

---

## Test Fixtures

### Fixture 1: Simple Workspace

```toml
# tests/fixtures/simple_workspace.toml
[workspace]
members = ["pkg_a", "pkg_b"]

[workspace.dependencies]
pkg_a = { version = "1.0.0", path = "pkg_a" }
pkg_b = { version = "1.0.0", path = "pkg_b" }
```

### Fixture 2: willbe-wca-former Workspace

```toml
# tests/fixtures/willbe_wca_former.toml
[workspace]
members = ["former", "wca", "willbe"]

[workspace.dependencies]
former = { version = "2.37.0", path = "former" }
wca = { version = "0.36.0", path = "wca" }
willbe = { version = "0.28.0", path = "willbe" }
```

### Fixture 3: Diamond Dependency

```toml
# tests/fixtures/diamond.toml
[workspace]
members = ["a", "b", "c", "d"]

# D at bottom, B and C in middle, A at top
```

---

## Automation

### Test Execution Commands

```bash
# Run all scenarios
cargo test --test dependency_staleness_test
cargo test --test cascade_test
cargo test --test integration_test
cargo test --test edge_cases_test

# Run specific category
cargo test s1_  # Basic staleness
cargo test s2_  # Cascade
cargo test s3_  # Original bug
cargo test s4_  # Edge cases
cargo test s5_  # Integration

# Run priority
cargo test --test '*' -- --include-ignored p0_
cargo test --test '*' -- --include-ignored p1_

# Performance tests
cargo test --ignored performance
```

### Coverage Target

- Unit tests: >95%
- Integration tests: 100% of critical paths
- Edge cases: All documented scenarios
- Bug reproduction: 100% (MRE tests)

---

## Success Criteria

### Functional

- ✅ All 40+ scenarios implemented as tests
- ✅ All tests passing
- ✅ Original bug scenario resolved
- ✅ No false positives (compatible versions)
- ✅ No false negatives (stale deps detected)

### Non-Functional

- ✅ Test execution <5 minutes total
- ✅ Clear test names and documentation
- ✅ Reusable test utilities
- ✅ Fixtures for common scenarios

---

## Test Documentation Template

For each test:

```rust
/// [Scenario ID]: [Scenario Name]
///
/// ## Purpose
/// [What this test verifies]
///
/// ## Setup
/// [Initial workspace state]
///
/// ## Action
/// [What operation is performed]
///
/// ## Expected Result
/// [What should happen]
///
/// ## Related Scenarios
/// [Links to similar tests]
// test_kind: [unit|integration|performance]
#[test]
fn scenario_name() {
  // Arrange
  let workspace = build_fixture();

  // Act
  let result = operation(workspace);

  // Assert
  assert_eq!(result, expected);
}
```

---

## Appendix: Test Utility Functions

### Workspace Builder

```rust
pub fn build_workspace() -> WorkspaceBuilder {
  WorkspaceBuilder::new()
}

impl WorkspaceBuilder {
  pub fn package(self, name: &str) -> PackageBuilder { }
  pub fn build(self) -> Workspace { }
}

impl PackageBuilder {
  pub fn version(self, v: &str) -> Self { }
  pub fn dependency(self, name: &str, req: &str) -> Self { }
  pub fn dev_dependency(self, name: &str, req: &str) -> Self { }
  pub fn build_dependency(self, name: &str, req: &str) -> Self { }
  pub fn build(self) -> Package { }
}
```

### Assertion Helpers

```rust
pub fn assert_stale(
  stale_map: &HashMap<PackageName, Vec<StaleDependency>>,
  pkg: &str,
  dep: &str,
) {
  assert!(stale_map.contains_key(&pkg.into()));
  let stale_deps = &stale_map[&pkg.into()];
  assert!(stale_deps.iter().any(|d| d.name == dep.into()));
}

pub fn assert_not_stale(
  stale_map: &HashMap<PackageName, Vec<StaleDependency>>,
  pkg: &str,
) {
  assert!(!stale_map.contains_key(&pkg.into()));
}

pub fn assert_publish_order(
  plan: &PublishPlan,
  expected_order: &[&str],
) {
  let actual: Vec<_> = plan.plans.iter()
    .map(|p| p.package_name.as_str())
    .collect();
  assert_eq!(actual, expected_order);
}
```
