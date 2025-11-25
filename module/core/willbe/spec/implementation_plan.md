# Implementation Plan: Dependency Staleness Detection

**Project:** willbe Publishing Algorithm Enhancement
**Goal:** Fix missing dependency staleness detection
**Approach:** Incremental TDD with specification-first development
**Estimated Duration:** 18-26 days

---

## Increment Structure

Each increment follows this template:

```
Increment N: [Name]
├─ Goal: What we achieve
├─ Input State: System before this increment
├─ Output State: System after this increment
├─ Implementation Steps: Ordered list of tasks
├─ Verification: How to verify success
└─ Rollback: How to undo if needed
```

---

## INCREMENT 1: Project Setup & Specification (2 days)

### Goal

Establish project foundation with complete specification and test infrastructure.

### Input State

- Existing publishing algorithm without staleness detection
- No formal specification
- Bug exists: version conflicts during publishing

### Output State

- Complete specification in `spec/` directory
- Test directory structure created
- Development environment ready

### Implementation Steps

**Step 1.1: Create Specification Directory**

```bash
mkdir -p spec/
touch spec/index.md
touch spec/publishing_algorithm.md  # Already created
touch spec/test_scenarios.md
touch spec/architecture.md
```

**Step 1.2: Write Test Scenarios Document**

Create `spec/test_scenarios.md` with comprehensive test matrix covering:
- Basic staleness detection (10 scenarios)
- Cascade publishing (8 scenarios)
- Edge cases (12 scenarios)
- Performance tests (5 scenarios)

**Step 1.3: Create Test Directory Structure**

```bash
mkdir -p tests/inc/publish/
touch tests/inc/publish/mod.rs
touch tests/inc/publish/dependency_staleness_test.rs
touch tests/inc/publish/cascade_test.rs
touch tests/inc/publish/version_conflict_test.rs
touch tests/inc/publish/edge_cases_test.rs
```

**Step 1.4: Set Up Test Utilities**

Create `tests/inc/publish/test_utils.rs`:
- Workspace builder for testing
- Package builder with dependencies
- Version requirement helpers
- Assertion utilities

### Verification

```bash
# All files created
ls spec/
ls tests/inc/publish/

# Specification readable and complete
cargo doc --no-deps --document-private-items
```

### Deliverables

- ✅ `spec/publishing_algorithm.md` (complete)
- ✅ `spec/test_scenarios.md` (35+ scenarios)
- ✅ `spec/architecture.md` (system design)
- ✅ `tests/inc/publish/test_utils.rs` (helper functions)

---

## INCREMENT 2: Core Data Structures (3 days)

### Goal

Implement all data structures needed for staleness detection.

### Input State

- Specification complete
- Test infrastructure ready
- No staleness-related types

### Output State

- `PublishReason` enum implemented
- `StaleDependency` struct implemented
- `StaleReason` enum implemented
- Unit tests passing

### Implementation Steps

**Step 2.1: Create `publish_reason.rs` Module**

Location: `src/entity/publish_reason.rs`

```rust
/// Reasons why a package needs publishing
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum PublishReason
{
  /// Local source code was modified
  LocalChanges,

  /// Package version was explicitly bumped
  VersionBump,

  /// One or more dependencies have incompatible versions
  StaleDependencies
  {
    /// List of dependencies causing staleness
    stale_deps: Vec< StaleDependency >,
  },

  /// Package depends on another package being published in this batch
  CascadeEffect
  {
    /// Package(s) triggering the cascade
    triggered_by: Vec< PackageName >,
  },
}

impl PublishReason
{
  /// Returns true if this is a local change reason
  pub fn is_local_change( &self ) -> bool
  {
    matches!( self, PublishReason::LocalChanges )
  }

  /// Returns true if this is a staleness reason
  pub fn is_stale( &self ) -> bool
  {
    matches!( self, PublishReason::StaleDependencies { .. } )
  }

  /// Returns true if this is a cascade reason
  pub fn is_cascade( &self ) -> bool
  {
    matches!( self, PublishReason::CascadeEffect { .. } )
  }

  /// Human-readable description
  pub fn description( &self ) -> String
  {
    match self
    {
      PublishReason::LocalChanges => "Local source code modified".to_string(),
      PublishReason::VersionBump => "Version explicitly bumped".to_string(),
      PublishReason::StaleDependencies { stale_deps } =>
        format!( "Stale dependencies: {}", stale_deps.len() ),
      PublishReason::CascadeEffect { triggered_by } =>
        format!( "Cascade from: {}", triggered_by.join( ", " ) ),
    }
  }
}
```

