# claude_storage specification

## responsibility

CLI tool for exploring and analyzing Claude Code's filesystem-based conversation storage.

**Core purpose**: Provide interactive and scriptable command-line interface for querying Claude Code storage at `~/.claude/`.

**Extraction context**: This is the CLI-focused crate after extracting core library functionality to `claude_storage_core` (2025-11-29). All storage access logic now lives in the core library; this crate provides the command-line interface.

**In scope**:
- Interactive REPL for storage exploration
- One-shot command execution for scripting
- Storage statistics and reporting (projects, sessions, entries, tokens)
- Project and session listing with filtering
- Session detail display
- Fast counting operations

**Out of scope**:
- Core storage library (see `claude_storage_core`)
- Process control (see `claude_session`)
- Workflow automation (see `wplan_agent`)
- High-level AI integration logic

## design principles

### backward compatibility is a non-goal

claude_storage explicitly rejects backward compatibility as a design constraint. Breaking changes are acceptable and expected:

- **No CLI stability guarantees**: Command names, parameter syntax can change between versions
- **No output format versioning**: JSON structure, text formatting can change freely
- **No deprecated command preservation**: Old commands deleted, not aliased
- **No command parameter stability**: Parameter names and formats can change

**Rationale**:
- **Simpler evolution**: CLI can evolve without legacy command burden
- **Better UX**: Can redesign commands based on user feedback
- **Cleaner code**: No compatibility shims for old command syntax
- **Utility focus**: Tool for developers who can adapt to changes

**Upgrade Protocol**:
1. Update binary
2. Scripts using claude_storage commands may need updates
3. No automated migration for command syntax

**Implications for Developers**:
- Don't preserve old command names "for backward compatibility"
- Don't maintain old output formats alongside new ones
- Don't add version detection logic for output format
- CLI commands can be renamed/restructured freely

### command versioning

While backward compatibility is a non-goal, command versions in `unilang.commands.yaml` serve as release metadata to communicate API changes to users.

**Versioning Policy**:

1. **Breaking Changes**: Commands with breaking changes have their version synchronized with the crate version for that release
   - Example: v1.3.0 changed `.show` default behavior → `.show` version = 1.3.0

2. **Related Commands**: Functionally related commands (especially deprecated pairs) should have synchronized versions when released together
   - Example: `.show` and `.show.project` (deprecated) both versioned 1.3.0 in same release

3. **Unchanged Commands**: Commands without changes retain their original versions across crate releases
   - Example: `.list`, `.status`, `.count` remain at 1.0.0 even when crate reaches 1.3.0

4. **Version Bounds**: Command versions never exceed the current crate version

**Rationale**:
- Command versions communicate breaking change boundaries to users
- Synchronized versions for related commands reduce confusion about what changed
- Metadata consistency aids release documentation and CHANGELOG accuracy

**Validation**: The test suite includes `command_version_consistency_test.rs` which validates these versioning contracts automatically.

### location-aware commands

Commands adapt to the current working directory when appropriate, following the principle of progressive disclosure (simple cases have simplest syntax).

**Location Awareness Pattern**:
- Commands work on current directory by default when it makes sense
- Parameters can override the default location
- No parameters = current location (most common case)
- Explicit parameters = specific location (when needed)

**Design Goals**:
- **Minimize typing**: Common case (current directory) requires no parameters
- **Stay contextual**: Commands respect where user is working
- **Allow flexibility**: Can still specify different locations when needed
- **Progressive disclosure**: Simple syntax for simple cases, detailed syntax available when needed

**Example**: `.show` command
- `cd /project && .show` → Shows current project
- `.show session_id::abc` → Shows session in current project
- `.show project::/other/path` → Shows different project
- `.show session_id::abc project::/other/path` → Shows session in different project

**Rationale**:
- Most common workflow: user navigates to project directory, then explores it
- Reduces typing friction for interactive exploration
- Matches user mental model ("show me what's here")
- Still supports scripting with explicit parameters

**Example**: `.list` command with smart session display

