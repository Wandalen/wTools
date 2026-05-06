# Feature Doc Entity

### Scope

- **Purpose**: Document the user-facing capabilities of unitore as navigational hubs.
- **Responsibility**: Aggregates source, test, and design links for each CLI command group.
- **In Scope**: The three command groups: subscription management, content retrieval, and data access.
- **Out of Scope**: Storage implementation details (→ `api/`); behavioral guarantees (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Subscription Management](001_subscription_management.md) | Manage feed config file subscriptions | ✅ |
| 002 | [Content Retrieval](002_content_retrieval.md) | Download and list feed content | ✅ |
| 003 | [Data Access](003_data_access.md) | Direct SQL query and table inspection | ✅ |
