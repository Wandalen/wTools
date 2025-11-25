# Unilang Critical Issue: Make Command/Help Divergence Unrepresentable

**Date**: 2025-10-21
**Priority**: CRITICAL
**Category**: API Design - Prevent Misuse
**Status**: Proposed

## Problem Statement

**Unilang currently allows commands to be registered in `CommandRegistry` but not appear in help/usage listings, violating the "make misuse impossible" principle.**

This architectural flaw was discovered in production usage where:
- Command `.languages` was properly registered in `wflow.commands.yaml`
- Build script correctly generated PHF map and loaded into registry
- Command executed successfully when invoked
- **Command was INVISIBLE in help output** because help was manually maintained separately

This is a **catastrophic usability failure** that violates unilang's core principle: "The most important principle is making misuse of the crate difficult."

## Minimal Reproducible Example

### Current API (Broken by Design)

```rust
use unilang::prelude::*;

// Step 1: User creates command registry
let mut registry = CommandRegistry::new();

// Step 2: User defines and registers commands
let cmd1 = CommandDefinition::former()
  .name( ".languages" )
  .description( "Detect languages in project" )
  .hint( "Language detection" )
  .examples( vec![ ".languages", ".languages src/" ] )
  .form();

registry.register( cmd1 );

// Step 3: User implements command dispatch
fn handle_command( input: &str, registry: &CommandRegistry ) {
  if input.starts_with( ".languages" ) {
    // Command executes successfully!
    detect_languages();
  }
}

// Step 4: User implements help (SEPARATE SYSTEM!)
fn print_help() {
  println!( "Available commands:" );
  println!( "  .cat - Concatenate files" );
  println!( "  .files - List files" );
  // ❌ FORGOT .languages - it's in registry but NOT in help!
}

// Result: Command works but is invisible to users!
// This is a DESIGN FAILURE, not a user error.
```

### The Failure Mode

**What happened in wflow:**

```
wflow.commands.yaml:
  - name: .cat           ✅ Registered
  - name: .files         ✅ Registered
  - name: .languages     ✅ Registered  ← ADDED

build.rs:
  Generated PHF map      ✅ All 3 commands included

main.rs::print_usage():
  .cat                   ✅ Manually listed
  .files                 ✅ Manually listed
  .languages             ❌ FORGOT TO ADD!  ← INVISIBLE!

Result:
  $ wflow .languages     ✅ Works perfectly
  $ wflow --help         ❌ Doesn't show .languages!
```

**This is IMPOSSIBLE TO DETECT** because:
- No compiler errors
- No runtime errors
- Command registry is valid
- Command executes correctly
- Tests pass (command works!)
- **Users just can't discover the command exists**

## Root Cause Analysis

### Architectural Flaw: Separation of Concerns Gone Wrong

Unilang provides:
1. `CommandRegistry` - stores command definitions
2. `HelpCommandGenerator` - generates help from definitions

But **doesn't enforce** that help generation is used!

**Current architecture allows:**
```rust
// This compiles and runs:
let registry = CommandRegistry::new();
registry.register( some_command );

// User can:
// A) Generate help from registry (correct) ✅
// B) Write help manually (wrong) ❌
// C) Mix both (disaster) ❌
// D) Forget help entirely (catastrophic) ❌

// All four options compile!
// Only option A is correct!
// Options B, C, D ship broken software!
```

### Why This Violates "Prevent Misuse First"

From `task/architectural_principles_toolkit_not_framework.md`:

> "The most important principle is making misuse of the crate difficult"
> "Enforce best practices at compile/registration time"
> "Make correct usage the easy path"
> "Type safety over convenience"

**Current API FAILS all four criteria:**
- ❌ Misuse is EASY (just forget to call help generator)
- ❌ No compile-time enforcement (divergence is invisible)
- ❌ Correct usage requires REMEMBERING to sync two systems
- ❌ No type safety prevents registry/help divergence

## Proposed Solution: Make Incorrect States Unrepresentable

### Principle: Command Registration MUST Include Help

**Core Insight:** If you can't register a command without also registering its help, then command/help divergence becomes **impossible** at the type level.

