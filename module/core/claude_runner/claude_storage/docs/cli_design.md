# claude_storage CLI Design

## Overview

**Purpose**: Minimalistic read-only CLI for exploring Claude Code conversation storage.

**Scope**: Phase 2 read-only operations (no write functionality).

**Framework**: `unilang` crate (mandatory per CLI rulebook § "Rust CLI Framework Mandate").

**Design Principles**:
- Minimalistic (5-7 core commands)
- Read-only (exploration and analysis)
- Stateless REPL design
- Follows CLI rulebook § "Command Naming Convention" (dot-prefix, snake_case, noun-verb order)

## unilang integration approach

### build-time yaml registration (recommended)

**unilang v0.35** uses build-time YAML processing for **50x faster command resolution**.

**Performance characteristics**:
- Lookup time: ~80ns (vs ~4,000ns runtime HashMap)
- Memory overhead: Zero (compile-time only)
- Binary size: Smaller (static data section)

**Implementation pattern**:
1. Define commands in `unilang.commands.yaml`
2. Build system auto-generates `static_commands.rs`
3. Include generated code: `include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));`
4. Create static registry: `StaticCommandRegistry::from_commands(&STATIC_COMMANDS)`
5. Use stateless pipeline for REPL/CLI

**No build.rs needed**: Multi-YAML approach (default) auto-discovers all `.yaml` files.

**Cargo.toml**:
```toml
[dependencies]
unilang = "0.35"  # Default includes approach_yaml_multi_build
```

### command definition formats

**Two equivalent YAML formats**:

**Format 1 (Recommended)** - Full command name:
```yaml
- name: ".storage.list_projects"      # Complete command with dots
  namespace: ""                       # Empty namespace
```

**Format 2 (Alternative)** - Namespace + name:
```yaml
- name: "list_projects"               # Command without prefix
  namespace: ".storage"               # Namespace includes dot
```

**Note**: All commands MUST include dot prefix (either in name or namespace).

### auto-help configuration

**Setting**: `auto_help_enabled: false` (recommended for this CLI)

**Rationale**: Prevents automatic generation of `.command.help` subcommands, reducing command list clutter. Help still available via `?` and `??` operators.

**Example**:
```yaml
- name: ".storage.list_projects"
  auto_help_enabled: false            # Disable auto .storage.list_projects.help
  description: "..."
  # ? and ?? operators still work for inline help
```

### command routine signature

All commands require implementation function:
```rust
fn command_routine(
  cmd: VerifiedCommand,
  _ctx: ExecutionContext
) -> Result<OutputData, ErrorData>
{
  // Extract arguments (type-safe)
  let param = cmd.require_string("param")?;
  let verbosity = cmd.get_integer("verbosity").unwrap_or(1);

  // Business logic
  let result = do_work(param, verbosity)?;

  // Return output
  Ok(OutputData {
    content: result,
    format: "text".to_string(),
    execution_time_ms: None,
  })
}
```

## Interface Type

**Classification**: Development utility (§ "Standard Interface Types")

**Required Commands** (per CLI rulebook):
- `.help` - Universal help
- `.status` - Storage status
- Other domain-specific commands

## Command Design

### Universal Commands (Required)

#### `.help`
**Scope**: Universal help (§ "Mandatory Help Commands")

**Description**: Display general help and list available commands

**Parameters**: None

**Output**:
```
claude_storage - Claude Code Storage Explorer
==============================================

Available commands:
  .storage.list_projects       List all conversation projects
  .storage.list_sessions       List sessions in a project
  .storage.read_session        Read entries from a session
  .storage.search              Search conversation history
  .storage.stats               Get storage statistics

For command-specific help: .command.help
Example: .storage.list_projects.help
```

**Implementation**:
```rust
CommandDefinition
{
  name : "help".to_string(),
  namespace : String::new(),
  description : "Display general help and list available commands".to_string(),
  hint : "Type . or .help for help".to_string(),
  arguments : vec![],
  aliases : vec![".".to_string()],
  ..Default::default()
}
```

---

#### `.status`
**Scope**: Development interface requirement (§ "Standard Interface Types")

**Description**: Display storage status and configuration

**Parameters**:
- `verbosity` (optional, integer 0-5) - Output detail level (§ "Standard Verbosity Parameter")

**Output** (verbosity::0):
```
Storage: ~/.claude/
Projects: 225
Sessions: -
Entries: -
```