**Step 2.2: Create `stale_dependency.rs` Module**

Location: `src/entity/stale_dependency.rs`

```rust
/// Details about a stale dependency
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct StaleDependency
{
  /// Name of the dependency
  pub name: PackageName,

  /// Version requirement in dependent's Cargo.toml
  pub required: VersionReq,

  /// Actual version in workspace
  pub workspace_version: Version,

  /// Why it's considered stale
  pub reason: StaleReason,
}

/// Why a dependency is stale
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum StaleReason
{
  /// Workspace version doesn't satisfy requirement
  IncompatibleVersion,

  /// Dependency is being published in this publish batch
  BeingPublished,
}

impl StaleDependency
{
  /// Check if workspace version satisfies requirement
  pub fn is_compatible( &self ) -> bool
  {
    self.required.matches( &self.workspace_version )
  }

  /// Human-readable description
  pub fn description( &self ) -> String
  {
    match self.reason
    {
      StaleReason::IncompatibleVersion =>
        format!(
          "{}: required {}, workspace has {}",
          self.name, self.required, self.workspace_version
        ),
      StaleReason::BeingPublished =>
        format!(
          "{}: dependency being published in this batch",
          self.name
        ),
    }
  }
}
```

**Step 2.3: Update `publish.rs` to Use New Types**

Add to `src/entity/publish.rs`:

```rust
/// Enhanced publish instruction with reasons
#[ derive( Debug, Clone ) ]
pub struct PackagePublishInstruction
{
  pub package_name: PackageName,
  pub old_version: Version,
  pub new_version: Version,

  /// NEW: Why this package needs publishing
  pub reasons: Vec< PublishReason >,

  // Existing fields...
  pub pack: cargo::PackOptions,
  pub bump: version::BumpOptions,
  pub git_options: git::GitOptions,
  pub publish: cargo::PublishOptions,
  pub dry: bool,
}
```

**Step 2.4: Write Unit Tests**

Create `tests/inc/publish/data_structures_test.rs`:

```rust
#[ test ]
fn publish_reason_local_changes()
{
  let reason = PublishReason::LocalChanges;
  assert!( reason.is_local_change() );
  assert!( !reason.is_stale() );
  assert!( !reason.is_cascade() );
}

#[ test ]
fn stale_dependency_incompatible_version()
{
  let stale = StaleDependency
  {
    name: "former".into(),
    required: VersionReq::parse( "~2.36.0" ).unwrap(),
    workspace_version: Version::parse( "2.37.0" ).unwrap(),
    reason: StaleReason::IncompatibleVersion,
  };

  assert!( !stale.is_compatible() );
  assert!( stale.description().contains( "required ~2.36.0" ) );
}

#[ test ]
fn stale_dependency_being_published()
{
  let stale = StaleDependency
  {
    name: "former".into(),
    required: VersionReq::parse( "~2.37.0" ).unwrap(),
    workspace_version: Version::parse( "2.37.0" ).unwrap(),
    reason: StaleReason::BeingPublished,
  };

  assert!( stale.is_compatible() );  // Version matches
  assert!( stale.description().contains( "being published" ) );
}
```

### Verification

```bash
# Compile check
cargo check --all-features

# Run unit tests
w3 .test level::1

# Verify types exist
cargo doc --no-deps --document-private-items --open
# Navigate to entity::publish_reason, entity::stale_dependency
```

### Deliverables

- ✅ `src/entity/publish_reason.rs` (complete with tests)
- ✅ `src/entity/stale_dependency.rs` (complete with tests)
- ✅ Updated `src/entity/publish.rs` (with reasons field)
- ✅ `tests/inc/publish/data_structures_test.rs` (passing)

---

## INCREMENT 3: Semver Utilities (2 days)

### Goal

Implement semver matching and version requirement utilities.

### Input State

- Data structures exist
- No semver utilities
- Relying on `semver` crate primitives

### Output State

- Semver matching helper functions
- Version requirement parsing utilities
- Comprehensive semver tests

### Implementation Steps

**Step 3.1: Create `semver_utils.rs` Module**

Location: `src/tool/semver_utils.rs`

