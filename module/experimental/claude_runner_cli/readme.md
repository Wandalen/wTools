# claude_runner

> **Workspace:** [wtools](https://github.com/Wandalen/wTools) — `module/core/claude_runner_cli`

CLI for executing Claude Code with configurable builder-pattern parameters.

### Responsibility Table

| Entity | Responsibility | Input→Output | Scope | Out of Scope |
|--------|---------------|--------------|-------|--------------|
| claude_runner_cli | Claude Code CLI launcher | CLI args → process exit code | Arg parsing, dry-run, help | ❌ Process execution → `claude_runner_core`<br>❌ Session paths → `claude_session`<br>❌ Config loading → `config_hierarchy` |

### Scope

**Responsibility:**
- Parse CLI flags into `ClaudeCommand` builder calls
- Dry-run mode: print command/env without invoking Claude
- Help text and usage documentation
- Exit code propagation

**In Scope:**
- `-m/--message`, `-d/--dir`, `-c/--continue`, `--max-tokens`
- `--skip-permissions`, `--dry-run`, `--session-dir`, `--model`
- `-h/--help` usage output

**Out of Scope:**
- ❌ Process execution → `claude_runner_core`
- ❌ Session storage paths → `claude_session`
- ❌ Configuration loading → `config_hierarchy`

## Usage

```sh
# Execute Claude with a message
claude_runner --message "Explain this code"

# Continue conversation in a working directory
claude_runner -m "Fix the bug" --dir /path/to/project --continue

# Dry run: show what would be executed
claude_runner --message "Do something" --dry-run

# Skip permissions for automation
claude_runner --message "Run tests" --skip-permissions
```

## Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--message <MSG>` | `-m` | Prompt message | — |
| `--dir <PATH>` | `-d` | Working directory | current dir |
| `--continue` | `-c` | Continue conversation | false |
| `--max-tokens <N>` | — | Max output tokens | 200000 |
| `--skip-permissions` | — | Skip permission prompts | false |
| `--dry-run` | — | Print command, don't execute | false |
| `--session-dir <PATH>` | — | Session storage directory | auto |
| `--model <NAME>` | — | Claude model | default |
| `--help` | `-h` | Show help | — |

## Architecture

```
claude_runner (CLI)
  └→ parse_args()          (std::env::args parsing)
  └→ run(args)             (translate args → builder calls)
      └→ ClaudeCommand::new()
          .with_working_directory()
          .with_message()
          .with_continue_conversation()
          .execute()        ← in claude_runner_core ONLY
```

## Dependencies

- **claude_runner_core**: All process execution logic
- **error_tools**: Workspace-standard error handling

## Testing

```sh
cargo nextest run -p claude_runner
```