**Output** (verbosity::2):
```
Storage: ~/.claude/
Projects: 225
  - UUID projects: 210
  - Path projects: 15
Sessions: 342
  - Main sessions: 320
  - Agent sessions: 22
Entries: ~45,230 (estimated)
Cache: ~/.claude/.cache/ (12.3 MB)
```

**Implementation**:
```rust
CommandDefinition
{
  name : "status".to_string(),
  namespace : String::new(),
  description : "Display storage status and configuration".to_string(),
  hint : "Usage: .status [verbosity::N]".to_string(),
  arguments : vec!
  [
    ArgumentDefinition
    {
      name : "verbosity".to_string(),
      kind : Kind::Integer,
      description : "Output detail level (0=quiet, 2=debug)".to_string(),
      hint : "verbosity::0".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("1".to_string()),
        ..Default::default()
      },
      validation_rules : vec!
      [
        ValidationRule::Range { min : 0, max : 5 },
      ],
      ..Default::default()
    },
  ],
  aliases : vec![],
  ..Default::default()
}
```

---

### Domain-Specific Commands

#### `.storage.list_projects`
**Scope**: Core read operation

**Description**: List all conversation projects in storage

**Parameters**:
- `filter` (optional, string) - Filter by path or UUID pattern
- `verbosity` (optional, integer) - Output detail level

**Output** (verbosity::0):
```
Projects (225):
  UUID: 26dd749d-5b4b-bfee-f4f3-9e03803b8cad (3 sessions)
  PATH: /home/user1/pro (2 sessions)
  PATH: /home/user1/pro/entry-proto-cld (1 session)
```

**Output** (verbosity::2):
```
Projects (225):
  [UUID] 26dd749d-5b4b-bfee-f4f3-9e03803b8cad
    Location: ~/.claude/projects/26dd749d-5b4b-bfee-f4f3-9e03803b8cad/
    Sessions: 3
    Entries: ~450 (estimated)

  [PATH] /home/user1/pro
    Location: ~/.claude/projects/-home-user1-pro/
    Sessions: 2 main + 12 agent
    Entries: ~5,230 (estimated)
    Last modified: 2025-11-25 15:00:00
```

**Implementation**:
```rust
CommandDefinition
{
  name : "list_projects".to_string(),
  namespace : "storage".to_string(),
  description : "List all conversation projects in storage".to_string(),
  hint : "Usage: .storage.list_projects [filter::PATTERN] [verbosity::N]".to_string(),
  arguments : vec!
  [
    ArgumentDefinition
    {
      name : "filter".to_string(),
      kind : Kind::String,
      description : "Filter projects by path or UUID pattern".to_string(),
      hint : "filter::/home/user".to_string(),
      attributes : ArgumentAttributes { optional : true, ..Default::default() },
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "verbosity".to_string(),
      kind : Kind::Integer,
      description : "Output detail level".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("1".to_string()),
        ..Default::default()
      },
      validation_rules : vec![ValidationRule::Range { min : 0, max : 5 }],
      ..Default::default()
    },
  ],
  aliases : vec![".storage.projects".to_string()],
  ..Default::default()
}
```

---

#### `.storage.list_sessions`
**Scope**: Core read operation

**Description**: List sessions in a specific project

**Parameters**:
- `project` (required, string) - Project path or UUID
- `verbosity` (optional, integer) - Output detail level

**Output** (verbosity::0):
```
Sessions in /home/user1/pro (2):
  8d795a1c-c81d-4010-8d29-b4e678272419 (127 entries)
  5c6f81bf-0a80-433c-9211-1a781421f745 (98 entries)
```

**Output** (verbosity::2):
```
Sessions in /home/user1/pro (2 main + 12 agent):

Main Sessions:
  [8d795a1c-c81d-4010-8d29-b4e678272419]
    File: ~/.claude/projects/-home-user1-pro/8d795a1c-c81d-4010-8d29-b4e678272419.jsonl
    Entries: 127
    Size: 434 KB
    Created: 2025-11-08 23:30:10
    Modified: 2025-11-25 15:00:00

Agent Sessions (showing 3 of 12):
  agent-167331a7.jsonl (4 entries, 1.3 KB)
  agent-2f6f1b11.jsonl (5 entries, 1.8 KB)
  ...
```

