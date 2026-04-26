# Pattern Doc Entity

### Scope

- **Purpose**: Documents reusable design solutions applied in this crate.
- **Responsibility**: Indexes all pattern doc instances in the `pattern/` collection.
- **In Scope**: Named, reusable design patterns with explicit Problem/Solution/Applicability/Consequences.
- **Out of Scope**: Algorithm implementations (→ `algorithm/`), API contracts (→ `api/`), code-level implementation details (→ source comments).

### Overview Table

| ID  | Name                                                                    | Purpose                                                         | Status |
|-----|-------------------------------------------------------------------------|-----------------------------------------------------------------|--------|
| 001 | [Trait-Based Dependency Injection](001_trait_based_di.md)               | Decouple terminal I/O for full testability without a real TTY   | ✅     |
| 002 | [Test Double Terminal](002_test_double_terminal.md)                     | Programmable terminal implementation for tests without mocking  | ✅     |
| 003 | [Progressive API Disclosure](003_progressive_api_disclosure.md)         | Layered public API scaled to user complexity needs              | ✅     |
| 004 | [Domain-Based Test Organization](004_domain_based_test_organization.md) | Test files grouped by functional domain, not test methodology   | ✅     |
