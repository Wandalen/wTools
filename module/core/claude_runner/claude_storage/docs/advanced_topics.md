# advanced topics

## overview

This document covers advanced storage features discovered through deep analysis: agent sessions, command system, history tracking, and session environment management.

## agent sessions (sub-agents)

### what are agent sessions

**Agent sessions** are sub-conversations spawned by Claude Code using the Task tool.

**File naming**: `agent-{short-uuid}.jsonl` (8-character hex ID, not full UUID).

**Location**: Stored alongside main session in project directory.

**Example directory structure**:
```
projects/-home-user1-pro-lib-willbe-module--default-topic/
├── 9425242b-1185-4788-993e-09852db0516d.jsonl    # Main session
├── agent-64bdad98.jsonl                           # Agent session 1
├── agent-e360ed21.jsonl                           # Agent session 2
├── agent-7f4703a7.jsonl                           # Agent session 3
└── agent-d37c87ca.jsonl                           # Agent session 4
```

### agent session format

**File format**: JSONL (same as main sessions).

**Entry structure**: Same as regular sessions with **key differences**:

```json
{
  "parentUuid": null,
  "isSidechain": true,
  "userType": "external",
  "cwd": "/home/user1/pro/lib/willbe/module/wplan_agent/-default_topic",
  "sessionId": "9425242b-1185-4788-993e-09852db0516d",
  "version": "2.0.31",
  "gitBranch": "master",
  "agentId": "64bdad98",
  "type": "user",
  "message": {
    "role": "user",
    "content": "Warmup"
  },
  "uuid": "7e47f77a-d849-4a93-a1d7-26997c972273",
  "timestamp": "2025-11-24T19:45:09.275Z"
}
```

**Key differences from main sessions**:

| Field | Value | Description |
|-------|-------|-------------|
| `isSidechain` | `true` | Marks entry as part of agent session |
| `agentId` | `"64bdad98"` | Short UUID identifying this agent |
| `sessionId` | `"9425242b..."` | References PARENT session (not agent session) |

**File naming pattern**: `agent-{agentId}.jsonl`

### agent session characteristics

**Size range**: 2 to 334 entries observed.

**Largest agent sessions** (by entry count):
```
334 entries - agent-88bbbc21.jsonl
271 entries - agent-fab24d8a.jsonl
206 entries - agent-79cc0ddb.jsonl
199 entries - agent-a4fbee7e.jsonl
197 entries - agent-950eb413.jsonl
```

**Usage pattern**:
- Spawned via Task tool from main session
- Independent conversation thread
- Can execute tools and interact with user
- Results reported back to parent session

**Threading**:
- Agent entries link via `parentUuid` (same as main sessions)
- First entry has `parentUuid: null`
- Subsequent entries reference previous entry in AGENT session
- NO direct threading to parent session entries

### detecting agent sessions

**Filename detection**:
```rust
fn is_agent_session(filename: &str) -> bool {
  filename.starts_with("agent-") && filename.ends_with(".jsonl")
}

fn extract_agent_id(filename: &str) -> Option<&str> {
  if filename.starts_with("agent-") && filename.ends_with(".jsonl") {
    Some(&filename[6..filename.len() - 6])  // "agent-64bdad98.jsonl" → "64bdad98"
  } else {
    None
  }
}
```

**Entry detection**:
```rust
fn is_agent_entry(entry: &Entry) -> bool {
  entry.is_sidechain && entry.agent_id.is_some()
}
```

### agent session discovery

**List all agents in project**:
```rust
fn list_agent_sessions(project_dir: &Path) -> Result<Vec<String>> {
  let mut agents = Vec::new();

  for entry in fs::read_dir(project_dir)? {
    let entry = entry?;
    let filename = entry.file_name().to_string_lossy().to_string();

    if filename.starts_with("agent-") && filename.ends_with(".jsonl") {
      let agent_id = &filename[6..filename.len() - 6];
      agents.push(agent_id.to_string());
    }
  }

  Ok(agents)
}
```

**Finding parent session**:
```rust
// Agent entry contains parent sessionId
let parent_session_id = agent_entry.session_id;  // "9425242b-1185-4788-993e-09852db0516d"
let parent_session_file = format!("{}.jsonl", parent_session_id);
```

### implementation considerations

**Session listing**: Include agent sessions in project session lists (with `is_agent` flag).

**Parent tracking**: Store `parent_session_id` for agent sessions (from `sessionId` field).

**Entry parsing**: Handle `agentId` and `isSidechain` fields.

**Display**: Mark agent sessions distinctly in CLI output (e.g., "agent-64bdad98 (sub-agent)").

## command system

### command file format

