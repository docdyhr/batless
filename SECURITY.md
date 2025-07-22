# Security Policy

## Supported Versions

We actively support the following versions of batless with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue in batless, please follow these steps:

### Reporting Process

1. **Do NOT create a public issue** for security vulnerabilities
2. Email security concerns to: [security@batless.dev] (or create a private security advisory on GitHub)
3. Include the following information:
   - Description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact
   - Any suggested fixes (if available)

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
- **Initial Assessment**: We will provide an initial assessment within 5 business days
- **Updates**: We will keep you informed of our progress
- **Resolution**: We aim to resolve critical vulnerabilities within 30 days

### Scope

This security policy covers:

- The batless CLI application
- The batless library (crate)
- Build and release processes
- Dependencies and supply chain

### Security Considerations

#### File Processing
- batless processes files locally and does not send data to external services
- File content is streamed and not stored permanently
- No network connections are made during normal operation

#### Dependencies
- We regularly audit dependencies for known vulnerabilities
- Dependencies are kept up to date with security patches
- We use `cargo audit` in our CI/CD pipeline

#### Input Validation
- File paths are validated to prevent directory traversal
- Input limits are enforced to prevent resource exhaustion
- Malformed files are handled gracefully without crashes

### Known Security Considerations

1. **Large Files**: While batless uses streaming, extremely large files could still impact system resources
2. **Malformed Files**: Syntect (our syntax highlighting library) is generally robust, but malformed files might cause parsing issues
3. **File Permissions**: batless respects system file permissions and will not attempt to read files without proper access

### Security Best Practices for Users

1. **Verify Downloads**: Always download batless from official sources
2. **Check Signatures**: Verify release signatures when available
3. **Update Regularly**: Keep batless updated to the latest version
4. **Limit File Access**: Run batless with minimal necessary file permissions
5. **Validate Output**: Be cautious when processing output in automated systems

### Disclosure Timeline

Once a security vulnerability is reported and confirmed:

1. **Day 0**: Vulnerability reported
2. **Day 1-2**: Acknowledgment and initial triage
3. **Day 3-7**: Detailed analysis and fix development
4. **Day 8-14**: Testing and validation of fix
5. **Day 15-30**: Release of patched version
6. **Day 30+**: Public disclosure (coordinated with reporter)

### Security Hall of Fame

We recognize security researchers who help improve batless security:

- [Your name could be here!]

### Contact Information

- **Security Email**: [INSERT EMAIL]
- **PGP Key**: [INSERT PGP KEY ID if available]
- **GitHub Security Advisories**: Use GitHub's private vulnerability reporting feature

### Legal

This security policy is provided in good faith. We reserve the right to modify this policy at any time. Security researchers acting in good faith under this policy will not face legal action from the batless project.

---

Thank you for helping keep batless secure! 🔒