**Implementation**:
```rust
CommandDefinition
{
  name : "list_sessions".to_string(),
  namespace : "storage".to_string(),
  description : "List sessions in a specific project".to_string(),
  hint : "Usage: .storage.list_sessions project::PATH [verbosity::N]".to_string(),
  arguments : vec!
  [
    ArgumentDefinition
    {
      name : "project".to_string(),
      kind : Kind::String,
      description : "Project path or UUID to list sessions from".to_string(),
      hint : "project::/home/user/pro".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![ValidationRule::Required],
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "verbosity".to_string(),
      kind : Kind::Integer,
      description : "Output detail level".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("1".to_string()),
        ..Default::default()
      },
      validation_rules : vec![ValidationRule::Range { min : 0, max : 5 }],
      ..Default::default()
    },
  ],
  aliases : vec![".storage.sessions".to_string()],
  ..Default::default()
}
```

---

#### `.storage.read_session`
**Scope**: Core read operation

**Description**: Read and display entries from a session

**Parameters**:
- `session` (required, string) - Session UUID or file path
- `project` (optional, string) - Project path (if session is UUID only)
- `limit` (optional, integer) - Max entries to display (default: 10)
- `offset` (optional, integer) - Skip first N entries (default: 0)
- `type` (optional, string) - Filter by entry type ("user" or "assistant")
- `verbosity` (optional, integer) - Output detail level

**Output** (verbosity::0):
```
Session: 8d795a1c-c81d-4010-8d29-b4e678272419
Entries (showing 10 of 127):

[1] USER (2025-11-08 23:30:10)
  command to repeat something every hour?

[2] ASSISTANT (2025-11-08 23:30:21)
  Looking at options for running something hourly...
  [Thinking: 512 tokens]
  [Text: 243 tokens]

[3] USER (2025-11-08 23:32:15)
  thanks, i'll use cron

Use limit:: and offset:: to paginate.
```

**Output** (verbosity::2):
```
Session: 8d795a1c-c81d-4010-8d29-b4e678272419
Location: ~/.claude/projects/-home-user1-pro/8d795a1c-c81d-4010-8d29-b4e678272419.jsonl
Entries: 127 total (showing 10)

─────────────────────────────────────────────────────────
[1] USER
  UUID: a6f3bd8c-5575-4eab-82b0-b856f7a02833
  Timestamp: 2025-11-08T23:30:10.039Z
  CWD: /home/user1/pro
  Git Branch: master
  Thinking: ultrathink (high level)

  Message:
  command to repeat something every hour?

─────────────────────────────────────────────────────────
[2] ASSISTANT
  UUID: 56a226b5-0ec6-4214-af16-b13cc326f8dc
  Parent: a6f3bd8c-5575-4eab-82b0-b856f7a02833
  Timestamp: 2025-11-08T23:30:21.913Z
  Model: claude-sonnet-4-5-20250929

  Content Blocks:
    [thinking] 512 tokens (signed)
    [text] 243 tokens

  Usage: 9 input + 6 output (12112 cached)

  Text:
  Looking at options for running something hourly...
  [truncated - use verbosity::3 for full content]

─────────────────────────────────────────────────────────
```

**Implementation**:
```rust
CommandDefinition
{
  name : "read_session".to_string(),
  namespace : "storage".to_string(),
  description : "Read and display entries from a session".to_string(),
  hint : "Usage: .storage.read_session session::UUID [project::PATH] [limit::N]".to_string(),
  arguments : vec!
  [
    ArgumentDefinition
    {
      name : "session".to_string(),
      kind : Kind::String,
      description : "Session UUID or file path".to_string(),
      hint : "session::8d795a1c-c81d-4010-8d29-b4e678272419".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![ValidationRule::Required],
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "project".to_string(),
      kind : Kind::String,
      description : "Project path (if session is UUID only)".to_string(),
      hint : "project::/home/user/pro".to_string(),
      attributes : ArgumentAttributes { optional : true, ..Default::default() },
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "limit".to_string(),
      kind : Kind::Integer,
      description : "Max entries to display".to_string(),
      hint : "limit::20".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("10".to_string()),
        ..Default::default()
      },
      validation_rules : vec![ValidationRule::Range { min : 1, max : 1000 }],
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "offset".to_string(),
      kind : Kind::Integer,
      description : "Skip first N entries".to_string(),
      hint : "offset::10".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("0".to_string()),
        ..Default::default()
      },
      validation_rules : vec![ValidationRule::Range { min : 0, max : 100000 }],
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "type".to_string(),
      kind : Kind::String,
      description : "Filter by entry type (user/assistant)".to_string(),
      hint : "type::user".to_string(),
      attributes : ArgumentAttributes { optional : true, ..Default::default() },
      validation_rules : vec![ValidationRule::OneOf(vec!["user".to_string(), "assistant".to_string()])],
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "verbosity".to_string(),
      kind : Kind::Integer,
      description : "Output detail level".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("1".to_string()),
        ..Default::default()
      },
      validation_rules : vec![ValidationRule::Range { min : 0, max : 5 }],
      ..Default::default()
    },
  ],
  aliases : vec![".storage.read".to_string()],
  ..Default::default()
}
```

