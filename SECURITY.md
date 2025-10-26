# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

### How to Report

Send an email to **security@wadah.ai** with:

1. **Description**: Clear description of the vulnerability
2. **Impact**: What could an attacker do?
3. **Steps to Reproduce**: Detailed reproduction steps
4. **Proof of Concept**: Code, screenshots, or demo
5. **Suggested Fix**: If you have ideas (optional)
6. **Contact Info**: How we can reach you

### Example Report

```
Subject: [SECURITY] Agent Policy Bypass via Race Condition

Description:
A race condition in the PolicyEnforcer allows an agent to bypass
ToolCaps restrictions by making rapid concurrent tool calls.

Impact:
An agent could access blocked tools or exceed rate limits, potentially
leading to unauthorized API access or resource exhaustion.

Steps to Reproduce:
1. Create agent with restrictive ToolCaps
2. Implement tool that makes 100 concurrent calls
3. Observe that some calls bypass policy checks

Proof of Concept:
See attached exploit.rs

Suggested Fix:
Use Arc<Mutex<>> around policy state or implement proper locking.

Contact:
alice@security-research.example
```

### What to Expect

1. **Acknowledgment**: Within 48 hours
2. **Initial Assessment**: Within 7 days
3. **Status Updates**: Every 7-14 days
4. **Patch Timeline**: Depends on severity
   - Critical: 7-14 days
   - High: 14-30 days
   - Medium: 30-60 days
   - Low: Best effort

### Disclosure Policy

- We follow **coordinated disclosure**
- We will not disclose until a patch is available
- We will credit researchers (unless you prefer anonymity)
- We may request embargo period for critical issues

### Security Advisories

- Published at: https://github.com/zenri/wadah/security/advisories
- Subscribe to security notifications

## Security Best Practices

### For Operators

1. **Always set ToolCaps**
   ```yaml
   policy:
     toolcaps: ToolCaps.json
   ```

2. **Enable tracing for audit**
   ```bash
   wadah run agent.wpkg --trace audit.jsonl
   ```

3. **Use environment variables for secrets**
   ```yaml
   env:
     OPENAI_API_KEY: "${OPENAI_API_KEY}"
   ```

4. **Set budget limits**
   ```yaml
   policy:
     budgets:
       usd_per_day: 10.0
   ```

5. **Verify packages before running**
   ```bash
   wadah verify untrusted.wpkg
   ```

### For Developers

1. **Input validation**
   ```rust
   fn sanitize_input(input: &str) -> String {
       // Remove potential injection patterns
   }
   ```

2. **Output validation**
   ```rust
   if response.contains_secrets() {
       return Err(SecurityError::LeakedSecret);
   }
   ```

3. **Least privilege**
   ```json
   {
     "tool": "filesystem",
     "actions": ["read"],
     "fs": {"paths": ["./data"], "write": false}
   }
   ```

4. **Audit dependencies**
   ```bash
   cargo audit
   ```

### For Users

1. **Review ToolCaps before running agents**
2. **Only install from trusted sources**
3. **Monitor trace files for anomalies**
4. **Rotate API keys regularly**
5. **Report suspicious behavior**

## Known Security Considerations

### Prompt Injection

**Status**: Active research area

**Mitigation**:
- Input sanitization (partial)
- Context separation
- User education

**Tracking**: Issue #TODO

### Model Backdoors

**Status**: Difficult to detect

**Mitigation**:
- Use trusted model providers
- Pin model versions in lockfiles
- Output anomaly detection (roadmap)

### Supply Chain

**Status**: Mitigated via digests

**Mitigation**:
- SHA256 verification of all artifacts
- Lockfiles with exact versions
- Package signing (roadmap)

## Security Updates

Subscribe to security updates:

- **GitHub Watch**: Click "Watch" → "Custom" → "Security alerts"
- **Email**: security-announce@wadah.ai
- **RSS**: https://github.com/zenri/wadah/security/advisories.atom

## Bug Bounty

**Status**: No formal program yet

Considering launching a bug bounty program for:
- Critical vulnerabilities in core runtime
- Policy bypass mechanisms
- Supply chain attacks

Stay tuned for updates.

## Security Audit

**Status**: Not yet audited

Planning professional security audit for v1.0 release.

Interested in auditing? Contact: security@wadah.ai

## Hall of Fame

Security researchers who have helped improve Wadah:

- (Your name could be here!)

## Additional Resources

- [Threat Model](docs/ThreatModel.md)
- [ToolCaps Security](docs/ToolCaps.md)
- [OWASP Top 10 for LLMs](https://owasp.org/www-project-top-10-for-large-language-model-applications/)

## Contact

- **Security Issues**: security@wadah.ai
- **General Questions**: team@wadah.ai
- **PGP Key**: https://wadah.ai/security.asc

---

We take security seriously. Thank you for helping keep Wadah and our users safe.

