# Extend Workspace Resolution for Installed Applications

## Description

Extend workspace_tools resolution strategy to support installed CLI applications that need to load secrets from user-configured locations. Currently, workspace resolution fails for installed binaries because it relies on `WORKSPACE_PATH` environment variable set only during `cargo` operations. This prevents installed applications like `wip2` from loading secrets via workspace_tools.

**Real-world problem**: When `wip2` (repository manager CLI) is installed via `cargo install --path .`, it can no longer load GitHub tokens from `secret/-secrets.sh` because:
1. `WORKSPACE_PATH` is not set (only set during cargo operations)
2. `resolve()` fails immediately
3. `resolve_or_fallback()` returns user's project directory, not the willbe workspace where secrets live
4. No fallback mechanism exists for user-configured secret locations

This task adds two new resolution strategies to the fallback chain: `$PRO` environment variable (for multi-project users) and `$HOME` directory (universal fallback), enabling installed applications to locate workspace-level secrets.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Problem Analysis

### Current Flow (Fails for Installed Apps)

```rust
// Current: workspace() -> Workspace::resolve()
pub fn resolve() -> Result<Self> {
  let root = Self::get_env_path("WORKSPACE_PATH")?;  // ❌ FAILS: Not set outside cargo
  if !root.exists() {
    return Err(WorkspaceError::PathNotFound(root));
  }
  Ok(Self { root })
}

// Current: resolve_or_fallback() doesn't help
pub fn resolve_or_fallback() -> Self {
  Self::from_cargo_workspace()    // ❌ Returns ~/my-project (wrong location)
    .or_else(|_| Self::resolve())  // ❌ No WORKSPACE_PATH
    .or_else(|_| Self::from_current_dir())  // ❌ Returns ~/my-project
    .or_else(|_| Self::from_git_root())     // ❌ Returns ~/my-project
    .unwrap_or_else(|_| Self::from_cwd())
}
```

### Use Case Examples

**Scenario 1: Developer in willbe workspace**
```bash
$ cd ~/pro/lib/willbe/module/wip
$ cargo run -- .orgs.list
✅ Works: WORKSPACE_PATH set → ~/pro/lib/willbe/
✅ Secrets: ~/pro/lib/willbe/secret/-secrets.sh
```

**Scenario 2: Installed wip2 with $PRO (CURRENTLY BROKEN)**
```bash
$ export PRO=~/pro
$ cd ~/my-projects/foo
$ wip2 .orgs.list
❌ FAILS: No WORKSPACE_PATH
❌ from_cargo_workspace() → ~/my-projects/foo (wrong!)
❌ Cannot find ~/pro/secret/-secrets.sh
```

**Scenario 3: Installed wip2 without $PRO (CURRENTLY BROKEN)**
```bash
$ cd ~/my-projects/foo
$ wip2 .orgs.list
❌ FAILS: No fallback to home directory
❌ Cannot find ~/secret/-secrets.sh
```

## Acceptance Criteria

### Phase 1: New Resolution Methods

-   [ ] **`from_pro_env()`**: Resolve workspace from `$PRO` environment variable
    -   Reads `PRO` environment variable
    -   Returns `WorkspaceError::EnvironmentVariableMissing` if not set
    -   Returns `WorkspaceError::PathNotFound` if path doesn't exist
    -   Returns `Workspace { root: PathBuf::from($PRO) }` on success

-   [ ] **`from_home_dir()`**: Resolve workspace from user home directory
    -   Reads `HOME` environment variable (Unix/Linux/Mac)
    -   Falls back to `USERPROFILE` environment variable (Windows)
    -   Returns `WorkspaceError::EnvironmentVariableMissing` if neither exists
    -   Returns `WorkspaceError::PathNotFound` if path doesn't exist
    -   Returns `Workspace { root: home_path }` on success

### Phase 2: Extended Fallback Chain

