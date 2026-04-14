# Publishing Algorithm Specification

**Version:** 2.0 (Enhanced with Dependency Staleness Detection)
**Status:** Draft
**Author:** Development Team
**Date:** 2025-11-08

---

## 1. Overview

The willbe publishing algorithm determines which packages in a workspace need to be published to crates.io and in what order. This specification defines the complete algorithm including dependency staleness detection and cascade publishing.

---

## 2. Core Concepts

### 2.1 Package States

A package in the workspace can be in one of these states:

- **Up-to-date**: Local version matches published version, no changes, dependencies compatible
- **Modified**: Local source code has changed since last publish
- **Bumped**: Version has been explicitly incremented in Cargo.toml
- **Stale**: Dependencies reference outdated versions compared to workspace
- **Cascaded**: Must be republished due to dependency being published in current batch

### 2.2 Dependency Types

The algorithm considers three dependency types:

1. **Normal dependencies** (`[dependencies]`)
2. **Development dependencies** (`[dev-dependencies]`)
3. **Build dependencies** (`[build-dependencies]`)

### 2.3 Version Requirements

Semver version requirements must be evaluated according to Cargo semantics:

| Requirement | Example | Matches |
|-------------|---------|---------|
| Exact | `= 1.2.3` | Only 1.2.3 |
| Caret | `^1.2.3` | >=1.2.3, <2.0.0 |
| Tilde | `~1.2.3` | >=1.2.3, <1.3.0 |
| Wildcard | `1.*` | >=1.0.0, <2.0.0 |
| Greater/Less | `>=1.2.3` | As specified |

### 2.4 Workspace vs Published Versions

- **Workspace version**: Version specified in `[workspace.dependencies]` or package's `Cargo.toml`
- **Published version**: Version currently available on crates.io
- **Required version**: Version requirement specified in dependent's `Cargo.toml`

---

## 3. Algorithm Specification

### 3.1 High-Level Flow

```
┌─────────────────────────────────────────────────┐
│ Phase 1: Initial Detection                     │
│ - Detect locally modified packages              │
│ - Detect version-bumped packages                │
│ └─→ Set S₁ = packages with local changes       │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│ Phase 2: Dependency Staleness Detection        │
│ - For each package P not in S₁:                │
│   - Check all dependencies D of P               │
│   - If D in S₁: mark P as stale                 │
│   - If workspace_version(D) ∉ required(P,D):    │
│       mark P as stale                           │
│ └─→ Set S₂ = S₁ ∪ packages with stale deps     │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│ Phase 3: Transitive Closure Computation        │
│ - Repeat until no changes:                      │
│   - For each package P in S₂:                   │
│     - Find all dependents D of P                │
│     - Add D to S₂ if not already present        │
│ └─→ Set S₃ = transitive closure of S₂          │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│ Phase 4: Topological Sort                      │
│ - Build dependency graph from S₃                │
│ - Perform topological sort                      │
│ - Detect cycles (error if found)                │
│ └─→ Ordered list L = [P₁, P₂, ..., Pₙ]         │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│ Phase 5: Publish Plan Construction             │
│ - For each package P in L:                      │
│   - Calculate new_version = bump(old_version)   │
│   - Determine publish reasons                   │
│   - Build PackagePublishInstruction             │
│ └─→ PublishPlan with instructions               │
└─────────────────────────────────────────────────┘
```

### 3.2 Phase 1: Initial Detection

**Input:** Workspace W
**Output:** Set S₁ of packages needing publish due to local changes

**Algorithm:**

```
function initial_detection(workspace: Workspace) -> Set<PackageName>:
  result = {}

  for package in workspace.packages():
    reasons = []

    # Check for local modifications
    if has_uncommitted_changes(package.path):
      reasons.append(LocalChanges)

    # Check for version bump
    if version_bumped(package):
      reasons.append(VersionBump)

    if not reasons.empty():
      result.insert(package.name, reasons)

  return result
```

**Specification:**

1. **Local changes detection**: Check git status in package directory
2. **Version bump detection**: Compare package version with last published version on crates.io
3. **Exclusions**: Packages marked with `publish = false` are skipped

### 3.3 Phase 2: Dependency Staleness Detection

**Input:** Workspace W, Set S₁
**Output:** Set S₂ = S₁ ∪ stale packages

**Algorithm:**

```
function detect_staleness(workspace: Workspace, publishing: Set<PackageName>) -> Set<PackageName>:
  result = publishing.clone()

  for package in workspace.packages():
    if package.name in result:
      continue  # Already marked for publishing

    stale_deps = []

    for dep_kind in [Normal, Dev, Build]:
      for dep in package.dependencies(dep_kind):
        # Only check workspace dependencies
        if not workspace.contains(dep.name):
          continue

        workspace_version = workspace.version_of(dep.name)

        # Check 1: Is dependency being published?
        if dep.name in publishing:
          stale_deps.append(StaleDependency {
            name: dep.name,
            required: dep.version_req,
            workspace_version: workspace_version,
            reason: BeingPublished
          })
          continue

        # Check 2: Version compatibility
        if not dep.version_req.matches(workspace_version):
          stale_deps.append(StaleDependency {
            name: dep.name,
            required: dep.version_req,
            workspace_version: workspace_version,
            reason: IncompatibleVersion
          })

    if not stale_deps.empty():
      result.insert(package.name)
      reasons[package.name] = StaleDependencies(stale_deps)

  return result
```

