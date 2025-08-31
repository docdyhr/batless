# Feature Development Roadmap

## Overview

Based on the current state of batless v0.1.3 and market positioning, here's a prioritized feature development roadmap.

## Priority Matrix

Features are ranked by Impact (user value) vs Effort (development time):

### 🚀 High Impact, Low Effort (Do First)

1. **Shell Completions** (1-2 days)
   - Add bash, zsh, fish completions using clap
   - Improves UX significantly with minimal effort
   - Implementation: Use clap_complete crate

2. **Configuration File Support** (2-3 days)
   - Add ~/.config/batless/config.toml support
   - Allow default themes, modes, limits
   - Use existing serde infrastructure

3. **Performance Marketing Update** (0.5 days)
   - Update README with actual <5ms performance
   - Add PERFORMANCE_REPORT.md to repo
   - Create comparison benchmarks vs bat

4. **Better Error Messages** (1 day)
   - Add helpful suggestions for common errors
   - Improve file not found messages
   - Add --help hints on invalid options

### 💎 High Impact, High Effort (Strategic)

5. **Tree-sitter Integration** (1-2 weeks)
   - More accurate parsing for summary mode
   - Better language detection
   - Improved token extraction
   - Consider as v0.2.0 feature

6. **Plugin System** (2-3 weeks)
   - Allow custom language definitions
   - Support user-defined summary extractors
   - WebAssembly plugin support
   - Major architectural change

7. **VS Code Extension** (1 week)
   - Quick file preview in VS Code
   - Integration with AI assistants
   - Syntax highlighting preview

### 🔧 Low Impact, Low Effort (Quick Wins)

8. **Additional Output Formats** (1 day each)
   - CSV output for data files
   - XML output for structured data
   - YAML output mode

9. **Stdin Support Enhancement** (1 day)
   - Better pipe detection
   - Language hints via CLI args
   - Streaming stdin processing

10. **More Package Managers** (1 day each)
    - Arch AUR package
    - Nix package
    - Scoop (Windows)
    - Snapcraft

### 📚 Low Impact, High Effort (Reconsider)

11. **Git Integration** (1 week)
    - Goes against "no blocking" philosophy
    - Users can use bat for this
    - Not aligned with core value prop

12. **Language Server Protocol** (2+ weeks)
    - Complex implementation
    - Limited use case overlap
    - Better suited for dedicated tools

## Recommended Development Order

### Phase 1: Quick Wins (v0.1.4)

- [ ] Shell completions
- [ ] Configuration file support
- [ ] Performance marketing update
- [ ] Better error messages

### Phase 2: Format Extensions (v0.1.5)

- [ ] CSV output mode
- [ ] XML output mode
- [ ] Enhanced stdin support

### Phase 3: Ecosystem (v0.1.6)

- [ ] Package manager submissions
- [ ] VS Code extension
- [ ] Community outreach

### Phase 4: Major Features (v0.2.0)

- [ ] Tree-sitter integration
- [ ] Plugin system architecture
- [ ] Breaking changes if needed

## Technical Debt to Address

1. **Security**: yaml-rust unmaintained warning
   - Wait for syntect to update or fork
   - Not critical but should monitor

2. **Testing**: Add more edge cases
   - Binary file handling
   - Unicode edge cases
   - Very large line handling

3. **Documentation**:
   - API documentation
   - Integration examples
   - Video demo

## Success Metrics

Track adoption through:

- GitHub stars growth rate
- Crates.io downloads
- GitHub issues/PRs from community
- Mentions in AI/automation contexts

## Community Building

1. Submit to awesome-rust
2. Write blog post about AI-friendly design
3. Create comparison video vs bat
4. Engage with AI tool developers
5. Conference talk proposals

## Not Doing (By Design)

- Interactive features (paging, etc.)
- Git integration
- Line numbers in output
- Terminal UI enhancements
- Anything that could block

These omissions are features, not bugs. They reinforce the core value proposition of a non-blocking, automation-first tool.
