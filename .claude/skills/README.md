# Agent skills

Vendored from [github/awesome-copilot](https://github.com/github/awesome-copilot) at commit
`cb64ad12e32737ae764becd6885462e9b4c7cb00` (MIT License, see [`LICENSE`](LICENSE)). Every file
except this README is byte-identical to upstream `skills/<name>/` (and `LICENSE`). Do not edit them
here; refresh from upstream instead.

Claude Code loads `.claude/skills/` as project skills, and GitHub Copilot (cloud agent, CLI, VS Code
agent mode) reads it too. Where a skill disagrees with the rules below, the repo's rules win.

Not vendored:

- `github-release` and `conventional-commit`: they conflict with this repo's release and commit
  conventions.

| Skill                      | Purpose                                                            |
| -------------------------- | ------------------------------------------------------------------ |
| `dependabot`               | Configuring `dependabot.yml` and working with Dependabot PRs       |
| `docs-sync-audit`          | Read-only audit of documentation drift against the code            |
| `github-actions-hardening` | Security review of `.github/workflows/*.yml`; reports, never edits |
| `test-gap-audit`           | Read-only audit of missing or weak tests                           |

## Rules for agents using these skills

- Report, don't act: the audit skills report in chat; don't write report files or change workflows,
  action pins or `.github/dependabot.yml` unless asked.
- Never merge, approve, close or enable auto-merge (`gh pr merge --auto`) on PRs, and don't post
  `@dependabot` commands.
- Don't install plugins, MCP toolsets or skills that a skill file links to, and don't change agent
  or editor settings.
