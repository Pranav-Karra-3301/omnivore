# Fix Bug

Systematic approach to fixing bugs in Omnivore.

## Investigation Phase

### 1. Reproduce the Bug

First, confirm the bug exists and understand its behavior:

```bash
# Run specific test if it exists
cargo test test_name

# Or reproduce manually
cargo run --bin omnivore -- [command that triggers bug]
```

### 2. Add Failing Test

Write a test that demonstrates the bug:

```rust
#[test]
fn test_bug_description() {
    // Setup that triggers the bug
    let input = problematic_input();

    // This should pass once the bug is fixed
    let result = function_with_bug(input);

    assert_eq!(result, expected_correct_behavior);
}
```

### 3. Locate the Problem

Use these techniques:

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin omnivore -- [command]

# Add tracing to narrow down
tracing::debug!("Variable state: {:?}", variable);
```

## Fix Phase

### 4. Implement the Fix

Make the minimal change needed:

```rust
// Before (buggy)
fn process(data: &str) -> Option<String> {
    data.split(',').next().map(|s| s.to_string())  // Bug: doesn't handle empty
}

// After (fixed)
fn process(data: &str) -> Option<String> {
    let trimmed = data.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.split(',').next().map(|s| s.trim().to_string())
}
```

### 5. Verify the Fix

```bash
# Run the previously failing test
cargo test test_bug_description

# Run all related tests
cargo test module_name

# Run full test suite
cargo test --workspace
```

## Documentation Phase

### 6. Update Comments if Needed

If the fix changes behavior:

```rust
/// Processes input data, extracting the first value.
///
/// Returns `None` if the input is empty or contains only whitespace.
///
/// # Note
/// Fixed in v0.2.1: Now correctly handles empty strings.
fn process(data: &str) -> Option<String> { ... }
```

## Checklist

- [ ] Bug is reproduced
- [ ] Failing test is added
- [ ] Fix is implemented
- [ ] Test passes
- [ ] No regressions (all tests pass)
- [ ] Fix doesn't introduce new issues
- [ ] Commit message references the bug
