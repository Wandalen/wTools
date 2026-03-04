# claude_storage

CLI tool for exploring and analyzing Claude Code's filesystem-based conversation storage.

## overview

This crate provides a command-line interface for querying Claude Code's conversation storage at `~/.claude/`. It wraps the `claude_storage_core` library with an interactive REPL and one-shot command interface.

**v1.0 Status**: Core library (`claude_storage_core`) is production-ready with comprehensive validation (122 tests, production session parsing). CLI wrapper commands `.status`, `.list`, and `.count` are fully validated. For programmatic access or advanced usage, we recommend using the `claude_storage_core` library API directly (see "library api" section below).

**Extraction context**: This is the CLI-focused crate after extracting core library functionality to `claude_storage_core` (2025-11-29).

## installation

```bash
cargo install --path . --features cli
```

Or run directly:
```bash
cargo run --features cli
```

## usage

### repl mode (interactive)

```bash
cargo run --features cli
```

```
claude_storage> .status
Storage: "/home/user/.claude"
Projects: 230 (UUID: 14, Path: 216)
Sessions: 7546 (Main: 1061, Agent: 6485)
Entries: 323231

claude_storage> .list target::projects
UUID projects: 14
Path projects: 216

claude_storage> .count target::sessions
Total sessions: 7546

claude_storage> exit
```

### one-shot mode (scripting)

```bash
# Get storage statistics
cargo run --features cli -- .status

# Count projects
cargo run --features cli -- .count target::projects

# List projects with filtering
cargo run --features cli -- .list target::projects filter::path

# Show session details
cargo run --features cli -- .show session::abc123 verbosity::2
```

## commands

### .status

Show storage statistics (projects, sessions, entries, tokens).

**Parameters**:
- `verbosity::N` (0-5, default 1) - Detail level

**Example**:
```bash
.status verbosity::2
```

### .list

List projects or sessions with optional filtering.

**Parameters**:
- `type::{uuid|path|all}` (optional, default: all) - Filter by project type
- `verbosity::N` (0-5, default: 1) - Output detail level
- `sessions::{0|1}` (optional, auto-detected) - Show sessions (auto-enabled when session filters provided)
- `path::{value}` (optional) - Filter projects by path (supports smart resolution, see below)
- `agent::{0|1}` (optional) - Filter sessions by type (auto-enables session display)
- `min_entries::N` (optional) - Filter sessions by minimum entry count (auto-enables session display)
- `session::{substring}` (optional) - Filter sessions by ID substring (auto-enables session display)

**Path Parameter - Smart Resolution**:

The `path::` parameter supports both shell-style path resolution and pattern matching:

- **Special paths** (resolved to absolute paths):
  - `path::.` → Current working directory
  - `path::..` → Parent directory
  - `path::~` → Home directory
  - `path::~/subdir` → Home directory + relative path

- **Patterns** (substring matching):
  - `path::willbe` → Match any path containing "willbe"
  - `path::storage` → Match any path containing "storage"

**Examples**:
```bash
# List all projects
.list

# List path-based projects only
.list type::path

# Path resolution (current directory)
cd /home/user/project
.list path::.

# Path resolution (parent directory)
.list path::..

# Path resolution (home directory)
.list path::~

# Pattern matching (backward compatible)
.list path::willbe

# Filter sessions with auto-enable
.list session::commit          # Auto-enables session display
.list agent::1 min_entries::10 # Agent sessions with 10+ entries

# Combine filters
.list path::claude_storage session::default
```

### .show

Display session or project details with **conversation content by default** (REQ-011: Content-First Display).

**Smart Behavior** (adapts to parameters):
- **No parameters** → Shows current directory project (all sessions)
- **session_id only** → Shows that session in current project with conversation content
- **project only** → Shows that project (all sessions)
- **Both parameters** → Shows that session in that project with conversation content

**Parameters**:
- `session_id::{uuid-or-agent-id}` (optional) - Session UUID or agent-{hex}
- `project::{path-or-id}` (optional) - Project path or UUID (default: current directory)
- `verbosity::N` (0-5, default 1) - Output detail level
- `entries::1` (optional) - Show all entries (backward compat with old UUID list format)
- `metadata::1` (optional) - Show metadata only (old behavior, no conversation content)

**Default Behavior** (NEW):
Shows actual conversation content in readable chat-log format. No parameters needed to read messages.

**Examples**:
```bash
# Show current directory project (all sessions)
cd /home/user/project
.show

# Show session with conversation content (default)
.show session_id::abc123

# Show session in different project
.show session_id::abc123 project::/home/user/project

# Metadata only (old behavior)
.show session_id::abc123 metadata::1

# Increase verbosity for metadata footer
.show session_id::abc123 verbosity::2
```

