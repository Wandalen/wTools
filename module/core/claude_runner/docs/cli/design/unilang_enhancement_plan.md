# Unilang Enhancement Plan — Comprehensive Analysis

## Executive Summary

**Objective:** Extend unilang crate to support plugin system, configuration files, enhanced error messaging, REPL mode, and async command execution.

**Estimated Effort:** 18 weeks (4.5 person-months)
**Risk Level:** Medium — Breaking changes to core APIs, requires migration strategy

**Strategic Alignment:**
- Addresses extensibility needs for `claude_runner` and `genfile`
- Provides foundation for future CLI enhancements (REPL mode, plugin system)
- Aligns with modern CLI development best practices (type safety, clear error messages)

---

## Phase 1: Plugin System (3 weeks)

### 1.1.1 Core Architecture

**Goal:** Enable dynamic command loading from external sources.

**Current State:**
```rust
// All commands are statically registered in build_registry()
// No runtime registration capability
```

**Proposed State:**
```rust
// Dynamic command discovery and loading
pub struct CommandRegistry {
    // ... existing static commands
    loaded_commands: Vec<Box<dyn CommandSource>>,  // NEW
    sources: Vec<Box<dyn CommandSource>>,
}

pub trait CommandSource {
    fn name(&self) -> &'static str;
    fn commands(&self) -> Vec<CommandDefinition>;
    fn can_reload(&self) -> bool;
    fn load(&mut self, registry: &mut CommandRegistry) -> Result<(), String>;
}
```

**Design Decision:** Use trait-based plugin system for maximum flexibility.

**Alternative Considered:**
- Registry-based with strings (simpler, but less flexible)
- Direct file loading (harder to implement hot-reload)

**Decision Rationale:** Trait-based approach provides:
- Clear contract for command sources
- Support for multiple source types (files, memory, network)
- Hot-reload capability
- Clear separation of concerns

**Migration Strategy:**
```rust
// Phase 1: Add trait to CommandRegistry (non-breaking)
pub trait CommandSource {
    fn name(&self) -> &'static str;
    fn commands(&self) -> Vec<CommandDefinition>;
    fn load(&mut self, registry: &mut CommandRegistry) -> Result<(), String>;
    fn can_reload(&self) -> bool;
}

// Phase 2: Add CommandRegistry::load_sources() method
impl CommandRegistry {
    pub fn load_sources(&mut self, sources: Vec<Box<dyn CommandSource>>) -> Result<(), String> {
        for source in &self.sources {
            source.load(&mut self.registry)?;
        }
        self.sources = sources;  // Update to loaded set
        Ok(())
    }
}

// Phase 3: Update Pipeline to use new methods
// No breaking changes to existing code
```

### 1.1.2 Source Implementations

**FileSource (files/YAML):**
```rust
pub struct FileSource {
    config_path: PathBuf,

    pub fn new(config_path: PathBuf) -> Self {
        Self {
            config_path,
            watch: RecommendedWatcher::new().expect("watcher failed"),
        }
    }

    fn name(&self) -> &'static str {
        "file"
    }

    fn commands(&self) -> Vec<CommandDefinition> {
        // Load and parse YAML files
    }

    fn load(&mut self, registry: &mut CommandRegistry) -> Result<(), String> {
        let yaml = fs::read_to_string(&self.config_path)?;
        let config = UnilangConfig::from_str(&yaml)?;

        for cmd_def in &config.commands {
            registry.command_add_runtime(&cmd_def, handler)?;
        }

        Ok(())
    }
    }
}
```

**DirectorySource (scan directories):**
```rust
pub struct DirectorySource {
    directory: PathBuf,

    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    fn commands(&self) -> Vec<CommandDefinition> {
        // Scan directory for `.command.yaml` files
        // Register found commands
    }

    fn load(&mut self, registry: &mut CommandRegistry) -> Result<(), String> {
        for entry in fs::read_dir(&self.directory)? {
            if entry.path().extension() == "yaml" {
                let yaml = fs::read_to_string(entry.path())?;
                let config = UnilangConfig::from_str(&yaml)?;

                for cmd_def in &config.commands {
                    registry.command_add_runtime(&cmd_def, handler)?;
                }
            }
        }

        Ok(())
    }
    }
}
```

