# Parser Integration

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| FT-1 | Single-pass integer parsing | Happy path | ✅ |
| FT-2 | Transformation callback | Happy path | ✅ |
| FT-3 | Token classification | Happy path | ✅ |
| FT-4 | Empty input | Boundary | ✅ |
| FT-5 | Error position tracking | Happy path | ✅ |

## Cases

### FT-1: Single-pass integer parsing

- **Given:** Input `"1,2,3"` with delimiter `","`
- **When:** Parser splits and transforms to integers in one pass
- **Then:** Returns parsed integers `[1, 2, 3]`

### FT-2: Transformation callback

- **Given:** Parser with trim callback registered
- **When:** Input with whitespace-padded tokens is parsed
- **Then:** Yielded tokens have whitespace trimmed

### FT-3: Token classification

- **Given:** Input `"cmd --flag value"`
- **When:** Parser classifies tokens
- **Then:** Each token carries correct classification (command, flag, value)

### FT-4: Empty input

- **Given:** Empty string `""`
- **When:** Parser is invoked
- **Then:** No tokens yielded

### FT-5: Error position tracking

- **Given:** Input with invalid token `"1,abc,3"`
- **When:** Parser attempts integer conversion
- **Then:** Error includes position and token name information
