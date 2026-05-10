# CLI Documentation Maintenance Guide

**Project:** genfile CLI Documentation
**Version:** 1.0.0
**Last Updated:** 2026-05-10

---

### Overview

This guide provides comprehensive instructions for maintaining the GenFile CLI documentation system. Follow these procedures to keep documentation accurate, consistent, and synchronized with code changes.

---

### Maintenance Scripts

### Available Scripts

| Script | Purpose | Frequency | Runtime |
|--------|---------|-----------|---------|
| `-validate_links.sh` | Validate all cross-reference links | Weekly | ~10s |
| `-update_stats.sh` | Update statistics in readme.md | Monthly | ~5s |

### Running Scripts

```bash
cd docs/cli

# Validate all links
bash -validate_links.sh

# Update statistics
bash -update_stats.sh
```

---

### When to Update Documentation

### Trigger 1: New Command Added

**When:** Adding a new command to GenFile CLI

**Steps:**
1. **Register in Rust** (`src/commands/*.rs`)
   - Add `CommandDefinition` with all parameters following existing patterns
   - Register with handler function

2. **Implement handler** (`src/handlers/*.rs`)
   - Create handler function with proper signature
   - Extract and validate parameters
   - Return OutputData with formatted response

3. **Update documentation**:
   - Add command to appropriate namespace file (`docs/cli/command/*.md`)
   - Update commands index (`docs/cli/command/readme.md`)
   - Add parameters to `docs/cli/param.md` if new
   - Add types to `docs/cli/type.md` if new
   - Update parameter groups if universally applicable
   - Run validation: `bash -validate_links.sh`
   - Update statistics: `bash -update_stats.sh`

**Estimated Time:** 2-3 hours

---

### Trigger 2: Command Behavior Changed

**When:** Modifying existing command functionality

**Steps:**
1. Update Rust registration if signature changed
2. Update handler implementation
3. **Update documentation**:
   - Update command section in namespace file
   - Update examples if behavior changed
   - Update parameter descriptions if semantics changed
   - Update exit codes if new errors introduced
   - Run validation: `bash -validate_links.sh`

**Estimated Time:** 30-60 minutes

---

### Trigger 3: Parameter Added/Modified

**When:** Adding new parameter or changing existing parameter

**Steps:**
1. Update Rust CommandDefinition
2. Update handler to extract/validate parameter
3. **Update documentation**:
   - Update parameter specification in `param.md`
   - Update type definition in `type.md` if new type
   - Update all command sections using this parameter
   - Check if parameter should be in a group (`param_group.md`)
   - Update cross-references (bidirectional)
   - Add examples showing new parameter usage
   - Run validation: `bash -validate_links.sh`

**Estimated Time:** 1-2 hours

---

### Trigger 4: Type Validation Changed

**When:** Modifying validation logic for a type

**Steps:**
1. **Update documentation**:
   - Update type definition in `type.md`
   - Update validation logic code example
   - Update all parameters using this type (`param.md`)
   - Update error handling guidance
   - Run validation: `bash -validate_links.sh`

**Estimated Time:** 30 minutes

---

### Documentation Structure

### File Organization

```
docs/cli/
├── readme.md                  # Navigation hub + Completion Matrix
├── command/                   # Namespace-organized command specifications
│   ├── readme.md              # Commands index + Quick Reference
│   ├── archive.md             # Archive operations (4 commands)
│   ├── content.md             # Content management (3 commands)
│   ├── file.md                # File operations (4 commands)
│   ├── param_mgmt.md          # Parameter management (3 commands)
│   ├── value.md               # Value operations (3 commands)
│   └── operations.md          # Core operations (7 commands)
├── param.md                   # Parameter specifications
├── type.md                    # Type system and validation rules
├── param_group.md             # Shared parameter sets
├── dictionary.md              # Domain terminology
├── env_param.md               # Environment variable configuration
├── config_param.md            # Configuration file mechanism
├── workflow_scenario.md       # Multi-command usage scenarios
├── format.md                  # Output format catalog
├── tutorial.md                # Hands-on beginner lessons
├── maintenance.md             # This file
├── -validate_links.sh         # Link validation script
└── -update_stats.sh           # Statistics updater script
```

### File Naming Conventions