**Specification:**

1. **Workspace-only**: Only workspace packages are checked for staleness
2. **All dependency types**: Normal, dev, and build dependencies are all checked
3. **Semver matching**: Use strict Cargo semver matching rules
4. **Being published**: If dependency is in current publish batch, dependent is stale

### 3.4 Phase 3: Transitive Closure

**Input:** Workspace W, Set S₂
**Output:** Set S₃ = transitive closure of S₂

**Algorithm:**

```
function compute_closure(workspace: Workspace, initial: Set<PackageName>) -> Set<PackageName>:
  result = initial.clone()
  changed = true
  iteration = 0
  MAX_ITERATIONS = workspace.packages().len() * 2

  while changed and iteration < MAX_ITERATIONS:
    changed = false
    old_size = result.size()

    for pkg_name in result.clone():
      # Find all packages that depend on pkg_name
      for dependent in workspace.find_dependents(pkg_name):
        if dependent.name not in result:
          result.insert(dependent.name)
          reasons[dependent.name] = CascadeEffect(triggered_by: [pkg_name])
          changed = true

    iteration++

    if result.size() == old_size:
      break  # Converged

  if iteration >= MAX_ITERATIONS:
    error("Closure computation did not converge")

  return result
```

**Specification:**

1. **Fixed-point iteration**: Repeat until no new packages added
2. **Convergence guarantee**: Terminates when no changes or max iterations
3. **Max iterations**: `2 × number_of_packages` to prevent infinite loops
4. **Reason tracking**: Record which package triggered the cascade

### 3.5 Phase 4: Topological Sort

**Input:** Workspace W, Set S₃
**Output:** Ordered list L

**Algorithm:**

```
function topological_sort(workspace: Workspace, packages: Set<PackageName>) -> List<PackageName>:
  # Build dependency graph
  graph = {}
  in_degree = {}

  for pkg_name in packages:
    graph[pkg_name] = []
    in_degree[pkg_name] = 0

  for pkg_name in packages:
    package = workspace.find_package(pkg_name)
    for dep in package.all_dependencies():
      if dep.name in packages:
        graph[dep.name].append(pkg_name)
        in_degree[pkg_name]++

  # Kahn's algorithm
  queue = []
  for pkg_name in packages:
    if in_degree[pkg_name] == 0:
      queue.append(pkg_name)

  result = []
  while not queue.empty():
    pkg_name = queue.pop_front()
    result.append(pkg_name)

    for dependent in graph[pkg_name]:
      in_degree[dependent]--
      if in_degree[dependent] == 0:
        queue.append(dependent)

  if result.len() != packages.len():
    error("Circular dependency detected")

  return result
```

**Specification:**

1. **Algorithm**: Kahn's algorithm for topological sorting
2. **Cycle detection**: Error if result length ≠ input length
3. **Stability**: When multiple orderings valid, use alphabetical as tiebreaker
4. **Dependency types**: Consider all dependency types (normal, dev, build)

### 3.6 Phase 5: Publish Plan Construction

**Input:** Workspace W, Ordered list L, Reasons map
**Output:** PublishPlan

**Algorithm:**

```
function build_publish_plan(workspace: Workspace, ordered: List<PackageName>, reasons: Map) -> PublishPlan:
  instructions = []

  for pkg_name in ordered:
    package = workspace.find_package(pkg_name)
    old_version = package.version()
    new_version = old_version.bump_patch()  # Default to patch bump

    instruction = PackagePublishInstruction {
      package_name: pkg_name,
      old_version: old_version,
      new_version: new_version,
      reasons: reasons[pkg_name],
      pack: build_pack_options(package, workspace),
      bump: build_bump_options(package, old_version, new_version, workspace),
      git_options: build_git_options(package, workspace, new_version),
      publish: build_publish_options(package, workspace),
      dry: workspace.config.dry_run,
    }

    instructions.append(instruction)

  return PublishPlan {
    workspace_dir: workspace.root,
    base_temp_dir: workspace.config.temp_dir,
    channel: workspace.config.channel,
    dry: workspace.config.dry_run,
    roots: determine_roots(instructions),
    plans: instructions,
  }
```

**Specification:**

1. **Version bumping**: Default to patch bump (0.1.0 → 0.1.1)
2. **Pack options**: Include temp directory, channel, allow_dirty based on dry mode
3. **Bump options**: Update workspace Cargo.toml and all dependents
4. **Git options**: Commit message format `{package}-v{version}`
5. **Publish options**: Include retry count (default: 2)

---

## 4. Data Structures

### 4.1 PublishReason

```rust
enum PublishReason {
  /// Local source code was modified
  LocalChanges,

  /// Package version was explicitly bumped
  VersionBump,

  /// Dependencies have incompatible versions
  StaleDependencies {
    stale_deps: Vec<StaleDependency>,
  },

  /// Package depends on another being published
  CascadeEffect {
    triggered_by: Vec<PackageName>,
  },
}
```

