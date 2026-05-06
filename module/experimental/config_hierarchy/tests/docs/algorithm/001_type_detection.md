# Algorithm Spec: Type Detection

### Scope

- **Element:** `algorithm/001_type_detection`
- **Source:** `docs/algorithm/001_type_detection.md`
- **Prefix:** `AC-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| AC-01 | boolean_true_all_variants | nominal | ✅ |
| AC-02 | boolean_false_all_variants | nominal | ✅ |
| AC-03 | integer_string_to_number | nominal | ✅ |
| AC-04 | finite_float_to_number | nominal | ✅ |
| AC-05 | nan_inf_fall_through_to_string | edge | ✅ |
| AC-06 | everything_else_falls_through | nominal | ✅ |
| AC-07 | zero_string_is_boolean_not_integer | ordering | ⏳ |
| AC-08 | integer_overflow_cascades_to_float | boundary | ⏳ |

---

### AC-01: boolean true variants all detected

- **Given:** Algorithm receives one of `"true"`, `"yes"`, `"1"`, `"on"` (any casing)
- **When:** `detect_and_convert_value()` is called
- **Then:** Returns `JsonValue::Bool(true)` — not a string, not a number
- **Tests:** `tests/type_detection_tests.rs::test_boolean_true_values`

### AC-02: boolean false variants all detected

- **Given:** Algorithm receives one of `"false"`, `"no"`, `"0"`, `"off"` (any casing)
- **When:** `detect_and_convert_value()` is called
- **Then:** Returns `JsonValue::Bool(false)` — not a string, not a number
- **Tests:** `tests/type_detection_tests.rs::test_boolean_false_values`

### AC-03: integer string detected as Number

- **Given:** A signed integer string such as `"42"`, `"-100"`, `"999999999"`
- **When:** `detect_and_convert_value()` is called
- **Then:** Returns `JsonValue::Number` with the integer value — not a string
- **Tests:** `tests/type_detection_tests.rs::test_integer_values`

### AC-04: finite float string detected as Number

- **Given:** A decimal or scientific float string such as `"3.14"`, `"1.23e-4"` that parses to a finite `f64`
- **When:** `detect_and_convert_value()` is called
- **Then:** Returns `JsonValue::Number` with the float value — not a string
- **Tests:** `tests/type_detection_tests.rs::test_float_values`

### AC-05: non-finite float falls through to String

- **Given:** Inputs `"NaN"`, `"Inf"`, `"-Inf"` — these parse as `f64` but are non-finite
- **When:** `detect_and_convert_value()` is called
- **Then:** Returns `JsonValue::String` with the original text — NOT `JsonValue::Number` (non-finite f64 cannot be serialized as JSON Number)
- **Tests:** `tests/type_detection_tests.rs::test_non_finite_float_fallback`

### AC-06: all other inputs fall through to String

- **Given:** An arbitrary string that is not a boolean literal, integer, or finite float (e.g., `"hello"`, `"2023-10-19"`, `"🔥"`, empty string, whitespace)
- **When:** `detect_and_convert_value()` is called
- **Then:** Returns `JsonValue::String` with the original input unchanged
- **Tests:** `tests/type_detection_tests.rs::test_string_fallback`, `test_unicode_strings`, `test_empty_string`, `test_whitespace_string`

### AC-07: `"0"` maps to boolean false, not integer zero

- **Given:** Input string `"0"` — a value that matches both the boolean-false literal table and a valid integer
- **When:** `detect_and_convert_value()` is called
- **Then:** Returns `JsonValue::Bool(false)` — not `JsonValue::Number(0)` — because the boolean check runs before the integer check per the algorithm step order
- **Tests:** `tests/type_detection_tests.rs::test_zero_is_boolean_not_integer`

### AC-08: integer string overflowing i64 cascades to float Number

- **Given:** Input string `"99999999999999999999"` — too large to fit in a signed 64-bit integer but within 64-bit float range
- **When:** `detect_and_convert_value()` is called
- **Then:** Returns `JsonValue::Number` with the nearest float64 approximation — the i64 parse fails, the f64 parse succeeds and the value is finite, so it is returned as a float number rather than a String
- **Tests:** `tests/type_detection_tests.rs::test_integer_overflow_cascades_to_float`
