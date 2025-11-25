# Implement preserved quotes stripping in parse_from_argv

## Description

Enhance `parse_from_argv` to detect and strip literal quote characters that occur when users over-quote parameters. For example, when a user types `'param::"value"'`, the shell preserves the inner double quotes as literal characters, resulting in `param::"value"` being passed to the parser. Currently this creates double-quoting issues.

This is a lower-priority enhancement that improves handling of edge cases. The natural syntax (without over-quoting) already works correctly.

## Critical Problems and Risks

**WARNING: This task has 22 identified problems, including critical breaking changes.**

**Recommendation: DO NOT IMPLEMENT as originally designed. See Alternative Approaches below.**

### The Fundamental Ambiguity (UNSOLVABLE)

The core issue is that **two different shell commands produce identical argv**:

```bash
# Case A: Over-quoting (accidental)
mycli .cmd 'param::"value"'
→ Shell passes: param::"value" (literal quote chars)

# Case B: Escaped quotes (intentional)
mycli .cmd param::\"value\"
→ Shell passes: param::"value" (literal quote chars)
```

**From argv perspective:** Both are `param::"value"` (with literal `"` characters)

**User Intent:**
- Case A wants: `value` (quotes were mistake)
- Case B wants: `"value"` (quotes were deliberate)

**The Problem:** We CANNOT distinguish these cases from argv alone!

Any solution that strips quotes will break Case B. Any solution that preserves quotes will not fix Case A.

### Complete Problem List

#### Category 1: Fundamental Assumption Errors

1. **No evidence over-quoting is common**
   - Severity: CRITICAL
   - Impact: Optimizing for potentially non-existent problem
   - Risk: Adding complexity and breaking changes without data

2. **Breaking changes based on guesses**
   - Severity: CRITICAL
   - Impact: Silent data corruption in production code
   - Risk: Cannot distinguish user intent from argv

3. **Doesn't solve ambiguity - picks arbitrary side**
   - Severity: CRITICAL
   - Impact: Breaks legitimate use cases (Case B above)
   - Risk: Users with intentional quotes get silently corrupted data

4. **Workaround is shell-specific and unclear**
   - Severity: HIGH
   - Impact: No clear path for users hit by breaking change
   - Example: `param::\\"value\\"` only works in bash/zsh

#### Category 2: Critical Design Flaws

5. **Can't distinguish intentional vs accidental quotes**
   - Severity: CRITICAL
   - Impact: Breaks Case B (escaped quotes)
   - See detailed analysis below

6. **Not actually conservative**
   - Severity: HIGH
   - Impact: Claims "conservative" but strips ALL boundary quotes
   - Risk: Too aggressive - catches legitimate cases

