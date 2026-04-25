# 001 clone_dyn Attribute

Public API contract for the `#[clone_dyn]` attribute macro.

### Scope

- **Purpose:** Define the caller-visible contract for `#[clone_dyn]`: valid forms, accepted properties, input requirements, and guaranteed outputs.
- **Responsibility:** Be the authoritative specification that callers and the `clone_dyn` facade rely on.
- **In Scope:** Valid invocation forms, attribute properties, trait item constraints, generated impl guarantees.
- **Out of Scope:** Internal parsing logic (`algorithm/001_macro_expansion.md`), end-user ergonomics (handled by `clone_dyn` facade).

### Invocation

Access via the `clone_dyn` facade (not directly from this crate):

```toml
[dependencies]
clone_dyn = { version = "*" }
```

```rust
use clone_dyn::clone_dyn;

#[clone_dyn]
pub trait MyTrait { ... }
```

### Parameters

| Parameter | Form | Effect |
|-----------|------|--------|
| _(none)_ | `#[clone_dyn]` | Standard expansion: supertrait injection + 4 `Clone` impls |
| `debug` | `#[clone_dyn(debug)]` | Same as above, plus prints expanded tokens to stdout |

### Input Constraints

- Input MUST be a trait item (`syn::ItemTrait`). Applying to structs, enums, or functions is a compile error.
- All generic parameters of the input trait are preserved verbatim in generated impls.
- Existing `where` clauses are extended (not replaced) with `Self: clone_dyn::CloneDyn`.

### Generated Output

For `#[clone_dyn] trait Foo<T>`, the macro emits:

1. The original trait with `where Self: clone_dyn::CloneDyn` added.
2. `impl<'c, T> Clone for Box<dyn Foo<T> + 'c>` — delegates to `clone_dyn::clone_into_box`.
3. `impl<'c, T> Clone for Box<dyn Foo<T> + Send + 'c>`
4. `impl<'c, T> Clone for Box<dyn Foo<T> + Sync + 'c>`
5. `impl<'c, T> Clone for Box<dyn Foo<T> + Send + Sync + 'c>`

### Cross-References

- **Feature:** `feature/001_clone_dyn_macro.md` — feature rationale and attribute configuration
- **Algorithm:** `algorithm/001_macro_expansion.md` — implementation of the expansion steps
- **Downstream:** `clone_dyn` — re-exports this attribute and provides `CloneDyn` + `clone_into_box`
