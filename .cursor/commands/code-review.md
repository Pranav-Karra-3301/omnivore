# Code Review

Checklist for reviewing code changes in Omnivore.

## Quick Checks

Run automated validation first:

```bash
# Format, lint, test, audit
make ci
```

## Review Categories

### 1. Correctness

- [ ] Does the code do what it's supposed to?
- [ ] Are edge cases handled?
- [ ] Is error handling appropriate?
- [ ] Are there any logic errors?

### 2. Safety

- [ ] No `.unwrap()` in production paths
- [ ] No ignored `Result` values
- [ ] Input validation present
- [ ] No security vulnerabilities introduced
- [ ] Resources properly cleaned up

### 3. Performance

- [ ] No unnecessary allocations
- [ ] Appropriate use of async/await
- [ ] No blocking operations in async code
- [ ] Reasonable algorithmic complexity

### 4. Rust Idioms

- [ ] Uses Result/Option appropriately
- [ ] Leverages ownership/borrowing correctly
- [ ] No unnecessary clones
- [ ] Uses standard library where appropriate

### 5. Code Style

- [ ] Follows project naming conventions
- [ ] Code is readable and well-organized
- [ ] No dead code
- [ ] Appropriate abstraction level

### 6. Documentation

- [ ] Public APIs are documented
- [ ] Complex logic has comments
- [ ] Examples provided where helpful
- [ ] Error conditions documented

### 7. Testing

- [ ] Tests cover the new/changed code
- [ ] Tests are meaningful (not just for coverage)
- [ ] Edge cases are tested
- [ ] Async tests use `#[tokio::test]`

## Review Comments Template

### Approval

```
LGTM! ✓

- Tests pass
- Code follows project standards
- Changes are well-documented
```

### Request Changes

```
Changes requested:

1. **[file:line]** - [Issue description]
   Suggestion: [How to fix]

2. **[file:line]** - [Issue description]
   Suggestion: [How to fix]

Please address these before merging.
```

### Questions

```
I have some questions:

1. **[file:line]** - Why is [approach] used here instead of [alternative]?

2. **[file:line]** - Could this cause [potential issue] in [scenario]?
```

## Common Issues to Watch For

```rust
// Issue: Unwrap in non-test code
let value = result.unwrap();  // ❌
let value = result?;          // ✓

// Issue: Ignoring errors
let _ = operation();          // ❌
operation()?;                 // ✓

// Issue: Blocking in async
std::fs::read(path)           // ❌ in async context
tokio::fs::read(path).await   // ✓

// Issue: Unbounded collections
let mut vec = Vec::new();     // ❌ if filled from untrusted source
let mut vec = Vec::with_capacity(MAX_SIZE);  // ✓

// Issue: Missing validation
fn process(url: &str) { ... }  // ❌
fn process(url: &Url) { ... }  // ✓ (URL already validated)
```
