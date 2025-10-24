# Test Organization Enforcement Tools

This directory contains automated tools for validating and maintaining the systematic test organization standards defined in `../readme.md`.

## Tools Overview

### 1. `test_organization_validator.rs`
**Core validation library**
- Rust library providing validation logic for test organization
- Validates naming conventions, directory structure, and categorization
- Generates detailed violation reports
- Used by other tools as the foundation validation engine

### 2. `validate_organization.rs`
**CLI validation tool**
- Standalone executable for validating test organization
- Can be run manually or in CI/CD pipelines
- Provides detailed reports and exit codes for automation
- Usage: `rustc validate_organization.rs && ./validate_organization`

### 3. `pre_commit_test_organization.sh`
**Git pre-commit hook**
- Prevents commits that violate test organization standards
- Validates only staged test files for efficiency
- Provides colored output with clear violation explanations
- Installation: Copy to `.git/hooks/pre-commit` and make executable

### 4. `test_organization_ci.yml`
**GitHub Actions workflow**
- Automated validation in CI/CD pipelines
- Runs on pull requests and pushes affecting test files
- Generates organization reports in CI output
- Place in `.github/workflows/test_organization.yml`

### 5. `Makefile.test_organization`
**Make targets for validation**
- Convenient make targets for common validation tasks
- Supports installation of pre-commit hooks
- Generates organization reports
- Include in main Makefile or use directly

## Quick Setup

### For Development
```bash
# Install pre-commit hook
cp tests/tools/pre_commit_test_organization.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# Or use Makefile
make -f tests/tools/Makefile.test_organization install-test-hooks
```

### For CI/CD
```bash
# Copy GitHub Actions workflow
cp tests/tools/test_organization_ci.yml .github/workflows/test_organization.yml
```

### Manual Validation
```bash
# Using Makefile
make -f tests/tools/Makefile.test_organization validate-tests

# Using shell script
./tests/tools/pre_commit_test_organization.sh

# Generate report
make -f tests/tools/Makefile.test_organization test-report
```

## Validation Rules

### Prohibited Naming Patterns
❌ **Avoid task-based naming:**
- `task_024_*`
- `issue_017_*`
- `fix_*`
- `bug_*`
- `feature_*`
- `enhancement_*`

✅ **Use feature-based naming:**
- `argument_parsing.rs`
- `semantic_analysis.rs`
- `help_generation.rs`

### Directory Structure
✅ **Allowed top-level directories:**
- `unit/` - Component isolation tests
- `integration/` - Component interaction tests
- `acceptance/` - User scenario tests
- `regression/` - Bug prevention tests
- `inc/` - Legacy compatibility
- `tools/` - Organization tools

### Nesting Limits
- Maximum 4 levels of directory nesting
- Example: `tests/unit/parser/argument_parsing.rs` ✅
- Example: `tests/unit/parser/complex/deep/nested/file.rs` ❌

### Category-Specific Conventions

**Unit Tests (`unit/`)**
- Should focus on single components
- Avoid names suggesting integration (`*integration*`, `*end_to_end*`)

**Integration Tests (`integration/`)**
- Should indicate component interaction
- Recommended: `*_integration*`, `*_workflow*`

**Acceptance Tests (`acceptance/`)**
- Should indicate user scenarios
- Recommended: `*cli*`, `*user*`, `*scenario*`

**Regression Tests (`regression/`)**
- Should indicate bug prevention
- Recommended: `*regression*`, `*fix*`

## Error Messages

### Common Violations

**Prohibited Naming Pattern**
```
❌ Prohibited naming pattern 'task_' in filename: task_024_multiple_parameters.rs
```
**Fix:** Rename to `multiple_parameters.rs` or `parameter_collection.rs`

**Invalid Directory Structure**
```
❌ Invalid directory structure: tests/random/test_file.rs
File is not in an allowed category directory
Allowed: unit/, integration/, acceptance/, regression/, inc/, tools/
```
**Fix:** Move file to appropriate category directory

**Excessive Nesting**
```
❌ Excessive nesting depth: tests/unit/parser/complex/deep/nested/file.rs
Directory nesting is too deep (max: 4 levels)
```
**Fix:** Flatten directory structure or consolidate files

## Integration with Development Workflow

### Git Workflow Integration
1. **Pre-commit validation** prevents problematic commits
2. **CI validation** catches issues in pull requests
3. **Make targets** provide convenient validation commands
4. **Automated reports** track organization health

### IDE Integration
- Tools can be integrated with IDE build systems
- Validation can run on file save or project build
- Error messages provide actionable feedback

## Maintenance

### Adding New Rules
1. Update `OrganizationRules` in `test_organization_validator.rs`
2. Add corresponding checks in shell scripts and CI workflows
3. Update documentation and examples

### Customizing Rules
- Modify `OrganizationRules::default()` for project-specific rules
- Adjust `prohibited_patterns` or `naming_conventions`
- Update shell script pattern matching

### Disabling Validation
```bash
# Temporary bypass (not recommended)
git commit --no-verify

# Remove pre-commit hook
rm .git/hooks/pre-commit

# Skip CI validation
git commit -m "message [skip ci]"
```

## Troubleshooting

### Permission Issues
```bash
chmod +x .git/hooks/pre-commit
chmod +x tests/tools/pre_commit_test_organization.sh
```

### Path Issues
- Ensure tools are run from repository root
- Check that `tests/` directory exists and has proper structure
- Verify relative paths in shell scripts

### CI Issues
- Ensure GitHub Actions workflow is in `.github/workflows/`
- Check that workflow triggers on correct paths
- Verify repository has proper test structure

## Benefits

✅ **Prevents regression** to problematic task-based naming
✅ **Enforces consistency** across team development
✅ **Automated validation** reduces manual review overhead
✅ **Clear feedback** helps developers understand standards
✅ **CI integration** catches issues before merge
✅ **Flexible configuration** allows project-specific customization

This enforcement system ensures the systematic test organization remains intact as the project evolves, preventing the chaos that existed in the original 69-file structure.