**MemorySource (in-memory registration):**
```rust
pub struct MemorySource {
    commands: Vec<CommandDefinition>,

    pub fn new(commands: Vec<CommandDefinition>) -> Self {
        Self { commands }
    }

    fn commands(&self) -> &Vec<CommandDefinition> {
        &self.commands
    }

    fn load(&mut self, registry: &mut CommandRegistry) -> Result<(), String> {
        for cmd_def in &self.commands {
            registry.command_add_runtime(&cmd_def, handler)?;
        }

        Ok(())
    }
    }
}
```

### 1.1.3 Implementation Considerations

**FileSource complexity:**
- Watcher integration for hot-reload
- YAML parsing error handling
- Path resolution for relative paths

**Complexity:** MEDIUM — ~500 LOC for trait definitions + implementations

**Testing Strategy:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_source_basic() {
        let mut registry = CommandRegistry::new();
        let config = UnilangConfig::from_str("commands:\n  .test \"test:\n    description: 'Test command'\n    arguments:\n      - name: String::new(\"test\")\n      ").unwrap();
        let handler = |cmd, _ctx| Ok(OutputData::empty(), _ctx);

        let mut registry = CommandRegistry::new();
        let source = FileSource::new(PathBuf::from("test.yaml").unwrap());
        source.load(&mut registry).unwrap();
        assert_eq!(source.commands()[0].name, "test");
    }

    #[test]
    fn directory_source_scanning() {
        // Test scanning for command files
        // Verify only `.yaml` files are loaded
    }
}
```

### 1.1.4 Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-------------|--------|-------------|
| **Watcher integration** | Medium | Inotify issues on macOS | Use cross-platform file watcher library |
| **YAML parsing errors** | Low | Clear error messages | Schema validation |
| **Path resolution** | Low | Use canonical paths | Document behavior |
| **Breaking changes** | Medium | New public methods in `CommandRegistry` | Existing code unaffected |

---

## Phase 2: Configuration Files (2 weeks)

### 2.1.1 Configuration File Format

**Goal:** Support `.unilang.yaml` configuration for CLI behavior and defaults.

**Proposed Schema:**
```yaml
version: 1.0

# Default command behavior
default_command:
  implicit: ".run"  # Command to use when none specified

# Plugin discovery
plugins:
  paths:
    - ./plugins/commands/
    - ./plugins/memory/
  auto_discovery: true  # Auto-scan for command files
  hot_reload: true     # Watch for changes and reload

# Error handling
error_display:
  style: "detailed"          # "brief", "detailed", or "json"
  include_context: true         # Include context in error messages
  include_suggestions: true   # Offer fix suggestions
  color_output: "auto"           # "auto", "always", "never"

# Performance
startup_timeout_ms: 5000     # 5 second timeout for large configs
watch_debounce_ms: 100        # 100ms debounce for file changes

# Command defaults
defaults:
  verbosity: 1                # Default verbosity level
  max_tokens: 200000         # Default max tokens
  dry_run: false             # Dry-run off
  timeout_ms: 300000         # Default command timeout

# REPL behavior
repl:
  prompt: "{tool}> "         # Custom prompt
  history_file: "~/.{tool}_history"
  history_size: 1000          # Max 1000 commands in history
  completion_mode: "fuzzy"     # "fuzzy" or "exact"
```

### 2.1.2 Implementation Architecture

```
┌─────────────────────────────────┐
│  ConfigLoader                    │
│  ├─> YAMLParser                  │
│  ├─> SchemaValidator              │
│  └─> PluginManager               │
├────────┬────────────────┤         └───────┬───────────────────┘
            ↓                    ↓                    ↓
┌─────────────────────────────────┐
│     CommandRegistry              │     ┌─────────────────────────┐    │
│     ├─> StaticCommands           │     │  ConfigLoader         │    │
│     ├─> DynamicCommands          │     │ PluginManager         │    │
│     └─> ConfigManager              │     └─────────────────────────┘    │
└───────────────────────────────┘                              └───────────┘
                         ↓                    ↓                    ↓
                 CommandRegistry::new()         PluginManager::load()     Pipeline::new()
                         ↓                    ↓                    ↓
                    process_command_from_argv()
```

### 2.1.3 Integration Points

```rust
// CommandRegistry integration
pub struct ConfigManager {
    registry: CommandRegistry,
    config: UnilangConfig,

    pub fn new(registry: CommandRegistry, config: UnilangConfig) -> Self {
        Self { registry, config }
    }