The `.list` command uses **smart parameter detection** for session display:
- Providing session filters (`session::`, `agent::`, `min_entries::`) automatically enables session display
- Explicit `sessions::0` or `sessions::1` overrides auto-detection
- No filters = projects only (default behavior)

Simple case (projects only):
```bash
# List all projects (no sessions shown)
claude_storage .list
```

Auto-enabled case (filter triggers display):
```bash
# List projects + sessions matching "commit"
# (sessions auto-enabled because session:: filter provided)
claude_storage .list session::commit

# List projects + agent sessions only
# (sessions auto-enabled because agent:: filter provided)
claude_storage .list agent::1

# List projects + sessions with 10+ entries
# (sessions auto-enabled because min_entries:: filter provided)
claude_storage .list min_entries::10
```

Explicit control (override auto-enable):
```bash
# Explicit enable (backward compatible)
claude_storage .list sessions::1

# Explicit disable (overrides auto-enable)
claude_storage .list sessions::0 session::commit  # Projects only, filter ignored
```

Combined filters:
```bash
# List projects + sessions matching "commit" with 10+ entries
claude_storage .list session::commit min_entries::10
```

**Rationale**:
- Progressive disclosure: Don't require `sessions::1` when user clearly wants to filter sessions
- Garbage parameter prevention: If user provides session filter, they want to see sessions
- Explicit control preserved: Power users can still force behavior with `sessions::0` or `sessions::1`

## architecture

### dependencies

**claude_storage_core**: Core library providing all storage access functionality
- Zero-dependency library for reading/writing Claude Code storage
- Provides Storage, Project, Session, Entry types
- JSON parser, path encoding, statistics
- See `claude_storage_core/spec.md` for complete library documentation

**unilang**: CLI framework for command parsing and REPL
- Version 0.35+
- Build-time YAML command registration
- Static PHF maps for O(1) command lookup

**phf**: Perfect hash functions for static command registry
- Compile-time command map generation
- Zero runtime overhead

### cli architecture

**Build system** (`build.rs`):
- Transforms simplified YAML (`unilang.commands.yaml`) to unilang nested schema
- Generates static PHF command registry at compile time
- 7 commands compiled into binary with O(1) lookup

**Command routines** (`src/cli/mod.rs`):
- `status_routine`: Aggregates global statistics from claude_storage_core
- `list_routine`: Lists projects/sessions using core library APIs
- `show_routine`: Displays session details with entry preview
- `show_project_routine`: Displays project details and all sessions
- `count_routine`: Fast counting leveraging core library optimizations
- `search_routine`: Full-text search across conversation content (REQ-012)
- `export_routine`: Export session data to multiple formats (REQ-013)

**Main entry point** (`src/main.rs`):
- Dual-mode support: REPL (interactive) and one-shot (scripting)
- Static routine map using `phf::phf_map!`
- Command registry built from generated static commands
- Stateless pipeline execution (each command independent)

**Command set** (7 commands):
- `.status` - Show storage statistics (projects, sessions, entries, tokens)
- `.list` - List projects or sessions with optional filtering
- `.show` - Display details about a specific session
- `.show.project` - Display project details and all sessions (DEPRECATED)
- `.count` - Fast counting of entries, sessions, or projects
- `.search` - Full-text search across conversation content (REQ-012)
- `.export` - Export session data to JSON/CSV/TSV formats (REQ-013)

### storage model context

Claude Code uses filesystem-native storage at `~/.claude/` (accessed via claude_storage_core):

```
~/.claude/
├── projects/              # All conversation projects
│   ├── {uuid}/           # UUID projects (web/IDE sessions)
│   │   ├── {session-id}.jsonl      # Main conversation
│   │   └── agent-{id}.jsonl        # Sub-agent sessions
│   └── -{path-encoded}/  # Path projects (CLI sessions)
│       └── {session-id}.jsonl
├── history.jsonl         # Global project index
├── .credentials.json     # API credentials
└── session-env/          # Session metadata
```

**Path encoding** (handled by core library):
- `/home/user/project` → `-home-user-project`
- Prefix with `-`, replace `/` with `-`