### Solution 1: Registry API Enforces Help Generation

```rust
/// NEW API: CommandRegistry enforces help generation at registration
pub struct CommandRegistry
{
  commands: HashMap< String, CommandDefinition >,
  help_generator: HelpCommandGenerator,  // ← REQUIRED, not optional
}

impl CommandRegistry
{
  /// Create registry (help generation is built-in)
  pub fn new() -> Self
  {
    Self
    {
      commands: HashMap::new(),
      help_generator: HelpCommandGenerator::new(),
    }
  }

  /// Register command AND its help (atomic operation)
  pub fn register( &mut self, cmd: CommandDefinition )
  {
    let cmd_name = cmd.name.clone();

    // Add to help generator AUTOMATICALLY
    self.help_generator.add_command( cmd.clone() );

    // Generate and register .help command AUTOMATICALLY
    if let Some( help_cmd ) = self.help_generator.generate_help_for_command( &cmd_name, &cmd )
    {
      self.commands.insert( help_cmd.name.clone(), help_cmd );
    }

    // Register original command
    self.commands.insert( cmd_name, cmd );
  }

  /// Get ALL commands (including auto-generated help commands)
  pub fn list_commands( &self ) -> Vec< String >
  {
    self.commands.keys()
      .filter( | name | !name.ends_with( ".help" ) )
      .cloned()
      .collect()
  }

  /// Get formatted help listing (ALWAYS in sync with registry!)
  pub fn format_command_listing( &self ) -> String
  {
    let mut output = String::from( "Available commands:\n" );

    for cmd_name in self.list_commands()
    {
      if let Some( cmd ) = self.commands.get( &cmd_name )
      {
        output.push_str( &format!( "  {:<30} - {}\n", cmd_name, cmd.hint ) );
      }
    }

    output
  }

  /// Get help text for command (guaranteed to exist if command exists!)
  pub fn get_help( &self, cmd_name: &str ) -> Option< String >
  {
    self.help_generator.get_help_text( cmd_name )
  }
}
```

**Usage (New API):**

```rust
// User's code (CORRECT USAGE IS NOW THE ONLY OPTION)
let mut registry = CommandRegistry::new();

// Register command
registry.register( CommandDefinition::former()
  .name( ".languages" )
  .description( "Detect languages" )
  .hint( "Language detection" )
  .form()
);

// Print help (ALWAYS CORRECT, ALWAYS COMPLETE)
println!( "{}", registry.format_command_listing() );
// Output includes .languages automatically!

// Get help for specific command (ALWAYS EXISTS)
println!( "{}", registry.get_help( ".languages" ).unwrap() );
// .languages.help was generated automatically!
```

**Impossibility Guarantees:**

1. ✅ **Can't register command without help** - `register()` generates help atomically
2. ✅ **Can't have help without command** - help generated FROM command definition
3. ✅ **Can't forget to show command in listing** - `format_command_listing()` uses registry
4. ✅ **Can't have stale help** - help regenerated from current command definition
5. ✅ **Can't mix manual and auto help** - no API for manual help exists

### Solution 2: Type-State Pattern (Advanced)

For even stronger compile-time guarantees:

```rust
/// Type-state: Command without help (can't be registered)
pub struct UnregisteredCommand
{
  definition: CommandDefinition,
}

/// Type-state: Command with help (ready to register)
pub struct RegisterableCommand
{
  definition: CommandDefinition,
  help_command: CommandDefinition,
}

impl UnregisteredCommand
{
  /// Create command (no help yet)
  pub fn new( def: CommandDefinition ) -> Self
  {
    Self { definition: def }
  }

  /// Generate help (transitions to Registerable state)
  pub fn with_help( self ) -> RegisterableCommand
  {
    let help_gen = HelpCommandGenerator::new();
    let help_cmd = help_gen
      .generate_help_for_command( &self.definition.name, &self.definition )
      .expect( "Help generation failed" );

    RegisterableCommand
    {
      definition: self.definition,
      help_command: help_cmd,
    }
  }
}

impl CommandRegistry
{
  /// Can ONLY register commands that have help!
  pub fn register( &mut self, cmd: RegisterableCommand )
  {
    self.commands.insert( cmd.definition.name.clone(), cmd.definition );
    self.commands.insert( cmd.help_command.name.clone(), cmd.help_command );
  }
}
```

