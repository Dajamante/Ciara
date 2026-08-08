# bors setup — full action plan (weekend toy/demo run)

Note: you mentioned you already have a repo (`bors-aira` or similar) with permissions set up — treat steps 2 and 5 below as likely already done; just confirm and skip if so.

## 1. Neon (database)
- [x] Go to neon.tech, sign up (personal account)
- [x] Create a new project
- [x] Copy the connection string (`postgres://user:pass@ep-xxxx.neon.tech/dbname`) — this is your `DATABASE_URL`
${DATABASE_URL}

## 2. Fork the repo
- [x] Fork github.com/rust-lang/bors into your personal GitHub account
- [x] *(You may already have this — the repo with permissions set up)*

## 3. Render (bot hosting)
- [x] Go to render.com, sign up, connect GitHub
- [x] Dashboard → New → Web Service → select your forked `bors` repo
- [x] Confirm Render detects the Dockerfile, selects "Docker"
- [x] Choose the **Free** instance type, pick a region
- [ ] Advanced settings:
  - [x] Port → `80`
  - [x] Health Check Path → `/health`
  - [x] Env var `DATABASE_URL` = Neon connection string
  - [x] Placeholder env vars: `APP_ID`, `PRIVATE_KEY`, `WEBHOOK_SECRET`
- [x] Create Web Service, wait for build, check logs for a clean deploy
- [x] Note your URL: `https://your-service-name.onrender.com`
https://bors-airas-lvlup.onrender.com

## 4. Test repo
- [x] Create a throwaway personal repo, e.g. `bors-demo`. We have called it Ciara.
github.com/Dajamante/Ciara

## 5. GitHub App
- [x] Settings → Developer settings → GitHub Apps → New GitHub App
- [x] Name it (e.g. `<yourname>-bors`)
- [x] Webhook URL: `https://bors-airas-lvlup.onrender.com/github`
- [x] Generate a webhook secret — save it
${WEBHOOK_SECRET}
- [x] Permissions (Read & Write): Actions, Checks, Contents, Issues, Pull requests
- [x] Subscribe to events: Issue comment, Push, Pull request, Pull request review, Pull request review comment, Workflow job, Workflow run
- [x] Create the app, copy the **App ID**
App ID: ${APP_ID}
Client ID: ${GITHUB_CLIENT_ID}
https://github.com/apps/bors-rs-ciaralvlup

- [x] Generate a **private key** (.pem) — save it
- [x] Install the app → select the repository or repositories that bors should manage
- [x] Update Render env vars with real values: `APP_ID`, `PRIVATE_KEY`, `WEBHOOK_SECRET`
- [x] Trigger a redeploy on Render
- [x] *(You may already have this done — confirm the app + permissions are correctly pointed at your test repo)*

## 6. Wire up the test repo
- [x] Add `rust-bors.toml` to `bors-demo` repo root (default branch):
  ```toml
  timeout = 3600
  merge_queue_enabled = true
  report_merge_conflicts = true

  [labels]
  approved = ["+approved"]
  unapproved = ["-approved"]
  ```
- [x] Add `.github/workflows/ci.yml` triggering on push to:
  - `automation/bors/try`
  - `automation/bors/auto`
  (a basic "echo hello, exit 0" job is enough)
- [x] Do **not** add CI triggers on `automation/bors/try-merge` or `automation/bors/auto-merge`
- [ ] Confirm the bot has push access to all four branches: `try`, `try-merge`, `auto`, `auto-merge` — configuration checked; waiting for the first successful bors command
- [ ] Enforce bors-only updates to `main` after confirming the bot can push:
  - [ ] Ciara → Settings → Rules → Rulesets
  - [ ] Create an active branch ruleset targeting `main`
  - [ ] Enable **Restrict updates**
  - [ ] Add your bors GitHub App to the bypass list with **Always allow**

## 7. Permission files
- [x] Check `src/permissions.rs` for how bors loads permissions
- [x] Add `data/team/bors.try.json` with the authorized user's numeric GitHub ID
- [x] Add `data/team/bors.review.json` with the authorized user's numeric GitHub ID
- [x] Add the required `data/team/people.json` and `data/team/teams.json` files
- [x] Include `data/team` in the bors fork's Render deployment
- [x] Set Render's `PERMISSIONS` environment variable to `data/team`

GitHub App permissions control what bors can do. These files control who is allowed to command bors.

## 8. Smoke test
- [ ] Open a throwaway PR on `bors-demo`
- [ ] Comment `@bors try` — confirm a workflow run starts on `automation/bors/try`, bot replies with status
- [ ] Comment `@bors r+` — confirm approval, merge queue picks it up (~30s), CI runs on `automation/bors/auto`, merges on success
- [ ] Open 2 more trivial PRs, mark both `r+ rollup=always` — confirm they batch together instead of running separately

## (Optional, skip for barebones demo) — Rollup web UI
- [ ] OAuth App: Settings → Developer settings → OAuth Apps → New OAuth App
- [ ] Callback URL: `https://your-service-name.onrender.com/oauth/callback`
- [ ] Copy Client ID + Client Secret → add to Render as `OAUTH_CLIENT_ID` / `OAUTH_CLIENT_SECRET`
- [ ] Add `WEB_URL` env var = your Render URL, redeploy
- [ ] Log into `https://your-service-name.onrender.com` via GitHub OAuth, confirm rollup UI loads

## Where is the information?
- **Webhook deliveries:** GitHub → Settings → Developer settings → GitHub Apps → `bors-RS-CiaraLvlUp` → Advanced → Recent Deliveries
- **Bors queue:** `https://bors-airas-lvlup.onrender.com/queue/ciara`
- **CI results:** Ciara repository → Actions → `Ciara — Level Up`
- **Bors application logs:** Render → `bors-airas-LvlUp` → Logs
- **Database:** Neon project → Tables, SQL Editor, or Monitoring
  - You normally do not need to look inside Neon; bors owns its tables
  - Read-only inspection is useful when diagnosing connections or advisory locks
  - Do not manually edit or delete bors records unless you understand the database schema
  - Keep `DATABASE_URL` private

## Where are permissions configured?
- **GitHub App capabilities:** GitHub → Settings → Developer settings → GitHub Apps → `bors-RS-CiaraLvlUp` → Permissions & events
  - Repository permissions set to **Read and write**: Actions, Checks, Contents, Issues, Pull requests
  - Optional: Self-hosted runners, only when bors creates EC2 CI runners
  - Events: Issue comment, Push, Pull request, Pull request review, Pull request review comment, Workflow job, Workflow run
  - Save changes and approve the updated permissions on every existing installation
- **People allowed to command bors:** custom bors fork → `data/team`
  - `bors.try.json`: users allowed to run `@bors try`
  - `bors.review.json`: users allowed to run commands such as `@bors r+`
  - Render uses these files through `PERMISSIONS=data/team`
  - This local-directory setup currently applies the same user lists to every managed repository

## If something breaks
- Render → Events/Logs tab first
- GitHub App → Advanced tab → Recent Deliveries (green = webhook delivered, red = failed)
- Free Render services sleep after 15 min idle — hit the URL once yourself before testing/demoing
- If deliveries return `200` but bors does not reply, check Render logs for `other concurrent bors instance` or advisory-lock warnings
  - Confirm no other bors service uses the same Neon database
  - Render → Events/Deploys → Manual Deploy → **Restart service**
  - Wait until the service is live, then post a fresh `@bors ping` or `@bors try`
