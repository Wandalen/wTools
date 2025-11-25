# Extract Process Management Utilities from wplan

**Date**: 2025-11-20
**Priority**: MEDIUM
**Category**: API Enhancement - Code Extraction
**Status**: 🔄 (Planned)
**Source**: wplan/src/daemon_routines.rs, wplan_client/src/cli/formatting.rs
**Task ID**: 001
**Advisability**: 1280 (Value: 8, Easiness: 4, Safety: 8, Priority: 5)

**⚠️ CRITICAL**: This task is INCOMPLETE without follow-up adoption. Task will be CANCELED if adoption not implemented.

**Follow-up Adoption Required:**
- [wplan_client/006](../../../../../willbe/module/wplan_client/task/006_adopt_process_utilities_from_process_tools.md) - Replace local process utilities with process_tools
- [wplan/086](../../../../../willbe/module/wplan/task/086_adopt_process_utilities_from_process_tools.md) - Replace local process utilities with process_tools

---

## Executive Summary

Extract process management and signal handling utilities from the wplan ecosystem to `process_tools`, making them available to all wTools projects. These utilities handle process lifecycle checks, signal name mapping, and daemon management - all useful for server applications, daemons, and CLI tools that spawn processes.

---

## Problem Statement

### Current Location

The wplan codebase contains process management utilities that would benefit other wTools projects:

**wplan/src/daemon_routines.rs**:
- Lines 99-107: `is_process_alive()` - Check if PID is alive via `kill(pid, 0)`
- Lines 150-200: Fork/daemonize operations with `setsid()`
- Functionality: Process lifecycle management for daemon mode

**wplan_client/src/cli/formatting.rs**:
- Lines 981-1011: `signal_name()` - Maps signal numbers to names (9 → "SIGKILL")
- Functionality: Human-readable signal reporting

### Why Extract to process_tools

1. **General Utility**: Process management is common for daemons, services, test harnesses
2. **Signal Mapping**: Universal need for displaying signal information
3. **Safety**: Centralized testing for platform-specific process operations
4. **Portability**: Abstracts Unix/Windows differences in one place
5. **Code Reuse**: willbe, wtest, benchkit all spawn processes

---

## Detailed Functionality Analysis

### 1. Process Alive Check

**Current Location**: `wplan/src/daemon_routines.rs:99-107`

```rust
pub fn is_process_alive( pid : i32 ) -> bool
{
  unsafe
  {
    // kill( pid, 0 ) doesn't send a signal, just checks if process exists
    libc::kill( pid, 0 ) == 0
  }
}
```

**Why This Works**:
- `kill(pid, 0)` is a Unix idiom for "check if process exists"
- Returns 0 if process alive and we have permission
- Returns -1 with `ESRCH` if process doesn't exist
- Returns -1 with `EPERM` if process exists but we lack permission

**Use Cases**:
- Daemon management (check if daemon is running)
- Test teardown (wait for process to exit)
- PID file validation
- Service health checks

### 2. Signal Name Mapping

**Current Location**: `wplan_client/src/cli/formatting.rs:981-1011`

```rust
pub fn signal_name( signal : i32 ) -> &'static str
{
  match signal
  {
    1 => "SIGHUP",
    2 => "SIGINT",
    3 => "SIGQUIT",
    6 => "SIGABRT",
    9 => "SIGKILL",
    14 => "SIGALRM",
    15 => "SIGTERM",
    _ => "UNKNOWN",
  }
}
```

**Features**:
- Maps Unix signal numbers to standard names
- Covers most common signals
- Returns "UNKNOWN" for unmapped signals

**Use Cases**:
- Displaying process exit status
- Log messages about killed processes
- Error reporting
- Test output

### 3. Daemon Management (Fork/Setsid)

**Current Location**: `wplan/src/daemon_routines.rs:150-200`

**Functionality**:
- Fork process to background
- Call `setsid()` to create new session
- Redirect stdin/stdout/stderr to /dev/null
- Write PID file

**Use Cases**:
- Server/daemon applications
- Background task runners
- Service management

---

## Proposed API Design

### Target Location

```
process_tools/src/lifecycle/
  mod.rs           # Module exports
  check.rs         # Process existence checks
  daemon.rs        # Daemonization utilities
  signal.rs        # Signal utilities
```

### API Structure

