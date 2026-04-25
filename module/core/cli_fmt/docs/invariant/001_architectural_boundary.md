# Architectural Boundary: cli_fmt vs strs_tools

### Statement

`cli_fmt` implements CLI-specific policy decisions. `strs_tools` provides general-purpose
text and ANSI manipulation without application-domain assumptions. No CLI-specific logic
belongs in `strs_tools`.

### Rationale

`strs_tools` is designed to be reusable across any application: web servers, embedded
systems, log processors, and CLI tools alike. CLI-specific policy decisions embedded in
`strs_tools` would impose unwanted assumptions on non-CLI consumers.

Policies that belong exclusively in `cli_fmt`:

- **Stream ordering** — stderr before stdout reflects CLI convention (errors visible first),
  not a general-purpose text-merging rule
- **Head/tail semantics** — match Unix tool conventions for output volume control; not a
  general string-slicing contract
- **Builder pattern configuration** — optimized for CLI configuration chains; strs_tools
  functions take plain parameters
- **Output metadata** — tracking `lines_omitted` and `width_truncated` is a CLI transparency
  concern; general-purpose functions return only the transformed text

### Enforcement

- `cli_fmt` depends on `strs_tools` (one direction only)
- `strs_tools` has zero knowledge of `cli_fmt`; it carries no stream concepts, head/tail
  conventions, or output-transparency types
- New CLI-specific utilities belong in `cli_fmt`, not `strs_tools`
- New general-purpose text/ANSI utilities belong in `strs_tools`, not `cli_fmt`
- Feature flags in `cli_fmt` are independent of `strs_tools` feature flags

### Violations

Placing CLI-specific policy in `strs_tools` breaks reusability for non-CLI consumers,
which would inherit CLI assumptions they don't need. Placing general text utilities in
`cli_fmt` prevents their reuse outside CLI applications and increases coupling.