-   [ ] **New `resolve_with_extended_fallbacks()` method**:
    ```rust
    pub fn resolve_with_extended_fallbacks() -> Self {
      Self::from_cargo_workspace()        // 1. Dev: cargo workspace
        .or_else(|_| Self::resolve())      // 2. Dev: WORKSPACE_PATH env
        .or_else(|_| Self::from_git_root())// 3. Dev: git root with Cargo.toml
        .or_else(|_| Self::from_pro_env()) // 4. USER: $PRO environment ← NEW
        .or_else(|_| Self::from_home_dir())// 5. USER: $HOME directory ← NEW
        .unwrap_or_else(|_| Self::from_cwd())
    }
    ```

-   [ ] **Deprecate old `resolve_or_fallback()`** with helpful migration message
-   [ ] **Keep backward compatibility**: existing code continues working

### Phase 3: Documentation and Examples

-   [ ] **Update module documentation** with extended resolution strategy explanation
-   [ ] **Add examples** for each new resolution method
-   [ ] **Document user setup** for installed CLI applications:
    ```bash
    # Option 1: $PRO (recommended for multi-project users)
    mkdir -p ~/pro/secret
    export PRO=~/pro

    # Option 2: Home directory (simple for casual users)
    mkdir -p ~/secret
    ```

-   [ ] **Migration guide** from `resolve_or_fallback()` to `resolve_with_extended_fallbacks()`

### Phase 4: Testing

-   [ ] **Unit tests for `from_pro_env()`**:
    -   Test with valid $PRO path
    -   Test with non-existent path
    -   Test with missing $PRO variable

-   [ ] **Unit tests for `from_home_dir()`**:
    -   Test with valid $HOME path
    -   Test with valid $USERPROFILE path (Windows)
    -   Test with both missing (error case)

-   [ ] **Integration tests for `resolve_with_extended_fallbacks()`**:
    -   Test fallback chain order
    -   Test that each fallback is tried in sequence
    -   Test that existing resolution still works

-   [ ] **Cross-platform tests**:
    -   Verify Unix/Linux behavior ($HOME)
    -   Verify Windows behavior ($USERPROFILE)
    -   Verify MacOS behavior ($HOME)

## Implementation Plan

### Step 1: Add `from_pro_env()` Method

