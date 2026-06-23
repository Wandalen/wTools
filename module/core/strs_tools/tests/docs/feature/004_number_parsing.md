# Number Parsing

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| FT-1 | Integer parse | Happy path | ✅ |
| FT-2 | Float parse | Happy path | ✅ |
| FT-3 | Scientific notation | Boundary | ⏳ |
| FT-4 | Invalid input | Error | ⏳ |
| FT-5 | Integer overflow | Boundary | ⏳ |

## Cases

### FT-1: Integer parse

- **Given:** Input `"42"`
- **When:** Parsed as i32
- **Then:** Returns `Ok(42)`

### FT-2: Float parse

- **Given:** Input `"3.14"`
- **When:** Parsed as f32
- **Then:** Returns `Ok(3.14)`

### FT-3: Scientific notation

- **Given:** Input `"1.5e10"`
- **When:** Parsed as f64
- **Then:** Returns `Ok(15000000000.0)`

### FT-4: Invalid input

- **Given:** Input `"not_a_number"`
- **When:** Parsed as i32
- **Then:** Returns typed error indicating parse failure

### FT-5: Integer overflow

- **Given:** Input `"99999999999999999999"`
- **When:** Parsed as i32
- **Then:** Returns typed error indicating overflow
