# Feature: Workspace Management

### Scope

- **Purpose**: Manage multi-crate Rust workspaces: publish crates in dependency order, generate CI/CD workflows, and maintain consistency across crates.
- **Responsibility**: Document the end-to-end workspace management capabilities exposed by the `will` CLI.
- **In Scope**: Multi-crate publishing with topological ordering, version bumping, CI/CD generation, health tables, dependency analysis, test execution with feature combinations, workspace listing and scaffolding.
- **Out of Scope**: Crate development/code generation, git version control operations, crates.io account management, IDE integration.

### Design

willbe manages multi-crate workspaces through 13 CLI commands targeting different workspace concerns:

| Command | Purpose |
|---------|---------|
| `.publish` | Publish crates in dependency order; supports `dry:1` for plan-only mode |
| `.publish.diff` | Show differences between local and published crate versions |
| `.test` | Run tests with varied feature combinations across workspace crates |
| `.list` | List workspace crates with optional tree format |
| `.cicd.renew` | Regenerate GitHub Actions workflow files for all workspace crates |
| `.readme.health.table.renew` | Update README health/status badge table |
| `.readme.header.renew` | Generate workspace-level readme header with badges and links |
| `.readme.modules.headers.renew` | Generate per-crate readme headers with badges and links |
| `.readme.headers.renew` | Aggregation of header.renew + modules.headers.renew |
| `.features` | List feature configurations for workspace crates |
| `.workspace.renew` | Create workspace template with static files and directories |
| `.deploy.renew` | Create GCP deployment template with Makefile-based deployment |
| `.crate.doc` | Generate single-file Markdown documentation for a crate |

The publish workflow builds a dependency graph, computes a topologically sorted publish order, checks which packages need publishing (local version absent from crates.io), and executes publish + git commit + git push per package.

**Known limitations (won't fix — legacy crate)**: Four interacting algorithmic defects cause the publish command to select a superset of required packages (graph traversal direction, cascade bypass, wrong oracle, staleness over-trigger). Full root-cause analysis is in the cross-referenced task file.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/command/mod.rs` | Command aggregator: wca grammar for all 13 commands |
| source | `../../src/action/publish.rs` | Publish action: plan construction and execution |
| source | `../../src/action/test.rs` | Test action: feature-combination test execution |
| source | `../../src/action/list.rs` | List action: crate enumeration and tree rendering |
| source | `../../src/tool/graph.rs` | Dependency graph: subgraph DFS, remove_not_required_to_publish |
| source | `../../src/entity/package.rs` | Package entity: publish_need oracle |
| source | `../../src/entity/staleness.rs` | Staleness detection: detect_stale_dependencies |
| doc | [api/001_cli_interface.md](../api/001_cli_interface.md) | Complete CLI command reference |
| doc | [pattern/001_layer_architecture.md](../pattern/001_layer_architecture.md) | Five-layer source architecture |
| doc | [feature/002_enhanced_publish_algorithm.md](002_enhanced_publish_algorithm.md) | Planned staleness-aware algorithm (not implemented) |
| task | `../../task/backlog/001_wrong_publish_set_graphs_tools.md` | Known algorithmic bugs (won't fix, legacy) |
