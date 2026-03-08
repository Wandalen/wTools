# Parameter Groups

Semantic groupings of `.run` parameters by concern. `claude_runner` has a single non-trivial command, so groups serve as organizational documentation rather than cross-command deduplication.

| # | Group | Parameters | Purpose |
|---|-------|------------|---------|
| 1 | Input | message | What to send Claude |
| 2 | Environment | dir, session_dir | Where Claude executes |
| 3 | Behavior Flags | continue, skip_permissions, dry, verbose | How to execute |
| 4 | Resource Control | max_tokens, model | What resources to use |

**Total:** 4 groups

---

### Group :: 1. Input

Defines the content of the Claude Code invocation. A message is optional — Claude Code can run interactively without one.

- **Parameters:**
  - [`message::`](params.md#parameter--1-message) — Prompt text sent to Claude as the task description
- **Commands using this group:** [`.run`](commands.md#command--1-run)
- **Coherence validation:** All parameters in this group control *what* is communicated to Claude. A second parameter (e.g. attachment path) would belong here once added.

---

### Group :: 2. Environment

Controls where Claude Code executes and where it persists session state. These parameters define the filesystem context Claude sees.

- **Parameters:**
  - [`dir::`](params.md#parameter--2-dir) — Working directory Claude Code operates in
  - [`session_dir::`](params.md#parameter--7-session_dir) — Directory for session storage files
- **Commands using this group:** [`.run`](commands.md#command--1-run)
- **Coherence validation:** All parameters in this group control *where* Claude runs or stores data. `dir` prepends a `cd /path` shell prefix; `session_dir` sets `CLAUDE_CODE_SESSION_DIR`; both shape the filesystem context the `claude` process sees.

---

### Group :: 3. Behavior Flags

Boolean switches that alter execution mode without affecting the prompt content or environment. All default to false (disabled).

- **Parameters:**
  - [`continue::`](params.md#parameter--3-continue) — Resume an existing conversation instead of starting fresh
  - [`skip_permissions::`](params.md#parameter--5-skip_permissions) — Bypass interactive tool permission prompts
  - [`dry::`](params.md#parameter--6-dry) — Print command without executing it
  - [`verbose::`](params.md#parameter--9-verbose) — Print command to stderr, then execute normally
- **Commands using this group:** [`.run`](commands.md#command--1-run)
- **Coherence validation:** All parameters in this group are boolean flags that modify execution behavior. They are orthogonal — any combination is valid. `dry` short-circuits execution entirely (supersedes `verbose`); `continue` and `skip_permissions` apply only when executing; `verbose` adds stderr preview without affecting the execution path.

---

### Group :: 4. Resource Control

Governs the model selection and resource budget for the Claude invocation. These parameters directly affect cost, latency, and output capability.

- **Parameters:**
  - [`max_tokens::`](params.md#parameter--4-max_tokens) — Maximum output tokens (caps response length)
  - [`model::`](params.md#parameter--8-model) — Claude model identifier
- **Commands using this group:** [`.run`](commands.md#command--1-run)
- **Coherence validation:** All parameters in this group control *what model runs* and *how much it can generate*. A `temperature` or `top_p` parameter would belong here once added.
