# Parameters

| # | Parameter | Type | Default | Commands | Purpose | Status |
|---|-----------|------|---------|----------|---------|--------|
| 1 | `[MESSAGE]` / `-m` / `--message` | [`MessageText`](types.md#type--messagetext) | — | 1 | Prompt message for Claude | ✅ |
| 2 | `-d` / `--dir` | [`PathArg`](types.md#type--patharg) | current dir | 1 | Working directory for Claude Code | ✅ |
| 3 | `-c` / `--continue` | `bool` | false | 1 | Continue an existing conversation | ✅ |
| 4 | `--max-tokens` | [`TokenCount`](types.md#type--tokencount) | 200000 | 1 | Maximum output tokens | ✅ |
| 5 | `--skip-permissions` | `bool` | false | 1 | Skip tool permission prompts | ✅ |
| 6 | `--dry-run` | `bool` | false | 1 | Print command without executing | ✅ |
| 7 | `--session-dir` | [`PathArg`](types.md#type--patharg) | — | 1 | Session storage directory | ✅ |
| 8 | `--model` | [`ModelName`](types.md#type--modelname) | — | 1 | Claude model to use | ✅ |
| 9 | `--verbose` / `-v` | `bool` | false | 1 | Print assembled command to stderr, then execute | ✅ |

**Total:** 9 parameters

---

### Parameter :: 1. `message::`

Free-form text prompt sent to Claude as the task description. Use this to provide the instruction or question you want Claude to act on.

- **Type:** [`MessageText`](types.md#type--messagetext) (newtype)
- **Default:** — (omitting starts Claude Code without a pre-supplied prompt)
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** Positional (first non-flag argument), `-m`, `--message`
- **Purpose:** Defines the primary instruction for the Claude Code invocation; forwarded verbatim to the `claude` process as the last argument.
- **Interaction:** Positional and `--message` are mutually exclusive — supplying both is a parse error.
- **Group:** [Input](parameter_groups.md#group--1-input)

---

### Parameter :: 2. `dir::`

Filesystem directory where Claude Code executes its session. Use this to point Claude at a specific project without first `cd`-ing into it.

- **Type:** [`PathArg`](types.md#type--patharg) (newtype)
- **Default:** Current working directory of the invoking shell
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** `-d`, `--dir`
- **Purpose:** Prepends `cd /path` to the assembled command (via `with_working_directory()`), so the invocation shell changes into the given directory before running `claude`.
- **Group:** [Environment](parameter_groups.md#group--2-environment)

---

### Parameter :: 3. `continue::`

Boolean flag that resumes an existing Claude Code conversation rather than starting a new one. Use this for multi-step iterative workflows where context from the previous run is needed.

- **Type:** `bool`
- **Default:** false
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** `-c`, `--continue`
- **Purpose:** Passes `-c` to the `claude` process, which loads the most recent conversation session from the session directory.
- **Group:** [Behavior Flags](parameter_groups.md#group--3-behavior-flags)

---

### Parameter :: 4. `max_tokens::`

Maximum number of tokens Claude may output in a single response. Use this to cap response length for cost control or to enforce brevity.

- **Type:** [`TokenCount`](types.md#type--tokencount) (newtype)
- **Default:** 200000
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** `--max-tokens`
- **Valid Values:** Any u32 (0–4294967295)
- **Purpose:** Passes `--max-tokens N` to the `claude` process; Claude stops generating once the limit is reached, which may result in a truncated response.
- **Group:** [Resource Control](parameter_groups.md#group--4-resource-control)

---

### Parameter :: 5. `skip_permissions::`

Boolean flag that bypasses Claude Code's interactive tool-permission prompts. Use this in automated or non-interactive environments where stdin is unavailable.

- **Type:** `bool`
- **Default:** false
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** `--skip-permissions`
- **Purpose:** Passes `--dangerously-skip-permissions` to the `claude` process, allowing tools (file edits, shell commands, etc.) to execute without user confirmation.
- **Notes:** Use with care in sensitive environments — skipping permission prompts allows unrestricted tool execution.
- **Group:** [Behavior Flags](parameter_groups.md#group--3-behavior-flags)

---

### Parameter :: 6. `dry::`

Boolean flag that suppresses execution and instead prints the environment variables and full command line that would be passed to `claude`. Use this to audit or debug parameter assembly.

- **Type:** `bool`
- **Default:** false
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** `--dry-run`
- **Purpose:** Outputs `describe_env()` (environment variables like `CLAUDE_CODE_MAX_OUTPUT_TOKENS`) then `describe()` (the full `claude` invocation string), then exits with code 0; no API call is made.
- **Group:** [Behavior Flags](parameter_groups.md#group--3-behavior-flags)

---

### Parameter :: 7. `session_dir::`

Directory where Claude Code stores its conversation session files. Use this to persist sessions in a project-specific location for later continuation with `--continue`.

- **Type:** [`PathArg`](types.md#type--patharg) (newtype)
- **Default:** — (Claude Code uses its own default session location)
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** `--session-dir`
- **Purpose:** Sets the `CLAUDE_CODE_SESSION_DIR` environment variable so Claude Code stores and reads session state from the specified directory.
- **Group:** [Environment](parameter_groups.md#group--2-environment)

---

### Parameter :: 8. `model::`

Claude model identifier selecting which model to use for this invocation. Use this to override the default model for cost optimization, capability requirements, or reproducibility.

- **Type:** [`ModelName`](types.md#type--modelname) (newtype)
- **Default:** — (Claude Code uses its own default model)
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** `--model`
- **Valid Values:** Any Claude model identifier; e.g. `claude-opus-4-6`, `claude-sonnet-4-6`, `claude-haiku-4-5-20251001`
- **Purpose:** Passes `--model NAME` to the `claude` process; unknown model names are rejected by Claude Code at startup with an error message.
- **Group:** [Resource Control](parameter_groups.md#group--4-resource-control)

---

### Parameter :: 9. `verbose::`

Boolean flag that prints the assembled environment variables and command line to **stderr** before executing. Use this to audit the exact invocation without separating preview and execution into two steps.

- **Type:** `bool`
- **Default:** false
- **Commands:** [`.run`](commands.md#command--1-run)
- **Aliases:** `-v`, `--verbose`
- **Purpose:** Immediately before calling `builder.execute()`, writes `describe_env()` then `describe()` to stderr. Execution proceeds normally after printing. Output goes to stderr so Claude's real stdout remains uncontaminated.
- **Interaction:** Superseded by `dry::` — when both `--verbose` and `--dry-run` are set, `--dry-run` wins and `--verbose` is a no-op (the dry-run stdout path exits before the verbose `eprintln!` is reached).
- **Group:** [Behavior Flags](parameter_groups.md#group--3-behavior-flags)
- **FR:** FR-12