## cli usage

### modes

**REPL mode** (interactive):
```bash
cargo run
# Or: claude_storage (when installed)

> .status
Storage Statistics:
  Projects: 227
  Sessions: 450
  Total Entries: 12,345
  ...

> .list target::projects
UUID projects: 150
Path projects: 77

> .show session::{session-id}
Session: abc123-def456
Entries: 234
First: 2025-11-20 10:30:45
Last: 2025-11-29 15:22:10
...

> exit
```

**One-shot mode** (scripting):
```bash
# Get quick statistics
cargo run -- .status

# Count specific resources
cargo run -- .count target::projects
cargo run -- .count target::sessions project::{project-id}

# List with filtering
cargo run -- .list target::projects filter::path

# Show session details
cargo run -- .show session::{session-id} verbosity::2
```

### commands

**.status** - Show storage statistics
- Parameters:
  - `path::{path}` (optional) - Custom storage path (default: ~/.claude/)
  - `verbosity::N` (0-5, default: 1) - Output detail level
- Output: Projects, sessions, entries, token usage
- Examples:
  - `.status` - Show default storage statistics
  - `.status verbosity::2` - Detailed statistics
  - `.status path::/custom/storage` - Statistics for custom path

**.list** - List projects or sessions

**Smart Session Display**:
- Providing session filters (`session::`, `agent::`, `min_entries::`) **automatically enables** session display
- Explicit `sessions::0` or `sessions::1` overrides auto-detection
- No filters → Projects only (default behavior)

**Parameters**:
- `type::{uuid|path|all}` (optional, default: all) - Filter by project type
- `verbosity::N` (0-5, default: 1) - Output detail level
- `sessions::{0|1}` (optional, auto-detected) - Show sessions for each project (auto-enabled when session filters provided)
- `path::{value}` (optional) - Filter projects by path using smart resolution (see Path Resolution below)
- `agent::{0|1}` (optional) - Filter sessions by type (0=main only, 1=agent only, unset=all) - **auto-enables session display**
- `min_entries::N` (optional) - Filter sessions by minimum entry count - **auto-enables session display**
- `session::{substring}` (optional) - Filter sessions by ID substring (case-insensitive) - **auto-enables session display**

**Path Resolution** (for `path::` parameter):

The `path::` parameter supports both shell-style path resolution and pattern matching with smart detection:

- **Special paths** (resolved to absolute paths):
  - `path::.` → Current working directory
  - `path::..` → Parent directory
  - `path::~` → Home directory
  - `path::~/subdir` → Home directory + relative path

- **Absolute paths** (used directly for matching):
  - `path::/home/user/project` → Match paths containing this string

- **Relative paths** (resolved to absolute, then matched):
  - `path::../other` → Resolve to absolute, then match

- **Patterns** (substring matching, unchanged):
  - `path::willbe` → Match any path containing "willbe" (backward compatible)
  - `path::storage` → Match any path containing "storage"

**Detection Algorithm**:
1. If parameter is `.`, `..`, `~`, or starts with `~/` → Resolve to absolute path
2. If parameter starts with `/` → Use as-is (already absolute)
3. If parameter contains `/` → Resolve relative path to absolute
4. Otherwise → Treat as pattern for substring matching (backward compatible)

After resolution, all paths use case-insensitive substring matching against project paths.

**Examples**:
```bash
# List all projects (no sessions)
.list

# List path-based projects
.list type::path

# List projects with sessions (explicit enable)
.list sessions::1

# Path resolution examples (NEW)
# Current directory (resolves to absolute path)
cd /home/user1/pro/lib/willbe/module/claude_storage
.list path::.

# Parent directory
.list path::..

# Home directory
.list path::~

# Home + relative path
.list path::~/pro/lib

# Pattern matching (backward compatible)
# Find projects containing "willbe" anywhere in path
.list path::willbe

# Find sessions containing "commit" (auto-enables session display)
.list session::commit

# List agent sessions with 10+ entries (auto-enables session display)
.list agent::1 min_entries::10

# Combine path and session filters
.list path::claude_storage session::default

# Explicit disable overrides auto-enable (projects only, filter ignored)
.list sessions::0 session::commit
```

