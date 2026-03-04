# claude code storage organization

## overview

Claude Code stores all conversation data, settings, and metadata in `~/.claude/` directory using filesystem-native architecture.

**Storage model**: Append-only JSONL files organized into project/session hierarchy.

**Key characteristics**:
- Single source of truth (no caching)
- Filesystem-native (no database engine)
- Human-readable formats (JSONL, JSON)
- Append-only write pattern
- No schema migrations required

## directory structure

```
~/.claude/                        # Root storage (1.9GB total)
├── projects/                     # 1.4GB - All conversation projects
│   ├── {uuid}/                   # UUID projects (web/IDE sessions)
│   │   ├── {session-id}.jsonl   # Main conversation
│   │   └── agent-{id}.jsonl     # Sub-agent sessions
│   └── -{path-encoded}/          # Path projects (CLI sessions)
│       └── {session-id}.jsonl   # CLI conversation
├── debug/                        # 429MB - Debug logs
├── todos/                        # 63MB - Task tracking
├── shell-snapshots/              # 45MB - Shell environment captures
├── session-env/                  # 2.2MB - Session metadata
├── commands/                     # <1MB - Command definitions
├── history.jsonl                 # 1.1MB - Global project index
├── .credentials.json             # API credentials
└── settings.json                 # User settings
```

## directory purposes

### projects/ - conversation storage (1.4GB)

**Purpose**: Primary storage for all Claude Code conversations.

**Organization**: Two project types based on launch context.

**UUID projects** (web/IDE sessions):
```
projects/26dd749d-5b4b-bfee-f4f3-9e03803b8cad/
├── 8d795a1c-c81d-4010-8d29-b4e678272419.jsonl    # Main session
├── agent-f3e2d1c4-a5b6-8910-1234-567890abcdef.jsonl  # Sub-agent
└── agent-a1b2c3d4-e5f6-7890-abcd-ef1234567890.jsonl  # Sub-agent
```

**Path projects** (CLI sessions):
```
projects/-home-user1-pro-lib-willbe-module-wplan_agent/
├── 3a4b5c6d-e7f8-9012-3456-789abcdef012.jsonl    # Session 1
└── 7e8f9a0b-c1d2-3456-7890-abcdef123456.jsonl    # Session 2
```

**Path encoding rules**:
1. Prefix with `-` (hyphen)
2. Replace all `/` with `-`
3. Preserve spaces and other characters

**Examples**:
- `/home/user/project` → `-home-user-project`
- `/home/user/my project/code` → `-home-user-my project-code`

**Session file naming**: UUID v4 format + `.jsonl` extension.

**Growth characteristics**:
- 225 total projects discovered
- Mix of UUID (web/IDE) and path (CLI) projects
- Average project size: ~6.2MB
- Session files range from KB to tens of MB

### debug/ - debug logs (429MB)

**Purpose**: Debug output from Claude Code operations.

**File format**: Plain text with `[DEBUG]` prefix per line.

**Content types**:
- Setting file watching
- Plugin loading status
- LSP server initialization
- Shell snapshot creation
- Process lifecycle events

**Typical entries**:
```
[DEBUG] Watching for changes in setting files...
[DEBUG] Found 0 plugins (0 enabled, 0 disabled)
[DEBUG] Total LSP servers loaded: 0
[DEBUG] Creating shell snapshot at /home/user1/.claude/shell-snapshots/...
```

**Growth pattern**: Grows continuously during Claude Code usage, can become large over time.

**Maintenance**: Can be safely deleted to reclaim space (no impact on conversations).

### todos/ - task tracking (63MB)

**Purpose**: Store todo lists for conversation sessions.

**File organization**: One JSON file per session UUID.

**File format**: JSON array of task objects.

**Example**:
```json
[
  {
    "content": "Plan comprehensive manual testing strategy",
    "status": "completed",
    "activeForm": "Planning comprehensive manual testing strategy"
  },
  {
    "content": "Implement manual test runner",
    "status": "in_progress",
    "activeForm": "Implementing manual test runner"
  }
]
```

**Task states**: `pending`, `in_progress`, `completed`.

**Size characteristics**: Varies widely (empty to several MB per file).

### shell-snapshots/ - environment captures (45MB)

**Purpose**: Preserve shell environment for session restoration.

**File format**: Bash script with base64-encoded functions.

**Content structure**:
```bash
# Snapshot file
# Unset all aliases to avoid conflicts with functions
unalias -a 2>/dev/null || true

# Functions
eval "$(echo 'Z2F3a2xpYnBhdGhfYXBwZW5k...' | base64 -d)"

# Shell options
shopt -s checkwinsize
shopt -s cmdhist
shopt -s expand_aliases
...
```

**Purpose**: Restore exact bash environment when resuming sessions.

**File naming**: UUID-based (matches session IDs).

**Growth**: One file per CLI session with shell context.

### session-env/ - session metadata (2.2MB)

**Purpose**: Store session-specific metadata.

**Organization**: Empty directories named by session UUID.

