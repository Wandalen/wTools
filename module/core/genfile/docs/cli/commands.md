# Commands Reference

Complete command listing for genfile CLI (24 commands).

## Quick Navigation

**By Namespace:**
- [Archive Operations](#namespace-archive) (4) - Create, load, save archives
- [Content Management](#namespace-content) (3) - Inline/reference content control
- [File Operations](#namespace-file) (4) - Add, remove, list files
- [Parameter Management](#namespace-parameter) (3) - Define parameters
- [Value Operations](#namespace-value) (3) - Set parameter values
- [Core Operations](#namespace-operations) (7) - Materialize, pack, analyze

**By Functionality:**
- Creating archives: [.archive.new](#5-archivenew), [.archive.from_directory](#8-archivefromdirectory), [.pack](#18-pack)
- Loading/saving: [.archive.load](#6-archiveload), [.archive.save](#7-archivesave)
- Template rendering: [.materialize](#16-materialize), [.unpack](#17-unpack)
- File management: [.file.add](#12-fileadd), [.file.remove](#13-fileremove), [.file.list](#14-filelist), [.file.show](#15-fileshow)
- Parameter setup: [.parameter.add](#19-parameteradd), [.value.set](#22-valueset)
- Analysis: [.analyze](#4-analyze), [.info](#1-info), [.status](#3-status), [.discover.parameters](#2-discoverparameters)

## Commands Index

| # | Command | Purpose | Params | Namespace | Status | Detailed Spec |
|---|---------|---------|--------|-----------|--------|---------------|
| 1 | `.info` | Display archive metadata and statistics | 1 | - | ✅ Implemented | [operations.md](commands/operations.md#command-1-info) |
| 2 | `.discover.parameters` | Auto-detect template parameters from content | 2 | discover | ✅ Implemented | [operations.md](commands/operations.md#command-2-discoverparameters) |
| 3 | `.status` | Archive readiness check with validation | 1 | - | ✅ Implemented | [operations.md](commands/operations.md#command-3-status) |
| 4 | `.analyze` | Comprehensive archive structure analysis | 2 | analyze | ✅ Implemented | [operations.md](commands/operations.md#command-4-analyze) |
| 5 | `.archive.new` | Create new empty template archive | 3 | archive | ✅ Implemented | [archive.md](commands/archive.md#command-5-archivenew) |
| 6 | `.archive.load` | Load archive from JSON or YAML file | 2 | archive | ✅ Implemented | [archive.md](commands/archive.md#command-6-archiveload) |
| 7 | `.archive.save` | Save current archive to JSON or YAML file | 5 | archive | ✅ Implemented | [archive.md](commands/archive.md#command-7-archivesave) |
| 8 | `.archive.from_directory` | Create archive from filesystem directory | 7 | archive | ✅ Implemented | [archive.md](commands/archive.md#command-8-archivefromdirectory) |
| 9 | `.content.internalize` | Convert file references to inline content | 2 | content | ✅ Implemented | [content.md](commands/content.md#command-9-contentinternalize) |
| 10 | `.content.externalize` | Convert inline content to file references | 3 | content | ✅ Implemented | [content.md](commands/content.md#command-10-contentexternalize) |
| 11 | `.content.list` | List files by content storage mode | 2 | content | ✅ Implemented | [content.md](commands/content.md#command-11-contentlist) |
| 12 | `.file.add` | Add file to current archive | 5 | file | ✅ Implemented | [file.md](commands/file.md#command-12-fileadd) |
| 13 | `.file.remove` | Remove file from archive | 2 | file | ✅ Implemented | [file.md](commands/file.md#command-13-fileremove) |
| 14 | `.file.list` | List all files in archive | 1 | file | ✅ Implemented | [file.md](commands/file.md#command-14-filelist) |
| 15 | `.file.show` | Display file content from archive | 2 | file | ✅ Implemented | [file.md](commands/file.md#command-15-fileshow) |
| 16 | `.materialize` | Render template archive to destination with parameter substitution | 3 | - | ✅ Implemented | [operations.md](commands/operations.md#command-16-materialize) |
| 17 | `.unpack` | Unpack raw template files without rendering | 3 | - | ✅ Implemented | [operations.md](commands/operations.md#command-17-unpack) |
| 18 | `.pack` | Create portable archive from directory with inline content | 4 | pack | ✅ Implemented | [operations.md](commands/operations.md#command-18-pack) |
| 19 | `.parameter.add` | Add parameter definition to archive with metadata | 5 | parameter | ✅ Implemented | [param_mgmt.md](commands/param_mgmt.md#command-19-parameteradd) |
| 20 | `.parameter.list` | List all parameter definitions in archive | 1 | parameter | ✅ Implemented | [param_mgmt.md](commands/param_mgmt.md#command-20-parameterlist) |
| 21 | `.parameter.remove` | Remove parameter definition from archive | 2 | parameter | ✅ Implemented | [param_mgmt.md](commands/param_mgmt.md#command-21-parameterremove) |
| 22 | `.value.set` | Set parameter value for template rendering | 3 | value | ✅ Implemented | [value.md](commands/value.md#command-22-valueset) |
| 23 | `.value.list` | List all parameter values | 1 | value | ✅ Implemented | [value.md](commands/value.md#command-23-valuelist) |
| 24 | `.value.clear` | Clear all parameter values | 2 | value | ✅ Implemented | [value.md](commands/value.md#command-24-valueclear) |

## Namespace Groups

### Namespace: Archive

Archive lifecycle management - creating, loading, and saving template archives.

| Command | Purpose | Complexity |
|---------|---------|------------|
| [.archive.new](#5-archivenew) | Create empty archive | Low (3 params) |
| [.archive.load](#6-archiveload) | Load from file | Low (2 params) |
| [.archive.save](#7-archivesave) | Save to file | Medium (5 params) |
| [.archive.from_directory](#8-archivefromdirectory) | Create from filesystem | High (7 params) |

**See:** [commands/archive.md](commands/archive.md)

### Namespace: Content

Content storage mode management - controlling inline vs reference content.

| Command | Purpose | Complexity |
|---------|---------|------------|
| [.content.internalize](#9-contentinternalize) | Convert refs to inline | Low (2 params) |
| [.content.externalize](#10-contentexternalize) | Convert inline to refs | Medium (3 params) |
| [.content.list](#11-contentlist) | List by mode | Low (2 params) |

**See:** [commands/content.md](commands/content.md)

### Namespace: File

File operations within archives - CRUD on individual files.

| Command | Purpose | Complexity |
|---------|---------|------------|
| [.file.add](#12-fileadd) | Add file | Medium (5 params) |
| [.file.remove](#13-fileremove) | Remove file | Low (2 params) |
| [.file.list](#14-filelist) | List files | Low (1 param) |
| [.file.show](#15-fileshow) | Show content | Low (2 params) |

**See:** [commands/file.md](commands/file.md)

### Namespace: Parameter

Parameter definition management - managing parameter schema/metadata.

| Command | Purpose | Complexity |
|---------|---------|------------|
| [.parameter.add](#19-parameteradd) | Add definition | Medium (5 params) |
| [.parameter.list](#20-parameterlist) | List definitions | Low (1 param) |
| [.parameter.remove](#21-parameterremove) | Remove definition | Low (2 params) |

**See:** [commands/param_mgmt.md](commands/param_mgmt.md)

### Namespace: Value

Parameter value management - managing parameter data/instances.

| Command | Purpose | Complexity |
|---------|---------|------------|
| [.value.set](#22-valueset) | Set value | Low (3 params) |
| [.value.list](#23-valuelist) | List values | Low (1 param) |
| [.value.clear](#24-valueclear) | Clear all | Low (2 params) |

**See:** [commands/value.md](commands/value.md)

### Namespace: Operations

Core operations - materialization, analysis, and utility commands.

| Command | Purpose | Complexity |
|---------|---------|------------|
| [.info](#1-info) | Show metadata | Low (1 param) |
| [.discover.parameters](#2-discoverparameters) | Auto-detect params | Low (2 params) |
| [.status](#3-status) | Readiness check | Low (1 param) |
| [.analyze](#4-analyze) | Analyze structure | Low (2 params) |
| [.materialize](#16-materialize) | Render templates | Low (3 params) |
| [.unpack](#17-unpack) | Unpack raw files | Low (3 params) |
| [.pack](#18-pack) | Pack to archive | Medium (4 params) |

**See:** [commands/operations.md](commands/operations.md)

## Cross-References

**By Parameter Usage:**
- Commands using [`verbosity::`](../params.md#parameter-1-verbosity): ALL (24 commands)
- Commands using [`dry::`](../params.md#parameter-2-dry): 6 commands (write operations only)
- Commands using [`path::`](../params.md#parameter-3-path): 3 commands (I/O operations)

**See Also:**
- [Parameters Reference](params.md) - Complete parameter documentation
- [Types Reference](types.md) - Type system and validation
- [Parameter Groups](parameter_groups.md) - Shared parameter sets
- [Dictionary](dictionary.md) - Domain terminology
