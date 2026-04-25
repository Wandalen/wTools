# 001 Macro Expansion

Code generation pipeline for the `#[clone_dyn]` attribute macro (`src/clone_dyn.rs`).

### Scope

- **Purpose:** Document how `clone_dyn.rs` transforms a trait definition into the trait plus four `Clone` impls.
- **Responsibility:** Specify each processing step, its inputs, outputs, and failure modes.
- **In Scope:** `src/clone_dyn.rs` function `clone_dyn`, attribute parsing via `ItemAttributes`, generic decomposition, where clause extension, `qt!` generation, debug path.
- **Out of Scope:** The `CloneDyn` trait itself (in `clone_dyn_types`), `clone_into_box` runtime (in `clone_dyn`), caller-visible contract (`api/001_clone_dyn_attr.md`).

### Pipeline

#### Step 1 — Parse Attribute Properties

`syn::parse::<ItemAttributes>(attr_input)` reads the attribute token stream. `ItemAttributes` holds a single property: `debug: AttributePropertyDebug` (boolean singletone, default `false`). Unknown properties produce a compile error listing the known keyword `"debug"`.

#### Step 2 — Parse Trait Definition

`syn::parse::<syn::ItemTrait>(item_input)` parses the annotated item. Any non-trait item fails here with a descriptive compile error; the original input is preserved in `original_input` for debug reporting.

#### Step 3 — Decompose Generic Parameters

`generic_params::decompose(&item_parsed.generics)` splits generics into four views:

- `generics_impl` — `<T: Bound>` form, used in `impl<...>` positions
- `generics_ty` — `<T>` form, used in `Trait<...>` positions
- `generics_where` — extracted where predicates, forwarded into each generated impl

#### Step 4 — Extend Where Clause

A synthetic predicate `Self: clone_dyn::CloneDyn` is appended to the trait's where clause (or a new clause is created if none exists). This enforces the object-safe clone bound without adding a visible supertrait to the trait definition signature.

#### Step 5 — Generate Token Stream

`qt!` (a `quote!` alias) emits:

- The mutated `item_parsed` trait (with extended where clause).
- Four `Clone` impl blocks for `Box<dyn Trait<T> + 'c>`, `+ Send + 'c`, `+ Sync + 'c`, `+ Send + Sync + 'c`. Each body calls `clone_dyn::clone_into_box(&**self)`.

All four impls carry `#[allow(non_local_definitions)]` to suppress the lint that fires when impls are generated outside the defining crate.

#### Step 6 — Debug Path (Optional)

When `has_debug` is `true`, `diag::report_print` formats and prints both the original input and generated output to stdout as a compilation diagnostic. Controlled by `#[clone_dyn(debug)]`.

### Cross-References

- **Feature:** `feature/001_clone_dyn_macro.md` — what this algorithm achieves and why
- **API:** `api/001_clone_dyn_attr.md` — caller-visible contract this algorithm satisfies
- **Source:** `src/clone_dyn.rs` lines 1–94 — canonical implementation
- **Upstream:** `macro_tools::generic_params::decompose` — generic decomposition utility
- **Runtime:** `clone_dyn::clone_into_box` — the object-safe clone dispatch (not in this crate)