**Smart Behavior Examples**:

| Command | Sessions Shown? | Reason |
|---------|----------------|--------|
| `.list` | No | No filters provided |
| `.list sessions::1` | Yes | Explicit enable |
| `.list sessions::0` | No | Explicit disable |
| `.list session::commit` | Yes | Auto-enabled (session filter) |
| `.list agent::1` | Yes | Auto-enabled (agent filter) |
| `.list min_entries::10` | Yes | Auto-enabled (min_entries filter) |
| `.list sessions::0 session::X` | No | Explicit disable wins |

**.show** - Display session or project details (location-aware)

**Smart Behavior** (adapts based on parameters provided):
- **No parameters** → Shows current directory project (all sessions)
- **session_id only** → Shows that session in current project
- **project only** → Shows that project (all sessions)
- **Both parameters** → Shows that session in that project

**Parameters**:
- `session_id::{uuid-or-agent-id}` (optional) - Session UUID or agent-{hex}
- `project::{path-or-id}` (optional) - Project path or UUID (default: current directory)
- `verbosity::N` (0-5, default 1)
- `entries::1` (optional) - Show all entries (only applies when showing specific session)
- `metadata::1` (optional) - Show metadata only (old default behavior)

**REQ-011: Content-First Display** (Breaking Change - 2025-12-05)

The `.show` command now displays **conversation content by default** when showing a specific session, replacing the previous metadata-only behavior. This change prioritizes readability and usefulness over technical metadata.

**Default Behavior** (verbosity::1, NEW):
- Shows actual conversation content in readable chat-log format
- Displays user messages and assistant responses
- Formats timestamps as human-readable (YYYY-MM-DD HH:MM)
- No `entries::1` parameter needed to see messages

**Metadata-Only Mode** (metadata::1, OLD behavior):
- Shows session statistics and metadata only
- Useful for quick stats without reading content
- Equivalent to old verbosity::1 behavior

**Verbosity Levels** (Content-Focused):
- `verbosity::0` - Metadata only (same as metadata::1)
- `verbosity::1` - **Full conversation content** (NEW default)
- `verbosity::2` - Conversation + metadata header
- `verbosity::3+` - Conversation + metadata + extended details

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

[2025-12-02 10:00] User:
apply the fix

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Design Rationale**:
- **Primary use case**: Users want to read conversations, not inspect UUIDs
- **Progressive disclosure**: Content first, metadata on demand
- **Backward compatibility**: `metadata::1` provides old behavior
- **Breaking change acceptable**: Spec allows CLI evolution without stability guarantees (see design principles)

**Examples (.show with content-first display)**:
```bash
# Show current directory project (all sessions)
cd /home/user1/pro/lib/willbe/module/willbe3/-commit
.show

# Show specific session in current project
cd /home/user1/pro/lib/willbe/module/willbe3/-commit
.show session_id::36bfd69b

# Show different project (all sessions)
.show project::/home/user1/pro/lib/willbe/module/willbe3

# Show session in different project
.show session_id::36bfd69b project::/home/user1/pro/lib/willbe/module/willbe3

# Show session with full UUID
.show session_id::36bfd69b-32f1-4d11-ac00-02f19c2fe90f project::/home/user1/pro/lib/willbe/module/willbe3

# Show agent session
.show session_id::agent-022ada42 project::-home-user1-pro-lib-willbe-module-willbe3--commit
```

**REQ-012: Search Command** (2025-12-06)

The `.search` command enables full-text search across conversation content in Claude Code storage. This addresses the "No search" limitation identified in earlier versions and provides actionable search results with session context.

**Purpose**:
- Find conversations by content across all projects and sessions
- Support case-sensitive and case-insensitive search
- Filter by project, session, or entry type (user/assistant messages)
- Provide actionable results with session IDs and context for follow-up with `.show`