- **User documentation:** `command/`, `param.md`, `type.md`, `param_group.md` (no hyphen prefix)
- **Temporary files:** `-alignment_report.md`, `-validation.md` (hyphen prefix)
- **Scripts:** `-validate_links.sh`, `-update_stats.sh` (hyphen prefix, executable)

---

### Quality Checklist

### Before Committing Documentation Changes

- [ ] All new commands documented with required sections
- [ ] All new parameters added to `param.md`
- [ ] All new types added to `type.md`
- [ ] Cross-references bidirectional (A→B and B→A)
- [ ] Examples syntactically correct
- [ ] Links validated: `bash -validate_links.sh`
- [ ] Statistics updated: `bash -update_stats.sh`
- [ ] Consistency maintained (terminology, formatting)

### Required Sections per Command

1. ✅ Syntax block
2. ✅ Purpose statement
3. ✅ Parameters list with links
4. ✅ Examples (minimum 2)
5. ✅ Exit codes
6. ✅ Related commands

### Required Sections per Parameter

1. ✅ Type reference (link to type.md)
2. ✅ Purpose statement
3. ✅ Default value
4. ✅ Used by commands (with links)
5. ✅ Examples (minimum 1)
6. ✅ Validation constraints

### Required Sections per Type

1. ✅ Rust struct/enum definition
2. ✅ Validation logic
3. ✅ Constructor signature
4. ✅ Error handling examples
5. ✅ Usage examples

---

### Common Maintenance Tasks

### Task 1: Add New Command to Archive Namespace

**Example:** Adding `.archive.merge` command

```bash
# 1. Update Rust registration
vim src/commands/archive.rs
# Add register_merge() function

# 2. Implement handler
vim src/handlers/archive.rs
# Add merge_handler() function

# 3. Update documentation
vim docs/cli/command/archive.md
# Add command section at end

vim docs/cli/command/readme.md
# Add row to index table

vim docs/cli/param.md
# Add any new parameters

# 4. Validate
cd docs/cli
bash -validate_links.sh
bash -update_stats.sh
```

---

### Task 2: Update Parameter Validation

**Example:** Enforce verbosity range (0-5)

```bash
# 1. Update documentation
vim docs/cli/type.md
# Update VerbosityLevel validation logic example

vim docs/cli/param.md
# Update verbosity:: parameter description

# 2. Update handlers (optional - add runtime validation)
vim src/handlers/*.rs
# Add: if verbosity > 5 { return Err(...) }

# 3. Validate
cd docs/cli
bash -validate_links.sh
```

---

### Task 3: Fix Broken Link

**Example:** Link validation finds broken anchor

```bash
# 1. Run validation to identify issue
cd docs/cli
bash -validate_links.sh
# Output: "❌ BROKEN: Anchor not found: param.md#parameter-24-newparam"

# 2. Fix the issue
# Option A: Fix anchor in target file
vim param.md
# Ensure anchor exists: ### Parameter 24: newparam::

# Option B: Fix link in source file
vim command/archive.md
# Update link to correct anchor

# 3. Re-validate
bash -validate_links.sh
# Output: "✅ All links valid"
```

---

### Consistency Guidelines

### Terminology Standards

| Term | Correct Usage | Avoid |
|------|---------------|-------|
| Archive | Capitalized in formal context | archive (lowercase in prose) |
| Parameter | Full word in prose | param (use only in code) |
| Materialization | Consistent spelling | Materialisation |
| Command | Lowercase in general | Command (uppercase mid-sentence) |
| Dry Run | Two words | Dryrun, dry-run |

### Example Syntax Standards

**Correct:**
```bash
genfile .archive.load path::"template.yaml" verbosity::2
genfile .materialize destination::"output/" dry::false
```

**Incorrect:**
```bash
genfile .archive.load path::template.yaml verbosity::2    # Missing quotes
genfile .materialize path::"output/" dry::false           # Wrong parameter name (path vs destination)
```

### Link Format Standards

**Correct:**
```markdown
[verbosity::](param.md#parameter--1-verbosity)
[VerbosityLevel](type.md#type--1-verbositylevel)
```

**Incorrect:**
```markdown
[verbosity::](param.md)                    # Missing anchor
[VerbosityLevel](type.md#verbositylevel)   # Wrong anchor format
```

---

### Troubleshooting

