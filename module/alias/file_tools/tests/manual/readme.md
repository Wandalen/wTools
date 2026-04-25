# file_tools Manual Testing Plan

Comprehensive manual testing procedures for file_tools crate functionality requiring human verification.

## Status

⚠️ **PENDING IMPLEMENTATION**: This crate is currently in placeholder state with no actual file manipulation functionality. Manual testing will be required once real functionality is implemented per specification.

## Testing Scope

When file manipulation utilities are implemented, manual testing MUST verify:

1. **Cross-Platform Behavior**: File operations work correctly on Linux, macOS, Windows
2. **Edge Case Handling**: Correct behavior with unusual file names, permissions, sizes
3. **Error Messages**: Clear, actionable error messages for failure conditions
4. **Performance**: Acceptable performance with large files and deep directory structures
5. **Safety**: No data corruption, no security vulnerabilities

## Exhaustive Corner Case Checklist

### File Reading Operations

- [ ] **Empty files** (0 bytes) - Read operation returns empty content without error
- [ ] **Single-byte files** - Correctly reads 1-byte files
- [ ] **Very large files** (>4GB) - Handles files exceeding 32-bit addressing
- [ ] **Binary files** - Non-UTF8 content read correctly without corruption
- [ ] **Files with special characters** - Unicode, spaces, newlines in filenames
- [ ] **Symbolic links** - Reading through symlinks resolves correctly
- [ ] **Hard links** - Multiple paths to same inode handled correctly
- [ ] **Files without read permissions** - Clear permission denied error
- [ ] **Files that disappear during reading** - TOCTOU race condition handled gracefully
- [ ] **Files on read-only filesystems** - Read operations succeed

### File Writing Operations

- [ ] **Writing to non-existent directories** - Clear error or auto-create behavior
- [ ] **Writing to files without write permissions** - Clear permission denied error
- [ ] **Writing to full filesystems** (ENOSPC) - Graceful failure, no corruption
- [ ] **Writing to read-only filesystems** - Clear read-only error
- [ ] **Atomic write failures** - Partial write recovery
- [ ] **Concurrent writes** - Data integrity maintained
- [ ] **Writing with fsync requirements** - Data durability guarantees
- [ ] **Writing to special files** - /dev/null, /dev/random handled correctly

### Path Operations

- [ ] **Absolute paths** - /home/user/file.txt
- [ ] **Relative paths** - ., .., ../../file.txt
- [ ] **Paths with . and .. components** - Normalized correctly
- [ ] **Paths with trailing slashes** - /path/to/dir/ handled same as /path/to/dir
- [ ] **Paths with multiple slashes** - ///path///file handled correctly
- [ ] **Canonicalization of paths** - Resolves symlinks, . and .. correctly
- [ ] **Paths longer than PATH_MAX** - Clear error message
- [ ] **Paths with null bytes** - Rejected with clear error

### Directory Operations

- [ ] **Empty directories** - Created and listed correctly
- [ ] **Nested directories** - Deep hierarchies (100+ levels) handled
- [ ] **Directories without permissions** - Clear permission errors
- [ ] **Creating existing directories** - Idempotent behavior or clear error
- [ ] **Recursive directory creation** - mkdir -p equivalent
- [ ] **Directory traversal** - Lists all entries correctly
- [ ] **Hidden files** - Files starting with . included in listings

### Metadata Operations

- [ ] **Getting file size** - Accurate for all file sizes including 0 and >4GB
- [ ] **Getting modification time** - Correct timestamps
- [ ] **Setting file permissions** - chmod equivalent works correctly
- [ ] **Checking file existence** - Distinguishes non-existent vs permission denied
- [ ] **Distinguishing files vs directories** - Correct type detection

### Error Conditions

- [ ] **ENOENT** (file not found) - Clear "file not found" message
- [ ] **EACCES** (permission denied) - Clear "permission denied" message
- [ ] **EISDIR** (is a directory) - Clear "is a directory" message
- [ ] **ENOTDIR** (not a directory) - Clear "not a directory" message
- [ ] **ENAMETOOLONG** (path too long) - Clear "path too long" message
- [ ] **EMFILE** (too many open files) - Clear resource limit message
- [ ] **ENOSPC** (no space left) - Clear "disk full" message

### Platform-Specific Behavior

#### Windows

- [ ] **Path separators** - Both \\ and / work correctly
- [ ] **Drive letters** - C:\path\file.txt handled correctly
- [ ] **UNC paths** - \\\\server\\share paths work
- [ ] **Case insensitivity** - File.txt and file.txt treated as same

#### Unix/Linux

- [ ] **Hidden files** - Files starting with . handled correctly
- [ ] **Case sensitivity** - File.txt and file.txt are different files
- [ ] **Symlink chains** - Multiple levels of symlinks resolved
- [ ] **FIFO/socket files** - Special file types detected correctly

## Manual Testing Procedure

### Setup

1. **Prepare test environment**:
   ```bash
   cd /tmp
   mkdir file_tools_manual_test
   cd file_tools_manual_test
   ```

2. **Create test fixtures**:
   ```bash
   # Empty file
   touch empty.txt

   # Single-byte file
   echo -n "x" > single_byte.txt

   # Large file (1GB)
   dd if=/dev/zero of=large.bin bs=1M count=1024

   # Special character filenames
   touch "file with spaces.txt"
   touch "файл_unicode.txt"

   # Symlink
   ln -s empty.txt symlink.txt

   # Directory structure
   mkdir -p deep/nested/directory/structure
   ```

3. **Compile test program**:
   ```bash
   cargo build --release --examples
   ```

### Execution

For each corner case:

1. **Execute operation** using example program
2. **Observe behavior** - Success, error message, side effects
3. **Verify correctness** - Compare with expected behavior
4. **Document result** - Pass/Fail with details

### Cleanup

```bash
cd /tmp
rm -rf file_tools_manual_test
```

## Test Results Log

| Corner Case | Date Tested | Result | Notes |
|-------------|-------------|--------|-------|
| (Pending implementation) | - | - | - |

## Known Limitations

- ⏳ **Pending specification**: Crate functionality not yet defined
- ⏳ **No examples exist**: Example programs need to be created
- ⏳ **Platform testing**: Requires testing on Windows, macOS, Linux

## Testing Frequency

- **Before major releases**: Full manual test suite
- **After significant changes**: Affected corner cases only
- **Platform-specific changes**: Platform-specific suite

## Reporting Issues

When manual testing reveals issues:

1. **Create minimal reproduction** in automated test
2. **File bug report** with issue number
3. **Mark corner case** as failing in checklist
4. **Document workaround** if any exists