**Parameters**:
- `query::{text}` (required) - Search query string (non-empty)
- `project::{path-or-id}` (optional) - Limit search to specific project
- `session::{id}` (optional) - Limit search to specific session within project
- `case_sensitive::{0|1}` (optional, default: 0) - Enable case-sensitive search
- `entry_type::{user|assistant|all}` (optional, default: all) - Filter by message type
- `verbosity::N` (0-5, default: 1) - Output detail level

**Validation Requirements** (V-012):
- V-012.1: Reject missing `query` parameter with error: "query parameter is required"
- V-012.2: Reject empty query string with error: "query cannot be empty"
- V-012.3: Validate `case_sensitive` accepts only 0 or 1 (error: "case_sensitive must be 0 or 1")
- V-012.4: Validate `entry_type` accepts only user, assistant, or all (error: "entry_type must be user, assistant, or all")
- V-012.5: Validate `verbosity` range 0-5 (error: "verbosity must be 0-5")
- V-012.6: Validate project exists when specified (error: "project not found: {path}")
- V-012.7: Validate session exists in project when specified (error: "session not found: {id}")

**Examples**:
```bash
# Basic search (case-insensitive, all messages)
.search query::"authentication bug"

# Case-sensitive search
.search query::"AuthenticationError" case_sensitive::1

# Search in specific project
.search query::"database" project::/home/user1/pro/lib/myapp

# Search user messages only
.search query::"how do I" entry_type::user

# Search assistant responses only
.search query::"you can use" entry_type::assistant

# Search specific session
.search query::"performance" session::36bfd69b

# High verbosity (show full context)
.search query::"error handling" verbosity::3
```

**Output Format**:
- **verbosity::1** (default): Session ID, project path, match count, first match preview
- **verbosity::2**: Above + all matching entry timestamps and roles
- **verbosity::3+**: Above + full matching entry content

**Design Rationale**:
- Required `query` prevents accidental full-scan operations
- Case-insensitive default matches common user expectations
- Entry type filtering enables analysis of user questions vs assistant responses
- Session context in results enables follow-up with `.show session_id::{id}`
- Verbosity control balances quick overview vs detailed inspection

**REQ-013: Export Command** (2025-12-06)

The `.export` command enables exporting session data to various formats for backup, sharing, and archiving workflows. This addresses the "Output formats" limitation and provides human-readable and machine-readable export options.

**Purpose**:
- Export conversation history to files for backup and archiving
- Support multiple output formats (Markdown, JSON, Plain Text)
- Enable conversation sharing and documentation
- Facilitate integration with external tools and workflows

**Parameters**:
- `session_id::{uuid}` (required) - Session to export
- `format::{markdown|json|text}` (optional, default: markdown) - Output format
- `output::{path}` (required) - Output file path
- `project::{path-or-id}` (optional) - Project containing session (default: current directory)

**Validation Requirements** (V-013):
- V-013.1: Reject missing `session_id` parameter with error: "session_id parameter is required"
- V-013.2: Reject missing `output` parameter with error: "output parameter is required"
- V-013.3: Validate `format` accepts only markdown, json, or text (error: "format must be markdown, json, or text")
- V-013.4: Validate session exists in project (error: "session not found: {id}")
- V-013.5: Validate output path directory exists (error: "output directory does not exist: {dir}")

**Format Specifications**:

**Markdown Format** (default):
- Formatted conversation with headers and separators
- Timestamps in human-readable format
- User/Assistant roles clearly distinguished
- Suitable for documentation and sharing

**JSON Format**:
- Array of entry objects with full metadata
- Preserves all fields from JSONL storage (timestamp, role, content, tokens, etc.)
- Suitable for programmatic processing and data analysis

**Text Format**:
- Plain text conversation transcript
- Simple format: `[timestamp] Role: content`
- Suitable for basic archiving and text processing

**Examples**:
```bash
# Export to Markdown (default)
.export session_id::36bfd69b output::/tmp/conversation.md

# Export to JSON for programmatic access
.export session_id::36bfd69b format::json output::/tmp/session.json

# Export to plain text
.export session_id::36bfd69b format::text output::/tmp/transcript.txt

# Export from specific project
.export session_id::36bfd69b project::/home/user1/pro/lib/myapp output::/tmp/export.md

# Export agent session
.export session_id::agent-022ada42 format::markdown output::/tmp/agent_session.md
```