### 4.2 StaleDependency

```rust
struct StaleDependency {
  /// Dependency name
  name: PackageName,

  /// Version requirement in Cargo.toml
  required: VersionReq,

  /// Current workspace version
  workspace_version: Version,

  /// Reason for staleness
  reason: StaleReason,
}

enum StaleReason {
  /// Workspace version doesn't satisfy requirement
  IncompatibleVersion,

  /// Dependency being published in this batch
  BeingPublished,
}
```

### 4.3 PackagePublishInstruction

```rust
struct PackagePublishInstruction {
  /// Package name
  package_name: PackageName,

  /// Current version
  old_version: Version,

  /// Target version after bump
  new_version: Version,

  /// Why this package needs publishing
  reasons: Vec<PublishReason>,

  /// Cargo pack options
  pack: PackOptions,

  /// Version bump options
  bump: BumpOptions,

  /// Git commit options
  git_options: GitOptions,

  /// Cargo publish options
  publish: PublishOptions,

  /// Dry run mode
  dry: bool,
}
```

---

## 5. Edge Cases & Error Handling

### 5.1 Circular Dependencies

**Scenario:** Package A depends on B, B depends on A

**Behavior:**
- Topological sort detects cycle
- Error: `CircularDependency([A, B])`
- Publishing aborts

**Rationale:** Circular dependencies cannot be published sequentially

### 5.2 Missing Workspace Dependencies

**Scenario:** Package A requires dependency B not in workspace

**Behavior:**
- Dependency B is ignored (assumed published externally)
- Only workspace dependencies trigger staleness

**Rationale:** External dependencies are outside our control

### 5.3 Compatible Version Bumps

**Scenario:** Package A requires B `^1.0.0`, B bumps `1.0.0 → 1.0.1`

**Behavior:**
- A is NOT marked as stale (^1.0.0 matches 1.0.1)
- A is NOT republished

**Rationale:** Semver compatibility preserved

### 5.4 Incompatible Version Bumps

**Scenario:** Package A requires B `~1.0.0`, B bumps `1.0.0 → 1.1.0`

**Behavior:**
- A IS marked as stale (~1.0.0 doesn't match 1.1.0)
- A IS republished with updated requirement

**Rationale:** Semver compatibility broken

### 5.5 Empty Publish Set

**Scenario:** No packages detected for publishing

**Behavior:**
- Display "Nothing to publish"
- Exit with code 0 (success)

**Rationale:** Valid state, not an error

### 5.6 Publish Failures

**Scenario:** Package P fails to publish midway through sequence

**Behavior:**
- Publish sequence stops at P
- All packages before P are published
- All packages after P are NOT published
- Error is displayed with retry instructions

**Rationale:** Prevents inconsistent state on crates.io

---

## 6. Performance Characteristics

### 6.1 Time Complexity

| Phase | Complexity | Explanation |
|-------|------------|-------------|
| Initial Detection | O(n) | n = number of packages |
| Staleness Detection | O(n × d) | d = avg dependencies per package |
| Closure Computation | O(n²) worst | Fixed-point iteration |
| Topological Sort | O(n + e) | e = dependency edges |
| Plan Construction | O(n) | Linear in package count |

**Overall:** O(n²) worst case

### 6.2 Space Complexity

- **Package set**: O(n)
- **Dependency graph**: O(n + e)
- **Reasons map**: O(n)
- **Instructions**: O(n)

**Overall:** O(n + e)

### 6.3 Optimization Strategies

1. **Caching**: Memoize workspace version lookups
2. **Early termination**: Stop closure iteration when converged
3. **Lazy evaluation**: Only build full graph for packages in publish set
4. **Parallel queries**: Fetch crates.io versions in parallel

---

## 7. Acceptance Criteria

### 7.1 Functional Correctness

✅ **AC1:** All locally modified packages detected
✅ **AC2:** All stale dependencies detected
✅ **AC3:** Transitive closure computed correctly
✅ **AC4:** Publish order respects dependencies
✅ **AC5:** Version conflicts prevented
✅ **AC6:** Circular dependencies detected and rejected

### 7.2 Error Handling

✅ **AC7:** Graceful handling of missing dependencies
✅ **AC8:** Clear error messages for failures
✅ **AC9:** Publish failures don't leave inconsistent state

### 7.3 Performance

✅ **AC10:** Algorithm completes in <10s for 100 packages
✅ **AC11:** No infinite loops (max iteration guards)

### 7.4 User Experience

✅ **AC12:** Clear display of publish reasons
✅ **AC13:** Tree view shows all affected packages
✅ **AC14:** Dry-run mode works correctly

---

## 8. Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2024-XX-XX | Initial algorithm (no staleness detection) |
| 2.0 | 2025-11-08 | Add dependency staleness detection and cascade publishing |

---

## 9. References

- **Cargo Version Requirements**: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
- **Topological Sort**: Kahn's algorithm (1962)
- **Semver Specification**: https://semver.org/
