# Examples

Runnable usage examples for `workspace_tools`.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `000_hello_workspace.rs` | Introduce workspace resolution fundamentals |
| `001_standard_directories.rs` | Demonstrate standard directory layout access |
| `002_path_operations.rs` | Demonstrate path joining, validation, and boundary checking |
| `003_error_handling.rs` | Demonstrate error handling patterns and workspace errors |
| `004_resource_discovery.rs` | Demonstrate glob-based resource discovery (`glob` feature) |
| `005_secret_management.rs` | Demonstrate secret file loading with environment fallbacks (`secrets` feature) |
| `006_improved_secrets_api.rs` | Demonstrate enhanced secrets API with error handling (`secrets` feature) |
| `006_testing_integration.rs` | Demonstrate testing utilities and TempWorkspace integration |
| `007_real_world_cli_app.rs` | Demonstrate complete CLI application using workspace resolution |
| `008_web_service_integration.rs` | Demonstrate web service configuration via workspace paths |
| `009_advanced_patterns.rs` | Demonstrate advanced workspace usage patterns |
| `010_cargo_and_serde_integration.rs` | Demonstrate cargo metadata and serde config integration (`serde` feature) |
| `resource_discovery.rs` | Minimal glob resource discovery demo (`glob` feature) |
| `secret_management.rs` | Minimal secret management demo (`secrets` feature) |
| `workspace_basic_usage.rs` | Minimal workspace path resolution demo |

## Running Examples

```bash
# Core examples (no extra features needed)
cargo run --example 000_hello_workspace
cargo run --example 001_standard_directories
cargo run --example 002_path_operations
cargo run --example 003_error_handling
cargo run --example 006_testing_integration
cargo run --example 007_real_world_cli_app
cargo run --example 008_web_service_integration
cargo run --example 009_advanced_patterns
cargo run --example workspace_basic_usage

# Feature-gated examples
cargo run --example 004_resource_discovery --features glob
cargo run --example 005_secret_management --features secrets
cargo run --example 006_improved_secrets_api --features secrets
cargo run --example 010_cargo_and_serde_integration --features serde
cargo run --example resource_discovery --features glob
cargo run --example secret_management --features secrets
```
