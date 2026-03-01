# Types

Type system for `claude_runner` CLI parameters.

| # | Type | Kind | Parameters |
|---|------|------|------------|
| 1 | `MessageText` | newtype (String) | message |
| 2 | `PathArg` | newtype (String) | dir, session_dir |
| 3 | `TokenCount` | newtype (u32) | max_tokens |
| 4 | `ModelName` | newtype (String) | model |

**Total:** 4 types

---

### Type :: `MessageText`

Free-form text passed to Claude as the task description or instruction. Use this when providing any natural-language prompt, question, or directive to Claude.

- **Fundamental Type:** String
- **Constraints:** Non-empty; no structural restrictions on content or length
- **Parsing:** Accepted as a positional argument or via `-m`/`--message`; spaces within the value are preserved as-is because the adapter passes it as a single token to unilang
- **Used by:** [`message::`](params.md#parameter--1-message)

**Examples:**

```bash
# Valid
"Fix the bug in main.rs"
"Explain this code"
"Add unit tests for the parser"

# Invalid
""          # empty string is rejected by Claude Code at runtime
```

---

### Type :: `PathArg`

Filesystem path (absolute or relative) pointing to a directory. Used for the working directory and session storage location passed to Claude Code.

- **Fundamental Type:** String
- **Constraints:** Must be a syntactically valid path string; directory existence is validated by Claude Code at runtime, not by the adapter
- **Parsing:** Accepted as-is from argv; the shell expands `~` before the value reaches `claude_runner`
- **Used by:** [`dir::`](params.md#parameter--2-dir), [`session_dir::`](params.md#parameter--7-session_dir)

**Examples:**

```bash
# Valid
/home/user/project
~/code/my_app
./relative/path
.

# Runtime-rejected (Claude Code validates these, not the adapter)
/nonexistent/directory
```

---

### Type :: `TokenCount`

Unsigned 32-bit integer representing the maximum number of tokens Claude may output. Limits response length to control cost and latency.

- **Fundamental Type:** u32
- **Constraints:** Decimal integer ≥1; must fit in u32 range (1–4294967295); parsed from string before being passed to unilang as `Kind::Integer`
- **Default:** 200000
- **Parsing:** Decimal string only; hex, floats, or negative values are rejected with `invalid --max-tokens value: <input>`
- **Used by:** [`max_tokens::`](params.md#parameter--4-max_tokens)

**Examples:**

```bash
# Valid
1
1000
200000      # default
4294967295  # u32::MAX

# Invalid
0           # zero not meaningful (maps to no output)
-1          # negative not accepted
abc         # non-numeric rejected
1.5         # float rejected
```

---

### Type :: `ModelName`

String identifier selecting a Claude model by its API name. Allows targeting a specific model version for reproducibility, cost optimization, or capability requirements.

- **Fundamental Type:** String
- **Constraints:** Non-empty; model existence is validated by Claude Code at startup, not by the adapter
- **Parsing:** Accepted as-is; passed directly to `claude --model NAME`
- **Used by:** [`model::`](params.md#parameter--8-model)

**Examples:**

```bash
# Known valid values (as of March 2026)
claude-opus-4-6
claude-sonnet-4-6
claude-haiku-4-5-20251001

# Invalid at runtime (rejected by Claude Code)
""                  # empty
nonexistent-model   # unknown identifier
```
