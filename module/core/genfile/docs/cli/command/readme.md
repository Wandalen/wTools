# GenFile Commands

### Scope

- **In Scope:** All genfile CLI commands — syntax, parameters, examples, and namespace organization
- **Out of Scope:** Parameter type validation details (see [`type.md`](../type.md)), parameter group semantics (see [`param_group.md`](../param_group.md))
- **Audience:** CLI users and integrators invoking genfile commands
- **Responsibility:** Complete command index and per-namespace command specifications

### Commands Index

| # | Command | Purpose | Params | Namespace |
|---|---------|---------|--------|-----------|
| 1 | `.info` | Display archive metadata and statistics | 1 | [operations.md](operations.md#command--1-info) |
| 2 | `.discover.parameters` | Auto-detect template parameters from content | 2 | [operations.md](operations.md#command--2-discoverparameters) |
| 3 | `.status` | Archive readiness check with validation | 1 | [operations.md](operations.md#command--3-status) |
| 4 | `.analyze` | Comprehensive archive structure analysis | 2 | [operations.md](operations.md#command--4-analyze) |
| 5 | `.archive.new` | Create new empty template archive | 3 | [archive.md](archive.md#command--5-archivenew) |
| 6 | `.archive.load` | Load archive from JSON or YAML file | 2 | [archive.md](archive.md#command--6-archiveload) |
| 7 | `.archive.save` | Save current archive to JSON or YAML file | 5 | [archive.md](archive.md#command--7-archivesave) |
| 8 | `.archive.from_directory` | Create archive from filesystem directory | 7 | [archive.md](archive.md#command--8-archivefrom_directory) |
| 9 | `.content.internalize` | Convert file references to inline content | 2 | [content.md](content.md#command--9-contentinternalize) |
| 10 | `.content.externalize` | Convert inline content to file references | 3 | [content.md](content.md#command--10-contentexternalize) |
| 11 | `.content.list` | List files by content storage mode | 2 | [content.md](content.md#command--11-contentlist) |
| 12 | `.file.add` | Add file to current archive | 5 | [file.md](file.md#command--12-fileadd) |
| 13 | `.file.remove` | Remove file from archive | 2 | [file.md](file.md#command--13-fileremove) |
| 14 | `.file.list` | List all files in archive | 1 | [file.md](file.md#command--14-filelist) |
| 15 | `.file.show` | Display file content from archive | 2 | [file.md](file.md#command--15-fileshow) |
| 16 | `.materialize` | Render template archive to destination with parameter substitution | 3 | [operations.md](operations.md#command--16-materialize) |
| 17 | `.unpack` | Unpack raw template files without rendering | 3 | [operations.md](operations.md#command--17-unpack) |
| 18 | `.pack` | Create portable archive from directory with inline content | 4 | [operations.md](operations.md#command--18-pack) |
| 19 | `.parameter.add` | Add parameter definition to archive with metadata | 5 | [param_mgmt.md](param_mgmt.md#command--19-parameteradd) |
| 20 | `.parameter.list` | List all parameter definitions in archive | 1 | [param_mgmt.md](param_mgmt.md#command--20-parameterlist) |
| 21 | `.parameter.remove` | Remove parameter definition from archive | 2 | [param_mgmt.md](param_mgmt.md#command--21-parameterremove) |
| 22 | `.value.set` | Set parameter value for template rendering | 3 | [value.md](value.md#command--22-valueset) |
| 23 | `.value.list` | List all parameter values | 1 | [value.md](value.md#command--23-valuelist) |
| 24 | `.value.clear` | Clear all parameter values | 2 | [value.md](value.md#command--24-valueclear) |

### Quick Reference

**Required parameters:**
- `name::` — `.archive.new`, `.parameter.add`, `.parameter.remove`, `.value.set`
- `source::` — `.archive.from_directory`
- `destination::` — `.materialize`, `.unpack`
- `input::` + `output::` — `.pack`
- `value::` — `.value.set`

**Most used parameters:**
- `verbosity::` — all cmds
- `dry::` — 6 cmds (write operations only)
- `path::` — 5 cmds
- `name::` — 4 cmds
- `destination::` / `description::` — 2 cmds each

**Commands by parameter count:**
- 1 param: `.info`, `.status`, `.file.list`, `.parameter.list`, `.value.list`
- 2 params: `.discover.parameters`, `.archive.load`, `.content.internalize`, `.file.remove`, `.file.show`, `.parameter.remove`, `.value.clear`
- 3 params: `.archive.new`, `.content.externalize`, `.content.list`, `.materialize`, `.unpack`, `.value.set`
- 4 params: `.pack`
- 5 params: `.archive.save`, `.file.add`, `.parameter.add`
- 7 params: `.archive.from_directory`

### Namespaces

| Namespace | Commands | Responsibility | File |
|-----------|----------|----------------|------|
| archive | 4 | Archive lifecycle management | [archive.md](archive.md) |
| content | 3 | Content storage mode management | [content.md](content.md) |
| file | 4 | File CRUD within archives | [file.md](file.md) |
| parameter | 3 | Parameter definition management | [param_mgmt.md](param_mgmt.md) |
| value | 3 | Parameter value management | [value.md](value.md) |
| — (core) | 7 | Core operations and analysis | [operations.md](operations.md) |

### Referenced Parameters

| # | Parameter | Used In |
|---|-----------|---------|
| 1 | [`verbosity::`](../param.md#parameter--1-verbosity) | all cmds |
| 2 | [`dry::`](../param.md#parameter--2-dry) | 6 cmds |
| 3 | [`path::`](../param.md#parameter--3-path) | 5 cmds |
| 4 | [`name::`](../param.md#parameter--4-name) | 4 cmds |
