# genfile

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
genfile REPL v0.1.0
Type '.help' for help, 'exit' to quit

genfile[0]> .archive.new name::"api-scaffold"
Created archive: api-scaffold

genfile[1]> .file.add path::"main.rs" content::"fn main() {}"
Added file: main.rs

genfile[2]> .archive.save path::"api.json"
Saved archive to: api.json
```

## Documentation

- [API Documentation](https://docs.rs/genfile) - Complete API reference
- [Specification](spec.md) - Detailed architecture and design
- [Examples](https://github.com/Wandalen/wTools/tree/master/module/core/genfile/examples) - Usage examples

## Architecture

genfile is built on:
- **genfile_core** - Core template archive library
- **unilang** - Universal CLI framework with REPL support
- **error_tools** - Structured error handling

## Commands Overview

| Category | Commands |
|----------|----------|
| Archive | `.archive.new`, `.archive.load`, `.archive.save`, `.archive.from_directory` |
| Files | `.file.add`, `.file.remove`, `.file.list`, `.file.show` |
| Parameters | `.parameter.add`, `.parameter.list`, `.parameter.remove` |
| Values | `.value.set`, `.value.list`, `.value.clear` |
| Content | `.content.internalize`, `.content.externalize`, `.content.list` |

## Development Status

Current version: **0.2.0**

- ✅ Archive lifecycle management
- ✅ File operations
- ✅ Parameter and value management
- ✅ Content transformation
- ✅ REPL mode with state persistence
- 🚧 Materialization (planned)
- 🚧 Analysis and introspection (planned)
- 🚧 Help system (planned)

## Contributing

See [wTools](https://github.com/Wandalen/wTools) repository for contribution guidelines.

## License

MIT - see [LICENSE](LICENSE) for details.