7. **Inconsistent Unicode handling**
   - Severity: MEDIUM
   - Impact: Only strips ASCII `"`, not Unicode quotes („", «»)
   - Risk: Inconsistent behavior by character type

12. **Named vs positional argument inconsistency**
    - Severity: MEDIUM
    - Impact: Stripping only applies to named args (with `::`)
    - Risk: Same over-quoting problem exists for positional args

#### Category 3: Breaking Changes and Risk

13. **Breaks legitimate use case - USER WANTS LITERAL QUOTES**
    - Severity: **CRITICAL - SILENT DATA CORRUPTION**
    - Impact: See extensive analysis below
    - Risk: Production data corruption with no error

17. **Risk assessment wrong**
    - Severity: HIGH
    - Claims: LOW risk
    - Reality: MEDIUM to HIGH risk
    - Priority: Claims MEDIUM, actually LOW (edge case)

23. **No migration strategy**
    - Severity: CRITICAL
    - Impact: Silent breakage in existing code
    - Risk: No deprecation, no warning period, no compatibility mode

#### Category 4: Test Coverage Gaps

9. **Missing test: Unicode whitespace + quotes**
   - Severity: MEDIUM
   - Impact: Interaction with Task 082 fix untested

10. **Missing test: Escaped quotes scenario**
    - Severity: CRITICAL
    - Impact: The breaking case (Case B) is not tested!
    - Risk: Don't even verify what breaks

11. **Missing test: Positional arguments**
    - Severity: MEDIUM
    - Impact: Inconsistent behavior unspecified

24. **No integration tests with real shell**
    - Severity: HIGH
    - Impact: Assumptions about shell behavior untested
    - Risk: What if shell understanding is wrong?

25. **Missing test: Multi-word + quote interaction**
    - Severity: MEDIUM
    - Impact: Edge case of edge case untested

#### Category 5: Documentation Issues

14. **Specification location unclear**
    - Severity: LOW
    - Impact: Documentation might not get written

15. **No user-facing documentation**
    - Severity: MEDIUM
    - Impact: Users won't know about behavior change

16. **No helpful error messages**
    - Severity: HIGH
    - Impact: Silent breakage with no guidance

#### Category 6: Better Alternatives Not Explored

18. **Didn't seriously explore heuristic approach**
    - Severity: HIGH
    - Impact: Less breaking alternative dismissed as "complex"
    - Alternative: Strip only if inner value has whitespace

19. **Didn't consider opt-in option**
    - Severity: CRITICAL
    - Impact: Could avoid ALL breaking changes!
    - Alternative: Add `strip_argv_quotes` option (default: false)

22. **Treating symptom, not root cause**
    - Severity: HIGH
    - Impact: Auto-fixing instead of user education
    - Alternative: Warn users, improve documentation

### PROBLEM 13: CRITICAL BREAKING CASE - USER WANTS LITERAL QUOTES

**This is the most serious problem: silent data corruption with no error.**

#### Why This is Critical

When users intentionally want literal quote characters in their values, the quote stripping silently corrupts their data:

1. **Silent Corruption** - No error, no warning, just wrong data
2. **Production Impact** - Stored data is wrong, propagates through system
3. **Hard to Debug** - User input looks correct, output is silently wrong
4. **No Recovery** - Once data is corrupted and stored, original intent is lost

#### Real-World Scenarios Where Users Want Literal Quotes

##### Scenario 1: Book Titles and Quoted Text

```bash
# User managing book database
mycli .book.add title::"The "Great" Adventure"

# Shell processing (using outer quotes to preserve inner quotes):
mycli .book.add 'title::"The "Great" Adventure"'

# What shell passes:
argv = ["title::\"The \"Great\" Adventure\""]
       # Literal quote chars: ^    ^      ^        ^

# Original Plan's Behavior:
# 1. Receives: title::"The "Great" Adventure"
# 2. Strips outer quotes: title::The "Great" Adventure
# 3. Detects whitespace, adds quotes: title::"The "Great" Adventure"
# 4. String parser gets: title::"The "Great" Adventure"
# 5. Result value: The "Great" Adventure ✓ (might work?)

# But wait - what if simpler case:
mycli .book.add 'title::"Chapter 1"'

# Shell passes: title::"Chapter 1"
# Strips to: title::Chapter 1
# User wanted: "Chapter 1" (with quotes)
# User gets: Chapter 1 ❌ QUOTES LOST!
```

**Impact:** Database stores book title without quotes. User expected:
```
Title: "Chapter 1"
```
But database contains:
```
Title: Chapter 1
```

No error occurred. User doesn't notice until they review the data later. **Silent data corruption.**

##### Scenario 2: Code Snippets and Technical Text

```bash
# User documenting code that uses quoted strings
mycli .doc.add example::'printf("hello\n");'

# Shell passes: example::"hello\n"
# (simplified - actual would be more complex)

# Or user adding JSON examples:
mycli .config.set template::'"name": "value"'

# Shell passes: template::"name": "value"
# Strips to: template::name": "value
# User wanted: "name": "value" (valid JSON key-value)
# User gets: name": "value ❌ INVALID JSON!
```

**Impact:** Documentation or configuration contains incorrect code examples. Users copy the wrong examples and their code breaks.

##### Scenario 3: Shell Command Escaping

```bash
# User wants to store a shell command that includes quoted args
mycli .cmd.store name::backup command::'rsync -av "/home/user" "/backup"'

# Shell passes: command::"rsync -av "/home/user" "/backup""
# Wait, this gets complex with the inner quotes...

# Simpler example:
mycli .cmd.store name::echo command::'"hello world"'

# Shell passes: command::"hello world"
# Strips to: command::hello world
# User wanted to store: "hello world" (with quotes)
# User gets: hello world ❌ QUOTES LOST!

# When command executes:
echo hello world  # Not what user wanted!
# Should have been:
echo "hello world"
```

**Impact:** Stored commands execute incorrectly. Could have serious consequences if commands are automated.

##### Scenario 4: CSV and Data Formats

```bash
# User importing CSV data with quoted fields
mycli .data.import field::'"Smith, John"'

# Shell passes: field::"Smith, John"
# Strips to: field::Smith, John
# User wanted: "Smith, John" (CSV quoted field)
# User gets: Smith, John ❌ QUOTES LOST!

# When exported back to CSV:
Smith, John  # Two fields! (comma splits)
# Should have been:
"Smith, John"  # One field
```

**Impact:** Data export/import is corrupted. CSV structure breaks. Data loss or misalignment.

##### Scenario 5: SQL Queries

```bash
# User wants to store SQL with string literals
mycli .db.query sql::'SELECT * FROM users WHERE name = "John"'

# Shell passes: sql::"SELECT * FROM users WHERE name = "John""
# Strips to: sql::SELECT * FROM users WHERE name = "John"
# (This might work for the outer quotes, but...)

# Simpler case:
mycli .db.query name::"John's account"

# Actually, let me think of better example:
mycli .db.literal value::'"admin"'

# Shell passes: value::"admin"
# Strips to: value::admin
# User wanted: "admin" (SQL string literal)
# User gets: admin (SQL identifier!) ❌ WRONG SEMANTICS!
```

**Impact:** SQL queries execute with wrong semantics. Identifiers vs literals are different in SQL.

##### Scenario 6: Embedded Markup and HTML

```bash
# User storing HTML with quoted attributes
mycli .html.add tag::'<a href="index.html">'

# Shell passes: tag::"<a href="index.html">"
# This gets complex with nested quotes...

# Simpler:
mycli .html.add title::'"Welcome"'

# Shell passes: title::"Welcome"
# Strips to: title::Welcome
# User wanted: "Welcome" (with quotes for emphasis)
# User gets: Welcome ❌ QUOTES LOST!
```

**Impact:** HTML rendering is wrong. Emphasis or style is lost.

##### Scenario 7: Configuration and Environment Variables

```bash
# User setting env var that should contain quotes
mycli .env.set PATH::'".:$HOME/bin"'

# Shell passes: PATH::".:$HOME/bin"
# Strips to: PATH::.:$HOME/bin
# User wanted: ".:$HOME/bin" (with quotes)
# User gets: .:$HOME/bin ❌ QUOTES LOST!

# When used in shell:
export PATH=.:$HOME/bin  # Different meaning!
# vs
export PATH=".:$HOME/bin"  # What user wanted
```

**Impact:** Environment variables have wrong values. Could break scripts or applications.

#### Existing Production Code Breakage

**Most Critical: Code already using parser will silently break after upgrade.**

```rust
// Production code written before Task 083 fix:
#[derive(Debug)]
struct BookMetadata {
  title: String,
  author: String,
}

fn parse_book_command(args: Vec<String>) -> Result<BookMetadata> {
  let parser = Parser::new(UnilangParserOptions::default());
  let instruction = parser.parse_from_argv(&args)?;

  let title = instruction.named_arguments
    .get("title")
    .and_then(|v| v.first())
    .map(|a| a.value.clone())
    .ok_or("Missing title")?;

  Ok(BookMetadata {
    title,
    author: "Unknown".to_string(),
  })
}

// User runs:
// mycli .book.add 'title::"Chapter 1"'

// BEFORE Task 083 fix:
// args = [".book.add", "title::\"Chapter 1\""]
// Parser receives: title::"Chapter 1"
// Result: BookMetadata { title: "\"Chapter 1\"" }
// Stores in DB: title = "Chapter 1" (with quotes) ✓

// AFTER Task 083 fix:
// args = [".book.add", "title::\"Chapter 1\""]
// Parser receives: title::"Chapter 1"
// Strips quotes: title::Chapter 1
// Result: BookMetadata { title: "Chapter 1" }
// Stores in DB: title = Chapter 1 (NO quotes!) ❌

// NO ERROR! NO WARNING! Just silently different data!
```

**Impact Assessment:**

1. **Silent Breakage** - Code doesn't fail, just produces wrong results
2. **Data Corruption** - Database now contains incorrect data
3. **No Detection** - Tests might pass (if they dont check for quotes)
4. **Hard to Diagnose** - No error logs, no stack traces, just wrong data
5. **Propagates** - Corrupted data spreads through system
6. **No Rollback** - Once stored, original intent is lost

#### Why This Can't Be Dismissed as "Rare"

The original analysis claimed this is "extremely rare" - but provides **ZERO data** to support this claim.

**Counter-arguments:**

1. **No Usage Data** - We dont know how current users use the parser
2. **Silent Failures** - Current users might already work around issues
3. **Multiple Domains** - Books, code, CSV, SQL, HTML, config - many use cases
4. **Production Code** - ANY existing code using parser could break
5. **Conservative Principle** - Better to be safe than assume rarity

**The Burden of Proof:**

Before making breaking changes, we need to show:
- Over-quoting IS common (need data)
- Intentional quotes are NOT common (need data)
- Benefits outweigh risks (need cost/benefit analysis)

**Currently: ZERO data supports the change.**

#### The "Normalized Behavior" Excuse

The original plan says:
> "Document this as normalized behavior"

**This is NOT a solution - it's just giving the breaking change a name.**

**Problems:**

1. **Doesn't Help Users** - "Normalized" doesn't explain why their data is wrong
2. **No Opt-Out** - Users can't disable it if they need literal quotes
3. **Breaking is Still Breaking** - Calling it "normalized" doesn't make it not break things
4. **No Migration Path** - Existing code just breaks, no transition period

**Better Approach:**

If a behavior change breaks existing code, either:
- Make it opt-in (users choose)
- Provide deprecation period (warning first)
- Only apply to new code (version flag)
- Don't make the change (wait for data)

#### Detection and Mitigation

**Current Plan: NONE**

No detection, no warnings, no migration help. Just breaks silently.

**What Should Exist:**

```rust
// Detection at parse time:
if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
  // Detect potential issue
  eprintln!("Warning: Parameter '{key}' has quoted boundaries: {value}");
  eprintln!("  If you used 'param::\"value\"', this is over-quoting.");
  eprintln!("  Try: param::\"value\" instead");
  eprintln!("  If you want literal quotes, use: param::\\\"value\\\"");
  eprintln!();

  // Then decide: strip or preserve?
  if self.options.strip_argv_quotes {
    // Strip with user's explicit consent
    value = value[1..value.len()-1].to_string();
  } else {
    // Preserve (safe default)
    // Let value pass through unchanged
  }
}
```

This gives users:
1. Visibility - They see the warning
2. Education - Learn correct syntax
3. Choice - Can enable stripping if they want
4. Safety - Default preserves existing behavior

### Alternative Approaches (RECOMMENDED)

#### Alternative 1: Opt-In Feature (BEST)

Add parser option to make stripping opt-in:

```rust
pub struct UnilangParserOptions {
  /// Strip surrounding quotes from argv values.
  /// Handles over-quoting like 'param::"value"' but breaks intentional quotes.
  /// Default: false (preserves existing behavior)
  pub strip_argv_quotes: bool,
}
```

**Advantages:**
- ✅ Zero breaking changes (default = false)
- ✅ Users who need it can opt-in
- ✅ Clear migration path
- ✅ Explicit user choice

#### Alternative 2: Heuristic (Strip Only If Whitespace)

Only strip quotes if inner value contains whitespace:

```rust
if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
  let inner = &value[1..value.len()-1];
  if inner.chars().any(char::is_whitespace) || inner.is_empty() {
    // Safe to assume over-quoting (multi-word needs quotes)
    value = inner.to_string();
  }
  // Else: preserve quotes (might be intentional)
}
```

**Advantages:**
- ✅ More conservative
- ✅ Handles main use case (multi-word)
- ✅ Fewer breaking cases (single-word preserved)

**Disadvantages:**
- ⚠️ Still breaks multi-word with intentional quotes

#### Alternative 3: Warning Only (SAFEST)

Detect and warn, but dont modify:

```rust
if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
  eprintln!("Warning: Parameter '{key}' has quoted boundaries.");
  eprintln!("  Did you mean: {key}::{}", &value[1..value.len()-1]);
}
// DON'T strip - just warn
```

**Advantages:**
- ✅ ZERO breaking changes
- ✅ Educates users
- ✅ Helps debugging
- ✅ Addresses root cause (education)

### Detailed Risk Assessment

| Risk Category | Current Plan | Alternative 1 (Opt-In) | Alternative 3 (Warning) |
|--------------|--------------|----------------------|------------------------|
| **Breaking Changes** | HIGH (multiple cases) | NONE (opt-in) | NONE (no modification) |
| **Data Corruption** | HIGH (silent) | LOW (explicit choice) | NONE (no modification) |
| **Migration Cost** | HIGH (all users affected) | LOW (gradual adoption) | NONE (backward compatible) |
| **User Confusion** | HIGH (silent breakage) | LOW (documented option) | LOW (clear warning) |
| **Testing Burden** | HIGH (verify no breaks) | MEDIUM (test both modes) | LOW (no behavior change) |
| **Documentation** | HIGH (explain breaking change) | MEDIUM (document option) | LOW (document warning) |
| **Overall Risk** | **UNACCEPTABLE** | **LOW** | **NONE** |

### Recommendation for Implementation

**DO NOT implement quote stripping as originally designed.**

**Instead:**

1. **Immediate Action** - Implement Alternative 3 (Warning Only)
   - Zero risk, educates users, gathers data
   - Learn if over-quoting is actually common

2. **Gather Data** - Monitor warnings for 2-3 months
   - How many users hit this?
   - Is it actually a problem?

3. **Future Action** - If data shows need:
   - Implement Alternative 1 (Opt-In)
   - No breaking changes, users choose
   - Clear migration path

4. **Or** - If data shows its rare:
   - Keep in backlog
   - Improve documentation instead
   - Current behavior is fine

### Data-Driven Decision Making

Before implementing ANY quote stripping:

**Required Data:**
- [ ] Number of users hitting over-quoting issue
- [ ] Number of users with intentional literal quotes
- [ ] Support ticket volume for this issue
- [ ] User feedback on warning messages
- [ ] Comparison: over-quoting frequency vs literal quotes frequency

**Decision Criteria:**
- IF over-quoting > 10x literal quotes → Consider opt-in stripping
- IF over-quoting < 2x literal quotes → Keep current behavior
- IF unclear → Default to safety (no stripping)

**Current Status:**
- Over-quoting frequency: UNKNOWN
- Literal quotes frequency: UNKNOWN
- Support tickets: ZERO mentioned

**Conclusion:** Insufficient data to justify breaking changes.

## Requirements

-   All work must strictly adhere to all applicable rulebooks
    (discover via `prompt .rulebooks.relevant`)

## Acceptance Criteria

**IMPORTANT: Do NOT implement the original naive quote-stripping approach.**

The original acceptance criteria below are INVALID due to critical problems identified:

~~-   Detection logic added to identify literal quote characters in parameter values~~ ❌
~~-   Quote stripping logic implemented while preserving intentionally escaped quotes~~ ❌ IMPOSSIBLE
~~-   Tests created covering over-quoting scenarios~~ ❌
~~-   Test `test_argv_multiword_parameter_with_shell_quotes_preserved` passes (currently ignored)~~ ❌
~~-   No regressions in existing functionality~~ ❌ WILL CAUSE REGRESSIONS
~~-   Documentation updated explaining quote handling behavior~~ ❌

**Revised Acceptance Criteria (Alternative 3 - Warning Only):**

-   Detection logic added to identify potential over-quoting (quoted boundaries)
-   Warning message emitted when quoted boundaries detected
-   Warning provides educational guidance on correct syntax
-   NO modification to values (preserves existing behavior)
-   Test `test_argv_multiword_parameter_with_shell_quotes_preserved` remains ignored (over-quoting still doesn't work, but neither does it break literal quotes)
-   Zero breaking changes (100% backward compatible)
-   Documentation added explaining:
    -   Correct quoting syntax for multi-word parameters
    -   Why over-quoting happens
    -   How to fix over-quoting
    -   How the warning can be disabled if intentional
-   Data collection mechanism to track warning frequency

**Alternative Path (If data shows need):**

If monitoring shows over-quoting is significantly more common than literal quotes:

-   Implement Alternative 1 (Opt-In Feature)
-   Add `strip_argv_quotes` option to `UnilangParserOptions` (default: false)
-   Comprehensive test suite for both modes (stripping and preserving)
-   Migration guide for users who want to enable stripping
-   Documentation on trade-offs and when to use each mode
