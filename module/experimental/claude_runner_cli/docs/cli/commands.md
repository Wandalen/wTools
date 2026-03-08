# Commands

| # | Command | Purpose | Params | Status |
|---|---------|---------|--------|--------|
| 1 | `.run` | Execute Claude Code with configurable parameters | 9 | ✅ |
| 2 | `.help` | Print usage information and exit | 0 | ✅ |

**Total:** 2 commands

---

### Command :: 1. `.run`

Builds a Claude Code invocation from the given parameters and executes it as a subprocess. Use this when you want to run Claude Code on a project with custom configuration — directory, model, token limits, session continuity, etc.

**Syntax:**

```bash
claude_runner
claude_runner "MESSAGE"
claude_runner "MESSAGE" --dir /path/to/project
claude_runner "MESSAGE" --dir /path --model claude-opus-4-6 --max-tokens 50000 --continue
```

**Parameters:**

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `message::` | [`MessageText`](params.md#parameter--1-message) | Prompt sent to Claude | — |
| `dir::` | [`PathArg`](params.md#parameter--2-dir) | Working directory for Claude Code | current dir |
| `continue::` | `bool` | Resume existing conversation | false |
| `max_tokens::` | [`TokenCount`](params.md#parameter--4-max_tokens) | Maximum output tokens | 200000 |
| `skip_permissions::` | `bool` | Skip tool permission prompts | false |
| `dry::` | `bool` | Print command without executing | false |
| `verbose::` | `bool` | Print command to stderr, then execute | false |
| `session_dir::` | [`PathArg`](params.md#parameter--7-session_dir) | Session storage directory | — |
| `model::` | [`ModelName`](params.md#parameter--8-model) | Claude model to use | — |

**Exit Codes:** 0 (success) | 1 (error)

**Error Handling:**

- Unknown flag → exit 1: `unknown argument: --flag`
- Missing flag value → exit 1: `--flag requires a value`
- Invalid `--max-tokens` value → exit 1: `invalid --max-tokens value: <input>`
- Both positional and `--message` provided → exit 1: conflict error
- Claude Code non-zero exit → exit 1: `Claude exited with code N`

**Examples:**

```bash
claude_runner "Fix the bug in src/main.rs"
# [Claude output streamed to stdout]

claude_runner "Refactor auth module" --dir ~/projects/app --model claude-opus-4-6
# [Claude output using Opus in ~/projects/app]

claude_runner "Continue the previous work" --continue --dry-run
# claude -c "Continue the previous work"

claude_runner "Debug the failing test" --dir ~/projects/app --verbose
# (stderr) CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
# (stderr) [... other env vars ...]
# (stderr) cd /home/user/projects/app
# (stderr) claude "Debug the failing test"
# [Claude output streamed to stdout]
```

---

### Command :: 2. `.help`

Prints usage information and the full list of available flags to stdout, then exits with code 0. Use this when you need a quick on-terminal reference to flag names without consulting the full documentation.

**Syntax:**

```bash
claude_runner --help
claude_runner -h
```

**Exit Codes:** 0 (success)

**Examples:**

```bash
claude_runner --help
# claude_runner — Execute Claude Code with configurable parameters
#
# USAGE:
#   claude_runner [OPTIONS] [MESSAGE]
#
# OPTIONS:
#   -m, --message <MSG>        Prompt message for Claude
#   -d, --dir <PATH>           Working directory (default: current dir)
#   -c, --continue             Continue existing conversation
#       --max-tokens <N>       Max output tokens (default: 200000)
#       --skip-permissions     Skip tool permission prompts
#       --dry-run              Print command without executing
#   -v, --verbose              Print command to stderr, then execute
#       --session-dir <PATH>   Session storage directory
#       --model <NAME>         Claude model to use
#   -h, --help                 Show this help
```

**Notes:** `--help` is parsed sequentially. If any unknown flag appears before `--help` in the argv, the unknown-flag error fires first and `.help` is never reached. This is by design — error detection takes priority over help display.
