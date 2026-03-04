# claude code file formats reference

## overview

This document provides detailed specifications for all file formats used by Claude Code storage system.

**Format categories**:
- Conversation data (JSONL)
- Configuration (JSON)
- Logs (plain text)
- Environment snapshots (bash scripts)
- Command definitions (markdown)

## conversation format (.jsonl)

### file location

`~/.claude/projects/{project-id}/{session-id}.jsonl`

### format specification

**Line-delimited JSON**: One complete JSON object per line.

**Encoding**: UTF-8.

**Line terminator**: LF (`\n`).

**Entry types**: User messages and assistant messages (different schemas).

### user message entry

**Complete example**:
```json
{
  "uuid": "a6f3bd8c-9e5f-4d2a-b1c3-8e7f6d5c4b3a",
  "parentUuid": null,
  "timestamp": "2025-11-08T23:30:10.039Z",
  "type": "user",
  "cwd": "/home/user1/pro/lib/willbe/module/wplan",
  "sessionId": "8d795a1c-c81d-4010-8d29-b4e678272419",
  "version": "0.9.6",
  "gitBranch": "master",
  "userType": "USER",
  "isSidechain": false,
  "message": {
    "role": "user",
    "content": "command to repeat something every hour?"
  },
  "thinkingMetadata": {
    "level": "high",
    "disabled": false,
    "triggers": [
      {
        "start": 58,
        "end": 68,
        "text": "ultrathink"
      }
    ]
  }
}
```

**Field specifications**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uuid` | string | yes | UUID v4 for this entry |
| `parentUuid` | string\|null | yes | UUID of parent entry (null for first message) |
| `timestamp` | string | yes | ISO 8601 timestamp |
| `type` | string | yes | Always `"user"` for user messages |
| `cwd` | string | yes | Current working directory path |
| `sessionId` | string | yes | Session UUID (matches filename) |
| `version` | string | yes | Claude Code version |
| `gitBranch` | string\|null | no | Current git branch (if in repo) |
| `userType` | string | yes | User type (`"USER"` or `"AGENT"`) |
| `isSidechain` | boolean | yes | Whether entry is from sub-agent |
| `message` | object | yes | Message payload |
| `message.role` | string | yes | Always `"user"` |
| `message.content` | string | yes | User's message text |
| `thinkingMetadata` | object | no | Thinking configuration |
| `thinkingMetadata.level` | string | yes | Thinking level (`"low"`, `"medium"`, `"high"`) |
| `thinkingMetadata.disabled` | boolean | yes | Whether thinking is disabled |
| `thinkingMetadata.triggers` | array | yes | Thinking trigger locations |

**Thinking trigger specification**:
```typescript
{
  start: number,    // Character offset where trigger starts
  end: number,      // Character offset where trigger ends
  text: string      // Trigger text (e.g., "ultrathink")
}
```

### assistant message entry

**Complete example**:
```json
{
  "uuid": "56a226b5-7c8d-9e0f-1a2b-3c4d5e6f7a8b",
  "parentUuid": "a6f3bd8c-9e5f-4d2a-b1c3-8e7f6d5c4b3a",
  "timestamp": "2025-11-08T23:30:15.234Z",
  "type": "assistant",
  "cwd": "/home/user1/pro/lib/willbe/module/wplan",
  "sessionId": "8d795a1c-c81d-4010-8d29-b4e678272419",
  "version": "0.9.6",
  "gitBranch": "master",
  "userType": "USER",
  "isSidechain": false,
  "message": {
    "id": "msg_01AbCdEfGhIjKlMnOpQrStUv",
    "model": "claude-sonnet-4-5-20250929",
    "role": "assistant",
    "type": "message",
    "content": [
      {
        "type": "thinking",
        "thinking": "User wants to run a command repeatedly...",
        "signature": "eyJhbGc..."
      },
      {
        "type": "text",
        "text": "Looking at options for repeating commands..."
      },
      {
        "type": "tool_use",
        "id": "toolu_01AbCdEfGhIjKlMn",
        "name": "Bash",
        "input": {
          "command": "man cron",
          "description": "Check cron documentation"
        }
      }
    ],
    "stop_reason": "tool_use",
    "stop_sequence": null,
    "usage": {
      "input_tokens": 9,
      "output_tokens": 6,
      "cache_creation_input_tokens": 0,
      "cache_read_input_tokens": 12112,
      "service_tier": "claude_code"
    }
  },
  "requestId": "req-01AbCdEfGhIjKlMnOpQrStUvWxYz"
}
```

**Field specifications**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uuid` | string | yes | UUID v4 for this entry |
| `parentUuid` | string | yes | UUID of user message this responds to |
| `timestamp` | string | yes | ISO 8601 timestamp |
| `type` | string | yes | Always `"assistant"` |
| `cwd` | string | yes | Current working directory |
| `sessionId` | string | yes | Session UUID |
| `version` | string | yes | Claude Code version |
| `gitBranch` | string\|null | no | Git branch |
| `userType` | string | yes | `"USER"` or `"AGENT"` |
| `isSidechain` | boolean | yes | Sub-agent flag |
| `message` | object | yes | Response payload |
| `message.id` | string | yes | API message ID |
| `message.model` | string | yes | Model name |
| `message.role` | string | yes | Always `"assistant"` |
| `message.type` | string | yes | Always `"message"` |
| `message.content` | array | yes | Content blocks |
| `message.stop_reason` | string\|null | no | Why generation stopped |
| `message.stop_sequence` | string\|null | no | Sequence that stopped generation |
| `message.usage` | object | yes | Token usage statistics |
| `requestId` | string | yes | API request ID |