**Usage (Type-State API):**

```rust
let cmd = UnregisteredCommand::new( CommandDefinition::former()
  .name( ".languages" )
  .form()
);

// This won't compile (no help yet):
// registry.register( cmd ); // ❌ Type error!

// Must add help first:
let cmd_with_help = cmd.with_help(); // ← Transitions type!

// Now registration works:
registry.register( cmd_with_help ); // ✅ Compiles!
```

**Type-level guarantees:**
- Can't pass `UnregisteredCommand` to `register()` - **compile error**
- Can't forget `.with_help()` - **compile error**
- Help divergence is **impossible by construction**

## Implementation Plan

### Phase 1: Non-Breaking Enhancement (v0.16.0)

Add new methods to `CommandRegistry` without breaking existing API:

```rust
impl CommandRegistry
{
  /// NEW: Register with automatic help generation
  pub fn register_with_auto_help( &mut self, cmd: CommandDefinition )
  {
    // Auto-generate help
    let mut help_gen = HelpCommandGenerator::new();
    help_gen.add_command( cmd.clone() );

    if let Some( help_cmd ) = help_gen.generate_help_for_command( &cmd.name, &cmd )
    {
      self.register( help_cmd );
    }

    self.register( cmd );
  }

  /// NEW: Get formatted command listing
  pub fn format_command_listing( &self ) -> String
  {
    // Generate from registry
  }

  /// NEW: Validate that all commands have help
  pub fn validate_help_completeness( &self ) -> Result< (), Vec< String > >
  {
    let mut missing_help = Vec::new();

    for cmd_name in self.list_commands()
    {
      let help_name = format!( "{}.help", cmd_name );
      if !self.commands.contains_key( &help_name )
      {
        missing_help.push( cmd_name );
      }
    }

    if missing_help.is_empty()
    {
      Ok( () )
    }
    else
    {
      Err( missing_help )
    }
  }
}
```

**Migration path:**
- Users can adopt `register_with_auto_help()` incrementally
- `validate_help_completeness()` catches divergence at runtime
- No breaking changes to existing code

### Phase 2: Breaking Change (v0.17.0 or v1.0.0)

Make help generation mandatory:

```rust
impl CommandRegistry
{
  /// register() now ALWAYS generates help
  pub fn register( &mut self, cmd: CommandDefinition )
  {
    self.register_with_auto_help( cmd );
  }

  /// Deprecated: Manual registration without help
  #[ deprecated( since = "0.17.0", note = "Help is now auto-generated" ) ]
  pub fn register_without_help( &mut self, cmd: CommandDefinition )
  {
    // Old behavior (emit deprecation warning)
  }
}
```

### Phase 3: Type-State API (v2.0.0)

Full type-safety with compile-time guarantees (opt-in feature):

```rust
#[ cfg( feature = "type-state-registry" ) ]
mod type_state
{
  // Type-state implementation
}
```

## Success Criteria

After implementation, these scenarios must be **impossible**:

1. ✅ Command registered but not in help listing
2. ✅ Command in help but not registered
3. ✅ Command help out of sync with command definition
4. ✅ Forgot to call help generator
5. ✅ Mixed manual and auto-generated help

**Verification:**

```rust
#[ test ]
fn test_command_help_divergence_impossible()
{
  let mut registry = CommandRegistry::new();

  registry.register( CommandDefinition::former()
    .name( ".test" )
    .description( "Test command" )
    .form()
  );

  // Help listing MUST include .test
  let listing = registry.format_command_listing();
  assert!( listing.contains( ".test" ), "Registered command missing from listing!" );

  // Help command MUST exist
  assert!( registry.get_help( ".test" ).is_some(), "Help not auto-generated!" );

  // Validation MUST pass
  assert!( registry.validate_help_completeness().is_ok(), "Help incomplete!" );
}
```

## Breaking Change Assessment

