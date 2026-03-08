# CLI Comparison Framework

Comparison of `claude_runner` with other CLI tools and frameworks, highlighting design choices and tradeoffs.

## Table of Contents

- [Design Comparison](#design-comparison) — Architecture and approach comparisons
- [Tool Comparisons](#tool-comparisons) — Feature-by-feature comparison with CLIs
- [Framework Comparisons](#framework-comparisons) — CLI framework evaluations
- [Recommendation Matrix](#recommendation-matrix) — Decision criteria for CLI design

---

## Design Comparison

### Comparison 1: Command Routing Approaches

| Approach | Description | Example Tools | Pros | Cons |
|-----------|-------------|---------------|------|-------|
| **Flag-based routing** | Commands selected by flags (e.g., `--help`) | claude_runner (current) | Familiar syntax; simple for single command | Poor extensibility; semantic confusion |
| **Explicit commands** | Commands are first-class arguments | genfile (unilang) | Clear structure; easy to extend | Less familiar to git users |
| **Subcommands** | Commands organized hierarchically | git, docker, kubectl | Natural grouping; consistent | More complex for simple CLIs |

**claude_runner position:** Hybrid (flag-based + explicit commands) — Phase 1 adds explicit `.help`/`.run` while keeping `--help` flag.

### Comparison 2: Parameter Syntax

| Syntax | Description | Example Tools | Pros | Cons |
|---------|-------------|---------------|------|-------|
| **`--flag value`** | POSIX-style long flags with values | claude_runner, cargo, npm | Familiar; widely understood | Verbose for common use |
| **`key::value`** | Double-colon syntax | unilang-based tools | Type-safe; framework handles parsing | Requires learning new syntax |
| **`-f value`** | Short flags with values | claude_runner, cargo, docker | Concise; common patterns | Can conflict with long flags |
| **Positional** | Values without flags | git, docker | Simple for main argument | Ambiguous with other values |

**claude_runner position:** Adapter layer converts `--flag value` to `key::value` for user familiarity + framework power.

### Comparison 3: Help Systems

| Approach | Description | Example Tools | Pros | Cons |
|-----------|-------------|---------------|------|-------|
| **Flag-based help** | Help triggered by `--help` or `-h` | claude_runner (current), cargo, npm | Easy to discover; no new syntax | Mixing concerns (help as flag vs command) |
| **Command-based help** | `.help` or `help` as explicit command | genfile, kubectl, docker | Clear separation; easier to extend | Must remember to type `.help` |
| **Built-in commands** | `help` as first argument | aws-cli, gcloud | Natural for users; discoverable | Different from other commands |

**claude_runner position:** Phase 1: dual support (`.help` command + `--help` flag) → Phase 2: deprecate → Phase 3: remove flag.

---

## Tool Comparisons

### Comparison: claude_runner vs cargo

| Feature | claude_runner | cargo |
|---------|-------------|--------|
| **Primary purpose** | Claude Code execution | Rust package management |
| **Command structure** | `.run` | `build`, `check`, `test`, `publish`, `doc`, etc. |
| **Command routing** | Flag-based + adapter | Subcommand-based |
| **Parameter syntax** | `--flag value` | `--flag value` |
| **Help** | `--help` flag (dual support) | `--help` flag |
| **Extensibility** | Planned explicit commands | Rich subcommand system |
| **Configuration** | Environment variables | Environment variables + `.cargo/config.toml` |

**Key difference:** `claude_runner` has single primary command with 9 parameters, while `cargo` has 20+ commands with many parameters each.

### Comparison: claude_runner vs git

| Feature | claude_runner | git |
|---------|-------------|------|
| **Primary purpose** | Claude Code execution | Version control |
| **Command structure** | `.run` | `status`, `log`, `commit`, `push`, `pull`, etc. |
| **Command routing** | Flag-based + adapter | Subcommand-based |
| **Parameter syntax** | `--flag value` | `--flag value` |
| **Positional args** | Yes (message) | Yes (paths, refs, etc.) |
| **Help** | `--help` flag (dual support) | `git help <command>` |
| **Extensibility** | Planned explicit commands | Rich built-in command system |
| **Config files** | None | `.git/config` |

**Key difference:** `git` demonstrates subcommand model that `claude_runner` is moving toward.

### Comparison: claude_runner vs docker

| Feature | claude_runner | docker |
|---------|-------------|--------|
| **Primary purpose** | Claude Code execution | Container management |
| **Command structure** | `.run` | `run`, `build`, `ps`, `exec`, etc. |
| **Command routing** | Flag-based + adapter | Subcommand-based |
| **Parameter syntax** | `--flag value` | `--flag value` |
| **Positional args** | Yes (message) | Sometimes (image name, container name) |
| **Help** | `--help` flag | `--help` flag |
| **Extensibility** | Planned explicit commands | Rich command system |
| **Context management** | Sessions (planned) | Containers |

**Key difference:** Both are single-primary tools with adapter layers, but `docker` has much richer command set.

### Comparison: claude_runner vs kubectl

| Feature | claude_runner | kubectl |
|---------|-------------|--------|
| **Primary purpose** | Claude Code execution | Kubernetes control |
| **Command structure** | `.run` | `get`, `describe`, `apply`, `delete`, `logs`, etc. |
| **Command routing** | Flag-based + adapter | Subcommand-based |
| **Parameter syntax** | `--flag value` | `--flag value` |
| **Help** | `--help` flag | `kubectl help` |
| **Extensibility** | Planned explicit commands | Very rich command system |
| **Configuration** | None | `~/.kube/config` |

**Key difference:** `kubectl` shows mature subcommand model with extensive command set.

---

## Framework Comparisons

### Comparison: unilang vs wca

| Feature | unilang | wca |
|---------|---------|------|
| **Purpose** | Universal CLI framework | CLI aggregation framework |
| **Command names** | `.command` (dot-prefixed) | `command` (no prefix) |
| **Parameter syntax** | `key::value` | `key::value` |
| **Routing** | Parser → Semantic → Interpreter | Parser → Dispatcher → Handler |
| **Extensibility** | Binary apps only | Supports external binaries and functions |
| **Registration** | Runtime registration | YAML-based command discovery |
| **Help generation** | Auto-help from command definitions | Built-in help system |
| **Learning curve** | Higher (full framework) | Lower (simpler model) |

**claude_runner choice:** `unilang` provides richer validation and semantic analysis needed for robust CLI.

### Comparison: unilang vs clap

| Feature | unilang | clap |
|---------|---------|------|
| **Purpose** | Semantic CLI framework | CLI parsing library |
| **Command names** | `.command` (dot-prefixed) | Any (no prefix requirement) |
| **Parameter syntax** | `key::value` | `--flag value` |
| **Routing** | Parser → Semantic → Interpreter | Parser → Arguments → App code |
| **Extensibility** | Binary apps only | Binary and library apps |
| **Registration** | Runtime registration | Derive-based (macro) |
| **Help generation** | Auto-help from command definitions | Auto-help from derive attributes |
| **Validation** | Kind-based type checking | Attribute-based validation |
| **Learning curve** | Higher (full framework) | Higher (but many features) |
| **Performance** | Good (lazy evaluation) | Excellent (compiled parser) |

**claude_runner choice:** `unilang` provides semantic validation and command registry that `clap` lacks, at cost of higher learning curve.

---

## Recommendation Matrix

### Criteria for CLI Framework Selection

| Criterion | Weight | unilang | wca | clap | Homegrown |
|-----------|--------|---------|------|-----------|
| **Semantic validation** | High | Medium | Low | Low (manual) |
| **Type safety** | High | Low | Low | Low (manual) |
| **Extensibility** | High | High | High | Variable |
| **Help generation** | High | High | High | Variable (manual) |
| **Learning curve** | Medium | Medium | High | Variable |
| **Performance** | Medium | High | High | Variable |
| **Community** | Medium | Low | High | High |
| **Maintenance** | Medium | Medium | Low | High (you own bugs) |
| **Documentation** | Medium | Medium | High | High (you write it) |

**Recommendation for claude_runner:** Use `unilang` framework. The semantic validation and command registry features justify the learning curve.

### Criteria for Command Design

| Criterion | Explicit Commands (Recommended) | Flag-Based Routing (Current) |
|-----------|----------------------------------|---------------------------|
| **Semantic clarity** | High | Medium |
| **Extensibility** | High | Low |
| **User familiarity** | Medium | High |
| **Implementation complexity** | Low | Medium |
| **Backward compatibility** | Low | High |

**Recommendation for claude_runner:** Move to explicit commands with 3-phase migration (dual support → deprecation → removal).

### Criteria for Parameter Syntax

| Criterion | `--flag value` | `key::value` |
|-----------|----------------|--------------|
| **User familiarity** | High | Low |
| **Tool support** | High (POSIX standard) | Low (custom parsing) |
| **Explicit typing** | Low (string only) | High (Kind-based) |
| **Type safety** | Low | High (framework validation) |
| **Composability** | High (pipes with values) | Low (single command) |
| **Verbosity** | High | Medium |

**Recommendation for claude_runner:** Keep `--flag value` with adapter layer. This gives user familiarity while leveraging unilang's power.

---

## Tradeoff Analysis

### Tradeoff 1: Simplicity vs Power

**Simplicity-first** (current approach):
- Pros: Easy to learn, minimal setup
- Cons: Hard to extend, semantic confusion, ad-hoc code

**Power-first** (explicit commands):
- Pros: Extensible, clear semantics, better organization
- Cons: More complex setup, higher learning curve

**Recommendation:** Hybrid approach (explicit commands optional, implicit default) — balances both concerns.

### Tradeoff 2: Familiarity vs Correctness

**Familiarity-first** (POSIX flags):
- Pros: Users already know the syntax
- Cons: Ambiguities, limitations of POSIX model

**Correctness-first** (semantic framework):
- Pros: Clear intent, type safety, better error messages
- Cons: New syntax to learn

**Recommendation:** Adapter layer provides both — familiar syntax internally converted to semantic model.

### Tradeoff 3: Verbosity vs Brevity

**Verbose flags** (`--max-tokens`):
- Pros: Self-documenting, no guessing required
- Cons: Longer commands

**Brevity conventions** (defaults baked in):
- Pros: Concise, quick to type
- Cons: May need docs for defaults

**Recommendation:** Explicit flags for critical options, good defaults for common cases.

---

## Migration Path Analysis

### Path A: Keep Current Design

**When to stay:**
- Simple CLI with single primary command
- User base familiar with POSIX flags
- No extensibility needed
- Quick development iterations

**Examples:** Single-purpose tools where user understands the domain well.

### Path B: Add Explicit Commands

**When to adopt:**
- Multiple commands or subcommands needed
- Clear separation of concerns
- Future extensibility required

**Migration cost:** Medium (new patterns, docs, tests)

**Benefits:** Better organization, easier to understand, clear command boundaries.

### Path C: Full Framework Migration

**When to adopt:**
- Need advanced features (subcommand groups, plugins)
- Rich help system required
- Complex parameter interactions needed

**Migration cost:** High (rewrite, extensive testing, docs)

**Benefits:** Rich feature set, professional help, better error messages.

---

## Decision Framework

### Step 1: Define Requirements

Create checklist of CLI requirements:

**Functional Requirements:**
- [ ] Primary use case(s)
- [ ] Required commands
- [ ] Required parameters
- [ ] Optional parameters
- [ ] Output format requirements
- [ ] Configuration needs

**Non-Functional Requirements:**
- [ ] User expertise level (beginner, intermediate, expert)
- [ ] Learning curve tolerance
- [ ] Backward compatibility requirements
- [ ] Performance requirements
- [ ] Security requirements
- [ ] Extensibility needs

### Step 2: Evaluate Options

Score each option against requirements:

**Option: Keep current design**
**Option: Add explicit commands to current design**
**Option: Full framework migration (clap, wca, etc.)**
**Option: Homegrown solution**

### Step 3: Make Decision

Choose option with best fit based on requirements and constraints.

---

## Summary

| Aspect | claude_runner Current | Recommended |
|---------|-------------------|-------------|
| **Framework** | unilang (good) | unilang (keep) |
| **Commands** | Implicit `.run` | Explicit `.help`/`.run` |
| **Parameters** | `--flag value` | Keep `--flag value` |
| **Help** | `--help` flag (dual support) | Deprecate → Remove |
| **Extensibility** | Planned | Enabled |
| **Migration** | 3-phase path | Follow plan |
| **Validation** | Framework-provided | Framework-provided |
| **Documentation** | Comprehensive | Comprehensive |

---

## References

- [Unilang Exploration](unilang_exploration.md) — Framework details
- [Command Design](command_design.md) — Design recommendations
- [Migration Guide](migration_guide.md) — Step-by-step migration path
- [Examples](examples.md) — Real-world usage patterns