    pub fn apply_defaults(&mut self, ctx: &mut ExecutionContext) {
        // Apply verbosity, token limits, timeouts from config
        if let Some(verbosity) = self.config.get("defaults.verbosity") {
            ctx.set_verbosity_level(verbosity);
        }
    }
    }
}
```

### 2.1.4 Breaking Changes

**CommandRegistry additions (non-breaking):**
```rust
// Add source tracking field
pub struct CommandRegistry {
    // ... existing fields

    loaded_sources: HashSet<&'static str>,           // Track loaded source names
    sources: Vec<Box<dyn CommandSource>>,           // All available sources
}
```

**Pipeline modifications (non-breaking):**
```rust
// Add source parameter to commands
pub fn process_command_from_argv(
    &self,
    argv: &[String],
    sources: Vec<&'static str>,
) -> ExecutionResult
```

### 2.1.5 Testing Plan

```rust
#[cfg(test)]
mod plugin_tests {
    use super::*;

    // Test YAML config loading
    #[test]
    fn config_yaml_loading() {
        let yaml = "commands:\n  .test \"test:\n    description: 'Test command'\n    ";
        let config = UnilangConfig::from_str(yaml).unwrap();
        assert_eq!(config.commands[0].name, "test");
    }

    // Test multiple sources
    #[test]
    fn multiple_sources() {
        let sources: vec![
            Box::new(FileSource::new(...)),
            Box::new(DirectorySource::new(...)),
            Box::new(MemorySource::new(...)),
        ];
    }

    let mut registry = CommandRegistry::new();
    let mut config = UnilangConfig::default();

        for source in sources {
            source.load(&mut registry, &config).unwrap();
            assert!(sources.iter().all(|s| s.commands(&mut registry).is_empty()));
        }
    }
}
```

---

## Phase 3: Enhanced Error Messages (3 weeks)

### 3.1.1 Current ErrorData Limitations

**Problem:**
```rust
pub struct ErrorData {
    code: ErrorCode,
    message: String,
    // Missing: source, context, suggestion
}
```

Only `code` and `message` available. Cannot provide actionable suggestions or context.

**Proposed Enhancement:**

```rust
pub struct ErrorData {
    pub code: ErrorCode,
    pub message: String,
    pub source: Option<String>,           // NEW: File, command, argument where error occurred
    pub suggestion: Option<Suggestion>,    // NEW: Suggestion for fix
    pub context: Option<ErrorContext>,    // NEW: Contextual information
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    command: Option<String>,        // Command that caused error
    argument: Option<String>,       // Argument that caused error
    value: Option<Value>,           // Value that failed validation
}

#[derive(Debug, Clone)]
pub struct Suggestion {
    display_text: String,
    insert_text: String,
    kind: SuggestionKind,
}

pub enum SuggestionKind {
    Command,
    Argument,
    Correction,
    Hint,
    Warning,
}

pub enum ErrorCode {
    // ... existing codes ...

    #[error("CommandSourceNotFound")]
    CommandSourceNotFound {
        #[error_info("Plugin error: command source not found")]
        source: Option<String>,
        suggestion: Some(Suggestion::Hint(
            "Available sources: file, directory, memory",
        "Run '.config sources' to list available sources",
        )),
    },
}
```

### 3.1.2 Error Message Examples

**Before:**
```rust
Err(ErrorData::new(
    ErrorCode::ValidationError,
    "invalid --max-tokens value: -1"
))
```

**After:**
```rust
Err(ErrorData::new(
    ErrorCode::ValidationError,
    "invalid --max-tokens value: -1",
    Some(source),                    // Which source was processing this value
    Some("command_name"),              // Which command was being processed
    Some(suggestion),                // Actionable suggestion
    Some(ErrorContext {
        command: Some("command_name"),
        argument: Some("--max-tokens"),
        value: Some("-1"),
    }),
))
```

### 3.1.3 Implementation

**No breaking changes to `ErrorData`** — Add new optional fields using `#[non_exhaustive]` on struct definition.

**Testing:**
```rust
#[test]
fn enhanced_error_messages() {
    // Test source tracking
    let out = run_cli(&["--config", "sources", "reload"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Available sources:"), "Error context includes source");
}
```

---

## Phase 4: REPL Mode (4 weeks)

### 4.1.1 REPL Architecture

**Goal:** Interactive command execution with history and auto-completion.