**Design Rationale**:
- Required `session_id` and `output` prevent accidental operations and ambiguous behavior
- Markdown default prioritizes human readability for sharing and documentation
- JSON format enables data analysis and programmatic processing
- Text format provides simple archiving option
- Location-aware (respects current directory project like `.show`)
- File-based output ensures clear export destination (no ambiguous stdout behavior)

**.show.project** - Display project details and sessions (DEPRECATED: use `.show` instead)

**Deprecation Notice**: This command is deprecated in favor of `.show` with smart parameter detection. Use `.show` or `.show project::{path}` instead.

**Parameters**:
- `project::{path-or-id}` (optional) - Project path, path-encoded, UUID, or Path(...) from .list output (default: current directory)
- `verbosity::N` (0-5, default 1)

**Examples**:
```bash
# Show current directory project
cd /home/user1/pro/lib/willbe/module/willbe3/-commit
.show.project

# Show specific project
.show.project /home/user1/pro/lib/willbe/module/willbe3/-commit
.show.project Path("/home/user1/pro/lib/willbe/module/willbe3/-commit")  # Copy from .list output
.show.project -home-user1-pro-lib-willbe-module-willbe3--commit  # Path-encoded
.show.project abc-123-def  # UUID project
```

**Migration**: Replace `.show.project` with `.show` (same behavior, better UX)

**.count** - Fast counting operations
- Parameters:
  - `target::projects|sessions|entries` (required)
  - `project::{id}` (for sessions/entries)
  - `session::{id}` (for entries)
- Examples:
  - `.count target::projects`
  - `.count target::sessions project::-home-user-pro`
  - `.count target::entries session::abc123`

### project parameter formats

The `project` parameter in `.show` and `.show.project` commands accepts multiple formats with smart detection:

**Format Detection Logic**:
1. Starts with `Path("` → Extracts path from Debug format (copy-paste from `.list` output)
2. Starts with `/` → Absolute path
3. Starts with `-` → Path-encoded format (decoded to path)
4. Otherwise → UUID

**Supported Formats**:

| Format | Example | Use Case |
|--------|---------|----------|
| Absolute path | `/home/user1/pro/lib/willbe/module/willbe3/-commit` | Direct path specification |
| Path-encoded | `-home-user1-pro-lib-willbe-module-willbe3--commit` | Encoded format (legacy) |
| UUID | `abc-123-def-456` | Web/IDE sessions |
| Debug output | `Path("/home/user1/...")` | Copy-paste from `.list` output |

**Recommended Usage**:
- For CLI sessions: Use absolute paths or copy Debug format from `.list`
- For web/IDE sessions: Use UUID
- Path-encoded format supported but not recommended (use absolute paths instead)

### scripting integration

**Exit codes**:
- 0: Success
- 1: Error (invalid command, not found, parse error)

**Output format**:
- Human-readable by default
- Structured for parsing with grep/awk
- Consistent verbosity levels

**Examples**:
```bash
# Get project count
PROJECT_COUNT=$(cargo run -- .count target::projects | grep -oP '\d+')

# Check if session exists
if cargo run -- .show session::abc123 &>/dev/null; then
  echo "Session exists"
fi

# Export statistics
cargo run -- .status verbosity::3 > storage_stats.txt
```

## implementation status

**Crate Split** ✅ Complete (2025-11-29)
- Core library extracted to `claude_storage_core` crate
- CLI functionality remains in `claude_storage` crate
- All 51 tests moved to core (45 unit + 3 bug + 3 doc)
- CLI crate depends on core library
- Zero test duplication

**CLI Implementation** ✅ Complete
- ✅ **Build System**
  - Build script (`build.rs`) for unilang code generation
  - Static PHF maps for O(1) command lookup
  - YAML to nested schema transformation