---

#### `.storage.search`
**Scope**: Enhanced read operation

**Description**: Search conversation history for text

**Parameters**:
- `query` (required, string) - Search query text
- `project` (optional, string) - Limit search to specific project
- `limit` (optional, integer) - Max results to display (default: 10)
- `verbosity` (optional, integer) - Output detail level

**Output** (verbosity::0):
```
Search results for "version_bump" (3 matches):

[Project: /home/user1/pro] [Session: 8d795a1c...]
  [USER] "help me integrate version_bump module..."
  [ASSISTANT] "I'll help you integrate version_bump..."

[Project: /home/user1/pro] [Session: 5c6f81bf...]
  [ASSISTANT] "The version_bump crate provides..."

Use verbosity::2 for full context.
```

**Output** (verbosity::2):
```
Search results for "version_bump" (3 matches in 2 sessions):

─────────────────────────────────────────────────────────
Match 1/3
  Project: /home/user1/pro
  Session: 8d795a1c-c81d-4010-8d29-b4e678272419
  Entry: a6f3bd8c-5575-4eab-82b0-b856f7a02833
  Type: USER
  Timestamp: 2025-11-08 23:30:10

  Context:
  help me integrate version_bump module into the
  workspace build process

─────────────────────────────────────────────────────────
Match 2/3
  Project: /home/user1/pro
  Session: 8d795a1c-c81d-4010-8d29-b4e678272419
  Entry: 56a226b5-0ec6-4214-af16-b13cc326f8dc
  Type: ASSISTANT
  Timestamp: 2025-11-08 23:30:21

  Context:
  I'll help you integrate version_bump. First, let me
  check the current workspace structure...
  [Shows 2 lines before and after match]

─────────────────────────────────────────────────────────
```

**Implementation**:
```rust
CommandDefinition
{
  name : "search".to_string(),
  namespace : "storage".to_string(),
  description : "Search conversation history for text".to_string(),
  hint : "Usage: .storage.search query::TEXT [project::PATH] [limit::N]".to_string(),
  arguments : vec!
  [
    ArgumentDefinition
    {
      name : "query".to_string(),
      kind : Kind::String,
      description : "Search query text".to_string(),
      hint : "query::version_bump".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![ValidationRule::Required, ValidationRule::MinLength(1)],
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "project".to_string(),
      kind : Kind::String,
      description : "Limit search to specific project".to_string(),
      hint : "project::/home/user/pro".to_string(),
      attributes : ArgumentAttributes { optional : true, ..Default::default() },
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "limit".to_string(),
      kind : Kind::Integer,
      description : "Max results to display".to_string(),
      hint : "limit::20".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("10".to_string()),
        ..Default::default()
      },
      validation_rules : vec![ValidationRule::Range { min : 1, max : 100 }],
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "verbosity".to_string(),
      kind : Kind::Integer,
      description : "Output detail level".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("1".to_string()),
        ..Default::default()
      },
      validation_rules : vec![ValidationRule::Range { min : 0, max : 5 }],
      ..Default::default()
    },
  ],
  aliases : vec![".storage.find".to_string()],
  ..Default::default()
}
```

---

#### `.storage.stats`
**Scope**: Enhanced read operation

**Description**: Display storage statistics and analytics

**Parameters**:
- `project` (optional, string) - Get stats for specific project
- `verbosity` (optional, integer) - Output detail level

**Output** (verbosity::0):
```
Storage Statistics:
  Projects: 225
  Sessions: 342
  Total entries: -
  Total tokens: -
```