```rust
use semver::{ Version, VersionReq };

/// Check if version satisfies requirement
pub fn matches( req: &VersionReq, version: &Version ) -> bool
{
  req.matches( version )
}

/// Parse version requirement from string
pub fn parse_req( s: &str ) -> Result< VersionReq, semver::Error >
{
  VersionReq::parse( s )
}

/// Parse version from string
pub fn parse_version( s: &str ) -> Result< Version, semver::Error >
{
  Version::parse( s )
}

/// Check if two version requirements are compatible
pub fn requirements_compatible( req1: &VersionReq, req2: &VersionReq ) -> bool
{
  // Two requirements are compatible if there exists a version
  // that satisfies both
  // Simplified check: see if max of one satisfies min of other
  // Full implementation requires range intersection
  todo!( "Implement full compatibility check" )
}

/// Get the maximum version that satisfies a requirement
pub fn max_satisfying< 'a, I >( req: &VersionReq, versions: I ) -> Option< &'a Version >
where
  I: IntoIterator< Item = &'a Version >,
{
  versions
  .into_iter()
  .filter( | v | req.matches( v ) )
  .max()
}
```

**Step 3.2: Write Comprehensive Semver Tests**

Create `tests/inc/tool/semver_test.rs`:

```rust
#[ test ]
fn caret_requirement_matching()
{
  let req = parse_req( "^1.2.3" ).unwrap();

  assert!( matches( &req, &parse_version( "1.2.3" ).unwrap() ) );
  assert!( matches( &req, &parse_version( "1.2.4" ).unwrap() ) );
  assert!( matches( &req, &parse_version( "1.3.0" ).unwrap() ) );
  assert!( !matches( &req, &parse_version( "2.0.0" ).unwrap() ) );
  assert!( !matches( &req, &parse_version( "1.2.2" ).unwrap() ) );
}

#[ test ]
fn tilde_requirement_matching()
{
  let req = parse_req( "~1.2.3" ).unwrap();

  assert!( matches( &req, &parse_version( "1.2.3" ).unwrap() ) );
  assert!( matches( &req, &parse_version( "1.2.4" ).unwrap() ) );
  assert!( !matches( &req, &parse_version( "1.3.0" ).unwrap() ) );
  assert!( !matches( &req, &parse_version( "2.0.0" ).unwrap() ) );
}

// Test the original bug scenario
#[ test ]
fn tilde_breaks_on_minor_bump()
{
  let req = parse_req( "~2.36.0" ).unwrap();
  let old_version = parse_version( "2.36.0" ).unwrap();
  let new_version = parse_version( "2.37.0" ).unwrap();

  assert!( matches( &req, &old_version ) );
  assert!( !matches( &req, &new_version ) );  // THIS IS THE BUG!
}
```

### Verification

```bash
# Run semver tests
cargo test --test semver_test

# Verify level 1 passes
w3 .test level::1
```

### Deliverables

- ✅ `src/tool/semver_utils.rs` (utilities)
- ✅ `tests/inc/tool/semver_test.rs` (30+ test cases)

---

## INCREMENT 4: Dependency Staleness Detection (4 days)

### Goal

Implement core staleness detection algorithm.

### Input State

- Data structures exist
- Semver utilities ready
- No staleness detection logic

### Output State

- `detect_stale_dependencies()` function working
- Staleness detection integrated into workspace
- Tests passing for simple staleness

### Implementation Steps

**Step 4.1: Add Workspace Helper Methods**

Update `src/entity/workspace.rs`:

```rust
impl Workspace
{
  // ... existing methods ...

  /// Find package by name
  pub fn find_package( &self, name: &PackageName ) -> Option< WorkspacePackageRef >
  {
    self.packages().find( | p | p.name() == Some( name ) )
  }

  /// Get version of a workspace package
  pub fn version_of( &self, name: &PackageName ) -> Option< Version >
  {
    self.find_package( name ).and_then( | p | p.version() )
  }

  /// Find all packages that depend on the given package
  pub fn find_dependents( &self, name: &PackageName ) -> Vec< WorkspacePackageRef >
  {
    self
    .packages()
    .filter( | p |
    {
      p.dependencies( DependencyKind::Normal ).any( | d | &d.name == name )
      || p.dependencies( DependencyKind::Dev ).any( | d | &d.name == name )
      || p.dependencies( DependencyKind::Build ).any( | d | &d.name == name )
    })
    .collect()
  }
}
```

