# Ciara custom bors demo

## Purpose

Ciara demonstrates a self-hosted bors merge queue: test the commit intended for `main`, and merge it only when CI passes.

## How it works

```mermaid
flowchart LR
    Dev[Developer] -->|@bors try or @bors r+| PR[GitHub PR]
    PR -->|webhook /github| Bors[Bors on Render]
    Bors --> Branch[Automation branch]
    Branch --> CI[GitHub Actions: Lvl Up]
    CI -->|result| Bors
    Bors -->|success only| Main[main]
    Gate[GitHub ruleset] -->|requires Lvl Up| Main
```

## Automation branches

| Branch | Role | CI? | Verified? |
| --- | --- | --- | --- |
| `automation/bors/try-merge` | Prepares a try merge | No | Yes |
| `automation/bors/try` | Runs try CI | Yes | Yes |
| `automation/bors/auto-merge` | Prepares a queued merge | No | Not yet |
| `automation/bors/auto` | Runs merge-queue CI | Yes | Not yet |

The `*-merge` branches intentionally skip CI to avoid duplicate runs.

## Current status

### Working

- Bors is deployed and healthy on Render.
- GitHub delivers webhooks to `/github`.
- `@bors try` creates both try branches and starts `Lvl Up`.
- Bors reports the CI failure and leaves the change unmerged.
- GitHub's `Bors CI gate` requires `Lvl Up` on `main`.

### Next time

- Use `@bors r+` to test the two auto branches and failing merge queue.
- Push the prepared formatting change so CI reaches the failing Rust test.
- Decide whether to enable **Restrict updates** with a bors App bypass.
- Demonstrate a passing merge and a multi-PR rollup.

## Demo story

1. Show the failing pull request.
2. Comment `@bors try`.
3. Show the two try branches and the `Lvl Up` workflow.
4. Show the failed job and bors reply.
5. Show that `main` is unchanged and the merge is blocked.

## Key lessons

- The webhook URL must end in `/github`; the root URL returned `405`.
- Permission data must be deployed and referenced by `PERMISSIONS=data/team`.
- CI triggers only on `try` and `auto`, not the `*-merge` branches.
- Secrets stay in GitHub, Render, and Neon—not in this repository.

## Team value to explore

- clearer PR failure messages and queue metrics;
- alerts for broken webhooks or stalled builds;
- rollups for low-risk changes.