**Output** (verbosity::2):
```
Storage Statistics:
==================

Projects: 225
  - UUID projects: 210
  - Path projects: 15

Sessions: 342 main + 78 agent = 420 total
  - Average per project: 1.8 sessions
  - Largest project: /home/user1/pro (14 sessions)

Entries: ~45,230 (estimated)
  - User messages: ~22,615 (50%)
  - Assistant messages: ~22,615 (50%)
  - Average per session: 132 entries

Token Usage (estimated):
  - Total input tokens: ~2,450,000
  - Total output tokens: ~1,230,000
  - Cache hits: 85% avg
  - Total cost: ~$45.20 (estimated)

Storage Size:
  - JSONL files: 156.7 MB
  - Debug logs: 45.2 MB
  - Total: 201.9 MB

Most Active Projects:
  1. /home/user1/pro (5,230 entries, 14 sessions)
  2. /home/user1/pro/entry-proto-cld (2,100 entries, 8 sessions)
  3. UUID:26dd749d... (1,450 entries, 3 sessions)
```

**Implementation**:
```rust
CommandDefinition
{
  name : "stats".to_string(),
  namespace : "storage".to_string(),
  description : "Display storage statistics and analytics".to_string(),
  hint : "Usage: .storage.stats [project::PATH] [verbosity::N]".to_string(),
  arguments : vec!
  [
    ArgumentDefinition
    {
      name : "project".to_string(),
      kind : Kind::String,
      description : "Get stats for specific project only".to_string(),
      hint : "project::/home/user/pro".to_string(),
      attributes : ArgumentAttributes { optional : true, ..Default::default() },
      ..Default::default()
    },
    ArgumentDefinition
    {
      name : "verbosity".to_string(),
      kind : Kind::Integer,
      description : "Output detail level".to_string(),
      attributes : ArgumentAttributes
      {
        optional : true,
        default_value : Some("1".to_string()),
        ..Default::default()
      },
      validation_rules : vec![ValidationRule::Range { min : 0, max : 5 }],
      ..Default::default()
    },
  ],
  aliases : vec![".storage.statistics".to_string()],
  ..Default::default()
}
```

---

## Command Summary

**Total commands**: 7 (2 universal + 5 domain-specific)

**Universal**:
- `.help` / `.` - General help
- `.status` - Storage status

**Domain-specific**:
- `.storage.list_projects` - List projects
- `.storage.list_sessions` - List sessions
- `.storage.read_session` - Read entries
- `.storage.search` - Search history
- `.storage.stats` - Statistics

All commands have corresponding `.command.help` variants (not listed per § "Help Command Exclusion").

## CLI Rulebook Compliance

### ✅ Framework Requirements

- **§ Rust CLI Framework Mandate**: Uses `unilang` crate exclusively
- **§ Pipeline API Requirement**: Uses high-level Pipeline API for command processing

### ✅ Naming Standards

- **§ Command Naming Convention**: All commands use:
  - Dot prefix (`.`)
  - `snake_case`
  - Noun-verb order (`.storage.list_projects` not `.list.storage.projects`)
  - Dots for multi-word separation

### ✅ Help System

- **§ Mandatory Help Commands**: All commands have `.command.help` variants
- **§ Help Command Filtering**: Help commands not listed in output
- **§ Universal Help Access**: Both `.` and `.help` provide general help

### ✅ Parameter Standards

- **§ Parameter Format Convention**: All parameters use `name::value` format
- **§ Standard Verbosity Parameter**: All commands support `verbosity::N` (0-5)
- **§ Parameter Validation**: All parameters have validation rules

### ✅ Structure Standards

- **§ CommandDefinition Structure**: All commands use explicit field names
- **§ ArgumentDefinition Structure**: All arguments use explicit field names

### ✅ REPL Standards

- **§ Stateless REPL Design**: Each command execution is independent
- **§ Verbosity Control**: Consistent verbosity levels (0-5)

### ✅ Error Handling

- **§ Standard Exit Codes**: Uses standard exit codes (0=success, 1=error, etc.)
- **§ Error Message Format**: Clear, actionable error messages

### ✅ Security

- **§ Input Validation**: All parameters validated
- **§ Secure Parameter Handling**: No sensitive data exposure

## Implementation Phases

### Phase 2A: Core CLI (Week 1)

**Tasks**:
1. Add `unilang` dependency to `Cargo.toml`
2. Create `src/cli.rs` module
3. Implement `.help` and `.status` commands
4. Implement `.storage.list_projects`
5. Implement `.storage.list_sessions`
6. Add basic REPL loop

