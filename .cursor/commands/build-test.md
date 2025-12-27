# Build and Test

Run the full build and test suite for the Omnivore project.

## Steps

1. **Format Check**: Verify code formatting
   ```bash
   cargo fmt --all -- --check
   ```

2. **Lint**: Run Clippy with warnings as errors
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

3. **Build**: Compile the project
   ```bash
   cargo build --workspace
   ```

4. **Test**: Run all tests
   ```bash
   cargo test --workspace
   ```

5. **Doc Tests**: Verify documentation examples
   ```bash
   cargo test --doc
   ```

## Expected Output

- All formatting checks pass
- No Clippy warnings
- Build succeeds without errors
- All tests pass

## If Tests Fail

1. Read the error message carefully
2. Identify which test failed and why
3. Check if it's a code issue or a test environment issue
4. Fix the underlying problem, don't just skip the test

## Quick Command

For a fast CI-like check:
```bash
make ci
```
