# dave-agent-v2

The DAVE agent: hygiene checks for Linux servers. This is a ground-up rebuild of
the prototype (`dave-agent`), with a new privilege model, reviewed pull requests
from the start, and a record of how each part was built.

**Status:** early development. Not for use yet.

## How changes get in

- After the bootstrap commit, every change is a pull request, squash-merged.
- Each pull request says what changed, whether AI wrote it, what was checked
  and how it's tested (see `.github/pull_request_template.md`).
- Significant decisions are recorded in `docs/decisions/` before the code.
- Pushes to `main` are refused locally by `.githooks/pre-push`, and CI flags any
  commit on `main` that didn't come from a merged pull request.

## After cloning

```sh
git config core.hooksPath .githooks
```

## Checks run on every pull request

| Check | Tool |
| --- | --- |
| Formatting | rustfmt |
| Lints, including no `unwrap`, `expect`, `panic` or unchecked indexing outside tests | Clippy |
| Tests | cargo test |
| Dependency advisories, licences and sources | cargo-deny |
| Risky code patterns | Semgrep |
| Secrets anywhere in history | gitleaks |
| Workflow security | zizmor |

## Licence

All rights reserved. This repository does not accept outside contributions.