**Location**: `~/.claude/commands/`

**File count**: 46 command definitions.

**Format**: Markdown with YAML frontmatter.

### command file structure

**YAML frontmatter** (required):
```yaml
---
description: Act as an expert git assistant. Analyze the repository state...
color: purple
---
```

**File identifier comment** (optional):
```markdown
<!--| file : git/commit.md -->
```

**Content sections** (markdown):
```markdown
# command-name

### Role
You are an expert...

### Objective
Your goal is to...

### Context
You will be provided with...

### Scope & Boundaries
**In-Scope:**
- ...

**Out-of-Scope:**
- ...

### Vocabulary
- **Term**: Definition...

### Procedures
| Procedure | Description |
|-----------|-------------|
| [Name](#procedure-name) | ... |

#### Procedure: Name
**Description:** ...
**Procedural Script:**
1. Step one
2. Step two
...
```

### command categories

**Analysis of 46 commands** (by prefix):

**Audit commands** (13):
- audit_assessment_questions.md
- audit_cli.md
- audit_codebase_hygiene.md
- audit_codestyle.md
- audit_configs.md
- audit_crate_distribution.md
- audit_design.md
- audit_hypothesis.md
- audit_output.md
- audit_rulebooks.md
- audit_specs.md
- audit_test_organization.md
- audit_unilang.md

**Development commands**:
- commit.md
- dev.md
- refactor_extracting.md
- spec_edit.md
- spec_implement.md

**Testing commands**:
- test_clean.md
- test_manual.md
- tdd_bug_fix.md

**Other commands**:
- agentic_cli.md
- cli_docs_analysis.md
- collections.md
- collections_discover.md
- command_creator.md
- feature_bookkeeper.md
- function_map.md
- git_conflict_fix.md
- internalize_rulebook.md
- md_split.md
- modules_cli_split.md
- modules_explore.md
- ops.md
- pr_review.md
- pr_review_org.md
- prompt_procedural.md
- project_review.md
- rules.md
- task_bookkeeper.md
- tasks_resolve_all.md
- tests_fix_all.md

### command design patterns

**Consistent structure**:
1. YAML frontmatter (description, color)
2. Role definition
3. Objective statement
4. Context description
5. Scope boundaries (in-scope / out-of-scope)
6. Vocabulary definitions
7. Procedural scripts

**Command invocation**: `/command-name` in Claude Code.

**Command expansion**: File contents replace slash command in conversation.

### example command file

**commit.md** (abbreviated):
```yaml
---
description: Act as an expert git assistant. Analyze the repository state, manage the staging area, and execute commits with high-quality, conventional commit messages.
color: purple
---

# git_commit

### Role
You are an expert git assistant. Your primary function is to analyze the repository's state, specifically the staged and unstaged changes, and then to execute a single, well-formed `git commit` command with a high-quality, conventional commit message.

### Objective
Your goal is to analyze the provided repository context (`git status`, `git diff`, etc.), generate a high-quality conventional commit message, display it to the user for review, and then provide the `git commit` command to execute it.

### Context
You will be provided with the following real-time repository information to inform your analysis:
*   **Current git status:** `!git status`
*   **Current git diff (staged and unstaged changes):** `!git diff HEAD`
*   **Current branch:** `!git branch --show-current`
*   **Recent commits:** `!git log --oneline -10`

### Scope & Boundaries
**In-Scope:**
*   Analyzing the provided `git diff` and `git status` to understand the code changes.
*   Determining the correct Conventional Commit type (`feat`, `fix`, `refactor`, etc.).
*   Generating a complete, multi-line commit message that follows all specified rules.
*   Displaying the generated commit message to the user before providing the command.
*   Executing a single `git commit` command via a `Bash` tool call.

**Out-of-Scope:**
*   This prompt will not stage or unstage files (e.g., `git add`, `git rm`). It assumes changes are already staged for commit.
*   This prompt will not execute any other shell commands besides `git commit`.
*   This prompt will not add any footers, signatures, or epilogues like `Co-authored-by:` or `Generated with` to the commit message.
```

### implementation notes

**Command discovery**: List all `.md` files in `~/.claude/commands/`.

**Command loading**: Parse YAML frontmatter + markdown content.

**Command execution**: User invokes via `/command-name`, Claude Code expands content into conversation.

**Storage access**: No need to parse command files for conversation reading (out of scope for claude_storage).

## history tracking (history.jsonl)

### detailed format

**File location**: `~/.claude/history.jsonl`

**Purpose**: Global index of all project accesses.

**Format**: JSONL (one entry per line).

**Total entries**: 4324 observed (1.1MB file size).

**Average entry size**: ~254 bytes.

