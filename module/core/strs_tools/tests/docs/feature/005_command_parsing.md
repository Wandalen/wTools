# Command Parsing

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| FT-1 | Single command | Happy path | ✅ |
| FT-2 | Command with arguments | Happy path | ✅ |
| FT-3 | Multiple whitespace separators | Boundary | ✅ |
| FT-4 | Empty input string | Boundary | ✅ |

## Cases

### FT-1: Single command

- **Given:** Input `"ls"`
- **When:** Command parsing is performed
- **Then:** Returns command name `"ls"` with empty argument list

### FT-2: Command with arguments

- **Given:** Input `"git push origin main"`
- **When:** Command parsing is performed
- **Then:** Returns command `"git"` with arguments `["push", "origin", "main"]`

### FT-3: Multiple whitespace separators

- **Given:** Input `"cmd  arg1   arg2"` with extra spaces
- **When:** Command parsing is performed
- **Then:** Returns command `"cmd"` with arguments `["arg1", "arg2"]` — whitespace collapsed

### FT-4: Empty input string

- **Given:** Empty string `""`
- **When:** Command parsing is performed
- **Then:** Returns empty/default result
