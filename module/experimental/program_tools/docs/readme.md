# Docs

### Scope

Design and API documentation for `program_tools`.

### System Actors

| Actor | Role |
|-------|------|
| Developer | Invokes the programmatic API from test code, build scripts, or application code |
| Test Harness | Drives the runner through single-expression test utilities inside Rust test functions |
| CI System | Executes test suites that use `program_tools` for integration testing of Rust programs |
| CLI User | Invokes script execution from the command line or shell scripts |
| Cargo Toolchain | The build tool invoked by the runner to compile source files |
| Temporary Filesystem | OS filesystem used for isolated workspace creation, file materialization, and cleanup |
| Artifact Cache | Persistent build directory shared across multiple runs to reduce compilation time |

### Vocabulary

| Term | Definition |
|------|------------|
| Plan | Top-level execution configuration containing one program and all runner parameters |
| Program | An ordered collection of source files representing a compilable crate |
| Source | An atomic unit: a single source file identified by a path and its code content |
| Runner | The component that materializes a plan, invokes Cargo, captures output, and cleans up |
| Script Execution | The process of compiling and running Rust source transparently, hiding all build complexity |
| CapturedOutput | The structured result of a completed run: exit status, standard output, and standard error |
| RunOptions | Runtime configuration parameters applied to a single execution |
| TempWorkspace | An isolated temporary directory created for a single run, removed afterward |
| Build Profile | The Cargo build mode — debug or release — applied during compilation |
| Artifact Cache | A persistent build directory shared across runs to reduce recompilation |
| Output Expectation | A declarative assertion about the expected output of a completed run |
| Execution Isolation | The guarantee that each run operates in its own independent workspace |

### Responsibility Table

| File | Responsibility |
|--------|----------------|
| `entities.md` | Master index of all doc entity types and instances |
| `doc_graph.yml` | Directed cross-reference graph: 15 nodes, annotated edges |
| `api/` | Public API surface: builders, runner entry points, output assertions, CLI |
| `feature/` | User-facing capabilities: execution, capture, artifacts, test integration, configuration |
| `invariant/` | Correctness guarantees: cleanup, isolation, determinism, error propagation |
| `pattern/` | Reusable design patterns: builder hierarchy, layered configuration |