**Typical structure**:
```
session-env/
├── 8d795a1c-c81d-4010-8d29-b4e678272419/    # Empty directory
├── a6f3bd8c-1234-5678-90ab-cdef12345678/    # Empty directory
└── ...
```

**Current status**: Directories exist but are empty (metadata storage mechanism, possibly unused).

**Size**: Minimal (directory metadata only).

### commands/ - command definitions

**Purpose**: Store custom command definitions for Claude Code.

**File format**: Markdown files (`.md`).

**File count**: 48 command files discovered.

**Typical filenames**:
- `commit.md`
- `refactor_extracting.md`
- `audit_assessment_questions.md`
- `test_clean.md`

**Purpose**: Define custom slash commands available in Claude Code sessions.

### history.jsonl - global project index (1.1MB)

**Purpose**: Track all project accesses and context across sessions.

**File format**: JSONL (one JSON object per line).

**Entry structure**:
```json
{
  "display": "https://www.youtube-transcript.io/api\nread page...",
  "pastedContents": {},
  "timestamp": 1758992388766,
  "project": "/home/user1/pro/lib/willbe/module/reasoner"
}
```

**Fields**:
- `display` (string) - User query or context preview
- `pastedContents` (object) - Pasted file contents (usually empty)
- `timestamp` (number) - Unix timestamp in milliseconds
- `project` (string) - Filesystem path of project directory

**Size**: 4324 entries (1.1MB).

**Growth**: Appends one entry per conversation start.

**Purpose**: Enables recent project discovery and conversation context tracking.

### .credentials.json - api credentials

**Purpose**: Store Claude AI API authentication tokens.

**File format**: JSON object.

**Structure**:
```json
{
  "claudeAiOauth": {
    ... (authentication tokens)
  }
}
```

**Security**: Sensitive file containing API credentials.

**Permissions**: Should be user-readable only.

### settings.json - user settings

**Purpose**: Store Claude Code configuration and preferences.

**File format**: JSON object.

**Structure**:
```json
{
  "hooks": {
    "Notification": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "INPUT=$(cat) && DIR=..."
          }
        ]
      }
    ],
    "Stop": [...]
  },
  "alwaysThinkingEnabled": true
}
```

**Key settings**:
- `hooks` - Shell hooks for events (Notification, Stop, etc.)
- `alwaysThinkingEnabled` - Enable/disable thinking mode by default

**Hook types**: Command hooks that execute on specific events.

## file formats summary

| Format | Usage | Size Range | Growth Pattern |
|--------|-------|------------|----------------|
| `.jsonl` | Conversations | 10KB - 50MB | Append-only per session |
| `.json` | Settings, todos, credentials | 1KB - 5MB | Overwrite updates |
| `.txt` | Debug logs | 1KB - 100MB | Continuous append |
| `.sh` | Shell snapshots | 5KB - 500KB | One per session |
| `.md` | Commands | 1KB - 50KB | Static definitions |

## storage characteristics

### total size distribution

```
Total: ~1.9GB
├── projects/          1.4GB (74%)
├── debug/             429MB (23%)
├── todos/             63MB  (3%)
├── shell-snapshots/   45MB  (<1%)
├── session-env/       2.2MB (<1%)
└── other files        <1MB  (<1%)
```

**Primary storage**: Conversation data (projects/) dominates at 74%.

**Secondary storage**: Debug logs (debug/) at 23%.

**Metadata**: Everything else is <3%.

### growth patterns

**Conversations (projects/)**:
- Linear growth with usage
- Each session adds 100KB - 10MB+
- Long conversations can reach 50MB+
- Growth rate: ~50-100MB per week (heavy usage)

**Debug logs (debug/)**:
- Continuous append during operation
- Can grow indefinitely
- Safe to delete/rotate
- Growth rate: ~20-50MB per week

**Todos (todos/)**:
- Grows with active sessions
- Modest size per file
- Growth rate: ~5-10MB per week

**Shell snapshots (shell-snapshots/)**:
- One per CLI session
- Fixed size per snapshot
- Growth rate: ~2-5MB per week

### maintenance considerations

**Safe to delete**:
- `debug/` - Debug logs (can be rotated)
- `shell-snapshots/` - Old snapshots (if sessions closed)

**Never delete**:
- `projects/` - Contains all conversation history
- `history.jsonl` - Project index
- `.credentials.json` - Required for authentication
- `settings.json` - User preferences

**Archive candidates**:
- Old projects in `projects/` (if not needed)
- Completed todos in `todos/`

## access patterns

### read operations

**High frequency**:
- `projects/{project}/{session}.jsonl` - Current session access
- `settings.json` - Settings lookup
- `.credentials.json` - Authentication

**Medium frequency**:
- `history.jsonl` - Recent project discovery
- `todos/{session}.json` - Task tracking
- `shell-snapshots/{session}.sh` - Environment restoration

**Low frequency**:
- Old sessions in `projects/`
- Debug logs in `debug/`
- Command definitions in `commands/`

### write operations

**Append-only**:
- `projects/{project}/{session}.jsonl` - Conversation entries
- `history.jsonl` - New project entries
- `debug/*.txt` - Debug messages

