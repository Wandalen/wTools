# Feature Spec: REPL Mode

### Scope

- **Element:** `feature/010_repl_mode`
- **Source:** `docs/feature/010_repl_mode.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | repl_starts_on_no_arguments | nominal | ✅ |
| FT-02 | archive_state_persists_across_commands | nominal | ✅ |
| FT-03 | exit_command_terminates_cleanly | nominal | ✅ |
| FT-04 | repl_exits_with_code_zero_on_clean_exit | nominal | ✅ |

---

### FT-01: REPL starts when invoked with no arguments

- **Given:** genfile binary is available
- **When:** `genfile` is invoked with no arguments and a prompt character is sent
- **Then:** A REPL prompt appears; the process does not immediately exit
- **Tests:** `tests/repl_exit_code_bug_test.rs`

### FT-02: archive state persists across commands in one session

- **Given:** REPL session is running
- **When:** `.archive.new name::foo` is run, then `.file.add path::x.txt content::hi`, then `.file.list`
- **Then:** `.file.list` output shows `x.txt`; state from first command is preserved into third
- **Tests:** `tests/repl_exit_code_bug_test.rs`

### FT-03: exit command terminates REPL cleanly

- **Given:** REPL session is running
- **When:** `exit` is entered
- **Then:** Process exits; no error output produced
- **Tests:** `tests/repl_exit_code_bug_test.rs`

### FT-04: REPL exits with code 0 on clean exit

- **Given:** REPL session is running with no errors
- **When:** `exit` or `quit` is entered
- **Then:** Exit code is 0
- **Tests:** `tests/repl_exit_code_bug_test.rs`
