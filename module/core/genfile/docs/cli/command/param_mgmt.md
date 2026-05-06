# Parameter Management

Parameter definition management commands for managing parameter schema and metadata. Controls parameter definitions (not values - see value.md for that).

- **Namespace:** parameter
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 19 | [.parameter.add](#command-19-parameteradd) | Add definition | 5 | Medium |
| 20 | [.parameter.list](#command-20-parameterlist) | List definitions | 1 | Low |
| 21 | [.parameter.remove](#command-21-parameterremove) | Remove definition | 2 | Low |

---

### Command :: 19. `.parameter.add`

Adds parameter definition to archive with metadata (name, description, default, mandatory flag). Use this to define template parameters.

**Syntax:**
```bash
genfile .parameter.add name::project_name mandatory::true
genfile .parameter.add name::port default::"8080" description::"Server port"
genfile .parameter.add name::author mandatory::false default::""
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `name::` | [IdentifierString](../type.md#type-identifierstring) | Parameter name | - | ✅ Yes |
| `mandatory::` | [MandatoryFlag](../type.md#type-mandatoryflag) | Whether parameter is required (0 or 1) | `0` | No |
| `default::` | [ContentString](../type.md#type-contentstring) | Default value for parameter | `null` | No |
| `description::` | [DescriptionText](../type.md#type-descriptiontext) | Parameter description | `""` | No |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Adds parameter definition to archive metadata
- Parameter name must be unique (error if exists)
- Mandatory parameters require values before materialization
- Default values used when value not explicitly set

**Examples:**

**Mandatory parameter:**
```bash
genfile .parameter.add name::project_name mandatory::true description::"Project name"
# Output:
# Added parameter: project_name (mandatory)
```

**Optional with default:**
```bash
genfile .parameter.add name::port mandatory::false default::"3000"
# Output:
# Added parameter: port (optional, default: 3000)
```

**Exit Codes:** 0 (success) | 1 (parameter already exists) | 2 (invalid name format)

**Interactions:**
- Typical workflow: `.parameter.add` (schema) → `.value.set` (instance data)
- Dependencies: None

**Related Commands:**
- [.parameter.list](#command-20-parameterlist) - List definitions
- [.value.set](value.md#command-22-valueset) - Set parameter values
- [.discover.parameters](operations.md#command-2-discoverparameters) - Auto-detect parameters

---

### Command :: 20. `.parameter.list`

Lists all parameter definitions in archive with metadata. Use this to inspect parameter schema.

**Syntax:**
```bash
genfile .parameter.list
genfile .parameter.list verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Lists all parameter definitions
- Shows name, mandatory flag, default value, description
- Sorted alphabetically by name

**Examples:**

**Basic listing:**
```bash
genfile .parameter.list
# Output:
# Parameters (3 total):
#   project_name (mandatory, no default)
#   port (optional, default: 3000)
#   author (optional, default: "")
```

**Verbose listing:**
```bash
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
#
# 3. author
#    Mandatory: no
#    Default: ""
#    Description: (none)
```

**Exit Codes:** 0 (success) | 2 (runtime error)

**Related Commands:**
- [.parameter.add](#command-19-parameteradd) - Add parameters
- [.value.list](value.md#command-23-valuelist) - List parameter values

---

### Command :: 21. `.parameter.remove`

Removes parameter definition from archive. Use this to clean up unused parameters.

**Syntax:**
```bash
genfile .parameter.remove name::old_param
genfile .parameter.remove name::unused verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `name::` | [IdentifierString](../type.md#type-identifierstring) | Parameter name to remove | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Removes parameter definition from archive
- Parameter must exist (error if not found)
- Also removes associated value if set

**Examples:**

**Basic usage:**
```bash
genfile .parameter.remove name::deprecated_param
# Output:
# Removed parameter: deprecated_param
```

**Exit Codes:** 0 (success) | 1 (parameter not found) | 2 (runtime error)

**Related Commands:**
- [.parameter.add](#command-19-parameteradd) - Add parameters
- [.value.clear](value.md#command-24-valueclear) - Clear values without removing definitions

---

### See Also

- [Value Operations](value.md) - Parameter value management
- [Dictionary: Parameter](../dictionary.md#parameter) - Parameter concept explanation
- [Parameters Reference](../param.md) - CLI parameter documentation
