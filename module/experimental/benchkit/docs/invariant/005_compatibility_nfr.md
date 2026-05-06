# Invariant: Platform and Environment Compatibility

### Scope

- **Purpose**: Ensure benchkit operates correctly across the standard Rust deployment environments and operating systems.
- **Responsibility**: States the mandatory platform and standard-library compatibility requirements.
- **In Scope**: Standard-library requirement for full feature set; no-std requirement for core timing; supported operating systems.
- **Out of Scope**: Browser/WASM targets (not supported); embedded targets without OS (not a goal); minimum Rust edition (governed by Cargo.toml).

### Invariant Statement

Three compatibility guarantees must hold:
1. All features must compile and function correctly in `std` environments on Linux, macOS, and Windows (the three major desktop/server platforms).
2. The core timing functionality — measuring elapsed duration of a closure — must compile and function correctly in `no_std` environments that provide a monotonic clock (e.g., RTOS with timer support).
3. No feature may introduce a platform-specific compile error on any of the three required platforms. Platform-specific code must be conditionally compiled and must have a fallback or stub on unsupported platforms.

### Enforcement Mechanism

CI runs the full test suite on Linux, macOS, and Windows matrix. `no_std` compatibility for the core measurement module is verified by a separate compilation target in CI that disables the standard library. Any PR that introduces a platform-specific compile error on a required platform fails CI automatically.

Feature flags gate all platform-specific dependencies (e.g., file I/O for report generation is behind `markdown_reports`; chart rendering behind `visualization`). The `enabled` feature (core timing) has no platform-specific dependencies.

### Violation Consequences

A compile error on any required platform breaks benchkit for the developer community on that platform. Since benchkit is a dev-dependency, such a break forces the developer to either exclude the platform from their own CI or remove benchkit entirely.

`no_std` incompatibility in the core timing module prevents benchkit from being used in embedded and RTOS contexts where performance measurement is equally important.

### Cross-References

| Type   | File                                     | Responsibility                                              |
|--------|------------------------------------------|-------------------------------------------------------------|
| source | `src/measurement.rs`                     | Core timing implementation — must satisfy no_std constraint |
| config | `Cargo.toml`                             | Feature flags gating platform-specific dependencies         |
| test   | `tests/`                                 | Platform compatibility CI matrix                            |
| doc    | `docs/feature/001_measurement_timing.md` | Feature whose platform support this invariant governs       |
