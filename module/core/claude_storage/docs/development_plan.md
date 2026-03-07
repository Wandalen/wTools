# claude_storage Development Plan

## Executive Summary

**Goal**: Implement read-only access to Claude Code's JSONL storage with zero dependencies.

**Scope**: Phase 2 (JSONL Parsing) - Read operations only, write operations postponed.

**Timeline**: 3-5 days of focused development

**Success Criteria**:
- ✅ Parse all fields from real Claude Code JSONL files
- ✅ Handle both user and assistant message types
- ✅ Zero dependencies (hand-written JSON parser)
- ✅ Comprehensive test coverage (unit + integration)
- ✅ Works with real `~/.claude/` storage
- ✅ Graceful error handling for malformed data

## Current State

### Phase 1: Foundation ✅ Complete

**Completed work**:
- Core types (Storage, Project, Session, Entry)
- Path encoding/decoding utilities
- Error handling framework
- 18 unit tests + 3 doc tests
- Zero dependencies
- Clippy clean
- Workspace integration

**Stubbed implementations**:
- `Entry::from_json_line()` - Returns error
- `Entry::to_json_line()` - Returns error (out of scope for now)
- `Session::entries()` - Would fail on real data

## Phase 2: Read-Only JSONL Parsing

### Overview

Implement complete read-only functionality:
1. Hand-written JSON parser (zero dependencies)
2. Entry parsing for both user and assistant messages
3. Content block parsing (text, thinking, tool_use, tool_result)
4. Integration tests with real Claude Code storage
5. Enhanced read API (filtering, queries)

### Task Breakdown

#### Task 1: JSON Parser Foundation (Day 1)

**Goal**: Build minimal JSON parser for JSONL format

**Subtasks**:
1. Create `src/json.rs` module with hand-written parser
2. Implement primitive parsing (string, number, boolean, null)
3. Implement object parsing
4. Implement array parsing
5. Add comprehensive tests

**Scope**:
- Parse valid JSON only (no need for full JSON spec compliance)
- Focus on Claude Code's actual format
- Optimize for JSONL (one object per line)

**Success criteria**:
```rust
let json = r#"{"name":"value","count":42}"#;
let obj = parse_json_object(json)?;
assert_eq!(obj.get_str("name")?, "value");
assert_eq!(obj.get_i64("count")?, 42);
```

**Files**:
- `src/json.rs` - JSON parsing
- `src/json_tests.rs` - Parser tests

**Time**: 4-6 hours

---

#### Task 2: Entry Type Parsing (Day 1-2)

**Goal**: Parse common fields shared by all entries

**Subtasks**:
1. Update `Entry` struct with all fields from spec
2. Implement `Entry::from_json_line()` for common fields
3. Parse UUID, timestamp, type, cwd, sessionId, version, gitBranch
4. Handle optional fields (parentUuid, gitBranch)
5. Add unit tests

**Success criteria**:
```rust
let line = r#"{"uuid":"a6f3bd8c...","type":"user",...}"#;
let entry = Entry::from_json_line(line)?;
assert_eq!(entry.uuid, "a6f3bd8c...");
assert_eq!(entry.entry_type, EntryType::User);
```

**Files**:
- `src/entry.rs` - Entry parsing
- `src/entry_tests.rs` - Entry tests

**Time**: 3-4 hours

---

#### Task 3: User Message Parsing (Day 2)

**Goal**: Parse user-specific fields

**Subtasks**:
1. Add `UserMessage` struct or enum variant
2. Parse `message.role` and `message.content` (string)
3. Parse `thinkingMetadata` (level, disabled, triggers)
4. Handle trigger positions (start, end, text)
5. Add tests with real user message samples

**Success criteria**:
```rust
let line = /* real user message from ~/.claude */;
let entry = Entry::from_json_line(line)?;
assert_eq!(entry.entry_type, EntryType::User);
assert!(entry.thinking_metadata.triggers.len() > 0);
```

**Files**:
- `src/entry.rs` - User message parsing
- `src/entry_tests.rs` - User tests

**Time**: 2-3 hours

---

#### Task 4: Assistant Message Parsing (Day 2-3)

**Goal**: Parse assistant-specific fields and content blocks

**Subtasks**:
1. Add `AssistantMessage` struct or enum variant
2. Parse message metadata (model, id, stop_reason, usage)
3. Parse content array
4. Implement content block parsing (text, thinking, tool_use, tool_result)
5. Parse usage object (token counts, cache stats)
6. Add tests with real assistant message samples

