# claude_storage_core specification

## responsibility

Pure library for Claude Code's filesystem-based storage access.

**Core purpose**: Provide zero-dependency, safe, structured read/write access to Claude Code's conversation storage at `~/.claude/`.

**Extraction context**: This is the core library extracted from the monolithic `claude_storage` crate (2025-11-29). The CLI functionality remains in the `claude_storage` crate, which depends on this core library.

**In scope**:
- Read/write access to JSONL conversation files
- Project and session management
- Path encoding/decoding utilities
- Format validation and safety guarantees
- Statistics aggregation and analysis
- Hand-written JSON parser (zero dependencies)

**Out of scope**:
- CLI interface (see `claude_storage` crate)
- Process control (see `claude_session`)
- Workflow automation (see `wplan_agent`)
- High-level AI integration logic

## design principles

### backward compatibility is a non-goal

claude_storage_core explicitly rejects backward compatibility as a design constraint. Breaking changes are acceptable and expected:

- **No API stability guarantees**: Public types, function signatures can change between versions
- **No storage format versioning**: JSONL structure reading logic can change freely
- **No deprecated API preservation**: Old function names deleted, not aliased
- **No path encoding stability**: Path encoding/decoding logic can change

**Rationale**:
- **Simpler evolution**: API can evolve to match Claude Code's storage changes
- **Better safety**: Can improve error handling and validation freely
- **Cleaner code**: No compatibility shims for old API patterns
- **Zero dependencies**: No version negotiation logic needed

**Upgrade Protocol**:
1. Update library version
2. Dependent crates (claude_storage, dream_agent) must update simultaneously
3. No automated migration for API changes

**Implications for Developers**:
- Don't preserve old function names "for backward compatibility"
- Don't maintain old storage format readers alongside new ones
- Don't add version detection logic for storage format
- Public API can change freely to match Claude Code evolution
- Storage types can evolve without maintaining old schemas

## architecture

### crate split rationale

**Why separate core from CLI**:
1. **Zero dependencies**: Library users don't need CLI dependencies (unilang, phf)
2. **Faster compilation**: Library-only users avoid building CLI frameworks
3. **Clear separation**: Library logic vs CLI presentation
4. **Reusability**: Other crates can depend on core without CLI baggage
5. **Standard Rust pattern**: Similar to ripgrep (grep-regex + grep-cli)

**Relationship to claude_storage**:
```
┌─────────────────────────────┐
│  claude_storage (CLI)       │
│  - Command parsing          │
│  - REPL interface           │
│  - Command routines         │
│  - Depends on: core + CLI   │
└──────────┬──────────────────┘
           │ depends on
           ▼
┌─────────────────────────────┐
│  claude_storage_core (lib)  │
│  - Storage types            │
│  - JSON parser              │
│  - Statistics               │
│  - Dependencies: ZERO       │
└─────────────────────────────┘
```

### storage model

Claude Code uses a filesystem-native storage architecture:

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

### core types

**Storage**: Entry point for all operations
- Manages `~/.claude/` root directory
- Lists/loads projects
- Provides global statistics

**Project**: Directory containing sessions
- Two types: UUID-based (web/IDE) or path-based (CLI)
- Contains multiple session JSONL files
- Path encoding: `/home/user/pro` → `-home-user-pro`

**Session**: Single conversation (JSONL file)
- Lazy-loaded entries
- Append-only operations
- Statistics without full parsing (performance optimization)

**Entry**: Individual message in conversation
- User or assistant message
- Metadata (timestamp, cwd, git branch, thinking data)
- JSON serialization
- Content blocks (text, thinking, tool_use, tool_result)
- **Non-conversation entries**: JSONL files may contain metadata entries (queue-operation, summary, file-history-snapshot) which are silently skipped during parsing (graceful degradation)

**Statistics types**: Aggregated metrics
- **SessionStats**: Per-session metrics (entries, tokens, timestamps)
- **ProjectStats**: Per-project aggregation (session counts, total entries)
- **GlobalStats**: Workspace-wide statistics with per-project breakdown