**Step 4.2: Implement Staleness Detection**

Create `src/entity/staleness.rs`:

```rust
use crate:: *;
use std::collections::{ HashMap, HashSet };

/// Detect packages with stale dependencies
pub fn detect_stale_dependencies
(
  workspace: &Workspace,
  already_publishing: &HashSet< PackageName >,
) -> HashMap< PackageName, Vec< StaleDependency > >
{
  let mut stale_map = HashMap::new();

  for package in workspace.packages()
  {
    let pkg_name = match package.name()
    {
      Some( name ) => name,
      None => continue,
    };

    // Skip if already marked for publishing
    if already_publishing.contains( &pkg_name )
    {
      continue;
    }

    let mut stale_deps = Vec::new();

    // Check all dependency kinds
    for dep_kind in [ DependencyKind::Normal, DependencyKind::Dev, DependencyKind::Build ]
    {
      for dep in package.dependencies( dep_kind )
      {
        // Only check workspace dependencies
        let workspace_version = match workspace.version_of( &dep.name )
        {
          Some( v ) => v,
          None => continue,  // External dependency, skip
        };

        // Check 1: Is dependency being published?
        if already_publishing.contains( &dep.name )
        {
          stale_deps.push( StaleDependency
          {
            name: dep.name.clone(),
            required: dep.version_req.clone(),
            workspace_version: workspace_version.clone(),
            reason: StaleReason::BeingPublished,
          });
          continue;
        }

        // Check 2: Version compatibility
        if !dep.version_req.matches( &workspace_version )
        {
          stale_deps.push( StaleDependency
          {
            name: dep.name.clone(),
            required: dep.version_req.clone(),
            workspace_version: workspace_version.clone(),
            reason: StaleReason::IncompatibleVersion,
          });
        }
      }
    }

    if !stale_deps.is_empty()
    {
      stale_map.insert( pkg_name, stale_deps );
    }
  }

  stale_map
}
```

**Step 4.3: Write Staleness Detection Tests**

Create `tests/inc/publish/dependency_staleness_test.rs`:

```rust
use willbe::*;

// Test fixture builder
fn build_test_workspace() -> Workspace
{
  // Build synthetic workspace with known dependencies
  todo!( "Implement workspace builder" )
}

#[ test ]
fn simple_stale_dependency()
{
  // Scenario: A depends on B ~1.0.0, B bumped to 1.1.0
  let workspace = build_test_workspace();
  let publishing = hashset![ "B".into() ];

  let stale = detect_stale_dependencies( &workspace, &publishing );

  assert!( stale.contains_key( &"A".into() ) );
  let stale_deps = &stale[ &"A".into() ];
  assert_eq!( stale_deps.len(), 1 );
  assert_eq!( stale_deps[ 0 ].name, "B".into() );
  assert_eq!( stale_deps[ 0 ].reason, StaleReason::BeingPublished );
}

#[ test ]
fn incompatible_version_stale()
{
  // Scenario: A depends on B ~1.0.0, workspace has B 1.1.0
  let workspace = build_test_workspace();
  let publishing = hashset![];

  let stale = detect_stale_dependencies( &workspace, &publishing );

  assert!( stale.contains_key( &"A".into() ) );
  let stale_deps = &stale[ &"A".into() ];
  assert_eq!( stale_deps[ 0 ].reason, StaleReason::IncompatibleVersion );
}

#[ test ]
fn compatible_version_not_stale()
{
  // Scenario: A depends on B ^1.0.0, workspace has B 1.0.1
  let workspace = build_test_workspace();
  let publishing = hashset![];

  let stale = detect_stale_dependencies( &workspace, &publishing );

  assert!( !stale.contains_key( &"A".into() ) );
}

// MRE test for original bug
// test_kind: bug_reproducer(issue-willbe-staleness-001)
#[ test ]
fn willbe_wca_former_conflict()
{
  // This test reproduces the exact scenario from the bug report
  //
  // ## Root Cause
  // willbe algorithm failed to detect that wca needed republishing
  // when former (its dependency) was bumped from 2.36.0 to 2.37.0.
  // wca's requirement ~2.36.0 became incompatible with workspace version 2.37.0.
  //
  // ## Why Not Caught Initially
  // Algorithm only checked for local code changes and version bumps.
  // Dependency staleness was never implemented.
  //
  // ## Fix Applied
  // Added dependency staleness detection in Phase 2 of publishing algorithm.
  // Detects when workspace version doesn't satisfy dependency requirement.
  //
  // ## Prevention
  // All packages now checked for stale dependencies before publishing.
  // Transitive closure ensures all affected packages republished.
  //
  // ## Pitfall to Avoid
  // Never assume workspace dependencies are always compatible.
  // Always validate semver requirements against workspace versions.

  let workspace = build_workspace_from_manifest( "tests/fixtures/willbe_wca_former.toml" );

  // Initial state: former 2.36.0, wca 0.36.0, willbe 0.28.0
  // wca depends on former ~2.36.0
  // willbe depends on wca ~0.36.0, former ~2.36.0

  // Simulate: former bumped to 2.37.0
  let publishing = hashset![ "former".into() ];

  // Should detect wca as stale
  let stale = detect_stale_dependencies( &workspace, &publishing );

  assert!(
    stale.contains_key( &"wca".into() ),
    "wca should be detected as stale when former is being published"
  );

  let wca_stale_deps = &stale[ &"wca".into() ];
  assert_eq!( wca_stale_deps.len(), 1 );
  assert_eq!( wca_stale_deps[ 0 ].name, "former".into() );
}
```