- ✅ **Commands** (5 total)
  - `.status` - Storage statistics with verbosity levels
  - `.list` - Project/session listing with filtering
  - `.show` - Session detail display
  - `.show.project` - Project detail display with all sessions
  - `.count` - Fast counting operations
- ✅ **Modes**
  - REPL mode: Interactive command shell
  - One-shot mode: Single command execution for scripting
- ✅ **Command Routines** (`src/cli/mod.rs`)
  - status_routine: Global statistics aggregation
  - list_routine: Filtered listing
  - show_routine: Session detail display
  - show_project_routine: Project detail display with all sessions
  - count_routine: Fast counting
- ✅ **Main Entry** (`src/main.rs`)
  - Dual-mode support (REPL/one-shot)
  - Static routine mapping
  - Command registry from build-time generation
  - Stateless pipeline execution

**Integration Ready**
- ✅ Depends on `claude_storage_core` for all storage operations
- ✅ Example tools available (`examples/`)
- ✅ Integration guide (`docs/integration_guide.md`)
- 📋 Optional CLI integration with `wplan_agent` (future)
- 📋 Scripting examples and best practices (future)

## format documentation

Complete storage and format specifications available in `docs/` and `claude_storage_core/spec.md`.

**Storage organization**: See `docs/storage_organization.md` for complete Claude Code storage architecture.

**File formats**: See `docs/file_formats.md` for all format specifications (JSONL, history, settings, credentials, etc.).

**JSONL conversation format**: See `docs/jsonl_format.md` for detailed entry schemas and field definitions.

**Advanced features**: See `docs/advanced_topics.md` for agent sessions, command system, history tracking.

**Core library API**: See `claude_storage_core/spec.md` for complete library documentation (types, safety guarantees, performance characteristics).

## cargo configuration

**Package**:
```toml
[package]
name = "claude_storage"
version = "0.1.0"
edition = "2021"
```

**Dependencies**:
```toml
[dependencies]
# Core library for all storage operations
claude_storage_core = { path = "../claude_storage_core" }

# CLI dependencies (optional, only for binary)
unilang = { workspace = true, optional = true }
phf = { workspace = true, features = ["macros"], optional = true }

[build-dependencies]
serde_yaml = { workspace = true }
unilang = { workspace = true }
```

**Binary target**:
```toml
[[bin]]
name = "claude_storage"
required-features = ["cli"]
```

**Features**:
```toml
[features]
default = ["cli"]
cli = ["unilang", "phf"]
```

**Default behavior**: The `cli` feature is enabled by default, making the binary immediately usable after installation.

**Build configuration**:
- `build.rs`: Transforms `unilang.commands.yaml` to static PHF registry
- Generated code: Static command map with O(1) lookup
- Build dependencies: `serde_yaml` (YAML parsing), `unilang` (schema generation)

## design principles

**Separation of concerns**: CLI layer separate from core library logic (via claude_storage_core).

**Stateless execution**: Each command executes independently (no shared state in REPL mode).

**Dual-mode support**: Interactive REPL for exploration, one-shot mode for scripting.

**Performance-first**: Leverages core library optimizations (fast counting, lazy loading, selective parsing).

**User-friendly**: Consistent verbosity levels, clear output formatting, built-in help.

**Scriptable**: Exit codes, parseable output, suitable for integration with shell scripts.

**Minimal dependencies**: Only CLI dependencies (unilang, phf) beyond core library.

## testing strategy

**Core library tests**: All 51 tests moved to `claude_storage_core` crate
- 45 unit tests (JSON parser, entry parsing, path encoding, session/project/storage operations)
- 3 bug reproducer tests (UTF-8 multi-byte character handling)
- 3 doc tests (path encoding/decoding, library usage)

**CLI tests** (in this crate): 3 tests total
- Build system test: YAML transformation correctness
- Command routing test: Static PHF map lookup
- REPL pipeline test: Stateless execution verification

**Integration tests**: (Future) End-to-end CLI tests with real Claude Code storage
- Command output verification
- Error handling scenarios
- Scripting mode behavior

