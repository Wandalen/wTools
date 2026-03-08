# claude_runner

CLI for executing Claude Code via builder pattern; YAML schema constants for willbe integration.

### Responsibility Table

| Entity | Responsibility | Input→Output | Scope | Out of Scope |
|--------|---------------|--------------|-------|--------------|
| claude_runner (lib) | YAML schema constants; `COMMANDS_YAML` path for willbe binaries | — | YAML path constant | ❌ Command handlers → `dream_agent` |
| claude_runner (bin) | Claude Code CLI launcher; subprocess target for `dream_agent` | CLI args → process exit code | Arg parsing, dry-run, help | ❌ Process execution → `claude_runner_core`<br>❌ Session paths → `claude_session`<br>❌ Orchestration → `dream_agent` |

### Scope

**Library (`src/lib.rs`):**
- `COMMANDS_YAML` constant — absolute path to `claude.commands.yaml`
- Zero willbe dependencies — pure constant, always available regardless of features

**Binary (`src/main.rs`, requires `enabled` feature):**
- Parse CLI flags: `-m/--message`, `-d/--dir`, `-c/--continue`, `--max-tokens`
- Parse CLI flags: `--skip-permissions`, `--dry-run`, `-v/--verbose`, `--session-dir`, `--model`
- `-h/--help` usage output
- Exit code propagation

### Files

| File | Responsibility |
|------|----------------|
| `Cargo.toml` | Crate manifest: lib + binary, optional feature-gated deps |
| `src/lib.rs` | Public API: `COMMANDS_YAML` constant |
| `src/main.rs` | CLI binary: arg parsing → `ClaudeCommand` builder → execute |
| `claude.commands.yaml` | Unilang command definitions for `.claude` and `.claude.help` |

### Architecture

```
claude_runner lib (YAML schema + COMMANDS_YAML constant, wtools)
    └─ COMMANDS_YAML → path to claude.commands.yaml

claude_runner binary (CLI subprocess target, wtools)
    └─ --flag value args → ClaudeCommand (claude_runner_core) → Command::new("claude")

dream_agent (handlers + orchestration, willbe)
    ├─ claude_routine         → parses unilang params → spawns subprocess
    ├─ claude_help_routine    → static help text
    └─ spawns: claude_runner (subprocess)

claude_runner_plugin (wplan runner plugin, standalone willbe crate)
    └─ routes .claude wplan commands → dream_agent::routines
```

### Separation of Concerns

- `claude_runner` (THIS crate): YAML schema + CLI subprocess. Zero willbe deps in lib.
- `dream_agent` (willbe): Runtime handlers, execution orchestration, wplan runner plugin.
- `claude_runner_core` (wtools lib): `ClaudeCommand` builder — used by CLI binary.

### Usage

**COMMANDS_YAML constant (lib, no features required):**
```rust
use claude_runner::COMMANDS_YAML;
aggregator.add(COMMANDS_YAML);
```

**CLI subprocess (binary, requires `enabled` feature):**
```sh
claude_runner "Fix the bug" --dir /path/to/project
claude_runner -m "Explain this" --continue
claude_runner --message "test" --dry-run
claude_runner --help
```