### Verification

```bash
# Run staleness tests
cargo test --test dependency_staleness_test

# Verify MRE test passes
cargo test willbe_wca_former_conflict

# Level 1 tests
w3 .test level::1
```

### Deliverables

- ✅ `src/entity/staleness.rs` (detection logic)
- ✅ Updated `src/entity/workspace.rs` (helper methods)
- ✅ `tests/inc/publish/dependency_staleness_test.rs` (10+ tests)
- ✅ MRE test for original bug (bug_reproducer marker)

---

## INCREMENT 5: Transitive Closure Computation (4 days)

### Goal

Implement cascade effect detection and transitive closure computation.

### Input State

- Staleness detection works
- No cascade/closure logic
- Can detect immediate stale deps only

### Output State

- Transitive closure algorithm implemented
- Cascade effects tracked
- Tests passing for multi-level cascades

### Implementation Steps

**Step 5.1: Implement Closure Computation**

Add to `src/entity/staleness.rs`:

```rust
/// Compute transitive closure of packages to publish
pub fn compute_publish_closure
(
  workspace: &Workspace,
  initial: HashSet< PackageName >,
) -> HashMap< PackageName, Vec< PublishReason > >
{
  let mut result = HashMap::new();

  // Initialize with initial set
  for pkg_name in &initial
  {
    result.insert( pkg_name.clone(), vec![ PublishReason::LocalChanges ] );
  }

  let mut changed = true;
  let mut iteration = 0;
  let max_iterations = workspace.packages().count() * 2;

  while changed && iteration < max_iterations
  {
    changed = false;
    let current_publishing: HashSet< _ > = result.keys().cloned().collect();
    let old_size = result.len();

    // Find stale dependencies
    let stale = detect_stale_dependencies( workspace, &current_publishing );

    for ( pkg_name, stale_deps ) in stale
    {
      if !result.contains_key( &pkg_name )
      {
        result.insert(
          pkg_name,
          vec![ PublishReason::StaleDependencies { stale_deps } ],
        );
        changed = true;
      }
    }

    // Find cascade dependents
    for pkg_name in current_publishing.clone()
    {
      for dependent in workspace.find_dependents( &pkg_name )
      {
        let dep_name = dependent.name().unwrap();
        if !result.contains_key( &dep_name )
        {
          result.insert(
            dep_name,
            vec![ PublishReason::CascadeEffect
            {
              triggered_by: vec![ pkg_name.clone() ],
            }],
          );
          changed = true;
        }
      }
    }

    iteration += 1;

    if result.len() == old_size
    {
      break;  // Converged
    }
  }

  if iteration >= max_iterations
  {
    panic!( "Closure computation did not converge after {} iterations", max_iterations );
  }

  result
}
```

**Step 5.2: Write Cascade Tests**

Create `tests/inc/publish/cascade_test.rs`:

```rust
#[ test ]
fn linear_cascade()
{
  // A → B → C → D
  // D has local changes
  // Should cascade: D, C, B, A

  let workspace = build_linear_dependency_workspace();
  let initial = hashset![ "D".into() ];

  let closure = compute_publish_closure( &workspace, initial );

  assert_eq!( closure.len(), 4 );
  assert!( closure.contains_key( &"D".into() ) );
  assert!( closure.contains_key( &"C".into() ) );
  assert!( closure.contains_key( &"B".into() ) );
  assert!( closure.contains_key( &"A".into() ) );

  // Verify reasons
  assert!( closure[ &"D" ].iter().any( | r | r.is_local_change() ) );
  assert!( closure[ &"C" ].iter().any( | r | r.is_cascade() ) );
  assert!( closure[ &"B" ].iter().any( | r | r.is_cascade() ) );
  assert!( closure[ &"A" ].iter().any( | r | r.is_cascade() ) );
}

#[ test ]
fn diamond_dependency()
{
  //     A
  //    / \
  //   B   C
  //    \ /
  //     D
  // D has local changes

  let workspace = build_diamond_dependency_workspace();
  let initial = hashset![ "D".into() ];

  let closure = compute_publish_closure( &workspace, initial );

  assert_eq!( closure.len(), 4 );
  assert!( closure.contains_key( &"D".into() ) );
  assert!( closure.contains_key( &"C".into() ) );
  assert!( closure.contains_key( &"B".into() ) );
  assert!( closure.contains_key( &"A".into() ) );
}

#[ test ]
fn no_cascade_for_compatible_versions()
{
  // A depends on B ^1.0.0
  // B bumped 1.0.0 → 1.0.1 (compatible)

  let workspace = build_compatible_version_workspace();
  let initial = hashset![ "B".into() ];

  let closure = compute_publish_closure( &workspace, initial );

  // Only B should be in closure (A compatible)
  assert_eq!( closure.len(), 1 );
  assert!( closure.contains_key( &"B".into() ) );
  assert!( !closure.contains_key( &"A".into() ) );
}
```

### Verification

```bash
# Run cascade tests
cargo test --test cascade_test

# Full test suite
w3 .test level::1
```

### Deliverables

- ✅ `compute_publish_closure()` function
- ✅ `tests/inc/publish/cascade_test.rs` (10+ tests)
- ✅ Convergence guarantees (max iterations)

---

## INCREMENT 6: Integration with Publish Plan (3 days)

### Goal

Integrate staleness detection into main publish plan builder.

### Input State

- Staleness detection works standalone
- Closure computation works standalone
- Not integrated into `PublishPlan::build()`

### Output State

- Enhanced `build_publish_plan()` using all phases
- Publish reasons tracked for all packages
- Integration tests passing

### Implementation Steps

**Step 6.1: Refactor `build_publish_plan()`**

Update `src/entity/publish.rs`:

```rust
impl PublishPlanFormer
{
  pub fn build( self ) -> PublishPlan
  {
    let workspace = /* get workspace */;

    // Phase 1: Initial detection
    let initial = self.detect_initial_packages( &workspace );

    // Phase 2 & 3: Staleness + Closure
    let all_packages = staleness::compute_publish_closure( &workspace, initial );

    // Phase 4: Topological sort
    let ordered = self.topological_sort( &workspace, &all_packages );

    // Phase 5: Build instructions
    let mut plans = Vec::new();
    for pkg_name in ordered
    {
      let reasons = all_packages[ &pkg_name ].clone();
      let instruction = self.build_instruction( &workspace, pkg_name, reasons );
      plans.push( instruction );
    }

    PublishPlan
    {
      workspace_dir: self.workspace_dir.unwrap(),
      base_temp_dir: self.base_temp_dir,
      channel: self.channel.unwrap_or_default(),
      dry: self.dry.unwrap_or( true ),
      roots: self.determine_roots( &plans ),
      plans,
    }
  }

  fn detect_initial_packages( &self, workspace: &Workspace ) -> HashSet< PackageName >
  {
    let mut result = HashSet::new();

    for package in workspace.packages()
    {
      // Check for local changes
      if self.has_local_changes( &package )
      {
        result.insert( package.name().unwrap() );
        continue;
      }

      // Check for version bump
      if self.version_bumped( &package )
      {
        result.insert( package.name().unwrap() );
      }
    }

    result
  }

  fn topological_sort
  (
    &self,
    workspace: &Workspace,
    packages: &HashMap< PackageName, Vec< PublishReason > >,
  ) -> Vec< PackageName >
  {
    // Kahn's algorithm
    // ... existing implementation ...
  }
}
```

**Step 6.2: Update Display Logic**

Update `src/action/publish.rs` to show reasons:

```rust
pub fn publish( /* ... */ ) -> Result< (), Error >
{
  let plan = build_publish_plan( /* ... */ );

  println!( "The following packages are pending for publication:" );
  for ( idx, instruction ) in plan.plans.iter().enumerate()
  {
    println!(
      "[{}] {} ({} -> {})",
      idx,
      instruction.package_name,
      instruction.old_version,
      instruction.new_version
    );

    // NEW: Show reasons
    for reason in &instruction.reasons
    {
      println!( "    Reason: {}", reason.description() );
    }
  }

  // ... rest of publish logic ...
}
```

**Step 6.3: Write Integration Tests**

Create `tests/inc/publish/integration_test.rs`:

```rust
#[ test ]
fn full_publish_plan_with_staleness()
{
  let workspace = build_test_workspace();
  let plan = build_publish_plan( workspace );

  // Verify all affected packages included
  assert!( plan.plans.iter().any( | p | p.package_name == "former" ) );
  assert!( plan.plans.iter().any( | p | p.package_name == "wca" ) );
  assert!( plan.plans.iter().any( | p | p.package_name == "willbe" ) );

  // Verify reasons
  let wca_plan = plan.plans.iter().find( | p | p.package_name == "wca" ).unwrap();
  assert!( wca_plan.reasons.iter().any( | r | r.is_stale() ) );

  let willbe_plan = plan.plans.iter().find( | p | p.package_name == "willbe" ).unwrap();
  assert!( willbe_plan.reasons.iter().any( | r | r.is_cascade() ) );
}
```

### Verification

```bash
# Integration tests
cargo test --test integration_test

# Full test suite
w3 .test level::3
```

### Deliverables

- ✅ Refactored `build_publish_plan()`
- ✅ Enhanced display with reasons
- ✅ `tests/inc/publish/integration_test.rs` (5+ tests)

---

## INCREMENT 7: Bug Documentation (2 days)

### Goal

Document the bug fix following rulebook standards.

### Input State

- Fix implemented and tested
- No documentation

### Output State

- Complete bug documentation (5 sections + 3 fields)
- Module-level Known Pitfalls section
- Architecture documentation updated

### Implementation Steps

**Step 7.1: Add Source Code Fix Comments**

In `src/entity/staleness.rs` at the top of `detect_stale_dependencies()`:

```rust
// Fix(issue-willbe-staleness-001): Add dependency staleness detection
// Root cause: Algorithm only checked local changes, never validated dependency versions
// Pitfall: Always verify workspace dependency versions satisfy dependent requirements
pub fn detect_stale_dependencies
(
  workspace: &Workspace,
  already_publishing: &HashSet< PackageName >,
) -> HashMap< PackageName, Vec< StaleDependency > >
{
  // ... implementation ...
}
```

**Step 7.2: Add Module-Level Known Pitfalls**

In `src/entity/publish.rs`:

```rust
//! Publishing infrastructure for multi-package workspaces.
//!
//! ## Known Pitfalls
//!
//! ### Dependency Staleness
//!
//! Never assume workspace dependencies are always compatible with dependent packages.
//! When a workspace package is bumped, all dependents must be checked for version
//! requirement compatibility.
//!
//! Root cause (issue-willbe-staleness-001): Original algorithm only detected local
//! code changes and explicit version bumps. It failed to detect when a package's
//! dependency requirements became incompatible with workspace versions.
//!
//! Prevention: Always run dependency staleness detection before computing publish plan.
//! Use `detect_stale_dependencies()` to find packages with incompatible requirements.
//!
//! ```rust
//! // ✅ CORRECT: Check staleness before publishing
//! let initial = detect_local_changes( workspace );
//! let with_stale = detect_stale_dependencies( workspace, &initial );
//! let full_closure = compute_publish_closure( workspace, with_stale );
//!
//! // ❌ FORBIDDEN: Only check local changes
//! let to_publish = detect_local_changes( workspace );  // MISSES STALE DEPS
//! ```
```

**Step 7.3: Update Architecture Documentation**

Update `spec/architecture.md` with:
- New algorithm flow diagram
- Staleness detection explanation
- Cascade effect mechanics
- Performance characteristics

### Verification

```bash
# Documentation builds
cargo doc --no-deps --all-features

