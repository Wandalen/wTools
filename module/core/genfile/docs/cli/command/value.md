# Value Operations

Parameter value management commands for setting, listing, and clearing parameter values. Controls parameter instances (not definitions - see param_mgmt.md for that).

- **Namespace:** value
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 22 | [.value.set](#command-22-valueset) | Set value | 3 | Low |
| 23 | [.value.list](#command-23-valuelist) | List values | 1 | Low |
| 24 | [.value.clear](#command-24-valueclear) | Clear all | 2 | Low |

---

### Command :: 22. `.value.set`

Sets parameter value for template rendering. Use this to provide values before materialization.

**Syntax:**
```bash
genfile .value.set name::project_name value::"my-app"
genfile .value.set name::port value::"8080"
genfile .value.set name::author value::"John Doe" verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `name::` | [IdentifierString](../type.md#type-identifierstring) | Parameter name | - | ✅ Yes |
| `value::` | [ContentString](../type.md#type-contentstring) | Parameter value | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Sets value for parameter in archive
- Parameter definition must exist (error if not defined)
- Overwrites previous value if already set
- Values persist until cleared or archive reloaded

**Examples:**

**Set value:**
```bash
genfile .value.set name::project_name value::"rust-cli-tool"
# Output:
# Set parameter value: project_name = "rust-cli-tool"
```

**Overwrite value:**
```bash
genfile .value.set name::port value::"3000"
# Output:
# Set parameter value: port = "3000" (previous: "8080")
```

**Exit Codes:** 0 (success) | 1 (parameter not defined) | 2 (invalid value)

**Interactions:**
- Dependencies: Parameter must be defined via `.parameter.add` first
- Typical workflow: `.parameter.add` → `.value.set` → `.materialize`

**Related Commands:**
- [.parameter.add](param_mgmt.md#command-19-parameteradd) - Define parameters first
- [.value.list](#command-23-valuelist) - List current values
- [.materialize](operations.md#command-16-materialize) - Use values for rendering

---

### Command :: 23. `.value.list`

Lists all parameter values currently set in archive. Use this to inspect parameter state before materialization.

**Syntax:**
```bash
genfile .value.list
genfile .value.list verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Lists all set parameter values
- Shows parameter name and current value
- Indicates which mandatory parameters still need values

**Examples:**

**Basic listing:**
```bash
genfile .value.list
# Output:
# Parameter Values (2 set, 1 unset):
#   project_name = "my-app"
#   port = "3000"
#   author = (not set, using default: "")
```

**Verbose listing:**
```bash
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

**Exit Codes:** 0 (success) | 2 (runtime error)

**Related Commands:**
- [.value.set](#command-22-valueset) - Set values
- [.parameter.list](param_mgmt.md#command-20-parameterlist) - List definitions

---

### Command :: 24. `.value.clear`

Clears all parameter values in archive. Use this to reset parameter state.

**Syntax:**
```bash
genfile .value.clear
genfile .value.clear verbosity::2
genfile .value.clear dry::1
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Clears all parameter values
- Parameter definitions remain (only values cleared)
- Mandatory parameters will need new values before materialization

**Examples:**

**Clear all values:**
```bash
genfile .value.clear
# Output:
# Cleared 2 parameter values
# Parameters ready for new values
```

**Dry run preview:**
```bash
genfile .value.clear dry::1
# Output:
# [DRY RUN] Would clear 2 parameter values:
#   project_name
#   port
# [DRY RUN] No changes made
```

**Exit Codes:** 0 (success) | 2 (runtime error)

**Related Commands:**
- [.value.set](#command-22-valueset) - Set new values after clearing
- [.parameter.remove](param_mgmt.md#command-21-parameterremove) - Remove definitions (not just values)

---

### See Also

- [Parameter Management](param_mgmt.md) - Parameter definition management
- [Dictionary: Parameter](../dictionary.md#parameter) - Parameter concept
- [Parameters Reference](../param.md) - CLI parameter documentation
