# genfile
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=genfile)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/genfile?color=e3e8f0&logo=docs.rs)](https://docs.rs/genfile) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

[![Crates.io](https://img.shields.io/crates/v/genfile.svg)](https://crates.io/crates/genfile)
[![docs.rs](https://docs.rs/genfile/badge.svg)](https://docs.rs/genfile)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

CLI for template archive management - create, manage, and materialize code generation templates.

## Quick Start

Install via cargo:

```bash
cargo install genfile
```

Create a portable template archive from a directory:

```bash
# Pack directory into archive
genfile .pack input::"./my-template" output::"template.json"

# Load and materialize
genfile .archive.load path::"template.json"
genfile .value.set name::"project_name" value::"my-project"
genfile .materialize destination::"./output"
```

## Features

- **Archive Management** - Create, load, save template archives
- **File Operations** - Add, remove, list template files
- **Parameter System** - Define and manage template parameters
- **Value Management** - Set parameter values for materialization
- **Content Control** - Inline or reference-based file storage
- **REPL Mode** - Interactive command-line interface
- **Dual Mode** - Works as single command or interactive REPL

## Interactive REPL

Run without arguments for interactive mode:

```bash
genfile
```

```
genfile REPL v0.4.0
Type '.help' for help, 'exit' to quit

genfile[0]> .archive.new name::"api-scaffold"
Created archive: api-scaffold

genfile[1]> .file.add path::"main.rs" content::"fn main() {}"
Added file: main.rs

genfile[2]> .archive.save path::"api.json"
Saved archive to: api.json
```

## Documentation

### CLI Documentation

Complete command-line interface reference:
- **[CLI Documentation](docs/cli/readme.md)** - Comprehensive CLI reference (24 commands, 23 parameters, 15 types)
  - [Quick Start Guide](docs/cli/readme.md#quick-start) - Common workflows and examples
  - [Commands Reference](docs/cli/commands.md) - All commands indexed by namespace
  - [Parameters Reference](docs/cli/params.md) - Complete parameter specifications
  - [Type System](docs/cli/types.md) - Type definitions for implementers
  - [Tutorial](docs/cli/readme.md#common-workflows) - Step-by-step learning guide

### API Documentation

- [API Documentation](https://docs.rs/genfile) - Complete Rust API reference
- [Examples](https://github.com/Wandalen/wTools/tree/master/module/core/genfile/examples) - Usage examples

## Architecture

genfile is built on:
- **genfile_core** - Core template archive library
- **unilang** - Universal CLI framework with REPL support
- **error_tools** - Structured error handling

### Command Specifications

All command definitions are maintained as YAML specifications in `commands/*.yaml`:

```
commands/
├── archive.yaml       - Archive lifecycle (.archive.*)
├── file.yaml          - File operations (.file.*)
├── parameter.yaml     - Parameter management (.parameter.*)
├── value.yaml         - Value management (.value.*)
├── content.yaml       - Content management (.content.*)
├── materialize.yaml   - Template materialization (.materialize, .unpack)
├── pack.yaml          - Archive serialization (.pack)
└── analysis.yaml      - Analysis and introspection (.info, .status, .analyze, .discover.*)
```

These YAML files serve as the **authoritative specification** for all commands, including arguments, descriptions, examples, and functional requirement mappings. The Rust implementation in `src/commands/` registers handlers that implement these specifications.

## Commands Overview

| Category | Commands |
|----------|----------|
| Archive | `.archive.new`, `.archive.load`, `.archive.save`, `.archive.from_directory` |
| Files | `.file.add`, `.file.remove`, `.file.list`, `.file.show` |
| Parameters | `.parameter.add`, `.parameter.list`, `.parameter.remove` |
| Values | `.value.set`, `.value.list`, `.value.clear` |
| Content | `.content.internalize`, `.content.externalize`, `.content.list` |

## Development Status

Current version: **0.4.0**

All core features implemented and tested (74 integration tests, 100% passing):

- ✅ Archive lifecycle management (FR1: `.archive.*`)
- ✅ File operations (FR2: `.file.*`)
- ✅ Parameter and value management (FR3/FR4: `.parameter.*`, `.value.*`)
- ✅ Content transformation (FR5: `.content.*`)
- ✅ Template materialization (FR6: `.materialize`, `.unpack`)
- ✅ Archive serialization (FR7: `.pack`)
- ✅ Analysis and introspection (FR8: `.info`, `.status`, `.analyze`, `.discover.*`)
- ✅ Help system (FR9: `.help`, `.`, `.command.help`)
- ✅ REPL mode with state persistence

## Contributing

See [wTools](https://github.com/Wandalen/wTools) repository for contribution guidelines.

## License

MIT - see [LICENSE](LICENSE) for details.