**Success criteria**:
```rust
let line = /* real assistant message from ~/.claude */;
let entry = Entry::from_json_line(line)?;
assert_eq!(entry.entry_type, EntryType::Assistant);
assert!(entry.content_blocks.len() > 0);
assert_eq!(entry.content_blocks[0].block_type, ContentBlockType::Text);
```

**Files**:
- `src/entry.rs` - Assistant message parsing
- `src/content_block.rs` - Content block types
- `src/entry_tests.rs` - Assistant tests

**Time**: 4-5 hours

---

#### Task 5: Integration Tests (Day 3-4)

**Goal**: Test against real Claude Code storage

**Subtasks**:
1. Create integration test that uses actual `~/.claude/` data
2. Test project discovery
3. Test session loading
4. Test entry parsing from real files
5. Test conversation threading (parentUuid chains)
6. Test error handling (malformed data)
7. Add performance benchmarks

**Success criteria**:
```rust
#[test]
fn test_load_real_project() {
  let storage = Storage::new()?;
  let project = storage.load_project_for_cwd()?;
  assert!(project.sessions()?.len() > 0);

  for mut session in project.sessions()? {
    let entries = session.entries()?;
    assert!(entries.len() > 0);

    // Verify threading
    for entry in entries {
      if let Some(parent_uuid) = &entry.parent_uuid {
        // Parent should exist
        assert!(entries.iter().any(|e| &e.uuid == parent_uuid));
      }
    }
  }
}
```

**Files**:
- `tests/integration_tests.rs` - Real storage tests
- `tests/conversation_tests.rs` - Threading tests
- `tests/performance_tests.rs` - Benchmarks

**Time**: 4-6 hours

---

#### Task 6: Error Handling & Edge Cases (Day 4)

**Goal**: Robust error handling for real-world data

**Subtasks**:
1. Gracefully handle malformed JSON (skip + warn)
2. Handle missing optional fields (use defaults)
3. Handle unknown fields (ignore for forward compatibility)
4. Handle very long messages (streaming)
5. Handle Unicode correctly
6. Add error recovery tests

**Success criteria**:
- Parser doesnt panic on any real Claude Code data
- Partial success (some entries parse, some skip)
- Clear error messages with line numbers and context

**Files**:
- `src/entry.rs` - Error recovery
- `src/error.rs` - Enhanced error messages
- `tests/error_handling_tests.rs` - Error tests

**Time**: 3-4 hours

---

#### Task 7: Enhanced Read API (Day 4-5)

**Goal**: Convenience methods for common operations

**Subtasks**:
1. Add filtering (by type, date range, content)
2. Add searching (text search, UUID lookup)
3. Add statistics (message counts, token usage)
4. Add iterators (streaming large sessions)
5. Add documentation and examples

**API design**:
```rust
// Filter by type
let user_messages = session.entries_by_type(EntryType::User)?;

// Filter by date range
let recent = session.entries_after(date)?;

// Search content
let matches = session.search("version_bump")?;

// Statistics
let stats = session.stats()?;
println!("Messages: {}, Tokens: {}", stats.message_count, stats.total_tokens);

// Streaming large sessions
for entry in session.iter_entries()? {
  process_entry(entry?)?;
}
```

**Files**:
- `src/session.rs` - Enhanced API
- `src/query.rs` - Filtering/search helpers
- `src/stats.rs` - Statistics
- `examples/search.rs` - Example tool

**Time**: 4-5 hours

---

#### Task 8: Documentation & Examples (Day 5)

**Goal**: Complete user-facing documentation

**Subtasks**:
1. Update `readme.md` with read examples
2. Update `spec.md` with Phase 2 status
3. Create example tools:
   - `examples/list_projects.rs` - List all projects
   - `examples/read_session.rs` - Read session entries
   - `examples/search_history.rs` - Search conversations
   - `examples/export_markdown.rs` - Export to markdown
4. Add doc comments to all public APIs
5. Run doc tests

**Files**:
- `readme.md` - Updated
- `spec.md` - Updated
- `examples/` - 4+ working examples
- All `src/*.rs` - Doc comments

**Time**: 3-4 hours

---

## Detailed Task Specifications

### JSON Parser Design

**Requirements**:
- Zero dependencies
- Parse JSONL format (one object per line)
- Handle Claude Code's actual format
- Error messages with position info
- No unsafe code

