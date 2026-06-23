# Parser Integration API

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| AP-1 | Builder produces token iterator | Happy path | ✅ |
| AP-2 | Transformation callbacks applied | Happy path | ✅ |
| AP-3 | Memory proportional to largest token | Invariant | ✅ |
| AP-4 | Token classification variants | Happy path | ✅ |

## Cases

### AP-1: Builder produces token iterator

- **Given:** Source string configured via parser builder
- **When:** Parser is executed
- **Then:** Returns an iterator over classified tokens

### AP-2: Transformation callbacks applied

- **Given:** Parser with zero or more callbacks registered
- **When:** Tokens are yielded
- **Then:** Each token reflects all registered transformations in registration order

### AP-3: Memory proportional to largest token

- **Given:** Large input string with many tokens
- **When:** Parser iterates through tokens
- **Then:** No intermediate collection — memory usage proportional to largest single token

### AP-4: Token classification variants

- **Given:** Input with mixed content
- **When:** Tokens are yielded
- **Then:** Each token classified as delimited segment, delimiter, or structural boundary
