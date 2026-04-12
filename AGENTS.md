# AGENTS.md

## Code Conventions & Standards

### Error Handling
*   **Library Code (`src/lib.rs`):** All library code must utilize `thiserror` to define custom, strongly-typed error enums. Functions must return a `Result<T, TzbuddyError>` to ensure all errors are explicitly propagated.
*   **Main Code (`src/main.rs`):** The main application should use `anyhow` or `thiserror`'s standard error handling at the entry point to gracefully translate library errors for the user.

### Build Validation
*   **Pre-Commit/Post-Change Validation:** After any code changes, the build must be validated using the following commands to ensure correctness and adherence to style:
    *   `cargo build`
    *   `cargo clippy -- -D warnings` (Run in pedantic mode)