**Proposed Components:**
```
┌─────────────────────────────────┐
│   ReplEngine                  │
│   ├─> PromptHandler          │
│   ├─> HistoryStore           │
│   ├─> Completer            │
│   ├─> StateManager           │
│   └─> ExecutionContext │
├────────────────────────────────┤         └───────┬───────────────────┘
            ↓                    ↓                    ↓                    ↓
```

**ReplEngine**
```rust
pub struct ReplEngine {
    prompt: String,
    history_store: HistoryStore,
    completer: Completer,
    state_manager: StateManager,
    ctx: ExecutionContext,
}

impl ReplEngine {
    pub fn execute_line(&mut self, input: &str) -> ReplResult {
        // Parse input
        let parsed = self.parse_input(input)?;

        // Add to history
        self.history_store.add(parsed.clone());

        // Handle exit commands
        if matches!(parsed.command.as_str(), "exit" | "quit") {
            return ReplResult::Exit;
        }

        // Route to pipeline
        let result = self.route_to_pipeline(parsed, &mut self.ctx);

        // Print prompt for next iteration
        self.print_prompt();
    }
}
}
```

### 4.1.2 Component: HistoryStore

```rust
pub struct HistoryStore {
    history: VecDeque<ReplHistoryEntry>,
    max_size: usize,

    pub fn new(max_size: usize) -> Self {
        Self { max_size }
    }

    pub fn add(&mut self, entry: ReplHistoryEntry) {
        self.history.push_back(entry.clone());
        if self.history.len() > self.max_size {
            self.history.pop_front();
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReplHistoryEntry {
    input: String,
    output: String,
    timestamp: DateTime<Local>,
}
```

### 4.1.3 Component: StateManager

```rust
pub struct StateManager {
    state: HashMap<String, Box<dyn std::any::Any + Send>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        state: state,
        }
    }

    pub fn set(&mut self, key: String, value: Box<dyn Any + Send>) {
        self.state.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&(dyn Any + Send)> {
        self.state.get(key).map(|v| v.as_ref())
    }
}
```

### 4.1.4 Integration with Pipeline

```rust
impl ReplEngine {
    fn route_to_pipeline(&self, parsed: ReplParsedInput, ctx: &mut ExecutionContext) -> ExecutionResult {
        let tokens = self.parsed_to_tokens(&parsed)?;
        Pipeline::new(self.registry).process_command(&tokens, ctx)
    }
}
```

### 4.1.5 REPL Session Workflow

```
User: claude_runner .repl
claude_runner>  Enter interactive mode

claude_runner> .status                # Check state
claude_runner> .set key value        # Set session variable
claude_runner> .history               # Show history
claude_runner> !status --unset         # Clear state
```

---

## Phase 5: Async Command Execution (5 weeks)

### 5.1.1 AsyncCommand Trait

**Goal:** Support asynchronous command execution for long-running operations.

**Proposed API:**
```rust
pub trait AsyncCommandRoutine: Box<
    Fn(&VerifiedCommand, &mut ExecutionContext) -> futures::Future<OutputData, ErrorData>
>;

pub type AsyncRoutine = Box<
    Fn(&VerifiedCommand, &mut ExecutionContext) -> Pin<Box<dyn Future<Output = Pin<Box<dyn Future<OutputData>> + Send>,
>;

pub struct AsyncRegistry {
    async_commands: HashMap<String, AsyncRoutine>,

    pub async fn execute_async(
        &self,
        command: &str,
        ctx: &mut ExecutionContext,
    ) -> ExecutionResult {
        let routine = self.async_commands.get(command)
            .ok_or_else(|| ErrorData::new(
                ErrorCode::CommandNotFoundError,
                format!("Unknown async command: {command}")
            ))?;

        routine(&cmd, ctx).await
    }
}
```

### 5.1.2 Usage Example

**In CLI:**
```bash
# Genfile (hypothetical - would use async)
genfile .run --message "Scan entire codebase" --async
```

**In Code:**
```rust
// Command registration
registry.command_add_runtime(&cmd_def, Box::new(|cmd, ctx| async handler)?)?);
```

### 5.1.3 Design Considerations

**Alternatives:**
1. **Simple**: Add `spawn` method to `ClaudeCommand` and return `Child` handle
2. **Tokio-based**: Use async runtime for all commands
3. **Process-based**: Background job queue with status reporting

**Decision:** Keep async as optional enhancement. Use futures/async for clean integration.

**Risks:**
- Increased complexity
- Requires runtime management for process lifecycle
- Error handling more complex

---

## Phase 6: Performance Optimizations (2 weeks)

