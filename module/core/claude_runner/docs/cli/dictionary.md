# Dictionary

## Core Terms

| Term | Definition |
|------|------------|
| `.run` | The primary command that builds and executes a `claude` process with the given parameters. Accepts all 8 configurable arguments. |
| `.help` | The help command that prints usage information and exits. Takes no arguments. Triggered by `-h` or `--help`. |
| `--dry-run` | A flag that suppresses execution and instead prints the environment variables and full command line that would be passed to `claude`. Useful for debugging. |
| `--continue` | A flag that resumes an existing Claude Code conversation in the working directory rather than starting a fresh one. Maps to `claude --continue`. |
| `--session-dir` | An optional path where Claude Code stores its conversation session files. When absent, Claude uses its own default session location. |
| `--skip-permissions` | A flag that bypasses Claude Code's interactive tool-permission prompts. Equivalent to `claude --dangerously-skip-permissions`. |
| `message` | The free-form text prompt sent to Claude as the task description. May be supplied as a bare positional argument or via `-m` / `--message`. |
| `unilang` | The internal command-routing framework used by `claude_runner`. User-facing argv (`--flag value`) is translated by an adapter layer into unilang token format (`key::value`) before dispatch. |