```rust
impl Workspace
{
  /// Create workspace from $PRO environment variable
  ///
  /// Intended for users who organize projects under a common root directory.
  /// The $PRO environment variable should point to the projects root.
  ///
  /// # Setup
  ///
  /// ```bash
  /// # Linux/Mac
  /// export PRO=~/pro
  /// echo 'export PRO=~/pro' >> ~/.bashrc
  ///
  /// # Windows
  /// set PRO=%USERPROFILE%\pro
  /// setx PRO "%USERPROFILE%\pro"
  /// ```
  ///
  /// # Examples
  ///
  /// ```rust
  /// use workspace_tools::Workspace;
  ///
  /// // User has: export PRO=~/pro
  /// let workspace = Workspace::from_pro_env().unwrap();
  /// // workspace.root() → /home/user/pro
  /// ```
  ///
  /// # Errors
  ///
  /// Returns error if:
  /// - $PRO environment variable is not set
  /// - Path specified by $PRO does not exist
  ///
  /// # Use Cases
  ///
  /// - Installed CLI tools needing workspace-level secrets
  /// - Multi-project users with organized directory structure
  /// - CI/CD environments with standardized project layouts
  #[inline]
  pub fn from_pro_env() -> Result<Self>
  {
    let pro_path = env::var("PRO")
      .map_err(|_| WorkspaceError::EnvironmentVariableMissing("PRO".to_string()))?;

    let root = PathBuf::from(pro_path);

    if !root.exists()
    {
      return Err(WorkspaceError::PathNotFound(root));
    }

    let root = Self::cleanup_path(root);
    Ok(Self { root })
  }
}
```

### Step 2: Add `from_home_dir()` Method

```rust
impl Workspace
{
  /// Create workspace from user home directory
  ///
  /// Universal fallback using the standard home directory location.
  /// Works cross-platform by checking both Unix ($HOME) and Windows (%USERPROFILE%).
  ///
  /// # Examples
  ///
  /// ```rust
  /// use workspace_tools::Workspace;
  ///
  /// let workspace = Workspace::from_home_dir().unwrap();
  /// // Linux/Mac: workspace.root() → /home/user
  /// // Windows:   workspace.root() → C:\Users\user
  /// ```
  ///
  /// # Errors
  ///
  /// Returns error if:
  /// - Neither $HOME nor %USERPROFILE% environment variables are set
  /// - Resolved path does not exist
  ///
  /// # Use Cases
  ///
  /// - Simple secret storage in ~/secret/ directory
  /// - Casual users without complex project organization
  /// - Minimal configuration requirement for CLI tools
  #[inline]
  pub fn from_home_dir() -> Result<Self>
  {
    let home_path = env::var("HOME")
      .or_else(|_| env::var("USERPROFILE"))  // Windows compatibility
      .map_err(|_| WorkspaceError::EnvironmentVariableMissing(
        "HOME or USERPROFILE".to_string()
      ))?;

    let root = PathBuf::from(home_path);

    if !root.exists()
    {
      return Err(WorkspaceError::PathNotFound(root));
    }

    let root = Self::cleanup_path(root);
    Ok(Self { root })
  }
}
```

### Step 3: Add `resolve_with_extended_fallbacks()` Method

```rust
impl Workspace
{
  /// Resolve workspace with extended fallback strategies
  ///
  /// Tries multiple strategies to find workspace root, including user-configured
  /// locations for installed CLI applications:
  ///
  /// 1. Cargo workspace detection (developer context)
  /// 2. WORKSPACE_PATH environment variable (cargo operations)
  /// 3. Git repository root with Cargo.toml (developer context)
  /// 4. $PRO environment variable (user-configured project root)
  /// 5. $HOME directory (universal fallback)
  /// 6. Current working directory (last resort)
  ///
  /// This method is designed for CLI applications that need to work both during
  /// development (via `cargo run`) and after installation (via `cargo install`).
  ///
  /// # Examples
  ///
  /// ```rust
  /// use workspace_tools::Workspace;
  ///
  /// // This will always succeed with some workspace root
  /// let workspace = Workspace::resolve_with_extended_fallbacks();
  /// ```
  ///
  /// # Resolution Priority
  ///
  /// **Developer contexts** (cargo operations):
  /// - from_cargo_workspace() → finds Cargo workspace via metadata
  /// - resolve() → uses WORKSPACE_PATH from .cargo/config.toml
  /// - from_git_root() → searches upward for .git + Cargo.toml
  ///
  /// **User contexts** (installed binaries):
  /// - from_pro_env() → uses $PRO environment variable
  /// - from_home_dir() → uses $HOME or %USERPROFILE%
  ///
  /// **Fallback**:
  /// - from_cwd() → current working directory
  #[must_use]
  #[inline]
  pub fn resolve_with_extended_fallbacks() -> Self
  {
    Self::from_cargo_workspace()
      .or_else(|_| Self::resolve())
      .or_else(|_| Self::from_git_root())
      .or_else(|_| Self::from_pro_env())     // ← NEW: $PRO fallback
      .or_else(|_| Self::from_home_dir())    // ← NEW: $HOME fallback
      .unwrap_or_else(|_| Self::from_cwd())
  }
}
```

### Step 4: Deprecate Old Method

```rust
impl Workspace
{
  /// Resolve workspace with fallback strategies
  ///
  /// # Deprecated
  ///
  /// Use `resolve_with_extended_fallbacks()` instead. This method lacks
  /// support for installed CLI application contexts ($PRO and $HOME fallbacks).
  ///
  /// # Migration
  ///
  /// ```rust
  /// // Old:
  /// let ws = Workspace::resolve_or_fallback();
  ///
  /// // New:
  /// let ws = Workspace::resolve_with_extended_fallbacks();
  /// ```
  #[deprecated(
    since = "0.8.0",
    note = "Use `resolve_with_extended_fallbacks()` for installed CLI app support"
  )]
  #[must_use]
  #[inline]
  pub fn resolve_or_fallback() -> Self
  {
    Self::from_cargo_workspace()
      .or_else(|_| Self::resolve())
      .or_else(|_| Self::from_current_dir())
      .or_else(|_| Self::from_git_root())
      .unwrap_or_else(|_| Self::from_cwd())
  }
}
```

### Step 5: Update Convenience Function

```rust
/// Get workspace instance with extended fallbacks
///
/// Convenience function that calls `Workspace::resolve_with_extended_fallbacks()`.
/// Always succeeds by falling back through multiple strategies.
///
/// # Examples
///
/// ```rust
/// use workspace_tools::workspace;
///
/// let ws = workspace();
/// let secrets = ws.load_secrets_from_path("secret/-secrets.sh").ok();
/// ```
#[must_use]
#[inline]
pub fn workspace() -> Workspace
{
  Workspace::resolve_with_extended_fallbacks()
}
```

## Success Metrics

-   **Installed CLI applications work**: wip2 can load secrets after `cargo install`
-   **Backward compatibility**: existing code using `workspace()` continues working
-   **Cross-platform support**: works on Linux, Mac, and Windows
-   **Clear documentation**: users understand how to set up $PRO or ~/secret/
-   **Zero breaking changes**: all existing tests pass without modification

## User Documentation

### For End Users (Installed CLI Applications)

```markdown
# Secret Setup for Installed Applications