### Issue 1: Link Validation Fails

**Symptom:**
```
❌ BROKEN: Anchor not found: param.md#parameter-5-newparam
```

**Diagnosis:**
- Target file exists but anchor is missing or misformatted
- Source link uses wrong anchor format

**Solution:**
```bash
# Check target file
rg "^### Parameter 5:" docs/cli/param.md

# If missing, add section
# If exists, check anchor format (should be lowercase-with-dashes)
```

---

### Issue 2: Statistics Outdated

**Symptom:** README shows "23 parameters" but there are now 24

**Solution:**
```bash
cd docs/cli
bash -update_stats.sh
# Manually verify counts if script doesn't detect new entries
```

---

### Issue 3: Examples Don't Match Implementation

**Symptom:** Documentation shows `path::` but code uses `destination::`

**Diagnosis:**
- Parameter renamed in code but documentation not updated

**Solution:**
```bash
# Check Rust registration
rg "destination" src/commands/materialize.rs

# Update documentation to match
vim docs/cli/command/operations.md
# Change all path:: to destination:: in materialize examples
```

---

### Scheduled Maintenance

### Weekly Tasks (10 minutes)

- [ ] Run link validation: `bash -validate_links.sh`
- [ ] Review recent commits for documentation needs
- [ ] Check GitHub issues for documentation requests

### Monthly Tasks (1 hour)

- [ ] Run statistics update: `bash -update_stats.sh`
- [ ] Review all examples for accuracy
- [ ] Check cross-references for broken links
- [ ] Update version numbers if CLI version changed

### Quarterly Tasks (2-3 hours)

- [ ] Full documentation review (read-through)
- [ ] Verify alignment with latest code (run Phase 2 validation)
- [ ] Update performance benchmarks if applicable
- [ ] Review and update use cases
- [ ] Collect and incorporate user feedback

---

### Quality Metrics

### Target Metrics

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Documentation Coverage | 100% | 100% (24/24 commands) | ✅ |
| Link Integrity | 100% | 100% (194+ links) | ✅ |
| Consistency | ≥95% | 98.6% | ✅ |
| Example Accuracy | ≥95% | 96% | ✅ |

### Measuring Metrics

```bash
cd docs/cli

# Coverage (should match command count in code)
rg "^### Command ::" command/*.md | wc -l

# Link integrity
bash -validate_links.sh
# Look for: "✅ All links valid" or error count

# Consistency (manual review required)
# Review terminology, formatting, example syntax

# Example accuracy (manual testing required)
# Copy examples and verify they execute correctly
```

---

### Emergency Procedures

### Critical Documentation Issue Found

**If documentation is incorrect and users are affected:**

1. **Immediate:** Create issue in GitHub with label `documentation` and `urgent`
2. **Within 4 hours:** Fix documentation and deploy corrected version
3. **Within 24 hours:** Notify users via release notes or announcement
4. **Post-mortem:** Add validation check to prevent recurrence

### Major Code Refactoring

**When commands/parameters are renamed or removed:**

1. **Before code merge:** Update all documentation
2. **Create migration guide** if breaking changes
3. **Update examples** to use new syntax
4. **Add deprecation notices** if commands replaced
5. **Run full validation:** `bash -validate_links.sh`

---

### Best Practices

### DO ✅

- Update documentation in same PR as code changes
- Run validation scripts before committing
- Maintain bidirectional cross-references
- Follow existing patterns and conventions
- Write clear, concise examples
- Test all examples before documenting

### DON'T ❌

- Commit documentation without running validation
- Create orphaned files (no incoming links)
- Use inconsistent terminology
- Skip required sections
- Reference temporary files from permanent docs
- Assume examples work without testing

---

### Getting Help

### Resources

- **Validation Reports:** See `-*.md` files for detailed validation results
- **Code Documentation:** See handler files for implementation details

### Contact

For documentation questions or issues:
1. Check this MAINTENANCE guide first
2. Review validation reports in `docs/cli/-*.md`
3. Create GitHub issue with label `documentation`

---

### Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-02-08 | Initial maintenance guide created |
| 1.1.0 | 2026-05-10 | Update file paths to reflect command/ structure and current file names |

---

**Maintenance Guide Complete**

**Keep this guide updated as processes evolve. Documentation maintenance is critical for long-term project success.**
