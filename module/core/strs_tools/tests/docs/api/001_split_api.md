# Split API

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| AP-1 | Builder produces iterator | Happy path | ✅ |
| AP-2 | Segment SplitType classification | Happy path | ✅ |
| AP-3 | Iterator borrows from source | Invariant | ✅ |
| AP-4 | Count limit caps segments | Boundary | ⛔ blocked: API not implemented |

## Cases

### AP-1: Builder produces iterator

- **Given:** Source string and delimiter configured via builder
- **When:** `perform()` is called on the builder
- **Then:** Returns an iterator over typed segments

### AP-2: Segment SplitType classification

- **Given:** Split result with delimiter preservation enabled
- **When:** Segments are iterated
- **Then:** Each segment has SplitType::Delimiter or SplitType::Delimited classification

### AP-3: Iterator borrows from source

- **Given:** Split with no stripping or unescaping
- **When:** Segments are collected
- **Then:** Segment content borrows from original source (zero-copy)

### AP-4: Count limit caps segments

- **Given:** Builder with count limit set to N
- **When:** Split is performed on input with more than N possible segments
- **Then:** Returns exactly N segments with remainder in final segment
- **Blocked:** `BasicSplitBuilder` has no `limit` or `count` method — count-limit is documented but not implemented
