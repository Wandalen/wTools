# Workflows

Usage scenarios for the `claude_runner` CLI showing common invocation patterns.

| # | Workflow | Commands | Purpose |
|---|----------|----------|---------|
| 1 | Basic prompt | `.run` | Send a simple message to Claude |
| 2 | Project-specific execution | `.run` | Run Claude in a specific directory |
| 3 | Dry-run preview | `.run` | Preview the command without executing |
| 4 | Session continuation | `.run` | Resume an existing Claude conversation |
| 5 | Resource-controlled run | `.run` | Cap token usage for cost control |
| 6 | Model selection | `.run` | Target a specific Claude model |
| 7 | Full pipeline | `.run` | Complex invocation with multiple options |
| 8 | Verbose preview | `.run` | Preview command on stderr, then execute |

**Total:** 8 workflows

---

### Workflow :: 1. Basic Prompt

Send a simple one-shot prompt to Claude in the current working directory.

```bash
claude_runner "Fix the bug in main.rs"
# [Claude output streamed to stdout]

claude_runner -m "Add unit tests for the parser"
# Equivalent using explicit --message flag
```

**Applicable when:** You want a quick Claude invocation without changing directory or tweaking defaults.

---

### Workflow :: 2. Project-Specific Execution

Run Claude Code inside a specific project directory, giving it full codebase context.

```bash
claude_runner "Refactor the auth module" --dir ~/projects/my_app
# Claude Code starts with working directory set to ~/projects/my_app
# It can read, edit, and run tools relative to that directory

claude_runner "Write a readme" -d /home/user/code/api_server
# Equivalent using -d short form
```

**Applicable when:** The project you want Claude to work on is not the shell's current directory.

---

### Workflow :: 3. Dry-Run Preview

Preview the exact `claude` invocation that would run, without executing it. Useful for debugging or auditing parameters.

```bash
claude_runner "Fix the tests" --dir /project --max-tokens 50000 --dry-run
# CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000
# [… other env vars …]
# cd /project
# claude "Fix the tests"
# (No Claude process is started)

claude_runner --dry-run
# CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
# [… other env vars …]
# claude
# (Shows minimal invocation — no message, no flags)
```

**Applicable when:** You want to verify parameter assembly before committing to a real API call, or when debugging unexpected behavior.

---

### Workflow :: 4. Session Continuation

Resume an ongoing Claude Code conversation to extend or follow up on previous work.

```bash
# Initial run (starts a new conversation)
claude_runner "Implement the login endpoint" --dir ~/project

# Follow-up run (continues same conversation)
claude_runner "Now add input validation to the endpoint" --dir ~/project --continue
# Claude resumes with full context from the previous run

claude_runner -c "Explain what you just changed"
# Short form: -c is alias for --continue
```

**Applicable when:** You have an iterative workflow where each Claude invocation builds on the previous one, such as multi-step refactoring or incremental feature development.

---

### Workflow :: 5. Resource-Controlled Run

Limit maximum output tokens to cap response length and control API costs.

```bash
claude_runner "Summarize this codebase" --max-tokens 2000
# Claude stops after 2000 tokens even if response is incomplete

claude_runner "Fix all TODOs" --dir ~/project --max-tokens 100000
# Allows up to 100k tokens for large tasks
# Default is 200000 if --max-tokens is omitted
```

**Applicable when:** Running Claude on tasks where response length is predictable (summaries, short explanations) or when operating under a strict token budget.

---

### Workflow :: 6. Model Selection

Target a specific Claude model version for reproducibility or capability requirements.

```bash
claude_runner "Review this PR" --model claude-opus-4-6
# Uses Opus (most capable) for complex review tasks

claude_runner "Fix the typo" --model claude-haiku-4-5-20251001
# Uses Haiku (fastest, most economical) for trivial tasks

claude_runner "Refactor this function" --model claude-sonnet-4-6 --dir ~/project
# Sonnet for balanced capability/cost
```

**Applicable when:** You need a specific model for cost optimization, capability requirements, or reproducibility across runs.

---

### Workflow :: 7. Full Pipeline

Combine multiple parameters for a complete, fully-configured invocation. Preview with `--dry-run` first.

```bash
# Preview first
claude_runner "Implement OAuth callback handler" \
  --dir ~/projects/auth_service \
  --model claude-opus-4-6 \
  --max-tokens 150000 \
  --session-dir ~/.claude_sessions/auth_project \
  --skip-permissions \
  --continue \
  --dry-run
# CLAUDE_CODE_MAX_OUTPUT_TOKENS=150000
# [… other env vars …]
# CLAUDE_CODE_SESSION_DIR=~/.claude_sessions/auth_project
# cd ~/projects/auth_service
# claude -c --dangerously-skip-permissions --model claude-opus-4-6 "Implement OAuth callback handler"

# Execute
claude_runner "Implement OAuth callback handler" \
  --dir ~/projects/auth_service \
  --model claude-opus-4-6 \
  --max-tokens 150000 \
  --session-dir ~/.claude_sessions/auth_project \
  --skip-permissions \
  --continue
# [Claude output]
```

**Applicable when:** Running Claude in automated or CI/CD pipelines where full parameter control is needed and interactive prompts must be suppressed.

---

### Workflow :: 8. Verbose Preview

Print the assembled command to stderr for auditing, then execute immediately — no separate dry-run step needed.

```bash
claude_runner "Fix the bug in auth.rs" --dir ~/projects/app --verbose
# (stderr) CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
# (stderr) [… other env vars …]
# (stderr) cd /home/user/projects/app
# (stderr) claude "Fix the bug in auth.rs"
# [Claude output streamed to stdout]

claude_runner "Run tests" --dir ~/project --model claude-opus-4-6 --verbose
# (stderr) CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
# (stderr) [… other env vars …]
# (stderr) cd /home/user/project
# (stderr) claude --model claude-opus-4-6 "Run tests"
# [Claude output streamed to stdout]
```

**Applicable when:** You want to verify the assembled invocation without a two-step dry-run → real-run workflow. The preview appears on stderr so Claude's stdout output remains uncontaminated and pipeable.
