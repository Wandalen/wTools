# Help Formatting Improvements

## Context

Current help output suffers from usability issues:
- Flat alphabetical listing makes commands hard to discover
- `.help` variants clutter the command list (27 items instead of ~13 actual commands)
- No hierarchical navigation of command namespaces
- Inconsistent help between installed binary and dev builds
- No auto-categorization from command structure

## Problem Evidence

**Current output** (`cargo run -- .`):
```
Available commands:

  .                    Show all available commands
  .add                 Add a repository to the workspace
  .add.help            Show detailed help for .add
  .clone               Clone repositories
  .clone.help          Show detailed help for .clone
  .discover            Discover repositories from GitHub or local filesystem
  .discover.help       Show detailed help for .discover
  ...
```

Issues:
- 27+ items (half are `.help` variants)
- No categories
- No prefix filtering
- Hard to scan

**Desired output**:
```
REPOSITORY MANAGEMENT
  .add                 Add repository to workspace
  .clone               Clone all configured repositories
  .list                List repositories

GIT OPERATIONS
  .git.status          Detailed git status
  .status              Quick status overview
  .pull                Pull changes

REMOVAL OPERATIONS
  .remove.both         Remove from config AND delete files
  .remove.local        Delete files, keep in config
  .remove.registry     Remove from config, keep files
  .remove.missing      Clean up missing repositories

Use: wip <command> help
Example: wip .list help
```

## Objectives

### 1. Prefix Filtering (Natural Namespace Navigation)

**Behavior**:
```bash
wip .                # Show all commands (current behavior)
wip .git             # Show only: .git.status
wip .remove          # Show: .remove.both, .remove.local, .remove.registry, .remove.missing
wip .orgs            # Show: .orgs.list
```

**Implementation Points**:
- Parse command argument as prefix filter when no exact match
- Filter command registry by prefix
- Display filtered results with grouping

### 2. Hide `.help` Variants from Brief Listing

**Behavior**:
- Main command list shows only actual commands (not `.help` variants)
- Add footer note: `For detailed help: wip <command> help`
- `.help` commands still callable, just not listed

**Implementation Points**:
- Add `hidden_from_list` flag to command metadata
- Auto-mark `*.help` commands as hidden
- Filter during help generation
- Keep them in registry for invocation

### 3. Auto-Categorization from Command Structure

**Rules**:
```yaml
categories:
  repository_management:
    patterns: [".add", ".clone", ".list", ".init"]

  git_operations:
    patterns: [".git.*", ".status", ".pull", ".push", ".sync", ".update"]

  removal_operations:
    patterns: [".remove.*"]

  github_integration:
    patterns: [".discover", ".orgs.*", ".users.*"]
```

**Implementation Points**:
- Extract categories from YAML command definitions OR
- Auto-detect from dotted prefix patterns
- Group commands by category in output
- Sort categories by importance (user-defined order)

### 4. Command Grouping Display

When showing multiple related commands (e.g., `.remove.*`):

```
.remove operations (4 commands):
  .remove.both         Remove from config AND delete files
  .remove.local        Delete files, keep in config
  .remove.registry     Remove from config, keep files
  .remove.missing      Clean up missing repositories

Use: wip .remove.both help
```

**Implementation Points**:
- Detect common prefix in filtered results
- Show group header with count
- Indent sub-commands
- Add usage hint

### 5. YAML Metadata Enhancement

Add to command YAML definitions:

```yaml
commands:
  .add:
    category: repository_management
    short_desc: "Add repository to workspace"
    hidden: false
    priority: 1  # For sorting within category

  .add.help:
    hidden_from_list: true

  .remove.both:
    category: removal_operations
    short_desc: "Remove from config AND delete files"
    group: ".remove"
```

**Required fields**:
- `category`: For auto-grouping
- `short_desc`: One-line description for listings
- `hidden_from_list`: Exclude from brief help
- `priority`: Sort order within category
- `group`: Explicit group membership (optional)

### 6. Unified Help Implementation

**Problem**: Two different help outputs suggest duplicate codepaths:
- Installed binary: Well-formatted with categories
- Dev build: Flat alphabetical list

**Solution**: Single source of truth

