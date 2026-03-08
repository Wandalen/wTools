# CLI Examples Gallery

Comprehensive collection of real-world usage examples, patterns, and workflows for `claude_runner` CLI.

## Table of Contents

- [Basic Patterns](#basic-patterns) — Essential usage patterns
- [Complex Workflows](#complex-workflows) — Multi-step operations
- [Session Management](#session-management) — Session-based workflows
- [Automation Scenarios](#automation-scenarios) — CI/CD and scripting
- [Error Handling Examples](#error-handling-examples) — Error case handling
- [Performance Optimization](#performance-optimization) — Efficient usage patterns

---

## Basic Patterns

### Pattern 1: Basic Task Execution

**Use Case:** Send a single task to Claude

```bash
# Short form (positional)
claude_runner "Fix the null pointer in user_service.rs"

# Flag form
claude_runner --message "Fix the null pointer in user_service.rs"

# With directory
claude_runner --message "Fix the bug" --dir ~/projects/myapp

# With continue
claude_runner --message "Continue refactoring" --continue
```

### Pattern 2: Dry Run Preview

**Use Case:** Preview command before execution

```bash
# Basic dry run
claude_runner --message "Review the PR" --dry-run

# Dry run with directory
claude_runner "Fix the bug" --dir /project --dry-run

# Dry run with multiple options
claude_runner "Comprehensive review" --dir /project --max-tokens 50000 --dry-run
```

### Pattern 3: Resource Control

**Use Case:** Control token usage and costs

```bash
# Limit tokens
claude_runner --message "Brief summary" --max-tokens 1000

# Maximum tokens
claude_runner --message "Full analysis" --max-tokens 4294967295

# Specific model
claude_runner --message "Task description" --model claude-opus-4-6
```

### Pattern 4: Verbose Output

**Use Case:** See command assembly before execution

```bash
# Verbose mode
claude_runner --verbose --message "Refactor module"

# Verbose with dry run
claude_runner --verbose --message "Test" --dry-run
# Shows env vars + command on stderr, then (optionally) executes
```

---

## Complex Workflows

### Workflow 1: Code Review Session

**Use Case:** Multi-turn code review with conversation continuation

```bash
# Step 1: Start session
claude_runner --message "Review the authentication module" --dir ~/projects/auth

# Step 2: Continue with feedback
claude_runner --message "The OAuth flow looks correct, but add error handling for edge cases" --continue --dir ~/projects/auth

# Step 3: Continue with next task
claude_runner --message "Now refactor the module to use error types" --continue --dir ~/projects/auth

# Step 4: Final summary
claude_runner --message "Summarize the refactoring work done today" --continue --dir ~/projects/auth
```

### Workflow 2: Batch File Operations

**Use Case:** Process multiple files with Claude

```bash
# Single file review
claude_runner --message "Review src/main.rs for improvements" --dir ~/projects/myapp

# Multiple file processing (loop in script)
for file in src/*.rs; do
    claude_runner --message "Optimize $file for performance" --dir ~/projects/myapp
done

# With dry run preview
claude_runner --message "Refactor these files" --dir ~/projects/myapp --dry-run
# Verify preview, then execute without --dry-run
```

### Workflow 3: Debugging Session

**Use Case:** Iterative debugging with verbose output

```bash
# Start verbose session
claude_runner --verbose --message "The application crashes on startup" --dir ~/projects/myapp

# Based on verbose output, investigate
# ... developer does investigation ...

# Continue with follow-up
claude_runner --verbose --continue --message "I found the crash is in the initialization module" --dir ~/projects/myapp

# Continue with fix verification
claude_runner --verbose --continue --message "After adding null checks, the crash is gone" --dir ~/projects/myapp
```

---

## Session Management

### Pattern 1: Project-Based Sessions

**Use Case:** Different projects have separate conversation contexts

```bash
# Project A: Backend API
claude_runner --message "Review the API endpoints" --dir ~/projects/backend
claude_runner --continue --message "Add rate limiting" --dir ~/projects/backend

# Project B: Frontend UI
claude_runner --message "Refactor the login component" --dir ~/projects/frontend
claude_runner --continue --message "Add social login" --dir ~/projects/frontend

# Switch between projects
# Work on frontend, then switch back to backend
claude_runner --message "Continue backend work" --dir ~/projects/backend --continue
```

### Pattern 2: Feature Development Sessions

**Use Case:** Maintain context across feature development

```bash
# Start feature session
claude_runner --message "Design a new authentication feature with OAuth support" --dir ~/projects/auth

# Continue feature work
claude_runner --continue --message "Add refresh token handling" --dir ~/projects/auth
claude_runner --continue --message "Implement the logout endpoint" --dir ~/projects/auth

# Testing session
claude_runner --message "Test the auth flow end-to-end" --dir ~/projects/auth/test --continue
claude_runner --message "Verify token expiration behavior" --dir ~/projects/auth/test
```

---

## Automation Scenarios

### Scenario 1: CI/CD Pipeline

**Use Case:** Automated testing with Claude

```bash
# GitHub Actions example
name: Run Claude Code Analysis
on: [push, pull_request]
jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install claude_runner
        run: cargo install --path ~/.local/bin claude_runner
      - name: Run analysis
        run: |
          export PATH="$HOME/.local/bin:$PATH"
          cd ${{ github.workspace }}
          claude_runner --message "Analyze this PR for security issues" --max-tokens 100000
```

### Scenario 2: Pre-Commit Hook

**Use Case:** Automatic code review before commits

```bash
# Git pre-commit hook
#!/bin/bash
# .git/hooks/pre-commit
export PATH="$HOME/.local/bin:$PATH"

FILES=$(git diff --cached --name-only --diff-filter=ACM)
if [ -n "$FILES" ]; then
    echo "Running Claude review on: $FILES"
    claude_runner --message "Review these changes: $FILES" --max-tokens 50000
    REVIEW_EXIT=$?
    if [ "$REVIEW_EXIT" -ne 0 ]; then
        echo "Claude review failed. Aborting commit."
        exit 1
    fi
fi
```

### Scenario 3: Batch Processing Script

**Use Case:** Process multiple code files with Claude

```bash
#!/bin/bash
# analyze-all.sh
export PATH="$HOME/.local/bin:$PATH"
PROJECT_DIR=$1

echo "=== Analyzing $PROJECT_DIR ==="

# Run analysis with verbose output
claude_runner --verbose --dir "$PROJECT_DIR" --message "Analyze all source files for improvements" --max-tokens 100000

echo "=== Analysis complete ==="

# Save analysis results to file
claude_runner --verbose --dir "$PROJECT_DIR" --message "Generate a summary document of all findings" --max-tokens 10000 > analysis_summary.txt
```

### Scenario 4: Scheduled Maintenance

**Use Case:** Nightly or weekly automated tasks

```bash
# Cron job for weekly code review
0 3 * * * claude_runner --message "Review all commits this week for code quality" --max-tokens 50000

# Nightly dependency check
0 2 * * * claude_runner --message "Check for outdated dependencies with security vulnerabilities" --max-tokens 30000
```

---

## Error Handling Examples

### Example 1: Handling Missing Dependencies

**Script Pattern:**
```bash
#!/bin/bash
set -e  # Exit on any error

PROJECT_DIR=$1

echo "Checking dependencies..."

if ! claude_runner --dry-run --message "Generate dependencies list" > /dev/null 2>&1; then
    echo "ERROR: claude_runner not found in PATH"
    echo "Install with: cargo install --path ~/.local/bin claude_runner"
    exit 1
fi

echo "Dependencies OK"
```

### Example 2: Retrying on Failure

**Script Pattern:**
```bash
#!/bin/bash
MAX_RETRIES=3
RETRY_DELAY=5

for i in $(seq 1 $MAX_RETRIES); do
    if claude_runner --message "Analyze code quality" --max-tokens 50000; then
        echo "Success on attempt $i"
        exit 0
    else
        echo "Attempt $i failed, retrying in $RETRY_DELAY seconds..."
        sleep $RETRY_DELAY
    fi
done

echo "All $MAX_RETRIES attempts failed"
exit 1
```

### Example 3: Graceful Degradation

**Pattern:** Reduce complexity on failures

```bash
#!/bin/bash
# First try: Full analysis
if claude_runner --message "Deep analysis of architecture" --max-tokens 100000; then
    echo "Full analysis complete"
else
    echo "Full analysis failed, trying reduced scope..."

    # Fallback: Reduced scope
    if claude_runner --message "Review core modules only" --max-tokens 30000; then
        echo "Reduced analysis complete"
    else
        echo "Reduced analysis failed, trying minimal..."

        # Last resort: Minimal
        claude_runner --message "Quick review of main.rs" --max-tokens 10000
fi
```

---

## Performance Optimization

### Optimization 1: Batch Processing

**Pattern:** Process multiple small tasks in single invocation

```bash
# Less efficient: Multiple invocations
claude_runner --message "Fix function A" & claude_runner --message "Fix function B" &
wait
claude_runner --message "Fix function C" &
wait

# More efficient: Single invocation with batched prompt
claude_runner --message "Review and fix these three functions: A, B, and C. For each function, identify issues, propose fixes, and explain the reasoning clearly."
```

### Optimization 2: Token Budget Management

**Pattern:** Use different token budgets for different tasks

```bash
# Lightweight tasks
claude_runner --message "List all files in src/" --max-tokens 1000

# Medium tasks
claude_runner --message "Review the main module structure" --max-tokens 10000

# Heavy analysis
claude_runner --message "Analyze the entire codebase for security vulnerabilities" --max-tokens 100000
```

### Optimization 3: Caching Results

**Pattern:** Save and reuse analysis results

```bash
#!/bin/bash
# Check for cached analysis
CACHE_FILE="$PROJECT_DIR/.claude_analysis_cache.json"
if [ -f "$CACHE_FILE" ] && [ "$(( $(date +%s) - $(stat -c %Y "$CACHE_FILE" ))" -lt 86400 ]; then
    echo "Using cached analysis (less than 1 day old)"
    cat "$CACHE_FILE"
else
    # Run analysis
    claude_runner --message "Analyze codebase" --dir "$PROJECT_DIR" > "$CACHE_FILE"
fi
```

---

## Integration Examples

### GitHub Actions Integration

```yaml
name: Code Quality Check
on: [pull_request, push]
jobs:
  claude-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup
        run: cargo install --path ~/.local/bin claude_runner

      - name: Run Claude Review
        run: |
          export PATH="$HOME/.local/bin:$PATH"
          claude_runner \
            --message "Review this PR for code quality, security, and performance" \
            --max-tokens 100000 \
            --verbose
```

### GitLab CI Integration

```yaml
stages:
  - review

claude_review:
  stage: review
  script:
    - cargo install --path ~/.local/bin claude_runner
    - |
      export PATH="$HOME/.local/bin:$PATH"
      claude_runner \
        --message "Review merge request for issues" \
        --max-tokens 100000
```

### Jenkins Pipeline Integration

```groovy
pipeline {
    agent any
    stages {
        stage('Claude Review') {
            steps {
                sh 'cargo install --path ~/.local/bin claude_runner || true'
                sh '''
                    export PATH="$HOME/.local/bin:$PATH"
                    claude_runner \
                        --message "Review changes" \
                        --max-tokens 100000
                '''
            }
        }
    }
}
```

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Chained Pipes

**Wrong:**
```bash
# Pipes output to another invocation
claude_runner --message "Analyze code" | claude_runner --message "Create tests"
```

**Problem:** Can't see intermediate results, hard to debug.

**Correct:**
```bash
# Use verbose mode or save to file
claude_runner --message "Analyze code" --verbose > analysis.txt
claude_runner --message "Create tests based on analysis.txt"
```

### Anti-Pattern 2: Hardcoded Paths

**Wrong:**
```bash
claude_runner --message "Review code in /home/user/projects/myapp"
```

**Problem:** Not portable, breaks for other developers.

**Correct:**
```bash
# Use relative paths or environment variables
PROJECT_DIR=~/projects/myapp
claude_runner --message "Review code in $PROJECT_DIR"
```

### Anti-Pattern 3: Ignoring Exit Codes

**Wrong:**
```bash
claude_runner --message "Analyze code" && rm -rf dist/
```

**Problem:** Removes directory even if analysis failed.

**Correct:**
```bash
# Check exit code
claude_runner --message "Analyze code"
if [ $? -eq 0 ]; then
    rm -rf dist/
fi
```

---

## Quick Reference by Use Case

| Use Case | Command | Key Flags |
|----------|---------|-----------|
| Basic task | `claude_runner --message "task"` | `--message`, `-m` |
| With directory | `claude_runner --message "task" --dir /path` | `--dir`, `-d` |
| Continue session | `claude_runner --message "task" --continue` | `--continue`, `-c` |
| Dry run preview | `claude_runner --message "task" --dry-run` | `--dry-run` |
| Token limit | `claude_runner --message "task" --max-tokens N` | `--max-tokens` |
| Verbose mode | `claude_runner --verbose --message "task"` | `--verbose`, `-v` |
| Specific model | `claude_runner --message "task" --model name` | `--model` |

---

## Common Workflows

### Workflow: Quick Bug Fix

```bash
# 1. Understand the issue
claude_runner --message "What does this error mean: [paste error]"

# 2. Get context
claude_runner --verbose --message "Show me the code around line 42 in user_service.rs" --dir ~/projects/myapp

# 3. Propose fix
claude_runner --message "Propose a fix for the null pointer issue"

# 4. Verify
claude_runner --verbose --continue --message "Does this fix look correct?" --dir ~/projects/myapp
```

### Workflow: Feature Implementation

```bash
# 1. Design phase
claude_runner --message "Design a new user profile feature with fields: name, email, bio, avatar URL"

# 2. Implementation phase
claude_runner --message "Implement the profile schema and database migration" --continue --dir ~/projects/backend

# 3. Testing phase
claude_runner --message "Create test cases for profile CRUD operations" --continue --dir ~/projects/backend

# 4. Review phase
claude_runner --message "Review the implementation and suggest improvements" --continue --dir ~/projects/backend
```

### Workflow: Refactoring Session

```bash
# 1. Analysis
claude_runner --message "Analyze the module structure and identify code smells" --dir ~/projects/myapp

# 2. Plan
claude_runner --message "Create a refactoring plan to extract common patterns into utility modules" --continue --dir ~/projects/myapp

# 3. Execute
claude_runner --message "Implement the utility modules" --continue --dir ~/projects/myapp

# 4. Verify
claude_runner --message "Verify the refactored code maintains all functionality" --continue --dir ~/projects/myapp
```

---

## Best Practices Summary

1. **Use --dry-run liberally** — Preview before execution to avoid wasted tokens
2. **Continue for sessions** — Use `--continue` to maintain conversation context
3. **Set reasonable token limits** — Different tasks need different budgets
4. **Use verbose when debugging** — See command assembly before execution
5. **Batch related tasks** — Combine multiple small requests into one
6. **Save important outputs** — Keep analysis results for reference
7. **Handle errors gracefully** — Check exit codes, implement retries
8. **Use portable paths** — Environment variables over hardcoding
9. **Design for automation** — Scripts should be idempotent and recoverable
10. **Document workflows** — Comment complex scripts for maintainability

---

## Summary

This examples gallery provides:
- **Basic patterns** for everyday use cases
- **Complex workflows** for multi-step operations
- **Automation scenarios** for CI/CD and scripting
- **Error handling** examples for robust scripts
- **Performance optimization** techniques for efficiency
- **Anti-patterns** to avoid common mistakes
- **Integration examples** for popular CI platforms

Each example includes the command, explanation, and practical use case context.

---

## References

- [Tutorial](tutorial.md) — Hands-on lessons
- [Quick Reference](quick_reference.md) — Fast lookup card
- [API Reference](api_reference.md) — Complete API documentation
- [Architecture](architecture.md) — System diagrams and data flow