# Check Known Pitfalls section
cargo doc --open
# Navigate to entity::publish module docs
```

### Deliverables

- ✅ Source code fix comments (3 fields)
- ✅ Module-level Known Pitfalls section
- ✅ Updated architecture documentation

---

## INCREMENT 8: Performance Optimization (2 days)

### Goal

Optimize algorithm for large workspaces.

### Input State

- Algorithm works correctly
- May be slow on large workspaces (100+ packages)

### Output State

- Optimized dependency lookups
- Cached version resolutions
- Performance tests passing

### Implementation Steps

**Step 8.1: Add Caching Layer**

```rust
struct WorkspaceCache
{
  version_cache: HashMap< PackageName, Version >,
  dependents_cache: HashMap< PackageName, Vec< PackageName > >,
}

impl WorkspaceCache
{
  fn new( workspace: &Workspace ) -> Self
  {
    // Pre-compute all versions and dependents
    // ...
  }
}
```

**Step 8.2: Optimize Closure Computation**

- Early termination when no changes
- Skip packages already in closure
- Batch dependency lookups

**Step 8.3: Write Performance Tests**

```rust
#[ test ]
#[ ignore ]  // Run with --ignored
fn large_workspace_performance()
{
  let workspace = build_large_workspace( 100 );  // 100 packages
  let initial = hashset![ "pkg_50".into() ];

  let start = Instant::now();
  let closure = compute_publish_closure( &workspace, initial );
  let duration = start.elapsed();

  assert!( duration < Duration::from_secs( 10 ) );
}
```

### Verification

```bash
# Performance tests
cargo test --ignored

# Benchmark
cargo bench
```

### Deliverables

- ✅ Caching layer implemented
- ✅ Performance tests (3+ tests)
- ✅ <10s for 100 packages

---

## INCREMENT 9: Final Verification & Deployment (3 days)

### Goal

Complete verification and prepare for deployment.

### Input State

- All features implemented
- All tests passing locally

### Output State

- All level 5 tests passing
- Documentation complete
- Ready for merge

### Implementation Steps

**Step 9.1: Full Test Suite**

```bash
# Level 5 verification
w3 .test level::5

# Manual testing
cd /home/user1/pro/lib/wTools/module/core/wca
will .publish dry:1  # Should detect staleness

cd ../willbe
will .publish dry:1  # Should show cascade effects
```

**Step 9.2: Update Changelog**

Add to `changelog.md`:

```markdown
## [0.29.0] - 2025-11-XX

### Added
- Dependency staleness detection in publishing algorithm
- Cascade effect tracking for transitive dependencies
- Enhanced publish plan with reasons display

### Fixed
- [CRITICAL] Publishing fails with version conflicts when dependencies bumped (#issue-willbe-staleness-001)
- Missing detection of packages needing republish due to stale deps
- Incomplete transitive closure in publish sequence

### Changed
- PublishPlan now includes PublishReason for each package
- Display shows why each package is being published
```

**Step 9.3: Review Checklist**

- [ ] All tests pass (level 5)
- [ ] Documentation complete
- [ ] Bug documented (5 sections + 3 fields)
- [ ] No breaking API changes
- [ ] Performance acceptable
- [ ] Manual testing successful
- [ ] Changelog updated

### Deliverables

- ✅ All tests passing
- ✅ Documentation complete
- ✅ Ready for deployment

---

## Total Timeline

| Increment | Duration | Cumulative |
|-----------|----------|------------|
| 1. Setup | 2 days | 2 days |
| 2. Data Structures | 3 days | 5 days |
| 3. Semver Utils | 2 days | 7 days |
| 4. Staleness Detection | 4 days | 11 days |
| 5. Closure Computation | 4 days | 15 days |
| 6. Integration | 3 days | 18 days |
| 7. Documentation | 2 days | 20 days |
| 8. Performance | 2 days | 22 days |
| 9. Final Verification | 3 days | 25 days |

**Total: 25 days (5 weeks)**

---

## Risk Mitigation

### Risk: Performance Issues

**Mitigation:** Increment 8 dedicated to optimization

### Risk: Breaking Changes

**Mitigation:** Maintain backward compatibility, add feature flag

### Risk: Complex Testing

**Mitigation:** Test utilities built in Increment 1, reused throughout

---

## Success Metrics

- ✅ Original bug scenario resolved
- ✅ Zero version conflicts in publishing
- ✅ <10s performance for 100 packages
- ✅ >90% test coverage
- ✅ Complete documentation
