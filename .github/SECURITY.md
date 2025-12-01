# Security Policy

## Supported Versions

We use a **latest release only** security support model:

| Version | Supported          | Notes |
| ------- | ------------------ | ----- |
| Latest (0.0.x) | :white_check_mark: | Current release only |
| Previous releases | :x: | Please upgrade to latest |

**Why latest-only?** As Gazelle is rapidly evolving (pre-1.0), we focus our resources on the most current release. This ensures:
- Faster security response times
- Simplified maintenance
- Users benefit from the latest features and improvements
- Clear upgrade path for security fixes

**Future Policy**: Once Gazelle reaches v1.0 and stabilizes, we will reassess our support policy based on user needs and adoption patterns.

## Reporting a Vulnerability

We take the security of Gazelle seriously. If you discover a security vulnerability, please follow these steps:

### 1. **DO NOT** create a public GitHub issue

Security vulnerabilities should not be reported through public GitHub issues as this could put users at risk.

### 2. Report privately via GitHub Security Advisories

1. Go to the [Security tab](https://github.com/jsbayley/gazelle/security) of this repository
2. Click "Report a vulnerability"
3. Fill out the security advisory form with:
   - **Summary**: Brief description of the vulnerability
   - **Details**: Technical details about the issue
   - **Affected versions**: Which versions are impacted
   - **Severity**: Your assessment of the impact level

### 3. Alternative: Email reporting

If GitHub Security Advisories are not available, you can report security issues by contacting the maintainers directly through:
- Creating a private issue via GitHub (if you have access)
- Contacting us through the project's main communication channels

### 4. What to include in your report

Please include as much of the following information as possible:

- **Type of vulnerability** (e.g., code injection, privilege escalation, etc.)
- **Affected components** (CLI, library, web interface, etc.)
- **Attack scenario** - how the vulnerability could be exploited
- **Impact assessment** - what an attacker could achieve
- **Proof of concept** - steps to reproduce (if safe to share)
- **Suggested fix** - if you have ideas for remediation

## Response Timeline

- **Initial response**: Within 48 hours of receiving the report
- **Status update**: Within 1 week with preliminary assessment
- **Resolution**: Security fixes will be prioritized and released as soon as possible

## Security Considerations for Gazelle

### File Processing Security

Gazelle processes structural engineering files from various sources. When reporting vulnerabilities, consider:

- **File parsing vulnerabilities**: Malicious input files that could cause crashes or code execution
- **Path traversal**: Issues with file path handling that could access unintended files
- **Memory safety**: Buffer overflows or excessive memory consumption
- **External tool integration**: Security issues with ETABS or other external software integration

### CLI Security

The Gazelle CLI processes user input and file paths. Relevant security concerns include:

- **Command injection**: Malicious arguments that could execute unintended commands
- **File system access**: Unauthorized access to files outside the intended scope
- **Input validation**: Improper handling of malformed inputs

### Dependency Security

Report vulnerabilities in:
- .NET dependencies and the runtime environment
- NuGet packages used by Gazelle
- Build and CI pipeline security

## Security Best Practices for Users

While using Gazelle, follow these security practices:

1. **Validate input files**: Only process files from trusted sources
2. **Run with minimal privileges**: Don't run Gazelle with unnecessary elevated permissions
3. **Keep updated**: Use the latest version to ensure you have security fixes
4. **Isolate processing**: Consider running Gazelle in isolated environments for untrusted files

## Upgrading for Security

When security updates are released:

1. **Download the latest release** from [Gazelle.sh](https://gazelle.sh)
2. **Replace your existing binary** with the updated version
3. **Verify the fix** by checking the release notes for security details
4. **Test your workflows** to ensure compatibility

## Disclosure Policy

- We will work with security researchers to understand and fix reported vulnerabilities
- We will publicly acknowledge security researchers who responsibly disclose vulnerabilities (unless they prefer to remain anonymous)
- Security advisories will be published after fixes are released to inform users of the issue and mitigation steps

## Security Updates

Security updates will be:
- Released as patch versions (e.g., 0.0.7 → 0.0.8)
- Announced through GitHub Releases with **[SECURITY]** tags in release notes
- Documented in the changelog with clear security impact descriptions
- Available immediately through our automated CI/CD pipeline

## Questions?

If you have questions about this security policy or need clarification about reporting procedures, please:
- Check the [Contributing Guidelines](.github/CONTRIBUTING.md)
- Open a general discussion in [GitHub Discussions](https://github.com/jsbayley/gazelle/discussions)
- Review existing [security advisories](https://github.com/jsbayley/gazelle/security/advisories)

Thank you for helping keep Gazelle and the structural engineering community safe! 🔒