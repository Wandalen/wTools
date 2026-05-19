# Value Operations

Parameter value management commands for setting, listing, and clearing parameter values. Controls parameter instances, not definitions — see [param_mgmt.md](param_mgmt.md) for definition operations.

- **Namespace:** value
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 22 | [.value.set](#command--22-valueset) | Set parameter value for rendering | 3 | 7 |
| 23 | [.value.list](#command--23-valuelist) | List all current parameter values | 1 | 1 |
| 24 | [.value.clear](#command--24-valueclear) | Clear all parameter values | 2 | 2 |

---

### Command :: 22. `.value.set`

### Description

Sets the value of a named parameter in the archive. Use this to provide concrete values for template placeholders before materialization.

-- **Parameters:** name::, value::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (parameter not defined) | 2 (invalid value)

### Syntax

```bash
genfile .value.set name::project_name value::"my-app"
genfile .value.set name::port value::"8080"
genfile .value.set name::author value::"John Doe" verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `name::` | [IdentifierString](../type.md#type--6-identifierstring) | — | ✅ Yes | Name of the parameter to set |
| `value::` | [ContentString](../type.md#type--9-contentstring) | — | ✅ Yes | Value to assign to the parameter |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .value.set name::project_name value::"rust-cli-tool"
# Output:
# Set parameter value: project_name = "rust-cli-tool"

genfile .value.set name::port value::"3000"
# Output:
# Set parameter value: port = "3000" (previous: "8080")

genfile .value.set name::author value::"Jane Smith" verbosity::2
# Output:
# Set parameter value: author = "Jane Smith"
# Archive status: 3/3 parameters set
```

### Notes

- Parameter must be defined via `.parameter.add` or `.discover.parameters` before setting a value; fails with exit code 1 if the parameter is undefined
- Overwrites the previous value without confirmation if already set
- Values are stored in-memory and persist until cleared or the archive is reloaded

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 19 | [`.parameter.add`](param_mgmt.md#command--19-parameteradd) | Define the parameter before setting a value |
| 23 | [`.value.list`](#command--23-valuelist) | List current values after setting |
| 16 | [`.materialize`](operations.md#command--16-materialize) | Use set values for template rendering |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Write
**Complexity:** 7
**API Requirement:** None
**Idempotent:** Yes
**Risk Level:** Low

---

### Command :: 23. `.value.list`

### Description

Lists all parameter values currently set in the archive. Use this to inspect parameter state before materialization and confirm all required values are provided.

-- **Parameters:** verbosity::
-- **Exit Codes:** 0 (success) | 2 (runtime error)

### Syntax

```bash
genfile .value.list
genfile .value.list verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .value.list
# Output:
# Parameter Values (2 set, 1 unset):
#   project_name = "my-app"
#   port = "3000"
#   author = (not set, using default: "")

genfile .value.list verbosity::2
# Output:
# Parameter Values:
#
# Set (2):
#   project_name = "my-app" (mandatory)
#   port = "3000" (optional, default: 8080)
#
# Unset (1):
#   author (optional, will use default: "")
#
# Readiness: OK (all mandatory parameters set)
```

### Notes

- Shows set values, unset parameters with their defaults, and overall readiness
- For unset mandatory parameters (no default), readiness shows NOT READY
- Lists values only — for parameter definitions (schema) see `.parameter.list`

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 22 | [`.value.set`](#command--22-valueset) | Set a value for a listed parameter |
| 20 | [`.parameter.list`](param_mgmt.md#command--20-parameterlist) | List parameter definitions |
| 3 | [`.status`](operations.md#command--3-status) | Readiness check with missing-value report |

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

### Command :: 24. `.value.clear`

### Description

Clears all parameter values in the archive, resetting them to unset state. Use this to reset values before applying a new set of parameters, while keeping parameter definitions intact.

-- **Parameters:** verbosity::, dry::
-- **Exit Codes:** 0 (success) | 2 (runtime error)

### Syntax

```bash
genfile .value.clear
genfile .value.clear verbosity::2
genfile .value.clear dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .value.clear
# Output:
# Cleared 2 parameter values
# Parameters ready for new values

genfile .value.clear dry::1
# Output:
# [DRY RUN] Would clear 2 parameter values:
#   project_name
#   port
# [DRY RUN] No changes made
```

### Notes

- Clears values only — parameter definitions remain; use `.parameter.remove` to remove definitions
- After clearing, mandatory parameters must receive new values before `.materialize` succeeds
- Succeeds with zero count when no values are set (no-op)

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 22 | [`.value.set`](#command--22-valueset) | Set new values after clearing |
| 21 | [`.parameter.remove`](param_mgmt.md#command--21-parameterremove) | Remove definitions (not just values) |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |
| 2 | [Universal Execution Control](../param_group.md#group--2-universal-execution-control) | Full | `dry::` |

---

**Category:** Write
**Complexity:** 2
**API Requirement:** None
**Idempotent:** Yes
**Risk Level:** Low