```rust
//! Process management utilities for process_tools
//!
//! Provides:
//! - Process lifecycle checks (alive, wait for exit)
//! - Signal name mapping and handling
//! - Daemonization support (Unix)
//! - Process spawning utilities

// ============================================================================
// check.rs - Process Existence Checks
// ============================================================================

use std::io;

/// Check if a process is alive by PID.
///
/// Uses `kill(pid, 0)` on Unix to check process existence without sending a signal.
/// On Windows, uses OpenProcess.
///
/// # Returns
///
/// - `Ok(true)` - Process exists and we have permission to signal it
/// - `Ok(false)` - Process does not exist
/// - `Err(_)` - System error (e.g., invalid PID)
///
/// # Platform Support
///
/// - Unix: Uses `kill(pid, 0)`
/// - Windows: Uses `OpenProcess` + `CloseHandle`
///
/// # Example
///
/// ```rust
/// use process_tools::lifecycle::is_process_alive;
///
/// let my_pid = std::process::id() as i32;
/// assert_eq!( is_process_alive( my_pid )?, true );
///
/// let invalid_pid = -1;
/// assert_eq!( is_process_alive( invalid_pid )?, false );
/// ```
pub fn is_process_alive( pid : i32 ) -> io::Result< bool >;

/// Wait for a process to exit (with timeout).
///
/// Polls `is_process_alive()` until process exits or timeout reached.
///
/// # Returns
///
/// - `Ok(())` - Process exited within timeout
/// - `Err(TimedOut)` - Process still alive after timeout
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
/// use process_tools::lifecycle::wait_for_exit;
///
/// let pid = 12345;
/// let timeout = Duration::from_secs( 10 );
///
/// wait_for_exit( pid, timeout )?;
/// // Process has exited
/// ```
pub fn wait_for_exit( pid : i32, timeout : std::time::Duration ) -> io::Result< () >;

/// Check if process with PID from file is alive.
///
/// Convenience wrapper that reads PID from file and checks if alive.
///
/// # Example
///
/// ```rust
/// use process_tools::lifecycle::is_pidfile_alive;
///
/// if is_pidfile_alive( "/var/run/mydaemon.pid" )?
/// {
///   println!( "Daemon is running" );
/// }
/// ```
pub fn is_pidfile_alive( pidfile_path : &str ) -> io::Result< bool >;

// ============================================================================
// signal.rs - Signal Utilities
// ============================================================================

/// Maps Unix signal number to standard signal name.
///
/// # Returns
///
/// Standard signal name (e.g., "SIGTERM", "SIGKILL") or "UNKNOWN" for unmapped signals.
///
/// # Platform Support
///
/// - Unix: Maps standard POSIX signals
/// - Windows: Limited support (maps common signals that have Windows equivalents)
///
/// # Example
///
/// ```rust
/// use process_tools::lifecycle::signal_name;
///
/// assert_eq!( signal_name( 9 ), "SIGKILL" );
/// assert_eq!( signal_name( 15 ), "SIGTERM" );
/// assert_eq!( signal_name( 2 ), "SIGINT" );
/// assert_eq!( signal_name( 999 ), "UNKNOWN" );
/// ```
pub fn signal_name( signal : i32 ) -> &'static str;

/// Maps signal name to signal number.
///
/// # Returns
///
/// Signal number or `None` if name not recognized.
///
/// # Example
///
/// ```rust
/// use process_tools::lifecycle::signal_number;
///
/// assert_eq!( signal_number( "SIGKILL" ), Some( 9 ) );
/// assert_eq!( signal_number( "SIGTERM" ), Some( 15 ) );
/// assert_eq!( signal_number( "UNKNOWN" ), None );
/// ```
pub fn signal_number( name : &str ) -> Option< i32 >;

/// All standard POSIX signals with descriptions.
///
/// # Example
///
/// ```rust
/// use process_tools::lifecycle::all_signals;
///
/// for ( num, name, desc ) in all_signals()
/// {
///   println!( "{:2}: {} - {}", num, name, desc );
/// }
/// ```
pub fn all_signals() -> Vec< ( i32, &'static str, &'static str ) >;

// ============================================================================
// daemon.rs - Daemonization (Unix only)
// ============================================================================

#[ cfg( unix ) ]
use std::path::Path;

/// Daemonization options.
#[ cfg( unix ) ]
#[ derive( Debug, Clone ) ]
pub struct DaemonizeOptions
{
  /// PID file path (optional).
  pub pidfile : Option< String >,
  /// Working directory for daemon (default: "/").
  pub working_dir : String,
  /// Redirect stdout to file (optional).
  pub stdout : Option< String >,
  /// Redirect stderr to file (optional).
  pub stderr : Option< String >,
}

