# Actor: validator

- **Canonical:** validator
- **Name:** Independent Validator
- **Role:** Validator
- **Aliases:** —
- **Notes:** Dispatched as a fresh, context-isolated agent per task (`validation.rulebook.md § Principles : Validator Independence`) — never resumes a prior session, never shares memory with the actor that executed the work. Used exclusively for the 🔎 (Validating) → ✅ (Completed) gate; never assigned as `actor` for 🔬 (Verifying) or ⚙️ (Executing) transitions.