### 6.1.1 Current Performance Characteristics

**Strengths:**
- Fast parsing: `parse_from_argv()` ~O(n) for n tokens
- Clean validation: Semantic analyzer
- Small allocations: `Value` enum has no large variants
- No GC pressure: Minimal allocation in hot path

**Weaknesses:**
- String allocations for each token (could reuse)
- No streaming: All outputs collected in memory
- No parallel processing: Single-threaded execution

### 6.1.2 Optimization Opportunities

**1. Token String Reuse**
```rust
// Current: Each token cloned from argv
for token in argv {
    tokens.push(token.clone());
}

// Optimized: Borrow from argv
for token in argv.iter() {
    tokens.push(token);  // No clone, borrow until last use
}
```

**2. Arena Allocation**
```rust
// For very large command lines (1000+ args)
pub fn parse_many(tokens: &[String]) -> Vec<&str> {
    let mut result = Arena::with_capacity(tokens.len(), |tokens.len() * 32);

    for (i, token) in tokens.iter().enumerate() {
        result[i] = result.alloc(token).as_str());
    }

    result
}
```

**3. Lazy Evaluation**
```rust
// Semantic analysis should be fast
// Already optimized: Command lookup by hash map
```

### 6.1.3 Performance Targets

| Metric | Current | Target | Achieved? |
|--------|--------|--------|----------|
| **Startup time** | <5ms | <10ms | ✅ |
| **Parse time** | O(n*m) | O(n*m) | ✅ |
| **Semantic analysis** | O(c*m) | O(c*m) | ✅ |
| **Memory per parse** | <1KB | <2KB | ✅ |

**Risk Assessment:**
- Arena allocation may add complexity
- FileSource I/O can block
- No blocking operations needed

---

## Phase 7: Configuration Management (2 weeks)

### 7.1.1 Configuration Hierarchy

```
~/.config/unilang/                    # Global config (read-only)
└───> tool/                           # Tool-specific config
    └──└─> project/                       # Project config
    └──└─> user/                         # User config
```

### 7.1.2 File Source Priority

```rust
// ConfigManager applies sources in priority
1. Global config (highest)
2. Tool config
3. Project config (lowest)
4. User config

// Resolution: Last value wins for conflicting keys
```

### 7.1.3 Configuration Watcher

**Implementation:**
```rust
pub struct ConfigWatcher {
    watcher: RecommendedWatcher,
    config_path: PathBuf,
    reload_signal: Arc<AtomicBool>,
}

impl ConfigWatcher {
    pub fn watch(&mut self) -> Receiver<ConfigEvent> {
        self.watcher.subscribe(self.reload_signal.subscribe());

        loop {
            select! {
                _ = self.watcher.recv().await;

                // Handle file changes
                if let ConfigEvent::FileModified(path) = _ {
                    if path.ends_with(".yaml") {
                        self.handle_config_change(path).await?;
                    }
                }
                }
            }
        }
    }
}
```

---

## Phase 8: Testing (3 weeks)

### 8.1.1 Test Coverage Strategy

**New Test Areas:**
1. Plugin system tests
2. Configuration file handling
3. Enhanced error messages
4. REPL functionality
5. Async command execution
6. Performance benchmarks
7. Breaking change verification

**Test Organization:**
```
unilang/src/tests/
├── plugin_system/
│   ├── file_source_test.rs
│   ├── directory_source_test.rs
│   ├── memory_source_test.rs
│   └── config_loader_test.rs
├── config/
│   ├── schema_validation_test.rs
│   └── config_manager_test.rs
├── errors/
│   ├── error_data_test.rs
│   ├── error_context_test.rs
│   └── suggestion_test.rs
├── repl/
│   ├── repl_engine_test.rs
│   ├── history_store_test.rs
│   └── state_manager_test.rs
├── async/
│   └── async_registry_test.rs
└── performance/
│   ├── parsing_test.rs
│   ├── arena_test.rs
│   └── lazy_evaluation_test.rs
└── integration/
│   └── config_watcher_test.rs
```

### 8.1.2 Test Metrics

| Category | Current | Target | Success Criteria |
|---------|--------|--------|--------------|
| **Unit tests** | 150+ | Line coverage 80%+ | 1 | All pass |
| **Integration tests** | 20+ | End-to-end scenarios | 5 min each | All pass |
| **Benchmarks** | N/A | N/A | N/A |
| **Code coverage** | 75%+ | 1 | All pass |
| **Performance** | Parse <10µs per token | Semantic <50µs | N/A |

