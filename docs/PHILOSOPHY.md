# The batless Philosophy

## Core Principle: Never Block, Always Stream

batless was born from a simple frustration: tools that hang in automation. When you're piping output in CI/CD, running scripts, or feeding code to AI assistants, the last thing you need is an interactive pager waiting for input that will never come.

## What batless IS

1. **A Streaming Syntax Viewer**: We process files as streams, never loading entire files into memory
2. **Automation-First**: Every decision prioritizes non-interactive use cases
3. **Predictable**: Same behavior whether in terminal, pipe, or subprocess
4. **Fast**: Startup time matters when you're calling it thousands of times

## What batless IS NOT

1. **Not a Pager**: We don't page, period. Use `less` if you want paging
2. **Not an Editor**: We display code, we don't modify it
3. **Not grep**: Pattern searching is grep's job, and it does it perfectly
4. **Not find**: File discovery is the shell's responsibility

## Design Decisions Explained

### Why No Line Numbers by Default?

Line numbers are decoration. Our default output should be clean and pipeable. We added `-n` for cat compatibility, but it's not our default because most automation doesn't need it.

### Why No Pattern Search?

Unix philosophy: do one thing well. grep exists, is incredibly optimized, and handles pattern matching better than we ever could. Instead of reimplementing grep poorly, we focus on what we do best: syntax highlighting.

### Why Streaming Architecture?

Memory is finite. Files are not. A truly robust tool must handle any size input without consuming unbounded memory. Streaming is the only way to guarantee this.

### Why JSON Output?

Modern automation isn't just bash scripts. AI assistants, observability platforms, and analysis tools need structured data. JSON is the universal language of automation.

## The Future

batless will grow, but always within these constraints:

- Performance over features
- Automation over interaction
- Composition over monoliths
- Streams over loads

If a feature would make batless block, wait, or consume unbounded resources, it doesn't belong here.

## Contributing

When contributing, ask yourself:

1. Does this feature maintain our streaming guarantee?
2. Does it work identically in pipe and terminal?
3. Does it complete in bounded time/memory?
4. Could this be better done by composing with another tool?

If you answered "no" to any of these, the feature probably belongs in a different tool.

---

*batless: Built for machines, loved by humans who automate.*
