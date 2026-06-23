# String Isolation

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| FT-1 | Left isolation | Happy path | ✅ |
| FT-2 | Right isolation | Happy path | ✅ |
| FT-3 | Between isolation | Happy path | ⛔ blocked: API not implemented |
| FT-4 | Delimiter not found | Boundary | ✅ |
| FT-5 | Between with missing right delimiter | Boundary | ⛔ blocked: API not implemented |

## Cases

### FT-1: Left isolation

- **Given:** Input `"hello::world"` with delimiter `"::"`
- **When:** Left isolation is performed
- **Then:** Returns `Some("hello")`

### FT-2: Right isolation

- **Given:** Input `"hello::world"` with delimiter `"::"`
- **When:** Right isolation is performed
- **Then:** Returns `Some("world")`

### FT-3: Between isolation

- **Given:** Input `"a[X]b"` with left delimiter `"["` and right delimiter `"]"`
- **When:** Between isolation is performed
- **Then:** Returns `Some("X")`
- **Blocked:** `isolate_between()` not implemented — `src/string/isolate.rs` only provides `isolate_left()` and `isolate_right()`

### FT-4: Delimiter not found

- **Given:** Input `"no delimiter here"`
- **When:** Left isolation with delimiter `"::"` is performed
- **Then:** Returns `None`

### FT-5: Between with missing right delimiter

- **Given:** Input `"a[X"` with left `"["` and right `"]"`
- **When:** Between isolation is performed
- **Then:** Returns `None` — right delimiter absent
- **Blocked:** `isolate_between()` not implemented — same as FT-3