#[ cfg( unix ) ]
impl Default for DaemonizeOptions
{
  fn default() -> Self
  {
    Self
    {
      pidfile : None,
      working_dir : "/".to_string(),
      stdout : None,
      stderr : None,
    }
  }
}

/// Daemonize current process (Unix only).
///
/// Performs standard Unix daemonization:
/// 1. Fork to background
/// 2. Create new session with `setsid()`
/// 3. Fork again (to prevent reacquiring terminal)
/// 4. Change working directory
/// 5. Close stdin, stdout, stderr (or redirect)
/// 6. Write PID file
///
/// # Returns
///
/// - `Ok(())` in child process (daemon)
/// - Never returns in parent process (exits)
/// - `Err(_)` on failure
///
/// # Safety
///
/// This function is Unix-only and uses `fork()`, `setsid()`, etc.
/// Call before spawning threads or opening resources.
///
/// # Example
///
/// ```rust,no_run
/// use process_tools::lifecycle::{ daemonize, DaemonizeOptions };
///
/// let opts = DaemonizeOptions
/// {
///   pidfile : Some( "/var/run/mydaemon.pid".to_string() ),
///   stdout : Some( "/var/log/mydaemon.log".to_string() ),
///   ..Default::default()
/// };
///
/// daemonize( &opts )?;
///
/// // Now running as daemon
/// loop
/// {
///   // Daemon work here
/// }
/// ```
#[ cfg( unix ) ]
pub fn daemonize( options : &DaemonizeOptions ) -> io::Result< () >;

/// Write PID file.
///
/// # Example
///
/// ```rust
/// use process_tools::lifecycle::write_pidfile;
///
/// write_pidfile( "/var/run/mydaemon.pid" )?;
/// ```
pub fn write_pidfile< P : AsRef< Path > >( path : P ) -> io::Result< () >;

/// Read PID from file.
///
/// # Example
///
/// ```rust
/// use process_tools::lifecycle::read_pidfile;
///
/// let pid = read_pidfile( "/var/run/mydaemon.pid" )?;
/// println!( "Daemon PID: {}", pid );
/// ```
pub fn read_pidfile< P : AsRef< Path > >( path : P ) -> io::Result< i32 >;

/// Remove PID file.
///
/// # Example
///
/// ```rust
/// use process_tools::lifecycle::remove_pidfile;
///
/// remove_pidfile( "/var/run/mydaemon.pid" )?;
/// ```
pub fn remove_pidfile< P : AsRef< Path > >( path : P ) -> io::Result< () >;
```

---

## Implementation Phases

### Phase 1: Process Checks (2 hours)

**Tasks**:
1. Create `process_tools/src/lifecycle/check.rs`
2. Implement `is_process_alive()` for Unix (kill with signal 0)
3. Implement `is_process_alive()` for Windows (OpenProcess)
4. Implement `wait_for_exit()` with polling + timeout
5. Implement `is_pidfile_alive()` convenience wrapper
6. Add platform-specific tests
7. Document platform differences

**Acceptance Criteria**:
- [ ] `is_process_alive()` works on Unix
- [ ] `is_process_alive()` works on Windows
- [ ] `wait_for_exit()` respects timeout
- [ ] PID file utilities work correctly
- [ ] Tests cover valid/invalid PIDs
- [ ] Documentation explains platform behavior

### Phase 2: Signal Utilities (1.5 hours)

**Tasks**:
1. Create `process_tools/src/lifecycle/signal.rs`
2. Implement `signal_name()` with comprehensive signal mapping
3. Implement `signal_number()` for reverse lookup
4. Implement `all_signals()` for enumeration
5. Add tests for all standard signals
6. Document signal meanings

**Acceptance Criteria**:
- [ ] All POSIX signals mapped (SIGHUP, SIGINT, SIGTERM, etc.)
- [ ] Bidirectional mapping works (name ↔ number)
- [ ] Tests verify all mappings
- [ ] Documentation includes signal descriptions

### Phase 3: Daemonization (3 hours - Unix only)

**Tasks**:
1. Create `process_tools/src/lifecycle/daemon.rs`
2. Implement `DaemonizeOptions` configuration
3. Implement `daemonize()` with fork + setsid + chdir
4. Implement PID file management (write/read/remove)
5. Handle stdout/stderr redirection
6. Add integration tests (spawn daemon, verify PID)
7. Document safety considerations

**Acceptance Criteria**:
- [ ] Daemonization follows standard Unix procedure
- [ ] PID file written correctly
- [ ] stdio redirection works
- [ ] Tests verify daemon process creation
- [ ] Documentation warns about pre-thread usage

### Phase 4: Integration and Migration (1 hour)

**Tasks**:
1. Update `process_tools/src/lib.rs` to export lifecycle module
2. Migrate wplan to use new API
3. Migrate wplan_client to use `signal_name()`
4. Delete old implementations
5. Verify all tests pass

**Acceptance Criteria**:
- [ ] All modules exported from `process_tools::lifecycle`
- [ ] wplan uses new daemonization API
- [ ] wplan_client uses new signal_name
- [ ] Old code deleted
- [ ] All tests pass

---

## Dependencies

```toml
# process_tools/Cargo.toml
[dependencies]
libc = { workspace = true }  # For Unix process operations