### entry structure

**Complete schema**:
```json
{
  "display": "string - truncated user message or context",
  "pastedContents": {},  // Usually empty object
  "timestamp": 1758992388766,  // Unix timestamp (milliseconds)
  "project": "/absolute/path/to/project"  // Filesystem path
}
```

**Field specifications**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display` | string | yes | User query or context preview (truncated) |
| `pastedContents` | object | yes | Pasted file metadata (usually empty) |
| `timestamp` | number | yes | Unix timestamp in milliseconds |
| `project` | string | yes | Absolute filesystem path to project |

### display field patterns

**User messages** (truncated at ~50 characters):
```json
"display": "command to repeat something every hour?"
"display": "list not resolved tasks please "
"display": "keep specification up to date. ultrathink "
```

**Pasted content indicators**:
```json
"display": "[Pasted text #1 +180 lines]\n\n[Pasted text #2 +48 l"
"display": "[Pasted text #2 +17 lines]\n\nI dont want to see noi"
```

**Shell commands** (from CLI):
```json
"display": "└ ~/pro/lib/willbe/module/reasoner ─> cargo run --"
"display": " curl -X GET \"https://www.youtube.com/watch?v=dQw4"
```

**Web URLs**:
```json
"display": "https://www.youtube-transcript.io/api\nread page an"
```

### pastedContents field

**Observed pattern**: Always empty object `{}` in sampled data.

**Potential use**: May contain file paste metadata (filename, size, hash) when files are pasted into conversation.

**Current status**: Not actively used in observed data.

### timestamp details

**Format**: Unix timestamp in **milliseconds** (not seconds).

**Example**: `1758992388766` = November 24, 2025, ~19:45 UTC.

**Conversion**:
```rust
use std::time::{SystemTime, UNIX_EPOCH};

// From timestamp
let millis = 1758992388766u64;
let duration = Duration::from_millis(millis);
let system_time = UNIX_EPOCH + duration;

// To timestamp
let now = SystemTime::now();
let millis = now.duration_since(UNIX_EPOCH)?.as_millis() as u64;
```

**Precision**: Millisecond-level granularity.

### project path format

**Always absolute paths**:
```
/home/user1/pro/lib/willbe/module/reasoner
/home/user1/pro/lib/knowledge-hr
/home/user1/pro/lib/knowledge-trial-tasks
```

**Mapping to storage**:
- Path: `/home/user1/pro/lib/willbe/module/reasoner`
- Encoded: `-home-user1-pro-lib-willbe-module-reasoner`
- Storage: `~/.claude/projects/-home-user1-pro-lib-willbe-module-reasoner/`

### history usage patterns

**Project discovery**: Find recent projects by timestamp.

**Search**: Find conversations containing specific text (in display field).

**Analytics**: Track project access frequency and patterns.

**Timeline**: Reconstruct user activity timeline.

### parsing strategy

**Line-by-line**:
```rust
use std::io::{BufRead, BufReader};