**Deliverables**:
- Working CLI binary
- 5 commands functional
- Basic help system
- REPL mode

### Phase 2B: Read Operations (Week 2)

**Tasks**:
1. Implement `.storage.read_session` (requires JSON parser from Phase 2)
2. Implement `.storage.search` (requires entry parsing)
3. Implement `.storage.stats`
4. Add verbosity levels (0-5)
5. Comprehensive error handling

**Deliverables**:
- All 7 commands functional
- Full verbosity support
- Search functionality
- Statistics analytics

### Phase 2C: Polish & Testing (Week 3)

**Tasks**:
1. Add CLI tests (command parsing, execution)
2. Add integration tests with real storage
3. Performance optimization
4. Documentation
5. Examples

**Deliverables**:
- Comprehensive test coverage
- Performance benchmarks
- User documentation
- Example usage

## File Structure

```
claude_storage/
├── Cargo.toml                # Add unilang dependency
├── src/
│   ├── lib.rs               # Library API
│   ├── cli.rs               # CLI module (NEW)
│   ├── commands/            # Command implementations (NEW)
│   │   ├── mod.rs
│   │   ├── help.rs
│   │   ├── status.rs
│   │   ├── list_projects.rs
│   │   ├── list_sessions.rs
│   │   ├── read_session.rs
│   │   ├── search.rs
│   │   └── stats.rs
│   └── main.rs              # CLI entry point (NEW)
├── tests/
│   └── cli_tests.rs         # CLI tests (NEW)
└── examples/
    └── cli_usage.rs         # CLI examples (NEW)
```

## Cargo.toml Update

```toml
[dependencies]
# Zero core dependencies for library

[dependencies.unilang]
version = "0.5" # Use latest version
optional = false  # Required for CLI

[[bin]]
name = "claude-storage"
path = "src/main.rs"
required-features = []  # CLI is part of core functionality
```

## Usage Examples

### REPL Mode

```bash
$ claude-storage
claude_storage> .help
claude_storage - Claude Code Storage Explorer
==============================================
[help output...]

claude_storage> .storage.list_projects
Projects (225):
  UUID: 26dd749d-5b4b-bfee-f4f3-9e03803b8cad (3 sessions)
  PATH: /home/user1/pro (2 sessions)
  ...

claude_storage> .storage.read_session session::8d795a1c... limit::5
Session: 8d795a1c-c81d-4010-8d29-b4e678272419
[entries...]

claude_storage> exit
```

### One-Shot Mode

```bash
# List projects
$ claude-storage .storage.list_projects

# Read session with parameters
$ claude-storage .storage.read_session session::8d795a1c-c81d-4010-8d29-b4e678272419 limit::5 verbosity::2

# Search history
$ claude-storage .storage.search query::version_bump

# Get statistics
$ claude-storage .storage.stats verbosity::2
```

## Testing Strategy

### Unit Tests

Test command parsing and validation:
```rust
#[test]
fn test_list_projects_parsing() {
  let cmd = parse_command(".storage.list_projects filter::/home/user verbosity::2");
  assert_eq!(cmd.name, "list_projects");
  assert_eq!(cmd.namespace, "storage");
  assert_eq!(cmd.get_param("filter"), Some("/home/user"));
  assert_eq!(cmd.get_param("verbosity"), Some("2"));
}
```

### Integration Tests

Test with real storage:
```rust
#[test]
fn test_list_projects_real_storage() {
  let output = execute_cli(".storage.list_projects");
  assert!(output.contains("Projects"));
  assert!(output.exit_code == 0);
}
```

### Verbosity Tests

Test verbosity compliance (§ "Verbosity Testing Requirements"):
```rust
#[test]
fn test_verbosity_computation_consistency() {
  let result_v0 = execute_cli(".storage.stats verbosity::0");
  let result_v2 = execute_cli(".storage.stats verbosity::2");

  // Same data, different display
  let data_v0 = parse_stats(result_v0.stdout);
  let data_v2 = parse_stats(result_v2.stdout);

  assert_eq!(data_v0.project_count, data_v2.project_count);
  assert_eq!(result_v0.exit_code, result_v2.exit_code);
}
```

## Design Rationale

### Minimalistic Scope

