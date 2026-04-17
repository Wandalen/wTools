# Feature: CI/CD Environment Detection

### Statement

The `environment` layer provides `is_cicd()` to detect whether the current process is running inside a continuous integration or deployment system. It checks standard environment variables set by GitHub Actions, GitLab CI, Travis CI, CircleCI, Jenkins, and any system setting the generic `CI` variable. The function is behind the `process_environment_is_cicd` feature flag so callers opt in explicitly; it has no additional dependencies.

### Status

- **Version introduced:** 0.5.0
- **Stability:** stable
- **Module path:** `process_tools::environment`
- **Feature gate:** `process_environment_is_cicd`
