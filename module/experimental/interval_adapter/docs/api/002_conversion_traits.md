# API: Conversion Traits

### Scope

- **Purpose**: Document the three helper traits that enable bound conversion, endpoint constraint bundling, and canonical interval creation.
- **Responsibility**: Canonical reference for the bound extension, endpoint constraint, and canonical conversion traits — operations and contracts.
- **In Scope**: Trait operations, bound conversion semantics, endpoint type requirements, and the canonical conversion contract.
- **Out of Scope**: Interval query traits (→ `api/001`); canonical type layout (→ `data_structure/001`); design rationale (→ `pattern/`).

### Abstract

Three auxiliary traits supporting the core interval abstraction. The endpoint constraint trait bundles the constraints all interval endpoint types must satisfy. The bound extension trait extends the standard bound type with closed-form conversion methods. The canonical conversion trait converts any interval type to the canonical interval representation. The standard bound and range-bounds types are re-exported at the crate root so callers do not need to import them from the core library directly.

### Operations

#### EndPointTrait

Constraint bundle for interval endpoint types. Blanket-implemented for all types satisfying all of: partial ordering, subtraction and addition (both returning the same type), cloneability, copyability, and fixed size. No methods — exists solely as a combined bound to avoid repeating five constraints everywhere.

#### BoundExt

Extension methods on the standard bound type, enabling conversion to closed-interval endpoint values. Provides the left/right asymmetric conversion logic required for closed-interval arithmetic. Implemented by providing the trait because the bound type is a foreign type that cannot receive new methods directly.

| Method | Returns | Notes |
|--------|---------|-------|
| `into_left_closed` | left closed endpoint value | Convert bound to a left endpoint of a closed interval |
| `into_right_closed` | right closed endpoint value | Convert bound to a right endpoint of a closed interval |

**Bound conversion:**

| Input Bound | Left closed result | Right closed result |
|-------------|---------------------|----------------------|
| Included value | the value | the value |
| Excluded value | next integer (increment by one) | previous integer (decrement by one) |
| Unbounded | minimum sentinel value | maximum sentinel value |

#### IntoInterval

Conversion trait for producing a canonical interval from any interval type. Blanket-implemented for all types implementing the non-iterable-interval trait.

| Method | Returns | Notes |
|--------|---------|-------|
| `into_interval` | canonical interval | Consume self and produce canonical interval |

### Error Handling

All conversion methods are infallible — the closed-form conversion methods and the canonical conversion method return values unconditionally. For unbounded bounds, minimum or maximum sentinel values are returned rather than an error.

### Compatibility Guarantees

- The sentinel values for unbounded bounds are fixed at the minimum and maximum of the platform-native signed integer type and will not change across minor releases.
- The endpoint constraint trait is blanket-implemented and callers do not implement it directly; additions to its constraints require a major version bump.
- The canonical conversion trait is blanket-implemented for all non-iterable-interval types; new implementations are non-breaking additions.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_interval_traits.md](001_interval_traits.md) | Non-iterable-interval and iterable-interval traits using these helpers |
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | Canonical type constructed via the conversion trait |
| doc | [feature/001_generic_interval_parameter.md](../feature/001_generic_interval_parameter.md) | Generic parameter pattern using the conversion trait for storage |
| doc | [feature/002_non_iterable_intervals.md](../feature/002_non_iterable_intervals.md) | Bound extension sentinel values for unbounded ends |
| doc | [invariant/001_integer_endpoints_only.md](../invariant/001_integer_endpoints_only.md) | Endpoint constraint trait excludes floats |
| doc | [pattern/002_canonical_interval_type.md](../pattern/002_canonical_interval_type.md) | Conversion trait as the canonical entry point |