**Test execution**:
- Core library: `cd ../claude_storage_core && w3 .test l::3`
- CLI: `w3 .test l::3` (minimal CLI-specific tests)
- Full verification: Run both suites

**Test organization**:
- Zero test duplication (all storage logic tests in core)
- CLI tests focus on command routing and presentation layer only

## performance considerations

**Core library optimizations**: All performance-critical operations handled by `claude_storage_core`
- Lazy loading: Entries loaded on-demand
- Fast counting: 100x speedup via line counting (~5ms vs 500ms for 1000 entries)
- Selective parsing: Statistics without full entry parsing
- Hand-written JSON parser: ~80ns per operation

**CLI-specific optimizations**:
- Static command registry: O(1) command lookup via PHF maps
- Stateless execution: No state management overhead
- Minimal allocations: Command routines reuse storage instances
- Streaming output: Results displayed as computed (no buffering)

**Scalability**:
- Tested with 227 projects, 60M+ tokens
- Fast statistics for large datasets
- Graceful handling of corrupted files

See `claude_storage_core/spec.md` for detailed performance characteristics.

## error handling

**Core library errors**: See `claude_storage_core::Error` for complete error types
- IoError, ParseError, PathEncodingError, NotFoundError, InvalidStructureError, WriteError

**CLI error handling**:
- **Exit codes**: 0 for success, 1 for any error
- **Error messages**: User-friendly formatting with context
- **Graceful degradation**: Continue processing when possible (e.g., skip corrupted sessions)
- **Verbose mode**: Additional error details at higher verbosity levels

**Error scenarios**:
- Invalid command: Display help and usage
- Missing parameters: Clear parameter error message
- Storage not found: Suggest checking `~/.claude/` directory
- Corrupted files: Warning + continue with valid files
- No projects: Empty result (not error)

## known limitations

**Storage vs Filesystem Persistence**:
- Project paths in storage may reference directories that no longer exist
- `.list` output shows paths as they were when sessions were created
- Commands accept these paths even if directories are deleted
- This is expected behavior - storage persists independently of filesystem
- Example: `/home/user1/project/-commit` may not exist, but its storage does

**Session Discovery**:
- Sessions are identified by UUID or `agent-{hex}` format only
- Topic names (like `-commit`, `-default_topic`) are part of project path, not session IDs
- To show a session, you need both: session UUID + project path
- Use `.show.project` to explore all sessions in a project without knowing UUIDs

**Read-only CLI**: No commands for modifying storage (matches safe-by-default principle).

**Basic filtering**: Advanced filtering options planned (date ranges, token thresholds, multi-criteria search).

**Single storage**: Always uses `~/.claude/` (no custom storage location support).

**Core library limitations**: See `claude_storage_core/spec.md` for library-level limitations (write operations, streaming, compression).

## future enhancements

**CLI Features** (priority order):
1. **Advanced filtering**: Date ranges, token thresholds, multi-criteria search refinements
2. **Export enhancements**: Bulk export, archive utilities, custom format templates
3. **Analytics commands**: Token usage trends, conversation patterns, search analytics
4. **Interactive improvements**: Tab completion, command history search, search result navigation
5. **Search refinements**: Regular expression support, fuzzy matching, proximity search

**Integration**:
- Optional CLI integration with `wplan_agent` workflows
- Shell completion scripts (bash, zsh, fish)
- Man pages and extended documentation

**Core library enhancements**: See `claude_storage_core/spec.md` for library roadmap
- Write operations (Phase 4)
- Streaming API (Phase 5)
- Query DSL (Phase 5)

## related crates

**claude_storage_core**: Core library providing all storage access functionality. This CLI crate is a thin wrapper around the core library.

**claude_session**: Process control layer for spawning Claude Code CLI. May use `claude_storage_core` for session detection (optional integration).

**wplan_agent**: AI integration layer providing Claude Code sessions with wplan context injection. May use `claude_storage_core` for conversation history analysis (optional integration).

**wplan_daemon**: Job execution daemon. No direct dependency on claude_storage (uses `wplan_agent` instead).