**Content Format**:
```
Session: 79f86582... (2893 entries)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[2025-12-02 09:57] User:
last 3 biig tasks solved in this context?

[2025-12-02 09:57] Assistant:
I'll analyze the recent conversation history...

**Recent Major Tasks Completed:**
1. **tree_fmt Standardization**
2. **Path Filter Bug Investigation**
3. **Test Suite Fixes**

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### .count

Fast counting operations (projects, sessions, entries).

**Parameters**:
- `target::projects|sessions|entries` (required)
- `project::{id}` (for sessions/entries)
- `session::{id}` (for entries)

**Examples**:
```bash
.count target::projects
.count target::sessions project::-home-user-pro
.count target::entries session::abc123
```

### .search

Search session content for query string.

**Parameters**:
- `query::{text}` (required) - Search query (case-insensitive by default)
- `project::{id}` (optional) - Limit search to specific project
- `session::{id}` (optional) - Limit search to specific session
- `case_sensitive::1` (optional) - Enable case-sensitive matching
- `entry_type::user|assistant` (optional) - Filter by entry type
- `verbosity::N` (0-5, default 1) - Detail level

**Examples**:
```bash
.search query::error
.search query::"session management" case_sensitive::1
.search query::implement project::-home-user-pro
```

### .export

Export session to file (markdown, JSON, or text).

**Parameters**:
- `session_id::{id}` (required) - Session ID to export
- `output::{path}` (required) - Output file path
- `format::markdown|json|text` (optional, default: markdown) - Export format
- `project::{id}` (optional) - Project ID if not in current directory

**Formats**:
- **markdown** (.md) - Human-readable with metadata and formatted entries
- **json** (.json) - Machine-readable structured format
- **text** (.txt) - Simple conversation transcript

**Examples**:
```bash
.export session_id::-default_topic output::conversation.md
.export session_id::abc123 format::json output::session.json
.export session_id::xyz789 format::text output::transcript.txt project::-home-user-pro
```

**Note**: Sessions may contain non-conversation metadata entries (queue-operation, summary) which are automatically skipped during export. Only conversation entries (user/assistant messages) are included in the exported output.

## scripting integration

**Exit codes**:
- 0: Success
- 1: Error

**Examples**:
```bash
# Get project count
PROJECT_COUNT=$(cargo run --features cli -- .count target::projects | grep -oP '\d+')

# Check if session exists
if cargo run --features cli -- .show session::abc123 &>/dev/null; then
  echo "Session exists"
fi

# Export statistics
cargo run --features cli -- .status verbosity::3 > storage_stats.txt
```

## library api

For programmatic access to Claude Code storage, use `claude_storage_core` directly:

```toml
[dependencies]
claude_storage_core = "1.0.0"
# Or for local development:
# claude_storage_core = { path = "../claude_storage_core" }
```

```rust
use claude_storage_core::{ Storage, ProjectId };

fn main() -> claude_storage_core::Result< () >
{
  let storage = Storage::new()?;
  for project in storage.list_projects()?
  {
    println!( "Project: {:?}", project.id() );
  }
  Ok( () )
}
```

## architecture

**Dependencies**:
- `claude_storage_core` - Core library for all storage operations
- `unilang` - CLI framework for command parsing
- `phf` - Perfect hash functions for static command registry

**Build system**:
- `build.rs` - Transforms YAML command definitions to static PHF registry
- `unilang.commands.yaml` - Command definitions (6 commands)
- Generated code: Static command map with O(1) lookup

**Command routines** (`src/cli/mod.rs`):
- `status_routine` - Global statistics aggregation
- `list_routine` - Filtered listing
- `show_routine` - Session detail display
- `count_routine` - Fast counting
- `search_routine` - Content search
- `export_routine` - Session export

## documentation

- **Specification**: `spec.md` - Complete CLI architecture and usage
- **Migration guide**: `docs/MIGRATION.md` - Migration from monolithic crate
- **Format docs**: `docs/` - Storage organization, file formats, advanced topics
- **Integration guide**: `docs/integration_guide.md` - Library integration examples

## testing

**Core library tests**: 105 tests in `claude_storage_core` crate
- Entry parsing and validation
- Path encoding/decoding
- JSON parser
- Filtering system
- Content search
- Export functionality (markdown, JSON, text)
- Statistics aggregation
- Bug reproducers with comprehensive documentation

**CLI tests**: 17 integration tests
- Storage operations tests (global stats, project listing)
- Session operations tests (show, stats, entry counts)
- Counting operations tests (projects, sessions, entries)
- Full workflow integration test
- CLI sanity tests (build, features)

## license

MIT
