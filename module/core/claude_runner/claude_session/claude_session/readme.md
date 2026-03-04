# claude_session

Session storage path resolution and continuation detection for Claude Code with zero dependencies.

### Responsibility Table

| Entity | Responsibility | Input→Output | Scope | Out of Scope |
|--------|---------------|--------------|-------|--------------|
| claude_session | Session storage path management | WorkingDir → StoragePath | Path resolution, continuation detection, path escaping, session cleanup | ❌ Claude Code execution → `claude_runner`<br>❌ Process lifecycle → `claude_runner`<br>❌ Command building → `claude_runner`<br>❌ Builder pattern → `claude_runner`<br>❌ Token limits → `claude_runner` |

### Scope

**Responsibility:**
- Session storage path resolution (~/.claude/projects/)
- Continuation detection (existing conversation check)
- Path escaping (special characters → hyphens)
- Session cleanup (fresh strategy support)
- Zero-dependency core functionality

**In Scope:**
- Session storage path construction from working directory
- Path escaping for filesystem compatibility
- Continuation availability detection
- Session storage deletion (fresh strategy)
- Zero external dependencies (std only)

**Out of Scope:**
- ❌ Claude Code process execution → delegated to `claude_runner` crate
- ❌ Command::new("claude") calls → delegated to `claude_runner` crate
- ❌ Builder pattern (ClaudeCommand::new()) → delegated to `claude_runner` crate
- ❌ Token limit configuration → delegated to `claude_runner` crate
- ❌ Process output capture → delegated to `claude_runner` crate
- ❌ Context injection from wplan → delegated to `dream_agent` crate
- ❌ Configuration hierarchy → delegated to `config_hierarchy` crate

## Features

- **Zero Dependencies**: Core functionality with no external dependencies
- **Session Storage Management**: Resolve session storage paths
- **Continuation Detection**: Check if Claude Code conversation exists
- **Path Escaping**: Handle special characters in paths

## Usage

```rust
use claude_session::{ SessionManager, Strategy, check_session_exists };
use std::path::PathBuf;

// Create session manager
let sessions_dir = PathBuf::from("./");
let mgr = SessionManager::new(&sessions_dir);

// Ensure session directory exists
let session_dir = mgr.ensure_session("my-session", Strategy::Resume)?;

// Check if session has conversation history
if check_session_exists(&session_dir) {
  println!("Session has conversation history");
}

// For execution, use claude_runner crate
// See claude_runner documentation for ClaudeCommand usage
# Ok::<(), std::io::Error>(())
```

## Testing

```bash
cargo nextest run
```