**Types**:
```rust
pub enum JsonValue {
  Null,
  Bool(bool),
  Number(f64),
  String(String),
  Array(Vec<JsonValue>),
  Object(HashMap<String, JsonValue>),
}

pub struct JsonObject {
  fields: HashMap<String, JsonValue>,
}

impl JsonObject {
  pub fn get_str(&self, key: &str) -> Result<&str>;
  pub fn get_i64(&self, key: &str) -> Result<i64>;
  pub fn get_bool(&self, key: &str) -> Result<bool>;
  pub fn get_array(&self, key: &str) -> Result<&Vec<JsonValue>>;
  pub fn get_object(&self, key: &str) -> Result<&JsonObject>;
  pub fn get_optional<T>(&self, key: &str) -> Result<Option<T>>;
}

pub fn parse_json_object(input: &str) -> Result<JsonObject>;
```

**Edge cases**:
- Escaped strings (`\"`, `\\`, `\n`, `\t`, `\u0000`)
- Unicode characters
- Scientific notation numbers
- Nested objects and arrays
- Trailing whitespace
- Empty objects/arrays

### Entry Struct Updates

Current stub needs complete fields:

```rust
pub struct Entry {
  // Existing
  pub uuid: String,
  pub parent_uuid: Option<String>,
  pub entry_type: EntryType,
  pub message: Message,
  pub timestamp: String,
  pub cwd: PathBuf,
  pub git_branch: Option<String>,
  pub is_sidechain: bool,

  // New fields
  pub session_id: String,
  pub version: String,
  pub user_type: String,
  pub request_id: Option<String>,  // Assistant only
  pub thinking_metadata: Option<ThinkingMetadata>,  // User only
}

pub enum Message {
  User {
    role: String,  // "user"
    content: String,
  },
  Assistant {
    model: String,
    id: String,
    message_type: String,  // "message"
    role: String,  // "assistant"
    content: Vec<ContentBlock>,
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
    usage: UsageStats,
  },
}

pub struct ThinkingMetadata {
  pub level: String,
  pub disabled: bool,
  pub triggers: Vec<ThinkingTrigger>,
}

pub struct ThinkingTrigger {
  pub start: usize,
  pub end: usize,
  pub text: String,
}

pub enum ContentBlock {
  Text { text: String },
  Thinking { thinking: String, signature: String },
  ToolUse { id: String, name: String, input: JsonObject },
  ToolResult { tool_use_id: String, content: String, is_error: bool },
}

pub struct UsageStats {
  pub input_tokens: i64,
  pub output_tokens: i64,
  pub cache_creation_input_tokens: i64,
  pub cache_read_input_tokens: i64,
  pub service_tier: String,
}
```

### Integration Test Strategy