### path encoding

Filesystem paths are encoded into storage directory names using a hybrid scheme that maintains backward compatibility with Claude Code while improving accuracy.

**Encoding rules (v2, 2025-11-30)**:

**New encoding (unambiguous)**:
1. Prefix entire path with `-`
2. Replace `/` with `-` for path separators
3. Replace `/-` (hyphen-prefixed component) with `--` (double hyphen)
4. Keep `_` as-is within components (no longer lossy!)

**Old encoding (Claude Code legacy, lossy)**:
1. Prefix with `-`
2. Replace all `/` with `-`
3. Replace all `_` with `-` (lossy!)

**Encoding examples (v2)**:
- `/home/user/project` → `-home-user-project` (unchanged from v1)
- `/commands/-default_topic` → `-commands--default_topic` (now unambiguous: `--` = `/-`)
- `/commands/-commit/-sessions` → `-commands--commit--sessions` (clear boundaries)
- `/foo_bar/baz` → `-foo_bar-baz` (underscore preserved!)

**Key improvement**: Double-hyphen `--` **always** means "component starting with hyphen", eliminating ambiguity.

**Decoding algorithm (backward compatible)**:

Uses v1 heuristic decoder for all paths to maintain compatibility:

1. **v1 heuristics** (universally applied):
   - `--` (double hyphen) → `/-` (component starts with hyphen)
   - `-` after normal component → `/` (path separator)
   - `-` within hyphen-prefixed component → `_` (underscore)

**Note**: Even though our encoder (v2) preserves underscores in component names, the decoder uses v1 heuristics for all paths. This creates a minor asymmetry:
- Encoding `/foo/-bar_baz` → `-foo--bar_baz` (underscore preserved)
- Decoding `-foo--bar_baz` → `/foo/-bar_baz` (hyphens become underscores in hyphen-prefixed components)

This design choice prioritizes backward compatibility with Claude Code's lossy encoding over perfect round-tripping of new paths. Future enhancement could use filesystem validation to distinguish v1 vs v2 encoded paths.

**Decoding examples**:
- `-home-user-project` → `/home/user/project` (v1/v2 identical)
- `-commands--default_topic` → `/commands/-default_topic` (v2 unambiguous)
- `-commands--default-topic` → `/commands/-default_topic` (v1 heuristic, works)
- `-foo-bar-baz--qux` → `/foo/bar/baz/-qux` (v2 clear)

**Backward compatibility guarantee**:
- All v1-encoded paths decode correctly (heuristic fallback)
- New v2 paths are unambiguous
- Mixed storage (v1 + v2 paths) works transparently
- Gradual migration: no flag day required

**Current implementation status (2025-11-30)**:
- Encoder: v2 (preserves underscores, uses `--` for hyphen-prefixed components) ✅
- Decoder: v1 heuristic (backward compatible, 72.5% success rate)
- Improvement: Encoder ready for future v2 decoder when filesystem validation added
- Current limitation: No improvement to decode success rate (still 72.5% for existing storage)

**Migration strategy**:
- New path projects: Use v2 encoding automatically
- Existing paths: Decode with fallback (no re-encoding needed)
- Optional: Migration tool to re-encode v1 paths to v2 (preserves old paths)

**Implementation note (2025-11-30)**: v2 encoding addresses the component boundary ambiguity identified in real-world testing where `-a--b-c` was ambiguous between `/-a_b/c` and `/-a_b_c`. Double-hyphen now has single unambiguous meaning.

### filtering

**Design philosophy**:
- Zero dependencies (no regex crate, use stdlib only)
- Case-insensitive substring matching by default (better UX)
- Composable filters with AND logic
- Optimization: skip filtering when default filter (no conditions)

**Filter types**:

```rust
/// Session-level filtering
pub struct SessionFilter
{
  /// Filter by agent session type
  /// - None: No filtering (show all)
  /// - Some(true): Only agent sessions
  /// - Some(false): Only main sessions
  pub agent_only : Option< bool >,

  /// Minimum entry count (inclusive)
  pub min_entries : Option< usize >,

  /// Session ID substring match (case-insensitive)
  pub session_id_substring : Option< String >,
}

/// Project-level filtering
pub struct ProjectFilter
{
  /// Path substring match (case-insensitive)
  pub path_substring : Option< String >,

  /// Minimum total entries across all sessions
  pub min_entries : Option< usize >,

  /// Minimum session count
  pub min_sessions : Option< usize >,
}

/// Zero-dependency substring matcher
pub struct StringMatcher
{
  pattern : String, // Lowercased for case-insensitive matching
}

impl StringMatcher
{
  /// Create new matcher (converts pattern to lowercase)
  pub fn new( pattern : impl Into< String > ) -> Self;

  /// Check if text matches (case-insensitive)
  /// Empty pattern matches all text
  pub fn matches( &self, text : &str ) -> bool;
}
```

**Filter composition**:

Filters use AND logic - ALL conditions must match:
- `agent_only: Some(true)` AND `min_entries: Some(10)` → Only agent sessions with 10+ entries
- `path_substring: Some("willbe")` AND `min_sessions: Some(5)` → Only projects with "willbe" in path AND 5+ sessions

**API usage examples**:

```rust
use claude_storage_core::{ Storage, SessionFilter, ProjectFilter };

// Filter sessions: agent sessions with 10+ entries
let filter = SessionFilter
{
  agent_only : Some( true ),
  min_entries : Some( 10 ),
  session_id_substring : None,
};
let sessions = project.sessions_filtered( &filter )?;

// Filter projects: path contains "willbe", min 5 sessions
let filter = ProjectFilter
{
  path_substring : Some( "willbe".to_string() ),
  min_entries : None,
  min_sessions : Some( 5 ),
};
let projects = storage.list_projects_filtered( &filter )?;

// Substring matching (case-insensitive)
use claude_storage_core::StringMatcher;
let matcher = StringMatcher::new( "WillBe" );
assert!( matcher.matches( "claude_storage/willbe/src" ) ); // ✓ case-insensitive
assert!( !matcher.matches( "claude_storage/src" ) );       // ✗ no match
```

**Performance characteristics**:

- **Short-circuit evaluation**: Filters applied in order of performance cost
  1. Agent filter (cheapest - just checks boolean)
  2. Entry count (medium - may use cached stats)
  3. Substring matching (still cheap - single pass with `to_lowercase()`)

- **Zero allocations for default filter**: When all filter fields are `None`, filtering is skipped entirely

- **Case-insensitive overhead**: `to_lowercase()` allocates, but only for strings being tested (not entire dataset)

**Known limitations**:

- **No regex support**: Only substring matching (zero-dependency constraint)
- **Unicode normalization**: `to_lowercase()` handles basic Unicode, but not full normalization (e.g., `"ß".to_lowercase() != "ss"`)
- **No OR logic**: Filters use AND composition only (multiple conditions must ALL match)
- **Session ID filtering requires loading sessions**: Cannot filter by session ID without reading session metadata

**Phase 1 implementation** (2025-11-30):
- `agent_only` filter (agent vs main sessions)
- `min_entries` filter (both session and project level)
- `path_substring` filter (case-insensitive)
- `session_id_substring` filter (case-insensitive)

**Future enhancements** (Phase 2+):
- Timestamp filtering (created_after, created_before)
- Token usage filtering (min_tokens, max_tokens)
- Multiple substring patterns (OR logic within substring)
- Negation filters (exclude patterns)

### content search

