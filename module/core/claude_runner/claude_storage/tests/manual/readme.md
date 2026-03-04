# manual testing plan - claude_storage v1.3.0

## responsibility

Comprehensive manual testing coverage for all CLI commands, parameter combinations, and edge cases in the claude_storage crate.

## testing scope

### commands to test

1. `.status` - Storage statistics (path parameter tested in Phase 1D)
2. `.list` - Project/session listing with filtering
3. `.show` - Session/project display (location-aware)
4. `.show.project` - Project display (deprecated)
5. `.count` - Fast counting operations (target parameter tested in Phase 1A)
6. `.search` - Full-text search (parameter validation tested in Phase 1B)
7. `.export` - Export sessions to file (parameter validation tested in Phase 1C)

### parameter validation

Each command must be tested with:
- Valid parameter values (all documented options)
- Invalid parameter values (typos, wrong types, out-of-range)
- Missing required parameters
- Extra unexpected parameters
- Parameter combinations (valid and invalid)
- Edge values (0, negative, very large numbers)
- Special characters and encoding

### data conditions

- Empty storage (no projects)
- Single project (UUID type)
- Single project (path type)
- Multiple projects (mixed types)
- Projects with no sessions
- Projects with main sessions only
- Projects with agent sessions only
- Projects with mixed session types
- Sessions with 0 entries
- Sessions with 1 entry
- Sessions with thousands of entries
- Corrupted JSONL files
- Missing history.jsonl
- Nonexistent paths in storage

### path handling

- Absolute paths (`/home/user/project`)
- Relative paths (`../project`, `./project`)
- Special paths (`.`, `..`, `~`, `~/subdir`)
- Path-encoded format (`-home-user-project`)
- Path patterns (substring matching)
- Paths with spaces
- Paths with special characters
- Paths with UTF-8 characters
- Very long paths (>255 characters)
- Nonexistent paths
- Paths to deleted directories

### session id handling

- Full UUID format (`abc-123-def-456-789`)
- Partial UUID (first 8 chars: `abc-123`)
- Agent format (`agent-022ada42`)
- Invalid formats
- Nonexistent session IDs
- Empty session ID

### REQ-011 content display

- Default verbosity::1 shows content
- verbosity::0 shows metadata only
- metadata::1 parameter
- entries::1 parameter
- Content truncation behavior
- Empty conversation content
- Very large conversation content
- UTF-8 content handling
- Special characters in content

## corner case matrix

### `.status` command

| Test Case | Parameters | Expected Behavior | Priority |
|-----------|------------|-------------------|----------|
| Default verbosity | (none) | Show basic statistics | High |
| Min verbosity | `verbosity::0` | Minimal output | High |
| Max verbosity | `verbosity::5` | Maximum detail | High |
| Out-of-range verbosity | `verbosity::10` | Handle gracefully | Medium |
| Negative verbosity | `verbosity::-1` | Handle gracefully | Medium |
| Invalid verbosity | `verbosity::abc` | Error message | Medium |
| Empty storage | (none) | Show 0 counts | High |
| Large storage | (none) | Performance check | Low |

### `.list` command

