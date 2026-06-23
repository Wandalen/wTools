# Text Indentation

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| FT-1 | Multi-line prefix | Happy path | ✅ |
| FT-2 | Empty source string | Boundary | ⏳ |
| FT-3 | Source with only newlines | Boundary | ⏳ |
| FT-4 | Prefix and postfix applied | Happy path | ✅ |

## Cases

### FT-1: Multi-line prefix

- **Given:** Input `"line1\nline2\nline3"` with prefix `"  "`
- **When:** Indentation is applied
- **Then:** Every line is prefixed: `"  line1\n  line2\n  line3"`

### FT-2: Empty source string

- **Given:** Empty string `""`
- **When:** Indentation with prefix `">"` is applied
- **Then:** Returns empty string or single prefixed empty line

### FT-3: Source with only newlines

- **Given:** Input `"\n\n\n"` with prefix `"| "`
- **When:** Indentation is applied
- **Then:** Each line receives the prefix identically

### FT-4: Prefix and postfix applied

- **Given:** Input `"a\nb"` with prefix `"["` and postfix `"]"`
- **When:** Indentation is applied
- **Then:** Returns `"[a]\n[b]"` — both prefix and postfix wrap each line
