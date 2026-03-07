# claude_storage Documentation

## Overview

This directory contains comprehensive documentation for the `claude_storage` crate.

## Documents

### Core Specifications

#### [Storage Organization](storage_organization.md)

**Complete storage architecture** for Claude Code's `~/.claude/` directory.

**Contents**:
- Directory structure (projects, debug, todos, shell-snapshots, session-env)
- Size characteristics and growth patterns
- Path encoding algorithm
- File naming conventions
- Access patterns and maintenance
- Security considerations
- Design principles (filesystem-native, append-only, single source of truth)

**Use this when**:
- Understanding Claude Code storage layout
- Implementing project/session discovery
- Planning storage access strategies
- Debugging file organization issues

---

#### [File Formats Reference](file_formats.md)

**Detailed specifications** for all Claude Code file formats.

**Contents**:
- Conversation format (.jsonl) - Complete entry schemas
- History format (history.jsonl) - Project tracking
- Settings format (settings.json) - User configuration
- Credentials format (.credentials.json) - API tokens
- Debug logs (debug/*.txt) - Log format
- Shell snapshots (shell-snapshots/*.sh) - Environment restoration
- Todo files (todos/*.json) - Task tracking
- Command definitions (commands/*.md) - Slash commands
- Parsing strategies and validation rules
- Error handling approaches

**Use this when**:
- Implementing parsers for any Claude Code format
- Understanding field types and constraints
- Handling edge cases and malformed data
- Writing format validation code

---

#### [JSONL Format Specification](jsonl_format.md)

**Complete format specification** for Claude Code's JSONL conversation files.

**Contents**:
- Entry types (user, assistant)
- Field definitions with types
- Content block types (text, thinking, tool_use, tool_result)
- Conversation threading via parentUuid
- Usage statistics and token tracking
- Parsing considerations and edge cases
- Testing scenarios

**Use this when**:
- Implementing JSON parser
- Understanding field meanings
- Writing tests with realistic data
- Debugging parsing issues

---

### Implementation Guides

#### [Development Plan](development_plan.md)

**Comprehensive development plan** for implementing read-only JSONL parsing (Phase 2).

**Contents**:
- Task breakdown (8 tasks, 3-5 days)
- Detailed specifications for each task
- JSON parser design
- Entry struct updates
- Integration test strategy
- Performance considerations
- Error handling strategy
- Timeline and risk analysis

**Use this when**:
- Starting Phase 2 implementation
- Planning work schedule
- Understanding technical approach
- Tracking progress

---

#### [CLI Design](cli_design.md)

**Complete CLI specification** for claude_storage REPL and one-shot modes.

**Contents**:
- 7 command specifications (.help, .status, .storage.*)
- CommandDefinition structures (unilang framework)
- ArgumentDefinition with validation rules
- Verbosity levels (0-5)
- Usage examples (REPL and one-shot modes)
- 100% CLI rulebook compliance

**Use this when**:
- Implementing CLI commands
- Understanding parameter formats (name::value)
- Designing user interactions
- Ensuring rulebook compliance

---

### Advanced Topics

#### [Advanced Topics](advanced_topics.md)

**Deep dive into advanced Claude Code storage features**.

**Contents**:
- **Agent sessions** - Sub-agent conversations (`agent-*.jsonl` format)
  - agentId field and isSidechain flag
  - Parent session tracking
  - Detection and discovery algorithms
- **Command system** - Slash command definitions (46 commands)
  - YAML frontmatter + markdown structure
  - Command categories (audit, test, dev, etc.)
  - Role/Objective/Scope/Procedures pattern
- **History tracking** - Global project index details
  - Display field patterns (truncated messages, pasted content indicators)
  - pastedContents field usage
  - Timestamp precision (milliseconds)
- **Session environment** - session-env/ directory purpose
  - 549 empty directories (session markers)
  - Future use considerations
- **Advanced search** - Cross-project search, agent tracking, history-based discovery

**Use this when**:
- Implementing agent session support
- Understanding command system integration
- Building search features
- Exploring session metadata

---

## Quick Reference

### Key Files to Read First

1. **`storage_organization.md`** - Understand Claude Code storage architecture
2. **`file_formats.md`** - Understand all file format specifications
3. **`jsonl_format.md`** - Understand conversation data format
4. **`advanced_topics.md`** - Understand agent sessions, commands, history, search
5. **`development_plan.md`** - Understand implementation approach
6. **`cli_design.md`** - Understand CLI command specifications
7. **`../spec.md`** - Understand overall crate architecture
8. **`../readme.md`** - User-facing documentation

### Implementation Order

**Phase 2: Read-Only Parsing**

1. ✅ JSON parser foundation (`src/json.rs`)
2. ✅ Entry type parsing (`src/entry.rs`)
3. ✅ User message parsing
4. ✅ Assistant message parsing
5. ✅ Integration tests (`tests/`)
6. ✅ Error handling
7. ✅ Enhanced read API
8. ✅ Documentation & examples

See [development_plan.md](development_plan.md) for detailed specifications.

---

## Current Status

**Phase 1: Foundation** ✅ Complete
- Core types implemented
- Path encoding working
- 18 unit tests passing
- Zero dependencies maintained

**Phase 2: Read-Only Parsing** ⏳ Next
- Format researched and documented
- Development plan complete
- Ready to begin implementation

**Phase 3: Integration** 🔜 Future
- Optional integration with `claude_session`
- Used by `wplan_agent`
- Example tools

---

## Testing Data

### Real Storage Location

```bash
~/.claude/projects/
```

### Sample Project (Current Directory)

```bash
~/.claude/projects/-home-user1-pro/
```

### Examining Real Data

```bash
# List projects
ls ~/.claude/projects/

# List sessions in current project
ls ~/.claude/projects/-home-user1-pro/

# View first entry
head -1 ~/.claude/projects/-home-user1-pro/SESSION_ID.jsonl | jq .

# View user message
head -1 FILE.jsonl | jq '.message'

# View assistant message
head -2 FILE.jsonl | tail -1 | jq '.message.content[]'
```

---

## Format Examples

### Minimal User Message

```json
{
  "uuid": "a6f3bd8c-5575-4eab-82b0-b856f7a02833",
  "parentUuid": null,
  "timestamp": "2025-11-08T23:30:10.039Z",
  "type": "user",
  "cwd": "/home/user",
  "sessionId": "8d795a1c-c81d-4010-8d29-b4e678272419",
  "version": "2.0.31",
  "gitBranch": null,
  "userType": "external",
  "isSidechain": false,
  "message": {
    "role": "user",
    "content": "Hello"
  },
  "thinkingMetadata": {
    "level": "low",
    "disabled": true,
    "triggers": []
  }
}
```

### Minimal Assistant Message

```json
{
  "uuid": "56a226b5-0ec6-4214-af16-b13cc326f8dc",
  "parentUuid": "a6f3bd8c-5575-4eab-82b0-b856f7a02833",
  "timestamp": "2025-11-08T23:30:21.913Z",
  "type": "assistant",
  "cwd": "/home/user",
  "sessionId": "8d795a1c-c81d-4010-8d29-b4e678272419",
  "version": "2.0.31",
  "gitBranch": null,
  "userType": "external",
  "isSidechain": false,
  "message": {
    "model": "claude-sonnet-4-5-20250929",
    "id": "msg_01ABC",
    "type": "message",
    "role": "assistant",
    "content": [
      {
        "type": "text",
        "text": "Hi there!"
      }
    ],
    "stop_reason": "end_turn",
    "stop_sequence": null,
    "usage": {
      "input_tokens": 5,
      "output_tokens": 3,
      "cache_creation_input_tokens": 0,
      "cache_read_input_tokens": 0,
      "service_tier": "standard"
    }
  },
  "requestId": "req_01ABC"
}
```

---

## Implementation Notes

### Zero Dependencies Strategy

**Rationale**: Hand-written JSON parser instead of `serde_json`

**Pros**:
- Fast compilation
- Minimal attack surface
- No version conflicts
- Full control over error messages

**Cons**:
- More implementation work (+8-10 hours)

**Decision**: Worth the tradeoff for long-term maintainability

### Error Handling Approach

**Graceful degradation**:
- Parse as many entries as possible
- Skip malformed entries with warnings
- Return both successes and errors
- Never panic on user data

**Error messages include**:
- What failed
- Why it failed
- Where it failed (line number)
- Context (snippet of problematic content)

### Performance Targets

- Parse 1000+ entries/second
- <100ms for typical session (100 entries)
- <1 second for large session (10,000 entries)
- Memory usage scales linearly
- Streaming available for huge sessions

---

## API Examples

### List Projects

```rust
use claude_storage::Storage;

let storage = Storage::new()?;
for project in storage.list_projects()? {
  println!("Project: {:?}", project.id());
}
```

### Read Session

```rust
use claude_storage::Storage;

let storage = Storage::new()?;
let project = storage.load_project_for_cwd()?;

for mut session in project.sessions()? {
  println!("Session: {}", session.id());

  for entry in session.entries()? {
    println!("  [{}] {}", entry.entry_type, entry.uuid);
  }
}
```

### Search History (Future)

```rust
use claude_storage::Storage;

let storage = Storage::new()?;
let results = storage.search("version_bump")?;

for result in results {
  println!("Found in {}: {}", result.session_id, result.snippet);
}
```

---

## Contributing

### Before Starting Implementation

1. ✅ Read `jsonl_format.md` - Understand data format
2. ✅ Read `development_plan.md` - Understand approach
3. ✅ Examine real data - Run jq commands above
4. ✅ Review existing code - Understand current structure

### During Implementation

1. Follow task order from development plan
2. Write tests first (TDD)
3. Test against real Claude Code data
4. Keep zero dependencies
5. Document as you go

### Testing Checklist

- ✅ Unit tests for each module
- ✅ Integration tests with real storage
- ✅ Doc tests for all public APIs
- ✅ Error handling tests
- ✅ Performance benchmarks
- ✅ No panics on any Claude Code data

---

## Resources

- **JSONL Spec**: https://jsonlines.org/
- **Claude API**: https://docs.anthropic.com/en/api/
- **ISO 8601**: https://www.iso.org/iso-8601-date-and-time-format.html
- **UUID v4**: RFC 4122

---

## Questions?

See the main [readme.md](../readme.md) or [spec.md](../spec.md) for more information.