| Test Case | Parameters | Expected Behavior | Priority |
|-----------|------------|-------------------|----------|
| Default | (none) | List projects only | High |
| UUID projects | `type::uuid` | Filter UUID projects | High |
| Path projects | `type::path` | Filter path projects | High |
| All projects | `type::all` | Show all projects | High |
| Invalid type | `type::invalid` | Error message | Medium |
| Explicit sessions enable | `sessions::1` | Show sessions | High |
| Explicit sessions disable | `sessions::0` | Hide sessions | High |
| Session filter auto-enable | `session::commit` | Auto-show sessions | High |
| Agent filter auto-enable | `agent::1` | Auto-show sessions | High |
| Min entries filter auto-enable | `min_entries::10` | Auto-show sessions | High |
| Override auto-enable | `sessions::0 session::test` | Projects only | High |
| Path current dir | `path::.` | Match current dir | High |
| Path parent dir | `path::..` | Match parent dir | High |
| Path home | `path::~` | Match home dir | High |
| Path home + subdir | `path::~/pro` | Match home subdir | High |
| Path absolute | `path::/home/user/pro` | Match absolute | High |
| Path relative | `path::../lib` | Resolve + match | High |
| Path pattern | `path::willbe` | Substring match | High |
| Path nonexistent | `path::/nonexistent` | No matches | Medium |
| Agent main only | `agent::0` | Filter main sessions | Medium |
| Agent sub only | `agent::1` | Filter agent sessions | Medium |
| Min entries zero | `min_entries::0` | All sessions | Medium |
| Min entries high | `min_entries::1000` | Few/no matches | Medium |
| Min entries negative | `min_entries::-5` | Error or 0 | Medium |
| Session substring | `session::commit` | Match session IDs | High |
| Session empty | `session::` | All or error | Low |
| Session nonexistent | `session::xyz999` | No matches | Medium |
| Combined filters | `path::willbe session::default agent::0 min_entries::5` | All filters apply | High |
| Empty storage | (none) | No projects | High |

### `.show` command

| Test Case | Parameters | Expected Behavior | Priority |
|-----------|------------|-------------------|----------|
| No params (in project dir) | (none) | Show current project | High |
| No params (not in project) | (none) | Error or empty | High |
| Session only (in project) | `session_id::abc123` | Show session in current project | High |
| Session only (not in project) | `session_id::abc123` | Error or search | Medium |
| Project only (absolute path) | `project::/home/user/pro` | Show all sessions | High |
| Project only (path-encoded) | `project::-home-user-pro` | Show all sessions | High |
| Project only (UUID) | `project::abc-123-def` | Show all sessions | High |
| Project only (Path format) | `project::Path("/home/user/pro")` | Show all sessions | High |
| Project only (current dir) | `project::.` | Show current project | Medium |
| Project only (parent dir) | `project::..` | Show parent project | Medium |
| Project only (home) | `project::~` | Show home project | Medium |
| Both params (full UUID) | `session_id::abc-123-def project::/path` | Show specific session | High |
| Both params (partial UUID) | `session_id::abc123 project::/path` | Show specific session | High |
| Both params (agent ID) | `session_id::agent-022ada42 project::/path` | Show agent session | High |
| Nonexistent session | `session_id::nonexistent project::/path` | Error message | High |
| Nonexistent project | `project::/nonexistent` | Error message | High |
| Invalid session format | `session_id::123` | Error message | Medium |
| Invalid project format | `project::???` | Error message | Medium |
| Verbosity 0 (metadata only) | `verbosity::0 session_id::abc` | Metadata only | High |
| Verbosity 1 (content - default) | `verbosity::1 session_id::abc` | Full content | High |
| Verbosity 2+ | `verbosity::2 session_id::abc` | Content + metadata | High |
| Metadata parameter | `metadata::1 session_id::abc` | Metadata only | High |
| Metadata + verbosity 1 | `metadata::1 verbosity::1 session_id::abc` | Which wins? | High |
| Entries parameter | `entries::1 session_id::abc` | Show all entries | Medium |
| Empty session (0 entries) | `session_id::{empty} project::/path` | Handle gracefully | High |
| Large session (1000+ entries) | `session_id::{large} project::/path` | Performance + truncation | Medium |
| UTF-8 in content | `session_id::{utf8} project::/path` | Display correctly | High |

### `.show.project` command (deprecated)

| Test Case | Parameters | Expected Behavior | Priority |
|-----------|------------|-------------------|----------|
| No params (in project) | (none) | Show current project | High |
| No params (not in project) | (none) | Error | High |
| Absolute path | `project::/home/user/pro` | Show project | High |
| Path-encoded | `project::-home-user-pro` | Show project | High |
| UUID | `project::abc-123-def` | Show project | High |
| Path format | `project::Path("/home/user/pro")` | Show project | High |
| Nonexistent project | `project::/nonexistent` | Error message | High |
| Verbosity levels | `verbosity::0..5` | Different detail levels | Medium |