### content blocks

**Text block**:
```json
{
  "type": "text",
  "text": "Looking at options for repeating commands..."
}
```

**Thinking block**:
```json
{
  "type": "thinking",
  "thinking": "User wants to run a command repeatedly every hour...",
  "signature": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9..."
}
```

**Fields**:
- `thinking` (string) - Assistant's internal reasoning
- `signature` (string) - Cryptographic signature (JWT format)

**Tool use block**:
```json
{
  "type": "tool_use",
  "id": "toolu_01AbCdEfGhIjKlMnOpQrStUv",
  "name": "Bash",
  "input": {
    "command": "man cron",
    "description": "Check cron documentation"
  }
}
```

**Fields**:
- `id` (string) - Tool call ID
- `name` (string) - Tool name
- `input` (object) - Tool-specific parameters

**Tool result block**:
```json
{
  "type": "tool_result",
  "tool_use_id": "toolu_01AbCdEfGhIjKlMnOpQrStUv",
  "content": "CRON(8)\n\nNAME\n       cron - daemon to execute scheduled commands...",
  "is_error": false
}
```

**Fields**:
- `tool_use_id` (string) - Matches tool_use.id
- `content` (string) - Tool output
- `is_error` (boolean) - Whether execution failed

### usage statistics

```json
{
  "input_tokens": 9,
  "output_tokens": 6,
  "cache_creation_input_tokens": 0,
  "cache_read_input_tokens": 12112,
  "service_tier": "claude_code"
}
```

**Fields**:
- `input_tokens` (number) - Tokens in user input
- `output_tokens` (number) - Tokens in assistant response
- `cache_creation_input_tokens` (number) - Tokens used to create cache
- `cache_read_input_tokens` (number) - Tokens read from cache
- `service_tier` (string) - API tier (`"claude_code"`)

### threading model

Entries linked via `parentUuid`:

```
Entry 1 (user):    uuid=A, parentUuid=null
Entry 2 (assistant): uuid=B, parentUuid=A
Entry 3 (user):    uuid=C, parentUuid=B
Entry 4 (assistant): uuid=D, parentUuid=C
```

**Root entry**: First entry has `parentUuid=null`.

**Chain**: Each subsequent entry references previous via `parentUuid`.

**Purpose**: Maintains conversation context and ordering.

## history format (history.jsonl)

### file location

`~/.claude/history.jsonl`

### format specification

**Line-delimited JSON**: One entry per line.

**Purpose**: Track project access history and context.

**Entry structure**:
```json
{
  "display": "https://www.youtube-transcript.io/api\nread page...",
  "pastedContents": {},
  "timestamp": 1758992388766,
  "project": "/home/user1/pro/lib/willbe/module/reasoner"
}
```