When using CLI tools like `wip2` installed via `cargo install`, secrets are loaded from:

## Option 1: $PRO Directory (Recommended)

For users with multiple projects organized under a common root:

```bash
# Setup
mkdir -p ~/pro/secret
export PRO=~/pro
echo 'export PRO=~/pro' >> ~/.bashrc
source ~/.bashrc

# Create secrets file
cp /usr/share/doc/wip2/secrets.template.sh ~/pro/secret/-secrets.sh
chmod 600 ~/pro/secret/-secrets.sh
vim ~/pro/secret/-secrets.sh  # Add your tokens
```

## Option 2: Home Directory (Simple)

For casual users wanting minimal configuration:

```bash
# Setup
mkdir -p ~/secret

# Create secrets file
cp /usr/share/doc/wip2/secrets.template.sh ~/secret/-secrets.sh
chmod 600 ~/secret/-secrets.sh
vim ~/secret/-secrets.sh  # Add your tokens
```

## Resolution Priority

The tool searches for secrets in this order:
1. Developer workspace (if running via `cargo run`)
2. `$PRO/secret/` (if $PRO is set)
3. `$HOME/secret/` (universal fallback)
```

### For Developers

```markdown
# Developer Setup (Building from Source)

When developing on workspace_tools or applications using it:

```bash
# Your workspace already has .cargo/config.toml with WORKSPACE_PATH
# Secrets are loaded from: $WORKSPACE_ROOT/secret/-secrets.sh

cd ~/pro/lib/willbe
cargo run --manifest-path module/wip/Cargo.toml -- .orgs.list
# Uses: ~/pro/lib/willbe/secret/-secrets.sh
```

No $PRO configuration needed during development.
```

## Migration Strategy

### Phase 1: Add New Methods (v0.8.0)
- Add `from_pro_env()` and `from_home_dir()`
- Add `resolve_with_extended_fallbacks()`
- Keep existing methods working

### Phase 2: Deprecation Warnings (v0.8.0)
- Mark `resolve_or_fallback()` as deprecated
- Update documentation to recommend new method

### Phase 3: Update Examples (v0.8.0)
- Update all examples to use `resolve_with_extended_fallbacks()`
- Add migration guide

### Phase 4: Future Consideration (v0.9.0+)
- Consider making `resolve_with_extended_fallbacks()` the default `workspace()` behavior
- Remove deprecated methods after sufficient adoption period

## Related Issues

This task addresses real-world issues from:
- **wip2**: Cannot load GitHub tokens when installed via `cargo install`
- **Ecosystem tools**: Any CLI tool installed via cargo cannot use workspace_tools secrets
- **User experience**: No clear way to configure secrets for installed applications

## Priority: Critical

**Advisability**: 10/10 - Essential for workspace_tools to work outside cargo context
**Value**: 10/10 - Unblocks entire category of use cases (installed CLI tools)
**Easiness**: 8/10 - Straightforward implementation, mostly additive
**Safety**: 9/10 - No breaking changes, extends existing functionality
**Priority**: 10/10 - Blocking critical use case for multiple tools