### `.count` command

| Test Case | Parameters | Expected Behavior | Priority | Automated |
|-----------|------------|-------------------|----------|-----------|
| Count projects | `target::projects` | Project count | High | ✅ Phase 1A |
| Count sessions (no project) | `target::sessions` | Error (needs project) | High | ✅ Phase 1A |
| Count sessions (with project) | `target::sessions project::/path` | Session count | High | |
| Count entries (no session) | `target::entries` | Error (needs session) | High | ✅ Phase 1A |
| Count entries (with session) | `target::entries session::abc project::/path` | Entry count | High | |
| Invalid target | `target::invalid` | Error message | High | ✅ Phase 1A |
| Missing target | (none) | Error message | High | |
| Nonexistent project | `target::sessions project::/nonexistent` | Error message | High | |
| Nonexistent session | `target::entries session::nonexistent project::/path` | Error message | High | |
| Empty project (0 sessions) | `target::sessions project::/empty` | Count = 0 | Medium | |
| Empty session (0 entries) | `target::entries session::empty project::/path` | Count = 0 | Medium | |

### `.search` command

| Test Case | Parameters | Expected Behavior | Priority | Automated |
|-----------|------------|-------------------|----------|-----------|
| Missing query | (none) | Error message | High | ✅ Phase 1B |
| Empty query | `query::` | Error message | High | ✅ Phase 1B |
| Simple query | `query::error` | Find matches | High | |
| Case sensitive search | `query::Error case_sensitive::1` | Case-sensitive matches | High | ✅ Phase 1B |
| Case insensitive search | `query::error case_sensitive::0` | Case-insensitive matches | High | |
| Filter by entry_type user | `query::test entry_type::user` | User messages only | High | ✅ Phase 1B |
| Filter by entry_type assistant | `query::test entry_type::assistant` | Assistant messages only | High | |
| Filter by entry_type all | `query::test entry_type::all` | All entries | High | |
| Invalid entry_type | `query::test entry_type::invalid` | Error message | High | ✅ Phase 1B |
| Filter by project | `query::test project::/path` | Project-specific results | High | |
| Filter by session | `query::test session::abc` | Session-specific results | High | |
| Invalid verbosity | `query::test verbosity::-1` | Error message | High | ✅ Phase 1B |
| Verbosity levels | `query::test verbosity::0..5` | Different detail levels | Medium | |
| Nonexistent project | `query::test project::/nonexistent` | No results or error | Medium | |
| Nonexistent session | `query::test session::nonexistent` | No results or error | Medium | |
| No matches | `query::xyz999` | Empty result set | Medium | |
| UTF-8 query | `query::日本語` | Unicode search | Medium | |
| Special chars query | `query::foo*bar` | Literal or pattern | Medium | |

### `.export` command

| Test Case | Parameters | Expected Behavior | Priority | Automated |
|-----------|------------|-------------------|----------|-----------|
| Missing session_id | `output::/tmp/test.md` | Error message | High | ✅ Phase 1C |
| Missing output | `session_id::abc` | Error message | High | ✅ Phase 1C |
| Invalid format | `session_id::abc output::/tmp/test format::csv` | Error message | High | ✅ Phase 1C |
| Format markdown | `session_id::abc output::/tmp/test.md format::markdown` | Export to markdown | High | |
| Format json | `session_id::abc output::/tmp/test.json format::json` | Export to JSON | High | |
| Format text | `session_id::abc output::/tmp/test.txt format::text` | Export to plain text | High | |
| Default format | `session_id::abc output::/tmp/test.md` | Defaults to markdown | High | |
| Nonexistent session | `session_id::nonexistent output::/tmp/test.md` | Error message | High | |
| Nonexistent directory | `session_id::abc output::/nonexistent/dir/test.md` | Error message | High | |
| File exists | `session_id::abc output::/existing/file.md` | Overwrite or error | Medium | |
| Filter by project | `session_id::abc project::/path output::/tmp/test.md` | Export from specific project | High | |
| Permission denied | `session_id::abc output::/root/test.md` | Error message | Low | |

