# claude_storage_core

Pure library for Claude Code's filesystem-based storage access (zero dependencies).

## overview

This is the core library extracted from the monolithic `claude_storage` crate (2025-11-29). It provides safe, structured read/write access to Claude Code's conversation storage at `~/.claude/` with **zero runtime dependencies**.

## features

- **Zero dependencies**: No runtime dependencies for fast compilation and minimal attack surface
- **Hand-written JSON parser**: ~690 lines, supports all JSON types, Unicode escaping
- **Path encoding/decoding**: Filesystem path encoding for storage directories
- **Statistics aggregation**: Fast counting and analytics without full parsing
- **Format validation**: JSONL structure validation with detailed error messages
- **Safety guarantees**: Append-only operations, atomic writes, graceful error handling

## usage

```toml
[dependencies]
claude_storage_core = { workspace = true }
```

```rust
use claude_storage_core::{ Storage, ProjectId };

fn main() -> claude_storage_core::Result< () >
{
  // List all projects
  let storage = Storage::new()?;
  for project in storage.list_projects()?
  {
    println!( "Project: {:?}", project.id() );

    // List sessions within project
    for mut session in project.sessions()?
    {
      println!( "  Session: {}", session.id() );
      println!( "  Entries: {}", session.count_entries()? );
    }
  }
  Ok( () )
}
```

## architecture

**Storage model**: Claude Code uses filesystem-native storage at `~/.claude/`
```
~/.claude/
├── projects/
│   ├── {uuid}/           # UUID projects (web/IDE sessions)
│   └── -{path-encoded}/  # Path projects (CLI sessions)
├── history.jsonl
└── .credentials.json
```

**Core types**:
- `Storage` - Entry point for all operations
- `Project` - Directory containing sessions (UUID or path-based)
- `Session` - Single conversation (JSONL file)
- `Entry` - Individual message (user or assistant)

**Path encoding**: `/home/user/pro` → `-home-user-pro`

## performance

- **Lazy loading**: Entries loaded on-demand, not at construction
- **Fast counting**: 100x speedup (~5ms vs 500ms for 1000 entries)
- **Selective parsing**: Statistics without loading all data
- **JSON parser**: ~80ns per operation

## testing

**51 tests total** (45 unit + 3 bug + 3 doc):
```bash
cargo nextest run --all-features  # 48 tests
cargo test --doc --all-features   # 3 doc tests
cargo clippy --all-targets --all-features -- -D warnings
```

## related crates

- **claude_storage**: CLI tool wrapping this library for command-line storage exploration
- **claude_session**: Process control layer (may use this for session detection)
- **wplan_agent**: AI integration layer (may use this for conversation history analysis)

## migration

**From monolithic claude_storage**:
```diff
[dependencies]
- claude_storage = { path = "../claude_storage" }
+ claude_storage_core = { workspace = true }
```

```diff
- use claude_storage::{ Storage, ProjectId };
+ use claude_storage_core::{ Storage, ProjectId };
```

See `../claude_storage/docs/MIGRATION.md` for complete migration guide.

## documentation

- **Specification**: `spec.md` - Complete architecture and API documentation
- **Format docs**: `../claude_storage/docs/` - JSONL format, storage organization, advanced features
- **Examples**: `examples/` - Usage examples and integration patterns

## license

MIT