### 8.1.3 Breaking Change Tests

**Migration tests:**
```rust
#[test]
fn backward_compatible_api() {
    // Test that old code still works
    let result = old_api_call();
    assert!(result.is_success());
}
```

---

## Phase 9: Documentation (2 weeks)

### 9.1.1 Documentation Requirements

**New Documentation Needed:**
1. Plugin system architecture guide
2. Configuration file schema documentation
3. Enhanced error messages guide
4. REPL mode user guide
5. Async command usage guide
6. API reference with new types and traits
7. Migration guide (unilang v1.1.0 vs v1.0.1 changes)

### 9.1.2 Documentation Structure

```
unilang/docs/
├── architecture/
│   ├── plugin_system.md
│   ├── configuration_files.md
│   ├── error_messages.md
│   └── repl_mode/
├── api/
│   ├── types.md
│   ├── traits.md
│   └── migration/
└── guides/
    ├── plugin_development.md
    ├── configuration.md
    └── repl_usage.md
└── async_execution.md
```

---

## Risk Assessment Matrix

| Phase | Risk | Likelihood | Impact | Mitigation Strategy |
|------|------|-------------|--------------------|-------------------|
| Phase 1: Plugin | Medium | Source trait design issues | Comprehensive test coverage |
| Phase 2: Config | Low | Schema validation | Clear error messages |
| Phase 3: Errors | Low | Breaking changes to core APIs | Add field with `#[non_exhaustive]` |
| Phase 4: REPL | Medium | State management complexity | Clear separation |
| Phase 5: Async | Medium | futures complexity | Thorough testing |
| Phase 6: Perf | Low | Arena allocation risk | Benchmarking coverage |
| Phase 7: Docs | Low | 12 new files | Update existing docs |
| Phase 8: Tests | Low | 170+ new tests | Integration complexity |
| Phase 9: Docs | Low | Migration guide needed |

**Overall Risk Assessment:** **MEDIUM** — Well-planned, but 18 weeks is ambitious.

---

## Implementation Timeline

| Phase | Duration | Dependencies | Breaking? | Notes |
|------|----------|------------|---------|-------|
| Phase 1: Plugin | 3 weeks | None | No | Trait additions only |
| Phase 2: Config | 2 weeks | None | No | Schema validation only |
| Phase 3: Errors | 3 weeks | Yes | Add optional fields to `ErrorData` | Extensive test coverage |
| Phase 4: REPL | 4 weeks | None | New components only | No API changes |
| Phase 5: Async | 5 weeks | Yes | Add trait to `CommandRegistry` | Comprehensive testing |
| Phase 6: Perf | 2 weeks | No | Arena allocation optional | Benchmarking |
| Phase 7: Docs | 2 weeks | Yes | 12 new docs | No API changes |
| Phase 8: Tests | 3 weeks | None | 170+ new tests | No API changes |
| Phase 9: Docs | 2 weeks | Yes | Migration guide | No code changes |

**Total:** 18 weeks

---

## Dependencies

### External Crates (none required)

All enhancements use only:
- `std` — For core types
- `notifyfs` or `crossbeam-channel` (for file watcher) — optional
- `serde` — For YAML parsing
- `chrono` — For timestamps in history

### Workspace Crates

**Affected Crates:**
- `unilang` — Main target
- `claude_runner_core` — Will consume new APIs
- `genfile` — Will benefit from config support

---

## Conclusion

This comprehensive plan addresses:

1. **Extensibility** — Plugin system with multiple source types
2. **Configuration** — YAML-based configuration with hot-reload
3. **Developer Experience** — Enhanced error messages with context and suggestions
4. **Interactivity** — REPL mode with history and state management
5. **Performance** — Optional arena allocation, lazy evaluation, benchmarking
6. **Testing** — Comprehensive test coverage for all new components

**Recommended Approach:** Implement phases sequentially, starting with Phase 1 (Plugin System) and working toward Phase 8 (Tests). Each phase adds clear value, can be delivered independently if needed.

**Key Success Criteria:**
1. All phases meet their objectives
2. Test coverage >80% for new components
3. Zero breaking changes without migration plan
4. Documentation complete before/after each phase
5. Performance targets met (startup <10ms, parse <10µs/token)

**Estimated Value:** This represents 6-12 months of development effort to transform unilang into a production-ready CLI framework with advanced capabilities.