**Field specifications**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display` | string | yes | User query or context preview |
| `pastedContents` | object | yes | Pasted file contents (usually empty) |
| `timestamp` | number | yes | Unix timestamp (milliseconds) |
| `project` | string | yes | Filesystem path to project |

**Growth**: Appends one entry per conversation start.

**Total entries**: Varies (4324 entries observed in research).

**Size**: ~1.1MB for 4324 entries (~254 bytes/entry average).

## settings format (settings.json)

### file location

`~/.claude/settings.json`

### format specification

**Single JSON object**: Complete file is one JSON object.

**Purpose**: Store user preferences and configuration.

**Structure**:
```json
{
  "hooks": {
    "Notification": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "INPUT=$(cat) && DIR=$(pwd) && ..."
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "..."
          }
        ]
      }
    ]
  },
  "alwaysThinkingEnabled": true
}
```

**Field specifications**:

| Field | Type | Description |
|-------|------|-------------|
| `hooks` | object | Event hook configurations |
| `hooks.Notification` | array | Notification event hooks |
| `hooks.Stop` | array | Stop event hooks |
| `alwaysThinkingEnabled` | boolean | Default thinking mode |

### hook specification

**Hook object**:
```json
{
  "type": "command",
  "command": "INPUT=$(cat) && DIR=$(pwd) && ..."
}
```

**Hook types**:
- `command` - Execute shell command

**Hook events**:
- `Notification` - On notification events
- `Stop` - On stop events

## credentials format (.credentials.json)

### file location

`~/.claude/.credentials.json`

### format specification

**Single JSON object**: Complete file is one JSON object.

**Purpose**: Store API authentication tokens.

**Structure**:
```json
{
  "claudeAiOauth": {
    ... (authentication data)
  }
}
```

**Security**: Sensitive file, user-readable only.

**Permissions**: `chmod 600 ~/.claude/.credentials.json`

## debug log format (debug/*.txt)

### file location

`~/.claude/debug/*.txt`

### format specification

**Plain text**: Line-oriented log format.

**Line format**: `[DEBUG] message`

**Example entries**:
```
[DEBUG] Watching for changes in setting files...
[DEBUG] Found 0 plugins (0 enabled, 0 disabled)
[DEBUG] Total LSP servers loaded: 0
[DEBUG] Creating shell snapshot at /home/user1/.claude/shell-snapshots/...
```

**Content types**:
- Setting file watching
- Plugin loading status
- LSP server initialization
- Shell snapshot creation
- Process lifecycle events

**Growth**: Continuous append during Claude Code operation.

**Size**: Can grow to 100MB+ over time.

**Maintenance**: Safe to delete or rotate.

## shell snapshot format (shell-snapshots/*.sh)

### file location

`~/.claude/shell-snapshots/{uuid}.sh`

### format specification

**Bash script**: Executable shell script.

**Purpose**: Preserve and restore shell environment.

**Structure**:
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
shopt -s extglob
shopt -s extquote
shopt -s force_fignore
shopt -s globasciiranges
shopt -s histappend
...
```

**Components**:

**1. Alias cleanup**:
```bash
unalias -a 2>/dev/null || true
```

**2. Function restoration**:
```bash
eval "$(echo 'BASE64_ENCODED_FUNCTIONS' | base64 -d)"
```

**3. Shell options**:
```bash
shopt -s option_name
shopt -u option_name
```

**Encoding**: Bash functions base64-encoded to preserve complex syntax.

**Restoration**: Source the script to restore environment.

**File naming**: UUID matches session ID.

**Size**: 5KB - 500KB per snapshot.

## todo format (todos/*.json)

### file location

`~/.claude/todos/{session-uuid}.json`

### format specification

**JSON array**: Array of task objects.

**Purpose**: Track tasks for conversation session.

**Structure**:
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
  },
  {
    "content": "Write test documentation",
    "status": "pending",
    "activeForm": "Writing test documentation"
  }
]
```

**Task object specification**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | string | yes | Task description (imperative form) |
| `status` | string | yes | Task state |
| `activeForm` | string | yes | Task description (present continuous form) |

**Status values**:
- `pending` - Not started
- `in_progress` - Currently working on
- `completed` - Finished

**Example forms**:
- `content`: "Implement feature X" (imperative)
- `activeForm`: "Implementing feature X" (present continuous)

**File organization**: One file per session UUID.

**Growth**: Updates on task status changes.

**Size**: Varies (empty to several MB for long sessions).

## command definition format (commands/*.md)

### file location

`~/.claude/commands/{command-name}.md`

### format specification

**Markdown**: Command definition in markdown format.

**Purpose**: Define custom slash commands.

**Example** (`commit.md`):
```markdown
Act as an expert git assistant. Analyze the repository state, manage the staging area, and execute commits with high-quality, conventional commit messages.

## Instructions

1. Run git status to see changes
2. Run git diff to understand modifications
3. Draft commit message following conventions
4. Execute git commit with message
...
```

**Structure**: Free-form markdown with instructions.

**Usage**: Commands available as `/{command-name}` in Claude Code.

**File count**: 48 command definitions observed.

**Examples**:
- `commit.md` - Git commit assistant
- `refactor_extracting.md` - Refactoring guidance
- `test_clean.md` - Test cleanup workflow

## format comparison

| Format | Size/Entry | Parsing | Mutability | Purpose |
|--------|-----------|---------|------------|---------|
| `.jsonl` | 500B-5KB | Line-by-line | Append-only | Conversations |
| `history.jsonl` | ~250B | Line-by-line | Append-only | Project tracking |
| `.json` | 1KB-5MB | Full parse | Overwrite | Configuration |
| `.txt` (debug) | ~100B | Line-by-line | Append-only | Logging |
| `.sh` | 5KB-500KB | Source | Create-once | Environment |
| `.md` | 1KB-50KB | Text | Static | Commands |

## parsing considerations

### jsonl parsing strategy

**Recommended approach**: Line-by-line streaming.

```rust
use std::io::{BufRead, BufReader};
use std::fs::File;

fn parse_jsonl(path: &Path) -> Result<Vec<Entry>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let mut entries = Vec::new();

  for line in reader.lines() {
    let line = line?;
    if line.trim().is_empty() {
      continue;  // Skip empty lines
    }

    match parse_entry(&line) {
      Ok(entry) => entries.push(entry),
      Err(e) => eprintln!("Warning: Skipping malformed entry: {}", e),
    }
  }

  Ok(entries)
}
```

**Benefits**:
- Memory efficient (one line at a time)
- Graceful error handling (skip bad lines)
- Works with large files

**Error handling**: Skip malformed lines with warnings, continue parsing.

### json parsing strategy

**Settings/credentials**: Parse entire file as single object.

**Todos**: Parse entire file as array.

**History**: Line-by-line (JSONL format).

### encoding considerations

**UTF-8**: All text formats use UTF-8 encoding.

**Line endings**: Unix-style LF (`\n`).

**Escaping**: JSON standard escape sequences in JSONL.

**Unicode**: Full Unicode support in all formats.

## validation rules

### uuid validation

**Format**: UUID v4 (8-4-4-4-12 hex digits).

**Example**: `a6f3bd8c-9e5f-4d2a-b1c3-8e7f6d5c4b3a`

**Validation**:
```rust
fn is_valid_uuid(s: &str) -> bool {
  let parts: Vec<&str> = s.split('-').collect();
  parts.len() == 5
    && parts[0].len() == 8
    && parts[1].len() == 4
    && parts[2].len() == 4
    && parts[3].len() == 4
    && parts[4].len() == 12
    && s.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}
```

### timestamp validation

**Format**: ISO 8601 (`YYYY-MM-DDTHH:MM:SS.sssZ`).

**Example**: `2025-11-08T23:30:10.039Z`

**Timezone**: Always UTC (Z suffix).

**Validation**: Parse with ISO 8601 parser.

### path validation

**Rules**:
- Absolute paths only
- Unix-style separators (`/`)
- No `.` or `..` components
- Valid UTF-8

### content block validation

**Required fields per type**:
- `text`: `type`, `text`
- `thinking`: `type`, `thinking`, `signature`
- `tool_use`: `type`, `id`, `name`, `input`
- `tool_result`: `type`, `tool_use_id`, `content`, `is_error`

## error handling

### malformed json

**Strategy**: Skip line with warning, continue parsing.

**Example**:
```rust
match parse_json_line(line) {
  Ok(entry) => entries.push(entry),
  Err(e) => {
    eprintln!("Line {}: Skipping malformed JSON: {}", line_num, e);
    continue;
  }
}
```

### missing required fields

**Strategy**: Return error for that entry, skip it.

**Example**:
```rust
if !obj.contains_key("uuid") {
  return Err(Error::MissingField("uuid"));
}
```

### unknown fields

**Strategy**: Ignore for forward compatibility.

**Rationale**: New Claude Code versions may add fields.

### type mismatches

**Strategy**: Return error with clear message.

**Example**:
```rust
let timestamp = obj.get_str("timestamp")
  .ok_or(Error::TypeMismatch("timestamp", "string"))?;
```

## format evolution

### backward compatibility

**Adding fields**: Always optional, old parsers ignore.

**Removing fields**: Never remove, deprecate instead.

**Changing types**: Avoid, use new field name instead.

### forward compatibility

**Unknown fields**: Parsers must ignore unknown fields.

**New content blocks**: Skip unknown types gracefully.

**New entry types**: Handle gracefully (skip or generic parse).

## related documentation

- `jsonl_format.md` - Detailed conversation JSONL specification (includes all field definitions with examples)
- `storage_organization.md` - Directory structure and storage model
- `development_plan.md` - Implementation roadmap
- `spec.md` - Overall crate specification