**Design philosophy**:
- Zero dependencies (no regex, stdlib only)
- Streaming search (memory-efficient, doesn't load full session)
- Case-insensitive by default (better UX)
- Contextual results (show surrounding text)
- Fast enough for interactive use (<5s for 1000 sessions)

**Use cases**:
- Find sessions discussing specific topics ("error_tools crate")
- Locate bug discussions ("double slash bug")
- Search agent conversations only
- Find code snippets across sessions

**Search types**:

```rust
/// Search filter for session content
pub struct SearchFilter
{
  /// Search query (case-insensitive substring match)
  pub query : String,

  /// Case-sensitive matching (default: false)
  pub case_sensitive : bool,

  /// Filter by entry role (user/assistant/system)
  pub match_role : Option< Role >,

  /// Filter by content type (text/code/thinking)
  pub match_content_type : Option< ContentType >,
}

/// Search match result with context
pub struct SearchMatch
{
  /// Zero-based entry index in session
  pub entry_index : usize,

  /// Entry role (user/assistant)
  pub role : Role,

  /// Line number within entry content
  pub line_number : usize,

  /// Matched line with surrounding context
  /// Format: "...{50 chars before}MATCH{50 chars after}..."
  pub excerpt : String,

  /// Full matched line (without truncation)
  pub full_line : String,
}
```

**Implementation strategy**:

**Streaming approach** (Phase 2B - implemented):
- Scan JSONL line-by-line without loading full session
- Yield matches as iterator
- Memory usage: O(1) - constant regardless of session size
- Performance: ~100-500ms per session for typical queries
- Total: <5 seconds for 1000 sessions

**Key methods**:

```rust
impl Session
{
  /// Search session content (streaming, memory-efficient)
  pub fn search( &self, filter : &SearchFilter ) -> Result< Vec< SearchMatch > >
  {
    // Stream through JSONL, yield matching entries
    // Skip parsing full entry structure when possible (performance)
  }
}

impl Storage
{
  /// Search all sessions in all projects
  pub fn search_all( &self, filter : &SearchFilter ) -> Result< Vec< ( Project, Session, Vec< SearchMatch > ) > >
  {
    // Iterate projects, sessions, collect matches
  }
}
```

**Performance characteristics**:
- **Per-session search**: 100-500ms (depends on session size)
- **Workspace search**: <5 seconds for 1000 sessions
- **Memory**: O(matches) - only matching entries loaded
- **Optimization**: Early exit on max_matches limit

**CLI interface** (Phase 2B):

```bash
# Basic search
claude_storage .search "error_tools"

# Scoped search
claude_storage .search "error_tools" path::willbe

# Role filtering
claude_storage .search "bug fix" agent::1       # Agent sessions only
claude_storage .search "help me" agent::0       # User conversations only

# Content type filtering
claude_storage .search "fn main" type::code     # Code blocks only
claude_storage .search "thinking" type::thinking # Thinking blocks only

# Case-sensitive search
claude_storage .search "ErrorTools" case::1
```

**Output format**:

```
Found 3 matches in 2 sessions:

Project: /home/user/pro/lib/willbe/module/claude_storage
Session: agent-abc123 (42 entries)
  Match 1: Entry #15 (assistant, line 23)
    "...implementing the error_tools crate for better error handling..."

  Match 2: Entry #28 (user, line 5)
    "Can you refactor to use error_tools instead of anyhow?"

Project: /home/user/pro/lib/willbe/module/wplan
Session: -default_topic (156 entries)
  Match 3: Entry #89 (assistant, line 12)
    "...migrated from thiserror to error_tools following the rulebook..."
```

**Future enhancements** (Phase 3+):
- Regex search (requires regex crate, feature-gated)
- Fuzzy matching (typo tolerance)
- Indexed search (inverted index for <10ms searches)
- Search within specific date ranges

### export functionality

**Design philosophy**:
- Zero dependencies (no external formatters, stdlib only)
- Multiple output formats (markdown, JSON, plain text)
- Session-level and project-level export
- Streaming export (memory-efficient for large sessions)
- Preserve conversation structure and metadata
- Graceful handling: Non-conversation metadata entries (queue-operation, summary) are automatically skipped

**Supported formats**:

1. **Markdown** (`.md`)
   - Human-readable conversation format
   - Preserves message structure (user/assistant)
   - Includes metadata (timestamps, tokens, thinking blocks)
   - Suitable for documentation and archival

2. **JSON** (`.json`)
   - Machine-readable structured format
   - Preserves complete session data
   - Identical to Claude Code's internal JSONL format (but pretty-printed)
   - Suitable for programmatic processing

3. **Plain text** (`.txt`)
   - Simple conversation transcript
   - Minimal formatting (just role: content)
   - No metadata included
   - Suitable for quick review

**API**:

```rust
/// Export format specification
#[derive( Debug, Clone, Copy )]
pub enum ExportFormat
{
  /// Markdown format (.md)
  Markdown,

  /// JSON format (.json)
  Json,

  /// Plain text format (.txt)
  Text,
}

impl ExportFormat
{
  /// Get file extension for format
  pub fn extension( &self ) -> &'static str;

  /// Parse format from string
  pub fn from_str( s : &str ) -> Result< Self >;
}

/// Export a session to a writer
pub fn export_session< W : Write >
(
  session : &mut Session,
  format : ExportFormat,
  writer : &mut W,
) -> Result< () >;

/// Export a session to a file
pub fn export_session_to_file
(
  session : &mut Session,
  format : ExportFormat,
  output_path : &Path,
) -> Result< () >;
```

**Markdown format example**:

```markdown
# Session: -default_topic

**Path**: `/home/user/pro/lib/willbe/module/claude_storage`
**Entries**: 42
**Created**: 2024-11-29 10:15:23 UTC
**Last Updated**: 2024-11-29 14:32:11 UTC

---

## Entry 1 - User
*2024-11-29 10:15:23 UTC*

Help me implement a search feature for the claude_storage crate.

---

## Entry 2 - Assistant
*2024-11-29 10:15:45 UTC*

<details>
<summary>Thinking (342 tokens)</summary>

The user wants to add search functionality. I should consider...
</details>

I'll help you implement search functionality for the claude_storage crate. Let's start by...

**Tokens**: 1,234 in, 567 out

---
```

**JSON format example**:

```json
{
  "session_id": "-default_topic",
  "project_path": "/home/user/pro/lib/willbe/module/claude_storage",
  "entries": [
    {
      "uuid": "msg_abc123",
      "timestamp": "2024-11-29T10:15:23Z",
      "role": "user",
      "content": "Help me implement a search feature..."
    },
    {
      "uuid": "msg_def456",
      "timestamp": "2024-11-29T10:15:45Z",
      "role": "assistant",
      "content": [
        {
          "type": "thinking",
          "thinking": "The user wants to add search functionality..."
        },
        {
          "type": "text",
          "text": "I'll help you implement search functionality..."
        }
      ],
      "usage": {
        "input_tokens": 1234,
        "output_tokens": 567
      }
    }
  ]
}
```

**Text format example**:

```
Session: -default_topic
Path: /home/user/pro/lib/willbe/module/claude_storage
Entries: 42

---

[User] 2024-11-29 10:15:23
Help me implement a search feature for the claude_storage crate.

[Assistant] 2024-11-29 10:15:45
I'll help you implement search functionality for the claude_storage crate. Let's start by...

---
```

**Performance characteristics**:
- Markdown export: ~500 KB/s (dominated by formatting overhead)
- JSON export: ~2 MB/s (direct serialization)
- Text export: ~1 MB/s (minimal formatting)
- Memory usage: O(1) streaming for all formats

**CLI integration**:

```bash
# Export session to markdown
claude_storage .export session::-default_topic format::markdown output::conversation.md

# Export entire project to JSON
claude_storage .export format::json output::project_export.json

# Export with specific verbosity
claude_storage .export session::-default_topic format::text verbosity::0 > transcript.txt
```

**Future enhancements** (Phase 3+):
- HTML export with syntax highlighting
- PDF export (requires external dependency)
- Export with filtering (date ranges, entry types)
- Incremental export (append new entries only)

### safety guarantees

**Append-only**: Write operations only append to JSONL files (no modification/deletion)

**Atomic writes**: Uses temp file + rename pattern to prevent corruption:
1. Write to temporary file
2. Sync to disk

**Thread safety**: All core types implement `Send` but not `Sync`
- Each thread should create its own `Storage` instance
- File system access is inherently racy across processes
- No internal locking or synchronization
- Concurrent reads from multiple threads are safe if each thread has its own instance
- Concurrent writes from multiple processes/threads may interleave in JSONL files
3. Rename over target (atomic on POSIX)

**Format validation**: All reads validate JSONL structure and JSON syntax

**Error handling**: All I/O operations return `Result` with context

**Path safety**: Automatic encoding/decoding prevents path traversal

### json parser

**Hand-written parser** (zero dependencies):
- Recursive descent parser (~690 lines)
- All JSON types supported (null, bool, number, string, array, object)
- String escaping (\n, \t, \r, \\, \", \uXXXX)
- Performance: ~80ns/operation
- **UTF-8 fix (bug-1)**: Byte-oriented indexing (see json.rs:417-421)

**Why hand-written**:
- Zero dependencies guarantee
- Full control over error messages
- Optimized for Claude Code JSONL format
- Minimal attack surface

## public api

### core types

```rust
// Entry point
pub struct Storage { /* ... */ }
pub struct Project { /* ... */ }
pub struct Session { /* ... */ }
pub struct Entry { /* ... */ }

// Type variants
pub enum ProjectId { Uuid(String), Path(PathBuf) }
pub enum EntryType { User, Assistant }
pub enum MessageContent { User(UserMessage), Assistant(AssistantMessage) }

// Statistics
pub struct SessionStats { /* ... */ }
pub struct ProjectStats { /* ... */ }
pub struct GlobalStats { /* ... */ }

// Filtering
pub struct SessionFilter { pub agent_only: Option<bool>, pub min_entries: Option<usize>, pub session_id_substring: Option<String> }
pub struct ProjectFilter { pub path_substring: Option<String>, pub min_entries: Option<usize>, pub min_sessions: Option<usize> }
pub struct StringMatcher { /* ... */ }

// JSON parser
pub enum JsonValue { Null, Bool(bool), Number(f64), String(String), Array(Vec<JsonValue>), Object(HashMap<String, JsonValue>) }
pub fn parse_json(input: &str) -> Result<JsonValue>

// Path utilities
pub fn encode_path(path: &Path) -> String
pub fn decode_path(encoded: &str) -> Result<PathBuf>

// Error handling
pub enum Error { Io { /* ... */ }, Parse { /* ... */ }, /* ... */ }
pub type Result<T> = std::result::Result<T, Error>
```

### api stability guarantees

**Stable** (will not change without major version bump):
- All public struct/enum types
- All public functions and methods
- Error type variants
- JSON value representation

**May change** (minor version):
- Internal implementation details
- Performance characteristics
- New optional methods

**Breaking changes**:
- Require major version bump (1.0 → 2.0)
- Documented in changelog
- Migration guide provided

## testing strategy

**Unit tests**: 45 tests covering:
- JSON parser (24 tests): all JSON types, escaping, Unicode, edge cases
- Entry parsing: user messages, assistant messages, all content blocks
- Path encoding/decoding: roundtrip, edge cases
- Session management: loading, counting, statistics
- Project management: UUID/path types, session listing
- Storage operations: project discovery, global statistics

**Bug reproducer tests**: 3 tests (bug-1: UTF-8 multi-byte character handling):
- Unicode em-dash in JSON strings
- Full Claude message with multi-byte characters
- Various UTF-8 characters (emoji, accents, smart quotes)

**Doc tests**: 3 runnable examples in documentation:
- Path encoding (src/path.rs)
- Path decoding (src/path.rs)
- Library usage example (src/lib.rs)

**Integration tests**: (Future) End-to-end tests with real Claude Code storage structure.

**Property tests**: (Future) Roundtrip encoding/decoding, format validation.

**Test execution**: All tests pass with `w3 .test l::3` (nextest + doctests + clippy).

**Test count**: 51 tests total (45 unit + 3 bug + 3 doc)

## performance characteristics

**Lazy loading**: Sessions load entries on-demand, not at construction time.

**Fast counting**: 100x speedup for counting entries without parsing:
- `count_entries()`: ~5ms for 1000 entries (just counts lines)
- `entries().len()`: ~500ms for 1000 entries (parses all JSON)

**JSON parsing**: ~80ns per JSON parse operation (hand-written parser)

**Statistics aggregation**: Selective parsing - only extracts needed fields:
- Skips parsing full entry content
- Extracts only type, timestamp, token usage
- Enables fast statistics without loading all data

**Memory efficiency**:
- Entries loaded on-demand
- Caching at session level
- Streaming not yet implemented (Phase 5)

**Known limitations**:
- Large sessions (>10MB) load fully into memory
- No streaming API yet (planned Phase 5)

## dependencies

**Runtime dependencies**: ZERO

**Build dependencies**: NONE

**Dev dependencies**:
- `tempfile` (workspace) - For testing only

**Zero dependency guarantee**: This core library will NEVER add runtime dependencies. All functionality is implemented using std only.

## known limitations

**Read-only** (current): Write operations (`Entry::to_json_line()`, `Session::append_entry()`) exist in API but are not fully implemented. This is Phase 4 work.

**No streaming**: Large sessions load all entries into memory. Streaming support is planned for Phase 5.

**Performance**: For sessions with >10MB of data, full loading may be slow. Use `count_entries()` or `stats()` for fast operations.

**Error handling**: Corrupted JSONL files cause warnings but don't stop processing (graceful degradation). Non-conversation metadata entries (queue-operation, summary, file-history-snapshot) are silently skipped when loading conversation entries.

## future work

**Phase 4: Write operations** (postponed)
- Complete `Entry::to_json_line()` serialization
- Implement `Session::append_entry()` with atomic writes
- Add write operation tests
- Corruption prevention with temp file pattern

**Phase 5: Advanced features** (future)
- Streaming API for large sessions
- Full-text search across conversations
- Query DSL for complex filtering
- Migration tools for format updates
- Analytics and visualization support

## related crates

**claude_storage**: CLI tool using this library for command-line storage exploration.

**claude_session**: Process control layer for spawning Claude Code CLI. May use `claude_storage_core` for session detection (future integration).

**wplan_agent**: AI integration layer. May use `claude_storage_core` for conversation history analysis (future integration).

**wplan_daemon**: Job execution daemon. No direct dependency on `claude_storage_core` (uses `wplan_agent` instead).

## migration from claude_storage

**For library users** (not CLI users):

Old way:
```toml
[dependencies]
claude_storage = { path = "../claude_storage" }
```

```rust
use claude_storage::{ Storage, ProjectId };
```

New way:
```toml
[dependencies]
claude_storage_core = { path = "../claude_storage_core" }
```

```rust
use claude_storage_core::{ Storage, ProjectId };
```

**For CLI users**: No changes needed. Continue using `claude_storage` binary.

**Breaking changes**: None. The API is identical, only the crate name changed.

## filesystem-native design

**Philosophy**: Work directly with Claude Code's storage without abstractions.

**Benefits**:
- No database overhead
- Human-readable format (JSONL)
- Easy debugging (cat/grep work)
- Atomic operations via filesystem semantics
- No schema migrations needed

**Tradeoffs**:
- No transactions across files
- No complex queries (future: Query DSL)
- Large datasets require careful memory management