**Test pyramid**:
1. **Unit tests** (src/*_tests.rs) - Test each module in isolation
2. **Integration tests** (tests/) - Test against real storage
3. **Doc tests** (doc comments) - Verify examples work

**Integration test scenarios**:
```rust
// Scenario 1: List all projects
#[test]
fn test_list_all_projects() {
  let storage = Storage::new().unwrap();
  let projects = storage.list_projects().unwrap();
  assert!(projects.len() > 0);
}

// Scenario 2: Load project by path
#[test]
fn test_load_project_by_path() {
  let storage = Storage::new().unwrap();
  let project = storage.load_project_for_path("/home/user1/pro").unwrap();
  assert!(project.sessions().unwrap().len() > 0);
}

// Scenario 3: Parse all entries in a session
#[test]
fn test_parse_all_entries() {
  let storage = Storage::new().unwrap();
  let project = storage.load_project_for_cwd().unwrap();
  let mut session = project.sessions().unwrap().into_iter().next().unwrap();

  let entries = session.entries().unwrap();
  assert!(entries.len() > 0);

  for entry in entries {
    // All entries should parse successfully
    assert!(entry.uuid.len() > 0);
    assert!(entry.timestamp.len() > 0);
  }
}

// Scenario 4: Verify conversation threading
#[test]
fn test_conversation_threading() {
  let storage = Storage::new().unwrap();
  let project = storage.load_project_for_cwd().unwrap();
  let mut session = project.sessions().unwrap().into_iter().next().unwrap();

  let entries = session.entries().unwrap();

  // First entry should have no parent
  assert!(entries[0].parent_uuid.is_none());

  // Subsequent entries should reference previous entries
  for i in 1..entries.len() {
    assert!(entries[i].parent_uuid.is_some());
    let parent_uuid = entries[i].parent_uuid.as_ref().unwrap();
    assert!(entries.iter().any(|e| &e.uuid == parent_uuid));
  }
}

// Scenario 5: Handle malformed data gracefully
#[test]
fn test_malformed_data_handling() {
  // Create temp file with mix of valid and invalid lines
  let content = r#"
{"uuid":"valid","type":"user",...}
{invalid json here
{"uuid":"also-valid","type":"assistant",...}
"#;

  let result = parse_session_content(content);
  assert!(result.is_ok());

  let entries = result.unwrap();
  assert_eq!(entries.len(), 2);  // Only 2 valid entries
}
```

## Testing Strategy

### Unit Tests

**Coverage targets**:
- JSON parser: 95%+ (all edge cases)
- Entry parsing: 90%+ (all field types)
- Path encoding: 100% (critical functionality)
- Error handling: 85%+ (error paths)

**Test data**:
- Minimal valid entries
- Maximal entries (all optional fields)
- Edge cases (empty strings, very long content, Unicode)
- Invalid data (missing fields, wrong types)

### Integration Tests

**Real storage tests**:
- Test against actual `~/.claude/` data
- Verify all projects load
- Verify all sessions parse
- Verify no panics on any data

**Isolation**:
- Read-only (no modifications)
- Use user's actual storage (or copied test data)
- Skip if no Claude Code storage exists

### Doc Tests

**All public APIs must have runnable examples**:
```rust
/// Load a project by path
///
/// # Example
///
/// ```no_run
/// use claude_storage::Storage;
///
/// let storage = Storage::new()?;
/// let project = storage.load_project_for_path("/home/user/pro")?;
/// # Ok::<(), claude_storage::Error>(())
/// ```
pub fn load_project_for_path<P: AsRef<Path>>(&self, path: P) -> Result<Project>
```

## Performance Considerations

### Memory Efficiency

**Problem**: Large sessions (>1MB JSONL files) can consume significant memory

**Solution 1: Lazy loading** (already implemented)
- Sessions load entries on demand
- `Session::entries()` only parses when called

**Solution 2: Streaming iterator**
```rust
pub struct EntryIterator<'a> {
  file: BufReader<File>,
  line_num: usize,
}

impl Session {
  pub fn iter_entries(&self) -> Result<EntryIterator> {
    // Stream entries line-by-line
  }
}
```

**Solution 3: Partial loading**
```rust
impl Session {
  pub fn load_recent(&mut self, count: usize) -> Result<Vec<Entry>> {
    // Load only last N entries
  }

  pub fn load_range(&mut self, start: usize, end: usize) -> Result<Vec<Entry>> {
    // Load entries in range
  }
}
```

### Parsing Performance

**Optimization targets**:
- Parse 1000 entries/second minimum
- <100ms for typical session (100 entries)
- <1 second for large session (10,000 entries)

**Profiling**:
- Use `cargo bench` for benchmarks
- Identify hot paths with profiler
- Optimize hand-written JSON parser

## Error Handling Strategy

### Error Categories

1. **I/O errors** - File not found, permission denied
2. **Parse errors** - Malformed JSON, missing fields
3. **Validation errors** - Invalid UUID, bad timestamp format
4. **Logic errors** - Parent UUID not found, broken threading

### Error Messages

**Good error messages include**:
- What operation failed
- Why it failed
- Where it failed (file, line number)
- How to fix it

**Example**:
```
Error: Failed to parse entry at line 42 in session 8d795a1c.jsonl
Reason: Missing required field 'uuid'
Content: {"type":"user","message":...}
```

### Partial Success

Allow partial parsing success:
```rust
pub struct SessionParseResult {
  pub entries: Vec<Entry>,
  pub errors: Vec<ParseError>,
}

impl Session {
  pub fn load_with_errors(&mut self) -> Result<SessionParseResult> {
    // Parse as many entries as possible
    // Collect errors for failed entries
    // Return both successes and failures
  }
}
```

## Dependencies Decision

### Option 1: Zero Dependencies (Recommended)

**Pros**:
- Fast compilation
- Minimal attack surface
- No version conflicts
- Matches claude_session philosophy

**Cons**:
- More implementation work
- Hand-written JSON parser

**Estimate**: +8-10 hours for JSON parser

### Option 2: serde_json (Optional Feature)

**Pros**:
- Robust, battle-tested
- Faster development
- Less maintenance

**Cons**:
- Dependency added
- Slower compilation
- Larger binary

**Estimate**: +2-3 hours for implementation

### Decision: Zero Dependencies

**Rationale**:
- Aligns with project philosophy
- JSON parsing is straightforward for known format
- Educational value
- Full control over error messages
- Long-term maintenance advantage

## Success Metrics

### Functional Metrics

- ✅ Parse 100% of fields from Claude Code JSONL
- ✅ Handle both user and assistant messages
- ✅ Parse all content block types
- ✅ Work with real `~/.claude/` storage
- ✅ Zero panics on any Claude Code data

### Quality Metrics

- ✅ 85%+ test coverage
- ✅ Zero clippy warnings
- ✅ Zero compiler warnings
- ✅ All doc tests pass
- ✅ Integration tests with real data pass

### Performance Metrics

- ✅ Parse 1000+ entries/second
- ✅ <100ms for typical session (100 entries)
- ✅ Memory usage scales linearly
- ✅ Streaming works for large sessions (10k+ entries)

### Documentation Metrics

- ✅ All public APIs documented
- ✅ 4+ working examples
- ✅ Format spec complete
- ✅ Development plan documented
- ✅ Readme updated with examples

## Timeline

### Day 1 (6-8 hours)
- ✅ Task 1: JSON parser foundation (4-6h)
- ✅ Task 2: Entry type parsing (3-4h)

### Day 2 (6-8 hours)
- ✅ Task 2 completion if needed
- ✅ Task 3: User message parsing (2-3h)
- ✅ Task 4: Assistant message parsing start (3-4h)

### Day 3 (6-8 hours)
- ✅ Task 4 completion (1-2h)
- ✅ Task 5: Integration tests (4-6h)

### Day 4 (6-8 hours)
- ✅ Task 5 completion if needed
- ✅ Task 6: Error handling (3-4h)
- ✅ Task 7: Enhanced read API start (2-3h)

### Day 5 (4-6 hours)
- ✅ Task 7 completion (2-3h)
- ✅ Task 8: Documentation & examples (3-4h)
- ✅ Final verification and cleanup

**Total**: 28-38 hours (3-5 days focused work)

## Risks & Mitigation

### Risk 1: JSON Parser Complexity

**Risk**: Hand-written parser takes longer than expected

**Likelihood**: Medium

**Impact**: High (blocks all other work)

**Mitigation**:
- Start with parser first (Day 1 priority)
- Focus on minimal working parser
- Defer edge cases if needed
- Fallback: serde_json as optional feature

### Risk 2: Format Variations

**Risk**: Real Claude Code data has unexpected format variations

**Likelihood**: Medium

**Impact**: Medium (some entries fail to parse)

**Mitigation**:
- Test against multiple real sessions early
- Implement graceful error handling
- Support partial success (some entries parse, some skip)
- Add logging for unknown fields

### Risk 3: Performance Issues

**Risk**: Large sessions parse too slowly

**Likelihood**: Low

**Impact**: Medium (user experience degraded)

**Mitigation**:
- Profile early (Day 3)
- Implement streaming if needed
- Optimize hot paths
- Defer optimization if not needed

### Risk 4: Integration Test Failures

**Risk**: Tests fail on real storage data

**Likelihood**: Low

**Impact**: High (blocks release)

**Mitigation**:
- Run integration tests early (Day 3)
- Fix issues immediately
- Add comprehensive error logging
- Test on multiple machines/setups

## Next Steps After Phase 2

### Phase 3: Integration (Future)

**Scope**:
- Optional integration with `claude_session`
- Used by `wplan_agent` for history analysis
- Example tools (export, search, backup)

**Timeline**: 1-2 weeks

### Phase 4: Write Operations (Postponed)

**Scope**:
- `Entry::to_json_line()` implementation
- `Session::append_entry()` implementation
- Atomic writes
- Format validation

**Timeline**: 1 week (when needed)

### Phase 5: Advanced Features (Future)

**Scope**:
- Full-text search
- Query DSL
- Analytics and statistics
- Migration tools
- Compression support

**Timeline**: 2-3 weeks (incremental)

## Conclusion

This plan provides a structured approach to implementing read-only JSONL parsing for `claude_storage`. Focus is on:

1. **Zero dependencies** - Hand-written JSON parser
2. **Real data** - Test against actual Claude Code storage
3. **Robustness** - Graceful error handling
4. **Performance** - Efficient parsing and streaming
5. **Documentation** - Complete format spec and examples

Estimated **3-5 days** of focused development to reach production-ready read-only functionality.