**Overwrite**:
- `settings.json` - Settings updates
- `todos/{session}.json` - Task updates

**Create once**:
- `shell-snapshots/{session}.sh` - Session creation
- `session-env/{session}/` - Session initialization

## security considerations

### sensitive files

**High sensitivity**:
- `.credentials.json` - API authentication tokens
- `projects/` - May contain proprietary code/data

**Medium sensitivity**:
- `history.jsonl` - Project paths reveal directory structure
- `shell-snapshots/` - May contain environment variables

**Low sensitivity**:
- `settings.json` - User preferences
- `todos/` - Task lists
- `debug/` - Debug logs

### permissions

**Recommended**:
```bash
chmod 700 ~/.claude/                    # rwx------
chmod 600 ~/.claude/.credentials.json   # rw-------
chmod 700 ~/.claude/projects/           # rwx------
chmod 644 ~/.claude/settings.json       # rw-r--r--
```

**Rationale**: Conversation data and credentials should be user-only; settings can be readable by others.

## design principles

### filesystem-native

**No database engine**: All data stored in plain files (JSONL, JSON, text).

**Advantages**:
- Human-readable formats
- Standard tools work (jq, grep, less)
- No schema migrations
- Easy backup/sync
- Language-agnostic access

### append-only writes

**Pattern**: Conversation files only append, never modify existing entries.

**Benefits**:
- No data loss from corruption
- Atomic writes (append is atomic)
- Simple concurrency model
- Audit trail preserved

**Implementation**: Write to temp file, then append to target.

### single source of truth

**No caching**: Always read from disk.

**Benefits**:
- No cache invalidation bugs
- Multiple processes can read safely
- Simpler implementation
- Memory efficient

**Trade-off**: Slightly higher I/O, but acceptable for conversation-scale data.

### lazy loading

**Pattern**: Only load data when explicitly requested.

**Example**: Session entries loaded on first access, not at Session::load() time.

**Benefits**:
- Fast session listing
- Low memory usage
- Scales to large projects

## implementation notes

### path encoding algorithm

```rust
// Encoding: /home/user/project → -home-user-project
fn encode_path(path: &Path) -> String {
  format!("-{}", path.display().to_string().replace('/', "-"))
}

// Decoding: -home-user-project → /home/user/project
fn decode_path(encoded: &str) -> Result<PathBuf> {
  if !encoded.starts_with('-') {
    return Err(Error::InvalidEncoding);
  }
  Ok(PathBuf::from(encoded[1..].replace('-', "/")))
}
```

**Edge cases**:
- Spaces preserved: `/home/my dir` → `-home-my dir`
- Multiple slashes normalized: `/home//user` → `-home-user`
- Trailing slash ignored: `/home/user/` → `-home-user`

### session detection

```rust
// Check if project exists for path
fn has_sessions_for_path(path: &Path) -> bool {
  let encoded = encode_path(path);
  let project_dir = storage_root.join("projects").join(encoded);
  project_dir.exists() && project_dir.is_dir()
}
```

### project listing

```rust
// List all projects (both UUID and path types)
fn list_projects() -> Result<Vec<Project>> {
  let projects_dir = storage_root.join("projects");
  let mut projects = Vec::new();

  for entry in fs::read_dir(projects_dir)? {
    let entry = entry?;
    let name = entry.file_name().to_string_lossy().to_string();

    if name.starts_with('-') {
      // Path project
      projects.push(Project::from_path_encoded(name)?);
    } else {
      // UUID project
      projects.push(Project::from_uuid(name)?);
    }
  }

  Ok(projects)
}
```

## comparison with traditional databases

| Aspect | Claude Code Storage | Traditional DB |
|--------|---------------------|----------------|
| Format | JSONL files | Binary/proprietary |
| Schema | Implicit (JSON) | Explicit (DDL) |
| Migrations | Not needed | Required |
| Tools | jq, grep, less | SQL client |
| Concurrency | File locks | MVCC/locks |
| Scale | Conversation-scale | Production-scale |
| Backup | cp/rsync | Dump/restore |
| Query | Sequential scan | Indexed |

**Trade-offs**:
- Simplicity over performance
- Transparency over optimization
- Portability over features

**Appropriate for**: Conversation history (read-mostly, modest scale, human-readable priority).

**Not appropriate for**: High-frequency writes, complex queries, multi-GB datasets.

## future considerations

### potential optimizations

**Indexing**: Create `.index` files for faster search (project-level decision).

**Compression**: Gzip old sessions (transparent with `.jsonl.gz` extension).

**Sharding**: Split large sessions into multiple files (backward-compatible).

**Caching**: Add optional in-memory cache for hot sessions (opt-in).

### migration paths

**Format evolution**: Add new fields to JSON (forward-compatible, old parsers ignore).

**Storage reorganization**: Keep both old and new structures during transition.

**Backup strategy**: Always preserve `.jsonl` originals during migrations.

## related documentation

- `jsonl_format.md` - Complete JSONL entry format specification
- `development_plan.md` - Phase 2 implementation plan
- `cli_design.md` - CLI command specifications
- `spec.md` - Overall crate specification