```rust
pub struct HelpFormatter {
  commands: Vec<CommandMetadata>,
  mode: HelpMode,
}

pub enum HelpMode {
  Brief,         // Command name + short desc, categorized, no .help variants
  Comprehensive, // + examples + parameters + see-also (like installed binary)
}

impl HelpFormatter {
  pub fn format(&self) -> String {
    match self.mode {
      HelpMode::Brief => self.format_brief(),
      HelpMode::Comprehensive => self.format_comprehensive(),
    }
  }

  fn format_brief(&self) -> String {
    let visible = self.commands.iter()
      .filter(|c| !c.hidden_from_list);
    let by_category = group_by_category(visible);
    render_categorized(by_category)
  }
}
```

**Implementation Points**:
- Consolidate help generation to single module
- Read metadata from YAML definitions
- Use same formatter for installed binary and dev builds
- Mode selection: `.` = brief, `.help` = comprehensive

### 7. Progressive Disclosure

**Levels**:
```bash
wip .                 # Brief: categories + commands + short descriptions
wip .help             # Comprehensive: + examples + parameters + workspace info
```

**No verbosity levels needed** - just two clear modes.

**Implementation Points**:
- `.` command → HelpMode::Brief
- `.help` command → HelpMode::Comprehensive
- Remove verbosity-based help variations

## Implementation Plan

### Phase 1: Foundation (Core Infrastructure)

1. **Add metadata fields to YAML schema**
   - `category`, `short_desc`, `hidden_from_list`, `priority`, `group`
   - Update YAML loader to parse new fields
   - Update all existing command YAMLs with metadata

2. **Create HelpFormatter module**
   - `src/help/formatter.rs`
   - Struct with command metadata + mode
   - Brief and comprehensive rendering methods

### Phase 2: Filtering & Categorization

3. **Implement prefix filtering**
   - Detect partial command match
   - Filter registry by prefix
   - Return filtered command set

4. **Implement auto-categorization**
   - Group commands by `category` field
   - Sort categories by predefined order
   - Fallback to prefix-based grouping

5. **Implement command grouping**
   - Detect common prefixes (`.remove.*`)
   - Render group headers
   - Indent sub-commands

### Phase 3: Integration & Refinement

6. **Hide `.help` variants**
   - Auto-mark in YAML loader
   - Filter during help generation
   - Keep callable but unlisted

7. **Unify help codepaths**
   - Replace all help generation with HelpFormatter
   - Ensure installed binary uses same code
   - Remove duplicate implementations

8. **Testing**
   - Help generation tests for all modes
   - Prefix filtering tests
   - Categorization tests
   - Regression tests for existing help output

### Phase 4: Documentation

9. **Update examples**
   - `examples/18_help_conventions_demo.rs` with new features
   - Document YAML metadata fields
   - Document HelpFormatter API

10. **Update tests**
    - `tests/inc/phase2/help_generation_test.rs`
    - Add tests for categorization
    - Add tests for filtering

## Acceptance Criteria

- ✅ `wip .` shows categorized command list (no `.help` variants)
- ✅ `wip .git` shows only git-prefixed commands
- ✅ `wip .remove` shows all removal operations grouped
- ✅ `wip .help` shows comprehensive help (like installed binary)
- ✅ Installed binary and dev build produce identical output
- ✅ YAML metadata is single source of truth
- ✅ All existing help tests pass
- ✅ No duplicate help generation code

## Files to Modify

### Core Implementation
- `src/help/formatter.rs` (NEW) - Unified help formatter
- `src/help/mod.rs` - Re-export formatter
- `src/command/metadata.rs` - Add new metadata fields
- `src/loader/yaml.rs` - Parse new YAML fields
- `src/registry.rs` - Expose prefix filtering method

### Integration
- `src/executor.rs` - Use HelpFormatter for `.` and `.help` commands
- `src/bin/unilang_cli.rs` - Use same formatter

### Tests
- `tests/inc/phase2/help_generation_test.rs` - Add new test cases
- `tests/inc/phase2/command_loader_yaml_test.rs` - Test new metadata fields

### Documentation
- `examples/18_help_conventions_demo.rs` - Demonstrate new features
- `readme.md` - Document YAML metadata fields

## Notes

- Maintain backward compatibility: commands without `category` still work
- Default `hidden_from_list: false` for all commands except `*.help`
- Prefix filtering is case-sensitive (matches command format)
- Categories are rendered in predefined order (not alphabetical)
- Brief help fits in terminal without scrolling (~50 lines max)

## Dependencies

None - uses existing unilang infrastructure.

## Priority

**High** - Directly impacts user experience and command discoverability.

## Estimated Effort

- Phase 1: 4 hours
- Phase 2: 6 hours
- Phase 3: 4 hours
- Phase 4: 2 hours
- **Total: ~16 hours**
