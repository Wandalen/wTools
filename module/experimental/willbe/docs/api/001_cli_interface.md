# API: CLI Interface

### Scope

- **Purpose**: Document the `will` CLI commands, parameters, and binary entry points available to workspace maintainers.
- **Responsibility**: Complete reference for all willbe CLI commands with their parameters and usage.
- **In Scope**: Binary names, command syntax, parameters with types and defaults, exit codes, dry-run behavior.
- **Out of Scope**: Implementation details, architectural patterns (see `../pattern/`), usage workflows (see `../feature/`).

### Abstract

willbe exposes three binary entry points (`will`, `willbe`, `cargo-will`) that delegate to the same command logic via the `wca` (workspace command aggregator) framework. Commands are prefixed with `.` following wca conventions.

### Operations

**`.publish [path_glob] [dry:bool]`**
Publish workspace crates in topological dependency order. `path_glob` filters which crates to consider (default: all crates under current directory). `dry:1` prints the publish plan without executing; `dry:0` (default) executes. On execution, each package is version-bumped, committed to git with a version tag message, and pushed to the remote, then published to the crate registry.

**`.test [path_glob] [power:int] [dry:bool]`**
Run tests across workspace crates. `power` controls feature combination breadth (1=default features only, higher values test more combinations). `path_glob` and `dry` behave as in `.publish`.

**`.list [path_glob] [format:str]`**
List workspace crates. `format:tree` renders the dependency tree; default renders a flat list.

**`.cicd.renew`**
Regenerate GitHub Actions workflow files for all workspace crates from templates.

**`.readme.health.table.renew`**
Update README health/status badge table across workspace crates.

**`.readme.header.renew`**
Generate header in workspace root readme.md containing general status badge, discord link, gitpod example, and documentation links. Reads workspace metadata from Cargo.toml.

**`.readme.modules.headers.renew`**
Generate header for each workspace member crate readme.md containing crate-level status badge, discord link, gitpod example, and documentation links. Reads per-crate metadata from each module's Cargo.toml.

**`.readme.headers.renew`**
Aggregation of `.readme.header.renew` and `.readme.modules.headers.renew` — generates headers in both workspace root and all member crate readmes in one pass.

**`.features [path_glob] [with_features_deps:bool]`**
List feature configurations for workspace crates. `with_features_deps:1` additionally displays feature dependency trees.

**`.publish.diff [path_glob]`**
Show differences between local crate state and published crates.io version.

**`.workspace.renew [branches:list] [repository_url:str]`**
Create workspace template: generates static files and directories for a new workspace. `branches` (required) specifies project branches for Cargo.toml metadata. `repository_url` (required) specifies the project repository link.

**`.deploy.renew [gcp_project_id:str] [gcp_region:str] [gcp_artifact_repo_name:str] [docker_image_name:str]`**
Create deployment template with GCP integration. `gcp_project_id` (required) identifies the Google Cloud project. Optional: `gcp_region` (default: europe-central2), `gcp_artifact_repo_name`, `docker_image_name`.

**`.crate.doc [path] [output:path]`**
Generate documentation for a single crate as a Markdown file. `path` specifies the crate directory (default: current directory). `output` specifies the output file path (default: `{crate_name}_doc.md` in the crate directory).

### Error Handling

Commands return a non-zero exit code on failure with an error message to stderr. The `.publish` command fails fast: if any package in the publish plan fails (pack, publish, git, or network), the entire run is aborted and subsequent packages are not published. No rollback is performed for already-published packages.

### Compatibility Guarantees

Semantic versioning. No backward compatibility guarantees while willbe is at `0.x.y`. willbe is a legacy crate; the CLI interface is stable for existing usage but no new commands or parameters will be added.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/bin/will.rs` | Binary entry point: will |
| source | `../../src/bin/willbe.rs` | Binary entry point: willbe |
| source | `../../src/bin/cargo-will.rs` | Binary entry point: cargo-will |
| source | `../../src/command/mod.rs` | Command aggregator: wca grammar for all 13 commands |
| source | `../../src/command/publish.rs` | .publish command definition |
| source | `../../src/command/test.rs` | .test command definition |
| source | `../../src/command/list.rs` | .list command definition |
| source | `../../src/command/cicd_renew.rs` | .cicd.renew command definition |
| source | `../../src/command/readme_health_table_renew.rs` | .readme.health.table.renew command definition |
| source | `../../src/command/readme_headers_renew.rs` | .readme.headers.renew command definition |
| source | `../../src/command/main_header.rs` | .readme.header.renew command definition |
| source | `../../src/command/readme_modules_headers_renew.rs` | .readme.modules.headers.renew command definition |
| source | `../../src/command/features.rs` | .features command definition |
| source | `../../src/command/publish_diff.rs` | .publish.diff command definition |
| source | `../../src/command/workspace_renew.rs` | .workspace.renew command definition |
| source | `../../src/command/deploy_renew.rs` | .deploy.renew command definition |
| source | `../../src/command/crate_doc.rs` | .crate.doc command definition |
| source | `../../src/action/publish.rs` | Publish action implementation |
| doc | [feature/001_workspace_management.md](../feature/001_workspace_management.md) | Feature-level workspace management documentation |
| doc | [feature/002_enhanced_publish_algorithm.md](../feature/002_enhanced_publish_algorithm.md) | Enhanced publish algorithm (not implemented, legacy) |
| doc | [pattern/001_layer_architecture.md](../pattern/001_layer_architecture.md) | Five-layer source architecture |