### general edge cases

| Test Case | Scenario | Expected Behavior | Priority |
|-----------|----------|-------------------|----------|
| No ~/.claude/ directory | Fresh system | Error message | High |
| Empty ~/.claude/projects/ | No projects | Empty results | High |
| Corrupted JSONL | Malformed JSON | Skip + warning | High |
| Missing history.jsonl | Deleted file | Warn + continue | Medium |
| UTF-8 paths | Non-ASCII dirs | Handle correctly | High |
| Spaces in paths | `/home/user/my project/` | Handle correctly | High |
| Very long path | 300+ chars | Handle correctly | Low |
| Special chars | `path::/home/$USER/test` | Handle correctly | Medium |
| Case sensitivity | `session::ABC` vs `session::abc` | Case-insensitive | Medium |
| Whitespace in params | `session:: abc ` | Trim or error | Medium |
| Empty param values | `session::` | Handle gracefully | Medium |
| Multiple same params | `verbosity::1 verbosity::2` | Last wins or error | Low |
| Unknown params | `unknown::value` | Ignore or error | Medium |
| Storage permission denied | No read access | Error message | Low |

## test execution plan

### phase 1: automated test coverage audit (pre-manual)

Before manual testing, verify automated test coverage:

1. ✅ Review all existing tests (90 tests: 81 passing, 9 ignored)
2. ✅ Identify which corner cases are already covered (Phase 1 complete - see tests/readme.md)
3. ✅ Create gap analysis (Parameter Coverage Matrix in -current_plan.md shows 58% coverage)
4. Focus manual testing on uncovered areas (integration tests, edge cases, performance)

**Phase 1 Automated Coverage Achievements**:
- `.status` path parameter: 4/5 tests passing (1 ignored for default path)
- `.count` target parameter: 4 comprehensive validation tests
- `.search` parameters: 5/8 validation tests passing (3 integration tests ignored)
- `.export` parameters: 3/8 validation tests passing (5 integration tests ignored)
- 1 bug found and fixed (Finding #010: verbosity validation)

**Remaining Manual Test Focus**:
- Integration tests for .search (project, session, entry_type parameters)
- Integration tests for .export (format, project parameters)
- .list path and session integration tests
- Performance testing (large storage, large sessions)
- Error handling (corrupted files, permissions, encoding)

### phase 2: command validation

For each command:

1. Test all valid parameter combinations
2. Test all invalid parameter combinations
3. Test all edge values (0, negative, very large)
4. Test parameter type errors

### phase 3: data condition testing

1. Setup test storage with various conditions
2. Run commands against each condition
3. Verify behavior matches spec
4. Document any unexpected behavior

### phase 4: integration testing

1. Test command sequences (`.list` → `.show`)
2. Test copy-paste workflows
3. Test scripting scenarios
4. Test REPL vs one-shot behavior differences

### phase 5: performance testing

1. Large storage (100+ projects)
2. Large sessions (1000+ entries)
3. Deep nesting
4. Many agent sessions

### phase 6: error handling

1. Corrupted files
2. Missing files
3. Permission issues
4. Invalid encoding
5. Disk full scenarios

## test result documentation

For each test executed:

1. Record command executed
2. Record actual output
3. Compare against expected behavior
4. Mark as PASS/FAIL
5. For failures:
   - Document expected vs actual
   - Create bug reproducer test
   - Fix issue properly (no workarounds)
   - Verify fix with ctest3
   - Re-run manual test

## test metrics

- Total corner cases identified: ~100+
- High priority cases: ~60
- Medium priority cases: ~30
- Low priority cases: ~10+

## test status

**Status**: NOT STARTED

**Last Updated**: 2025-12-06

**Test Run Log**: See `-test_results.md` (created during execution)
