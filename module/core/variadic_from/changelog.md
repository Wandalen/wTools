# Changelog

*   **2025-06-29:**
    *   Implemented the `VariadicFrom` derive macro and `from!` helper macro, adhering to `spec.md`. Defined `FromN` traits, added blanket `From1` implementations, implemented `from!` macro with argument count validation, and ensured the derive macro generates `FromN` and `From<T>`/`From<tuple>` implementations based on field count (1-3 fields). Removed `#[from(Type)]` attribute handling. All generated code compiles without errors, passes tests (including doc tests, with `Readme.md` examples now runnable), and adheres to `clippy` warnings. Improved `Readme.md` content and scaffolding for new developers.

*   **2025-07-01:**
    *   Generalized `CONTRIBUTING.md` to be about all crates of the `wTools` repository, including updating the title, removing specific crate paths, and generalizing commit message examples.

*   [2025-07-06] Refactored `variadic_from_meta` to align with spec v1.1.

*   [Increment 1 | 2025-07-06 15:54 UTC] Cleaned up test directory and refactored library structure.

*   [Increment 2 | 2025-07-06 16:07 UTC] Refactored macro input parsing using `macro_tools`.

*   [Increment 3 | 2025-07-06 16:11 UTC] Implemented core `FromN` and `From<Tuple>` generation.

*   [Increment 4 | 2025-07-06 16:13 UTC] Implemented conditional convenience `FromN` generation.
