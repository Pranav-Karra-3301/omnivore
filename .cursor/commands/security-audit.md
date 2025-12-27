# Security Audit

Comprehensive security review for Omnivore code.

## Automated Checks

### 1. Dependency Vulnerabilities

```bash
# Check for known vulnerabilities in dependencies
cargo audit

# Update if vulnerabilities found
cargo update
```

### 2. Unsafe Code Audit

Search for unsafe blocks and verify each is justified:

```bash
# Find all unsafe blocks
grep -rn "unsafe" --include="*.rs" omnivore-core/ omnivore-cli/
```

For each `unsafe` block, verify:
- [ ] There's a `# Safety` doc comment explaining why it's safe
- [ ] The invariants are documented
- [ ] There's no safer alternative

## Manual Review Checklist

### Input Validation

Check all external inputs are validated:

- [ ] URLs are parsed and validated before use
- [ ] File paths are sanitized (no path traversal)
- [ ] User-provided patterns are bounded
- [ ] Size limits are enforced

### Injection Prevention

Verify no injection vulnerabilities:

- [ ] No string formatting for SQL queries (use parameterized)
- [ ] No shell command construction from user input
- [ ] No eval-like patterns with user data

### Secret Handling

- [ ] No hardcoded credentials in source
- [ ] Secrets loaded from environment variables
- [ ] No secrets in logs or error messages
- [ ] .env files are gitignored

### Resource Limits

- [ ] Connection pools have maximum size
- [ ] Request/response sizes are bounded
- [ ] Timeouts are set for all network operations
- [ ] Recursive operations have depth limits

### Error Handling

- [ ] Errors don't leak sensitive information
- [ ] Stack traces not exposed to users
- [ ] Graceful degradation on failures

## Crawling-Specific Security

### robots.txt Compliance

- [ ] robots.txt is always checked
- [ ] Disallow directives are honored
- [ ] Crawl-delay is respected

### Rate Limiting

- [ ] Per-domain rate limits exist
- [ ] Global rate limits exist
- [ ] Backoff on 429/503 responses

### Data Handling

- [ ] No storage of authentication data
- [ ] Personal data handling follows principles
- [ ] Crawled data properly attributed

## Report Template

```markdown
## Security Audit Report - [Date]

### Automated Scans
- cargo audit: [PASS/FAIL]
- clippy security lints: [PASS/FAIL]

### Manual Review
- Input validation: [OK/ISSUES FOUND]
- Injection risks: [OK/ISSUES FOUND]
- Secret handling: [OK/ISSUES FOUND]
- Resource limits: [OK/ISSUES FOUND]

### Findings
1. [Description of any issues]

### Recommendations
1. [Suggested fixes]
```
