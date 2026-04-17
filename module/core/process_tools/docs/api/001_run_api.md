# API: Run Builder

### Statement

`Run` is the configuration struct for a subprocess invocation. It is constructed via `Run::former()` which returns a `RunFormer` builder. Fields: `bin_path: PathBuf`, `current_path: PathBuf`, `args: Vec<OsString>`, `joining_streams: bool` (default `false`), `env_variable: HashMap<String, String>`. `RunFormer` exposes `.run() -> Result<Report, Report>` for direct execution and `.run_with_shell(exec_path: &str) -> Result<Report, Report>` for cross-platform shell invocation.

### Status

- **Version:** 0.1.0+
- **Module path:** `process_tools::process::{Run, RunFormer}`
- **Derives:** `Debug`, `Former`
