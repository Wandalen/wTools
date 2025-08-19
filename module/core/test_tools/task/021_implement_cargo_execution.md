# Task 021: Implement Cargo Command Execution

## Overview
Implement SmokeModuleTest execution of cargo test and cargo run with proper success verification (FR-6).

## Specification Reference
**FR-6:** The smoke testing utility must execute `cargo test` and `cargo run` within the temporary project and assert that both commands succeed.

## Acceptance Criteria
- [ ] Implement robust cargo test execution in temporary project directory
- [ ] Implement robust cargo run execution in temporary project directory
- [ ] Add proper success assertion for cargo test command results
- [ ] Add proper success assertion for cargo run command results
- [ ] Implement comprehensive command output capture and handling
- [ ] Add proper error detection and reporting for failed commands
- [ ] All cargo command execution tests from task 020 must pass
- [ ] Maintain backward compatibility with existing perform() method

## Implementation Notes
- This task implements the GREEN phase of TDD - making the failing tests from task 020 pass
- Build upon existing perform() method implementation (lines 194-221 in current implementation)
- Enhance robustness and error handling of command execution
- Focus on improving reliability and diagnostics

## Technical Approach
1. **Enhance Command Execution**
   - Improve cargo test execution with better error handling
   - Enhance cargo run execution with proper argument handling
   - Add timeout handling for long-running commands

2. **Improve Success Verification**
   - Strengthen success assertions beyond just exit status
   - Add output validation for expected success patterns
   - Implement proper error classification

3. **Better Output Handling**
   - Improve stdout/stderr capture and logging
   - Add structured output parsing where beneficial
   - Implement better error message extraction

## Code Areas to Enhance
- Strengthen command execution in perform() method (lines 200-221)
- Improve error handling and assertions (lines 208, 218)
- Add better output capture and diagnostics
- Enhance working directory management

## Success Metrics
- All cargo command execution tests pass
- Cargo test and cargo run execute reliably in temporary projects
- Success/failure detection is accurate and comprehensive
- Error messages provide clear diagnostics for failures
- Command execution is robust against edge cases

## Related Tasks
- **Previous:** Task 020 - Write Tests for Cargo Command Execution
- **Next:** Task 022 - Refactor Cargo Execution Error Handling
- **Context:** Core implementation of specification requirement FR-6