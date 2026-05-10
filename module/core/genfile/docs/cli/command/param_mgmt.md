# Parameter Management

Parameter definition management commands for managing parameter schema and metadata. Controls parameter definitions, not values — see [value.md](value.md) for value operations.

- **Namespace:** parameter
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 19 | [.parameter.add](#command--19-parameteradd) | Add definition | 5 | 7 |
| 20 | [.parameter.list](#command--20-parameterlist) | List definitions | 1 | 1 |
| 21 | [.parameter.remove](#command--21-parameterremove) | Remove definition | 2 | 4 |

---

### Command :: 19. `.parameter.add`

### Description

Adds a parameter definition to the archive with metadata including name, description, default value, and mandatory flag. Use this to define template variables.

-- **Parameters:** name::, mandatory::, default::, description::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (parameter already exists) | 2 (invalid name format)

### Syntax

```bash
genfile .parameter.add name::project_name mandatory::true
genfile .parameter.add name::port default::"8080" description::"Server port"
genfile .parameter.add name::author mandatory::false default::""
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `name::` | [IdentifierString](../type.md#type--6-identifierstring) | — | ✅ Yes | Parameter name (alphanumeric + underscore) |
| `mandatory::` | [MandatoryFlag](../type.md#type--15-mandatoryflag) | `0` | No | Whether parameter is required for materialization (0 or 1) |
| `default::` | [ContentString](../type.md#type--9-contentstring) | `null` | No | Default value used when no value is set |
| `description::` | [DescriptionText](../type.md#type--7-descriptiontext) | `""` | No | Human-readable parameter description |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .parameter.add name::project_name mandatory::true description::"Project name"
# Output:
# Added parameter: project_name (mandatory)

genfile .parameter.add name::port mandatory::false default::"3000"
# Output:
# Added parameter: port (optional, default: 3000)

genfile .parameter.add name::author mandatory::false default::""
# Output:
# Added parameter: author (optional, default: "")
```

### Notes

- Parameter name must be unique — error if a parameter with that name already exists
- Mandatory parameters require a value to be set before `.materialize` succeeds
- Default values are used automatically when no explicit value is set via `.value.set`
- Typical workflow: `.parameter.add` (schema) → `.value.set` (instance data)

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 20 | [`.parameter.list`](#command--20-parameterlist) | List definitions after adding |
| 21 | [`.parameter.remove`](#command--21-parameterremove) | Remove a definition that was added |
| 22 | [`.value.set`](value.md#command--22-valueset) | Set a value for the added parameter |
| 2 | [`.discover.parameters`](operations.md#command--2-discoverparameters) | Auto-detect parameters instead of adding manually |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Write
**Complexity:** 7
**API Requirement:** None
**Idempotent:** No
**Risk Level:** Low

---

### Command :: 20. `.parameter.list`

### Description

Lists all parameter definitions in the archive with their metadata. Use this to inspect the parameter schema before setting values or materializing.

-- **Parameters:** verbosity::
-- **Exit Codes:** 0 (success) | 2 (runtime error)

### Syntax

```bash
genfile .parameter.list
genfile .parameter.list verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .parameter.list
# Output:
# Parameters (3 total):
#   project_name (mandatory, no default)
#   port (optional, default: 3000)
#   author (optional, default: "")

genfile .parameter.list verbosity::2
# Output:
# Parameters (3 total):
#
# 1. project_name
#    Mandatory: yes
#    Default: (none)
#    Description: Project name
#
# 2. port
#    Mandatory: no
#    Default: 3000
#    Description: Server port
```

### Notes

- Sorted alphabetically by parameter name
- Shows name, mandatory flag, default value, and description for each parameter
- To see current values (not definitions), use `.value.list`

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 19 | [`.parameter.add`](#command--19-parameteradd) | Add parameters shown in the list |
| 23 | [`.value.list`](value.md#command--23-valuelist) | List current values (not definitions) |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Query
**Complexity:** 1
**API Requirement:** None
**Idempotent:** Yes
**Risk Level:** Low

---

### Command :: 21. `.parameter.remove`

### Description

Removes a parameter definition from the archive. Use this to clean up unused or deprecated parameters.

-- **Parameters:** name::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (parameter not found) | 2 (runtime error)

### Syntax

```bash
genfile .parameter.remove name::old_param
genfile .parameter.remove name::unused verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `name::` | [IdentifierString](../type.md#type--6-identifierstring) | — | ✅ Yes | Name of the parameter to remove |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .parameter.remove name::deprecated_param
# Output:
# Removed parameter: deprecated_param

genfile .parameter.remove name::port verbosity::2
# Output:
# Removed parameter: port (was optional, default: 3000)
# Also cleared associated value (if set)
```

### Notes

- Parameter must exist; fails with exit code 1 if not found
- Also removes the associated value if one was set via `.value.set`
- Use `.analyze` first to identify unused parameters before removing them

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 19 | [`.parameter.add`](#command--19-parameteradd) | Add a parameter definition |
| 24 | [`.value.clear`](value.md#command--24-valueclear) | Clear a value without removing the definition |
| 4 | [`.analyze`](operations.md#command--4-analyze) | Identify unused parameters before removal |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Write
**Complexity:** 4
**API Requirement:** None
**Idempotent:** No
**Risk Level:** Low