### Phase 1 (v0.16.0) - **Non-Breaking** ✅
- Add new methods
- Deprecation warnings for manual help
- Full backward compatibility

### Phase 2 (v0.17.0) - **Breaking** ⚠️
- `register()` behavior changes (auto-generates help)
- Users relying on manual help will break
- **Justification:** Prevents shipping broken CLI apps

### Phase 3 (v2.0.0) - **Major Breaking** 🔴
- Type-state API is opt-in feature
- Requires code refactoring for full benefits
- **Justification:** Compile-time safety > convenience

## Documentation Requirements

### New Section: "Automatic Help Generation"

```markdown
## Automatic Help Generation

**IMPORTANT:** Every command registered in `CommandRegistry` automatically gets a
corresponding `.command.help` entry. You **cannot** and **should not** create help
text manually.

### Why Automatic Help?

Manual help maintenance leads to:
- Commands missing from help listings
- Outdated help text
- Inconsistent formatting
- Broken user experience

Unilang prevents these issues by making help generation **automatic and mandatory**.

### How It Works

When you register a command:

```rust
registry.register( CommandDefinition::former()
  .name( ".languages" )
  .description( "Detect programming languages" )
  .hint( "Language detection with statistics" )
  .examples( vec![ ".languages", ".languages src/" ] )
  .form()
);
```

Unilang **automatically**:
1. Generates `.languages.help` command
2. Extracts help text from `CommandDefinition`
3. Formats help with consistent styling
4. Adds to command listing

### Usage

```rust
// Get formatted command listing
println!( "{}", registry.format_command_listing() );

// Get help for specific command
if let Some( help ) = registry.get_help( ".languages" )
{
  println!( "{}", help );
}

// Validate help completeness (testing)
registry.validate_help_completeness()
  .expect( "All commands must have help!" );
```

### Migration from Manual Help

**Before (manual help, WRONG):**
```rust
fn print_help()
{
  println!( "Commands:" );
  println!( "  .cat - Concatenate files" );
  println!( "  .languages - Detect languages" );
}
```

**After (automatic help, CORRECT):**
```rust
fn print_help( registry: &CommandRegistry )
{
  println!( "{}", registry.format_command_listing() );
}
```
```

## Related Issues

- `task/architectural_principles_toolkit_not_framework.md` - Principle #1: Prevent Misuse
- `task/-ergonomic_improvements_plan.md` - Mandatory Help System (Item #1)
- `spec.md` - Section on Help System Design

## Real-World Impact

**This issue was discovered in production** in the `wflow` CLI tool where:
- 8 commands were defined in registry
- Only 7 appeared in help output
- Users couldn't discover `.languages` command existed
- No error messages, no warnings, no indication of the problem
- Bug existed for weeks before manual discovery

**This would never happen** with the proposed API because:
```rust
// Old API (allowed divergence):
registry.register( languages_cmd );
print_help(); // ← Manual function, can forget commands

// New API (divergence impossible):
println!( "{}", registry.format_command_listing() ); // ← Always complete
```

## Acceptance Criteria

- [ ] `CommandRegistry::register()` auto-generates `.command.help` entries
- [ ] `CommandRegistry::format_command_listing()` returns complete command list
- [ ] `CommandRegistry::validate_help_completeness()` catches divergence
- [ ] No API exists for manual help registration
- [ ] Documentation explains why help is automatic
- [ ] Migration guide for existing users
- [ ] Comprehensive tests prevent regression
- [ ] Real-world example (wflow) refactored successfully

## References

### Comparison with Other Frameworks

**clap (Rust):**
```rust
// Help is AUTOMATIC from derive macro
#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

// Can't have command without help - it's generated!
```

**click (Python):**
```python
# Help is AUTOMATIC from decorator
@click.command()
@click.option('--name', help='Your name')
def hello(name):
    click.echo(f'Hello {name}!')

# --help is auto-generated, can't be forgotten!
```

**Unilang should follow this pattern:** Help is mandatory, automatic, and impossible to forget.

---

**Status**: Awaiting review and approval for implementation
**Priority**: CRITICAL - This prevents users from shipping broken CLI applications
**Effort**: Medium (2-3 weeks for all phases)