**Why only 7 commands?**
- Covers all core read operations
- Easy to learn and remember
- Focused functionality
- Room for future expansion

### Read-Only Operations

**Why no write operations?**
- Phase 2 scope is read-only
- Write operations postponed to Phase 4
- Reduces complexity and risk
- Easier to test and validate

### Stateless Design

**Why stateless REPL?**
- Per § "Stateless REPL Design"
- No persistent state between commands
- More reliable and predictable
- Easier to test

### Verbosity Levels

**Why 0-5 verbosity scale?**
- Per § "Standard Verbosity Parameter"
- Consistent with other CLI tools
- Flexible detail control
- 0=quiet, 1=normal, 2+=debug

## complete yaml definition (reference)

Complete `unilang.commands.yaml` for all 7 commands:

```yaml
# claude_storage CLI Commands (unilang v0.35)
# Build-time static registry for zero-overhead lookups

# Universal help command
- name: ".help"
  namespace: ""
  description: "Display general help and list available commands"
  hint: "Type . or .help for help"
  status: "stable"
  version: "1.0.0"
  auto_help_enabled: false
  aliases: ["."]
  arguments: []
  examples:
    - ".help"
    - "."

# Storage status command
- name: ".status"
  namespace: ""
  description: "Display Claude Code storage status and configuration"
  hint: "Show storage overview"
  status: "stable"
  version: "1.0.0"
  auto_help_enabled: false
  arguments:
    - name: "verbosity"
      kind: "Integer"
      description: "Output detail level (0=quiet, 1=normal, 2+=debug)"
      hint: "Detail level 0-5"
      attributes:
        optional: true
        default: 1
      validation_rules:
        - type: "Range"
          min: 0
          max: 5
  examples:
    - ".status"
    - ".status verbosity::0"
    - ".status verbosity::2"

# List projects command
- name: ".storage.list_projects"
  namespace: ""
  description: "List all conversation projects in Claude Code storage"
  hint: "List projects with optional filtering"
  status: "stable"
  version: "1.0.0"
  auto_help_enabled: false
  aliases: [".storage.projects"]
  arguments:
    - name: "filter"
      kind: "String"
      description: "Filter projects by path or UUID pattern"
      hint: "Path or UUID pattern"
      attributes:
        optional: true
        default: ""
    - name: "verbosity"
      kind: "Integer"
      description: "Output detail level (0-5)"
      hint: "Detail level"
      attributes:
        optional: true
        default: 1
      validation_rules:
        - type: "Range"
          min: 0
          max: 5
  examples:
    - ".storage.list_projects"
    - ".storage.list_projects filter::/home/user"
    - ".storage.list_projects verbosity::2"

# List sessions command
- name: ".storage.list_sessions"
  namespace: ""
  description: "List all sessions in a project"
  hint: "List sessions for project"
  status: "stable"
  version: "1.0.0"
  auto_help_enabled: false
  aliases: [".storage.sessions"]
  arguments:
    - name: "project"
      kind: "String"
      description: "Project UUID or path (defaults to current directory)"
      hint: "Project identifier"
      attributes:
        optional: true
        default: ""
    - name: "include_agents"
      kind: "Boolean"
      description: "Include agent (sub-agent) sessions"
      hint: "Include agents"
      attributes:
        optional: true
        default: false
    - name: "verbosity"
      kind: "Integer"
      description: "Output detail level (0-5)"
      hint: "Detail level"
      attributes:
        optional: true
        default: 1
      validation_rules:
        - type: "Range"
          min: 0
          max: 5
  examples:
    - ".storage.list_sessions"
    - ".storage.list_sessions project::/home/user/pro"
    - ".storage.list_sessions include_agents::true"

# Read session command
- name: ".storage.read_session"
  namespace: ""
  description: "Read and display entries from a session"
  hint: "Read session entries"
  status: "stable"
  version: "1.0.0"
  auto_help_enabled: false
  arguments:
    - name: "session"
      kind: "String"
      description: "Session UUID or agent ID (required)"
      hint: "Session identifier"
      attributes:
        optional: false
    - name: "limit"
      kind: "Integer"
      description: "Maximum number of entries to display"
      hint: "Entry limit"
      attributes:
        optional: true
        default: 10
      validation_rules:
        - type: "Min"
          value: 1
        - type: "Max"
          value: 1000
    - name: "entry_type"
      kind: "Enum"
      description: "Filter by entry type"
      hint: "user|assistant|all"
      attributes:
        optional: true
        default: "all"
      validation_rules:
        - type: "Enum"
          values: ["user", "assistant", "all"]
    - name: "verbosity"
      kind: "Integer"
      description: "Output detail level (0-5)"
      hint: "Detail level"
      attributes:
        optional: true
        default: 1
      validation_rules:
        - type: "Range"
          min: 0
          max: 5
  examples:
    - ".storage.read_session session::9425242b-1185-4788-993e-09852db0516d"
    - ".storage.read_session session::agent-64bdad98 limit::5"
    - ".storage.read_session session::abc123 entry_type::user"

# Search command
- name: ".storage.search"
  namespace: ""
  description: "Search conversation history across all projects"
  hint: "Search conversations"
  status: "stable"
  version: "1.0.0"
  auto_help_enabled: false
  arguments:
    - name: "query"
      kind: "String"
      description: "Search query text (required)"
      hint: "Search text"
      attributes:
        optional: false
    - name: "project"
      kind: "String"
      description: "Limit search to specific project"
      hint: "Project filter"
      attributes:
        optional: true
        default: ""
    - name: "entry_type"
      kind: "Enum"
      description: "Filter by entry type"
      hint: "user|assistant|all"
      attributes:
        optional: true
        default: "all"
      validation_rules:
        - type: "Enum"
          values: ["user", "assistant", "all"]
    - name: "limit"
      kind: "Integer"
      description: "Maximum results to return"
      hint: "Result limit"
      attributes:
        optional: true
        default: 20
      validation_rules:
        - type: "Min"
          value: 1
        - type: "Max"
          value: 100
    - name: "verbosity"
      kind: "Integer"
      description: "Output detail level (0-5)"
      hint: "Detail level"
      attributes:
        optional: true
        default: 1
      validation_rules:
        - type: "Range"
          min: 0
          max: 5
  examples:
    - ".storage.search query::version_bump"
    - ".storage.search query::\"error handling\" limit::10"
    - ".storage.search query::rust project::/home/user/pro"

# Statistics command
- name: ".storage.stats"
  namespace: ""
  description: "Display storage statistics and metrics"
  hint: "Show storage stats"
  status: "stable"
  version: "1.0.0"
  auto_help_enabled: false
  aliases: [".storage.statistics"]
  arguments:
    - name: "scope"
      kind: "Enum"
      description: "Statistics scope"
      hint: "global|project"
      attributes:
        optional: true
        default: "global"
      validation_rules:
        - type: "Enum"
          values: ["global", "project"]
    - name: "project"
      kind: "String"
      description: "Project for scope::project (defaults to cwd)"
      hint: "Project path/UUID"
      attributes:
        optional: true
        default: ""
    - name: "verbosity"
      kind: "Integer"
      description: "Output detail level (0-5)"
      hint: "Detail level"
      attributes:
        optional: true
        default: 1
      validation_rules:
        - type: "Range"
          min: 0
          max: 5
  examples:
    - ".storage.stats"
    - ".storage.stats scope::project"
    - ".storage.stats scope::global verbosity::2"
```

**Key YAML features**:
- `auto_help_enabled: false` - Prevents command list clutter
- Unquoted defaults for primitives (`default: 1` not `default: "1"`)
- Validation rules for ranges and enums
- Aliases for common command shortcuts
- Rich examples for each command

## Future Enhancements

**Phase 3: Integration**
- Export commands (`.storage.export_markdown`, `.storage.export_json`)
- Integration with external tools

**Phase 4: Write Operations**
- `.storage.append_entry` (write mode)
- `.storage.create_session` (write mode)

**Phase 5: Advanced Features**
- `.storage.analyze` (sentiment, topics)
- `.storage.visualize` (conversation graphs)
- `.storage.compare` (diff sessions)

## References

- **CLI Rulebook**: `/home/user1/pro/genai/cli/cli.rulebook.md`
- **unilang Framework v0.35**: `/home/user1/pro/lib/wTools/module/core/unilang/`
  - readme.md - Overview and quick start
  - usage.md - Complete usage guide
  - examples/ - 40+ examples including static_01_basic_compile_time.rs
- **unilang Integration Findings**: `-unilang_integration_findings.md` (detailed research notes)
- **claude_storage Spec**: `../spec.md`
- **Development Plan**: `-development_plan_v2.md` (extended with CLI implementation)
