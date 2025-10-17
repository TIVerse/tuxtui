# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of tuxtui seriously. If you believe you have found a security vulnerability, please report it to us responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **eshanized@proton.me**

Include the following information:
- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### What to Expect

- You will receive an acknowledgment within 48 hours
- We will investigate and provide updates on our progress
- We will work with you to understand and resolve the issue
- We will credit you in any public disclosure (unless you prefer to remain anonymous)

### Safe Harbor

We support safe harbor for security researchers:
- We will not pursue legal action against you for good faith security research
- We will work with you to understand and resolve the issue
- We will not disclose your personal information without your consent

## Security Updates

Security updates will be released as patch versions and documented in:
- CHANGELOG.md
- GitHub Security Advisories
- Crates.io release notes

## Best Practices for Users

- Always use the latest stable version
- Review the CHANGELOG for security updates
- Follow security best practices in your own code
- Be cautious when using unstable features
- Report any suspicious behavior

## No Unsafe Code Policy

tuxtui forbids unsafe code (`#![forbid(unsafe_code)]`) to minimize security risks. Any exception must be:
- Absolutely necessary
- Thoroughly reviewed
- Well-documented
- Covered by tests

Thank you for helping keep tuxtui and its users safe!
