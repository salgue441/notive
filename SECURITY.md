# Security Policy

Thank you for helping keep Notive and its users safe!

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take the security of Notive seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### Please Do

- **Report privately**: Use [GitHub Security Advisories](https://github.com/YOUR_USERNAME/notive/security/advisories/new) to report vulnerabilities privately
- **Provide details**: Include as much information as possible to help us understand and reproduce the issue
- **Be patient**: Allow reasonable time for us to respond and address the issue
- **Act in good faith**: Avoid accessing or modifying user data without permission

### Please Don't

- **Don't disclose publicly**: Avoid posting about the vulnerability on social media, forums, or other public channels until we've addressed it
- **Don't exploit**: Don't use the vulnerability to access data beyond what's necessary to demonstrate the issue
- **Don't demand payment**: We don't offer bug bounties, but we do acknowledge security researchers in our release notes

## What to Include in Your Report

To help us triage and respond quickly, please include:

1. **Type of vulnerability**
   - Remote Code Execution (RCE)
   - Cross-Site Scripting (XSS)
   - Privilege Escalation
   - Information Disclosure
   - Denial of Service (DoS)
   - Other

2. **Affected component**
   - Tauri backend (Rust)
   - Frontend (TypeScript)
   - WebView configuration
   - IPC commands
   - Auto-updater
   - Other

3. **Steps to reproduce**
   - Detailed steps to reproduce the vulnerability
   - Proof of concept code if applicable
   - Screenshots or videos if helpful

4. **Impact assessment**
   - What can an attacker achieve?
   - What user interaction is required?
   - What data or systems are at risk?

5. **Environment**
   - Notive version
   - Linux distribution
   - Desktop environment
   - Installation method (AppImage, .deb, etc.)

## Response Timeline

| Stage | Target Time |
|-------|-------------|
| Initial response | 48 hours |
| Issue triage | 7 days |
| Fix development | Depends on severity |
| Public disclosure | After fix is released |

### Severity-Based Response

| Severity | Fix Timeline | Description |
|----------|--------------|-------------|
| **Critical** | 24-72 hours | RCE, privilege escalation, data breach |
| **High** | 7 days | Significant security impact |
| **Medium** | 30 days | Limited security impact |
| **Low** | Next release | Minimal security impact |

## Security Measures

Notive implements several security measures:

### Content Security Policy (CSP)

We enforce a strict CSP to prevent XSS and other injection attacks:

```
default-src 'self' https://www.notion.so https://*.notion.so
script-src 'self' https://*.notion.so (with necessary exceptions for Notion)
frame-src 'self' https://*.notion.so https://accounts.google.com
```

### WebView Isolation

- Tauri's WebView runs with restricted permissions
- IPC commands are explicitly whitelisted via capabilities
- External links open in the system browser

### Secure Storage

- Sensitive data stored using OS keyring
- Settings stored in user-only accessible directories
- No plaintext credential storage

### Update Security

- Updates are signed with our private key
- Signature verification before installation
- HTTPS-only update channels

### Dependency Management

- Regular security audits via `cargo audit` and `pnpm audit`
- Dependabot for automated dependency updates
- CodeQL analysis on every PR

## Security Best Practices for Users

1. **Keep Notive updated** - Enable auto-updates or check for updates regularly
2. **Download from official sources** - Only install from GitHub releases or official package repositories
3. **Verify signatures** - For manual downloads, verify file checksums
4. **Report suspicious behavior** - If something seems wrong, let us know

## Acknowledgments

We gratefully acknowledge security researchers who help improve Notive:

<!-- Security researchers will be listed here -->

*No vulnerabilities have been reported yet.*

---

## Contact

- **Security issues**: [GitHub Security Advisories](https://github.com/YOUR_USERNAME/notive/security/advisories/new)
- **General questions**: [GitHub Discussions](https://github.com/YOUR_USERNAME/notive/discussions)

Thank you for helping keep Notive secure!
