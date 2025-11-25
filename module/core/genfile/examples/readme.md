# genfile Examples

## Purpose

This directory contains examples demonstrating genfile workflows. Due to genfile's stateful CLI design (archive state persists across commands in REPL mode), most real-world usage is through:
- **REPL mode** - Interactive command-line sessions
- **Shell scripts** - Batch command sequences

## Organization Principles

Examples are organized by format and use case:
- **Shell scripts** (`.sh`) - Practical workflow demonstrations
- **Rust programs** (`.rs`) - Programmatic API usage (future)

## Shell Script Examples

### basic_workflow.sh

Demonstrates the fundamental genfile workflow in REPL mode:
1. Creating a new archive
2. Adding template files with parameters
3. Defining parameter metadata
4. Saving the archive to disk

**Run with:**
```bash
chmod +x examples/basic_workflow.sh
./examples/basic_workflow.sh
```

**What it teaches:**
- Archive lifecycle management (FR1)
- File operations (FR2)
- Parameter definitions (FR3)
- Archive persistence

**Output:**
Creates a template archive at `/tmp/basic-example.json` with a simple project template.

## Running Examples

Shell script examples can be executed directly:

```bash
# Make executable
chmod +x examples/*.sh

# Run specific example
./examples/basic_workflow.sh

# Or use bash directly
bash examples/basic_workflow.sh
```

## Adding New Examples

When adding new examples:
1. Prefer shell scripts for CLI workflow demonstrations
2. Add clear comments explaining each step
3. Include error handling and status checks
4. Update this readme with the example description
5. Test that it runs successfully

## Future Examples (Planned)

- **materialization_workflow.sh** - Template rendering and output generation
- **pack_from_directory.sh** - Creating portable archives from existing projects
- **parameter_discovery.sh** - Auto-detecting template parameters
- **content_internalization.sh** - Managing external references
- **api_usage.rs** - Programmatic genfile_core API usage (when API stabilizes)