[target.'cfg(windows)'.dependencies]
windows = { workspace = true, features = ["Win32_System_Threading"] }  # For Windows OpenProcess
```

---

## Expected Impact

| Metric | Before | After |
|--------|--------|-------|
| wplan daemon_routines.rs LOC | ~150 | ~30 (imports) |
| wplan_client formatting.rs signal code | ~30 | 0 (deleted) |
| Code duplication | Isolated | Shared |
| Platform abstraction | Per-project | Centralized |
| Test coverage | Per-project | Comprehensive |

---

## Testing Strategy

### Unit Tests

**Process Checks**:
- Valid PID (self)
- Invalid PID (-1, 999999)
- Wait with timeout (mock or real process)
- PID file operations

**Signal Utilities**:
- All standard signals mapped correctly
- Bidirectional mapping (name ↔ number)
- Unknown signals return "UNKNOWN"

**Daemonization** (Unix):
- Fork succeeds
- PID file written
- stdio redirected
- Process becomes session leader

### Integration Tests

- Spawn daemon, verify running, kill, verify exit
- PID file lifecycle (create, read, remove)
- Signal name display in logs

---

## Platform Considerations

### Unix

- `kill(pid, 0)` for process checks
- Standard POSIX signals
- Full daemonization support

### Windows

- `OpenProcess` + `CloseHandle` for process checks
- Limited signal support (SIGINT, SIGTERM via Ctrl+C/Break)
- No daemonization (use Windows Services API instead)

### Cross-Platform Strategy

- Use `#[cfg(unix)]` and `#[cfg(windows)]` extensively
- Provide platform-specific implementations with same API
- Document platform limitations clearly

---

## Documentation Requirements

Each module must include:
1. Module-level documentation with platform support
2. Function documentation with platform notes
3. Safety considerations for daemonization
4. Signal reference table
5. Example daemon implementation

---

## Acceptance Criteria

- [ ] All 3 modules implemented (check, signal, daemon)
- [ ] Cross-platform support (Unix + Windows where applicable)
- [ ] Comprehensive test coverage
- [ ] Platform-specific tests pass
- [ ] Documentation complete with platform notes
- [ ] wplan successfully migrated
- [ ] wplan_client successfully migrated
- [ ] Old implementations deleted
- [ ] `cargo test -p process_tools` passes on Unix
- [ ] `cargo test -p process_tools` passes on Windows

---

## References

**Source Files**:
- `/home/user1/pro/lib/willbe/module/wplan/src/daemon_routines.rs:99-107` (is_process_alive)
- `/home/user1/pro/lib/willbe/module/wplan/src/daemon_routines.rs:150-200` (daemonize)
- `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/formatting.rs:981-1011` (signal_name)

**Related Projects**:
- wtest - needs process management for test harness
- benchkit - needs process checks for benchmark isolation
- willbe - needs daemonization for background builds

**Dependencies**:
- libc (workspace) - Unix syscalls
- windows (workspace) - Windows process APIs

---

## Estimated Effort

- Phase 1: 2 hours (process checks)
- Phase 2: 1.5 hours (signal utilities)
- Phase 3: 3 hours (daemonization)
- Phase 4: 1 hour (migration)

**Total**: 7.5 hours

---

## Priority Justification

**MEDIUM Priority** because:
1. **Specialized Use**: Not all projects need daemonization
2. **Platform-Specific**: Significant platform differences to handle
3. **Safety-Critical**: Fork/daemonization requires careful testing
4. **Immediate Value**: wplan migration benefits, but fewer downstream users
5. **Complexity**: Higher implementation complexity than other extractions
