# Agent skills

Installed from [github/awesome-copilot](https://github.com/github/awesome-copilot) at commit `cb64ad12e32737ae764becd6885462e9b4c7cb00`.
Lives in `.claude/skills/`, which Claude Code loads as project skills and GitHub Copilot (cloud agent, CLI, VS Code agent mode) also reads.
`github-release` and `conventional-commit` were deliberately left out: they conflict with this repo's release and commit conventions.

| Skill | Purpose |
|---|---|
| `github-actions-hardening` | Security hardening reviewer for GitHub Actions workflow files (.github/workflows/*.yml). Reasons about the Act… |
| `dependabot` | Comprehensive guide for configuring and managing GitHub Dependabot. Use this skill when users ask about … |
| `test-gap-audit` | Run a read-only audit for missing, weak, stale, or mis-scoped test coverage. If the user does not name a scope… |
| `docs-sync-audit` | Run a read-only documentation drift audit for a feature, PR, branch, release, API, configuration change, workf… |
