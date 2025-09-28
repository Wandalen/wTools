# Manual Testing Plan for Unilang

## Overview

This directory contains manual testing procedures for the Unilang framework. Manual testing is essential for validating user experience aspects that automated tests cannot fully capture.

## Relationship to Automated Tests

Manual tests complement the domain-based automated test structure (see `../readme.md`):
- **Automated tests** validate technical correctness within each domain (parser, semantic, interpreter, etc.)
- **Manual tests** validate user experience and cross-domain workflows that automated tests cannot fully capture

Manual testing scenarios often inform new automated tests, especially for regression prevention.

## Test Categories

### 1. User Experience Testing
Manual validation of CLI behavior from end-user perspective:
- Command discovery and help system usability
- Error message clarity and actionability
- Performance perception under various loads
- Cross-platform compatibility verification

### 2. Integration Testing
Manual validation of complex integration scenarios:
- Real-world command sequences
- Error recovery workflows
- Help system navigation
- Configuration and customization flows

### 3. Stress Testing
Manual validation under extreme conditions:
- Large parameter sets (1000+ parameters)
- Complex nested command structures
- Memory pressure scenarios
- Concurrent usage patterns

## Manual Test Procedures

### Procedure 1: Basic Command Flow
1. Start with empty command line
2. Execute `.` command to see help
3. Try a few basic commands like `.greet`
4. Test error conditions with invalid commands
5. Use help system (`?` operator) for command details
6. Verify error messages are clear and actionable

### Procedure 2: Multiple Parameter Testing
1. Execute commands with multiple same-name parameters
2. Test quote handling with complex strings
3. Verify parameter collection works correctly
4. Test performance with large parameter sets
5. Validate backward compatibility scenarios

### Procedure 3: Help System Validation
1. Test global help (`.` command)
2. Test specific command help (`command ?`)
3. Test alternative help access (`??` parameter)
4. Verify help content is comprehensive and accurate
5. Test help system navigation flow

### Procedure 4: Error Scenario Testing
1. Test invalid command syntax
2. Test missing required parameters
3. Test invalid parameter types
4. Test malformed command structures
5. Verify error recovery and guidance

## Execution Schedule

### Daily Testing (Automated CI/CD)
- Basic command flow validation
- Core functionality regression testing
- Performance baseline verification

### Weekly Testing (Manual)
- Complete user experience walkthrough
- Cross-platform compatibility check
- Performance stress testing
- Error scenario validation

### Release Testing (Manual)
- Full manual test suite execution
- User acceptance criteria validation
- Performance benchmarking
- Documentation accuracy verification

## Test Environment Setup

### Minimum Test Environment
- Rust toolchain (latest stable)
- Basic CLI environment (bash/zsh)
- Standard system utilities

### Comprehensive Test Environment
- Multiple OS platforms (Linux, macOS, Windows)
- Different shell environments
- Various terminal emulators
- Memory-constrained environments

## Test Data and Scenarios

### Basic Test Data
```bash
# Simple commands
.greet
.greet name::"Alice"

# Multiple parameters
.run command::"cargo build" command::"cargo test" command::"cargo clippy"

# Complex quoting
.video.search query::"rust programming tutorial" title::"Advanced Rust"
```

### Stress Test Data
```bash
# Large parameter sets
.test $(for i in {1..1000}; do echo "param::\"value$i\""; done)

# Complex nested structures
.complex nested::"{\"key\": \"value with spaces\", \"list\": [1, 2, 3]}"
```

## Expected Outcomes

### Success Criteria
- ✅ All commands execute without errors
- ✅ Help system provides comprehensive guidance
- ✅ Error messages are clear and actionable
- ✅ Performance meets baseline requirements
- ✅ User workflows are intuitive and efficient

### Failure Indicators
- ❌ Commands hang or crash
- ❌ Error messages are cryptic or misleading
- ❌ Help system is incomplete or inaccurate
- ❌ Performance significantly degrades
- ❌ User workflows are confusing or broken

## Test Reporting

### Test Report Template
```markdown
# Manual Test Report - [Date]

## Environment
- OS: [Operating System]
- Shell: [Shell Version]
- Terminal: [Terminal Emulator]
- Rust: [Rust Version]

## Test Results
- Procedure 1: [PASS/FAIL] - [Notes]
- Procedure 2: [PASS/FAIL] - [Notes]
- Procedure 3: [PASS/FAIL] - [Notes]
- Procedure 4: [PASS/FAIL] - [Notes]

## Issues Identified
- [Issue Description] - [Severity: HIGH/MEDIUM/LOW]

## Performance Observations
- [Performance Notes]

## Recommendations
- [Recommended Actions]
```

## Maintenance

### Monthly Review
- Update test procedures based on new features
- Review and update test data scenarios
- Validate test environment requirements
- Update expected outcomes and success criteria

### Quarterly Audit
- Complete test procedure walkthrough
- Performance baseline recalibration
- Cross-platform compatibility verification
- User feedback integration