fn parse_history(path: &Path) -> Result<Vec<HistoryEntry>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let mut entries = Vec::new();

  for line in reader.lines() {
    let line = line?;
    if line.trim().is_empty() {
      continue;
    }

    match parse_history_entry(&line) {
      Ok(entry) => entries.push(entry),
      Err(e) => eprintln!("Warning: Skipping malformed history entry: {}", e),
    }
  }

  Ok(entries)
}
```

**Struct definition**:
```rust
pub struct HistoryEntry
{
  pub display : String,
  pub pasted_contents : HashMap< String, serde_json::Value >,  // Usually empty
  pub timestamp : u64,  // Milliseconds since UNIX epoch
  pub project : PathBuf,
}
```

### implementation notes

**Recent projects**: Sort by timestamp descending, take N entries.

**Project filtering**: Filter entries by project path prefix.

**Search**: Simple substring match on display field (case-insensitive).

**Display truncation**: Remember that display is truncated (~50 chars) - not full message.

## session environment (session-env/)

### directory purpose

**Location**: `~/.claude/session-env/`

**Total directories**: 549 observed.

**Total size**: 2.2MB (directory metadata only).

**File count**: 0 (all directories are empty).

### directory structure

**One directory per session UUID**:
```
session-env/
├── 0125f61c-eb16-47c0-8e56-7efff691f990/    # Empty
├── 0191760d-9be9-4e1f-88a5-f56db1b2bff1/    # Empty
├── 01dbe072-4cb6-45cb-ace7-8e5d37e373dd/    # Empty
├── 0365ba5f-306e-4920-8b11-d9b4fed5fb02/    # Empty
...
```

**Directory naming**: UUID v4 format (matches session IDs).

**Contents**: All directories contain only `.` and `..` (empty).

### current status

**Hypothesis**: Session initialization markers or reserved for future use.

**Observed behavior**:
- Directories created at session start
- Never populated with files
- Persist after session ends

**Size overhead**: ~4KB per directory (ext4 filesystem metadata).

**Total overhead**: 549 directories × 4KB ≈ 2.2MB.

### implementation considerations

**Session existence check**:
```rust
fn session_exists(session_id: &str) -> bool {
  let session_env_dir = storage_root.join("session-env").join(session_id);
  session_env_dir.exists() && session_env_dir.is_dir()
}
```

**Session listing**:
```rust
fn list_active_sessions() -> Result<Vec<String>> {
  let session_env_dir = storage_root.join("session-env");
  let mut sessions = Vec::new();

  for entry in fs::read_dir(session_env_dir)? {
    let entry = entry?;
    if entry.file_type()?.is_dir() {
      sessions.push(entry.file_name().to_string_lossy().to_string());
    }
  }

  Ok(sessions)
}
```

**Cleanup consideration**: Empty directories can be safely deleted (no data loss).

**Future use**: May be populated with session-specific metadata in future Claude Code versions.

## advanced search capabilities

### cross-project search

**Use case**: Find conversations mentioning specific topics across all projects.

**Approach**:
1. List all projects
2. For each project, list sessions
3. For each session, parse entries
4. Filter entries matching search criteria
5. Return matches with context

**Implementation sketch**:
```rust
pub struct SearchResult
{
  pub project_id : String,
  pub session_id : String,
  pub entry_uuid : String,
  pub entry_type : EntryType,
  pub snippet : String,
  pub timestamp : String,
}

pub fn search_all_projects( query : &str ) -> Result< Vec< SearchResult > >
{
  let storage = Storage::new()?;
  let mut results = Vec::new();

  for project in storage.list_projects()? {
    for mut session in project.sessions()? {
      for entry in session.entries()? {
        if matches_query(&entry, query) {
          results.push(SearchResult {
            project_id: project.id().to_string(),
            session_id: session.id().to_string(),
            entry_uuid: entry.uuid.clone(),
            entry_type: entry.entry_type,
            snippet: extract_snippet(&entry, query),
            timestamp: entry.timestamp.clone(),
          });
        }
      }
    }
  }

  Ok(results)
}
```

### history-based project discovery

**Use case**: Find recently accessed projects.

**Approach**:
1. Parse history.jsonl
2. Sort by timestamp (descending)
3. Deduplicate by project path
4. Take N most recent
5. Map paths to encoded project IDs

**Implementation sketch**:
```rust
pub fn recent_projects( count : usize ) -> Result< Vec< String > >
{
  let history_file = storage_root().join("history.jsonl");
  let mut entries = parse_history(&history_file)?;

  // Sort by timestamp (newest first)
  entries.sort_by_key(|e| std::cmp::Reverse(e.timestamp));

  // Deduplicate by project
  let mut seen = HashSet::new();
  let mut projects = Vec::new();

  for entry in entries {
    if !seen.contains(&entry.project) {
      seen.insert(entry.project.clone());
      projects.push(encode_path(&entry.project));

      if projects.len() >= count {
        break;
      }
    }
  }

  Ok(projects)
}
```

### agent session tracking

**Use case**: Find all agent sessions spawned from a main session.

**Approach**:
1. Load main session
2. List project directory
3. Filter files matching `agent-*.jsonl`
4. Parse first entry of each agent session
5. Filter by `sessionId` matching main session ID

**Implementation sketch**:
```rust
pub fn find_agent_sessions( session : &Session ) -> Result< Vec< AgentSession > >
{
  let project_dir = session.project_dir();
  let main_session_id = session.id();
  let mut agents = Vec::new();

  for entry in fs::read_dir(project_dir)? {
    let entry = entry?;
    let filename = entry.file_name().to_string_lossy().to_string();

    if filename.starts_with("agent-") && filename.ends_with(".jsonl") {
      let agent_id = &filename[6..filename.len() - 6];
      let agent_session = AgentSession::load(project_dir, agent_id)?;

      // Check if agent belongs to this session
      if agent_session.parent_session_id() == main_session_id {
        agents.push(agent_session);
      }
    }
  }

  Ok(agents)
}
```

## related documentation

- `storage_organization.md` - Directory structure and storage model
- `file_formats.md` - Complete file format specifications
- `jsonl_format.md` - Conversation entry format details
- `development_plan.md` - Implementation roadmap
- `cli_design.md` - CLI command specifications
- `spec.md` - Overall crate specification
