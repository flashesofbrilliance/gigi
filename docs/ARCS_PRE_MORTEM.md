# ARCS v8.5 — LAB/COURT/KKL Pre-Mortem & Disaster Recovery Plan

> **Simulation date:** 2026-05-15  
> **Horizon:** This quarter (deployment) + 1–3 years (substrate evolution)  
> **Stakes:** HIGH — personal OS substrate; failure modes include silent data loss, cognitive dependency lock-in, unrecoverable schema drift  
> **Mode:** DEEP (high-stakes, explicit request)

---

## Executive Summary

Six failure modes dominate the risk surface. Three are deployment-time (schema drift on rollback, secrets leakage in Actions, edge-function assumption mismatch). Three are architectural (gap_signal_type coupling fragility, NOTOMATION boundary erosion, single-author bus risk). The disaster recovery plan addresses each with a dedicated runbook, a rollback ladder, and a GitHub Actions workflow that makes safe deployment the path of least resistance.

---

## LAB Macro — Likelihood / Adversary / Bayes

The LAB macro stress-tests assumptions by asking: *What would have to be true for this to fail badly, and how likely is that world?*

### Failure Mode Register

| # | Failure Mode | Likelihood (1–5) | Impact (1–5) | Risk Score | LAB Verdict |
|---|---|---|---|---|---|
| FM-1 | Schema drift: `gap_signal_type` enum adds a value, old consumers silently ignore it | 4 | 4 | **16** | Inevitable without contract tests |
| FM-2 | Rollback produces ghost rows — new columns not nullable, old migrations don't clean up | 4 | 5 | **20** | Most likely deployment-hell entry point |
| FM-3 | GitHub Actions secrets leak via log echo or third-party action version pin drift | 3 | 5 | **15** | Low-probability, catastrophic if hit |
| FM-4 | Edge-function assumption: personal substrate runs on shared infra that rate-limits or cold-starts | 4 | 3 | **12** | Architectural bet that may not hold |
| FM-5 | NOTOMATION boundary erosion: tooling starts substituting decisions, not scaffolding them | 3 | 4 | **12** | Slow-burn, hardest to detect |
| FM-6 | Single-author bus risk: no one else can read the Field OS card and operate the system | 2 | 5 | **10** | Latent; becomes critical at handoff |

---

## COURT Macro — Challenge, Opposite, Understand, Reframe, Test

The COURT macro runs each high-risk assumption through an adversarial challenger.

### FM-2 COURT (Highest Risk Score: 20)

| Step | Content |
|---|---|
| **Challenge** | "Rollbacks always work" — assumes migrations are reversible and schema is additive-only |
| **Opposite** | What if the rollback itself is the failure? (Column removed, data truncated, FK violated) |
| **Understand** | Most rollback pain comes from non-nullable column additions without defaults and from enum changes that aren't backward-compatible |
| **Reframe** | Treat every migration as a two-phase deploy: *expand* (add nullable cols, new enum values) → ship → *contract* (enforce not-null, drop old cols) |
| **Test** | Add a migration linter to the CI pipeline; block merges where `ALTER TABLE` removes a column or changes an enum without a corresponding expand migration |

### FM-1 COURT (Schema Drift: 16)

| Step | Content |
|---|---|
| **Challenge** | "Consumers handle unknown enum values gracefully" |
| **Opposite** | TypeScript `switch` without exhaustive check silently falls to default; data is recorded wrong, not errored |
| **Reframe** | `gap_signal_type` is a discriminated union contract, not a free string. Treat it like a protobuf enum — any addition requires a version bump and a migration of consumers |
| **Test** | Add `@typescript-eslint/no-unsafe-enum-comparison` + exhaustiveness lint rule; add a contract test that fails if a new enum value exists without a corresponding consumer handler |

### FM-3 COURT (Secrets Leakage: 15)

| Step | Content |
|---|---|
| **Challenge** | "GitHub Actions is secure by default" |
| **Opposite** | Third-party actions pinned to `@v3` (tag, not SHA) can be silently updated to exfiltrate secrets |
| **Reframe** | Pin every third-party action to a full commit SHA. Use `permissions: contents: read` minimal grants. Never echo env vars in run steps |
| **Test** | Add `actionlint` + `zizmor` secret scanning to CI; block any action reference that isn't SHA-pinned |

---

## KKL Macro — Known / Known-Unknown / Latent

Maps the epistemic landscape so the recovery plan targets the right unknowns.

| Category | Items |
|---|---|
| **Known (handled)** | gap_signal_type schema exists; Field OS card drafted; README insertion block ready |
| **Known-Unknown (plan exists, answer TBD)** | Actual infra substrate (edge vs. server vs. local); rate limit behavior under load; whether `incoherence_flag` query patterns will stay fast at scale |
| **Latent (unknown-unknown, requires probe)** | How NOTOMATION boundary degrades over time under real usage; whether the eval UI stub's signal taxonomy matches what users actually report; third-party dependency drift over 12 months |

**KKL Action:** Add a quarterly "Latent Risk Probe" task to the backlog. The probe is a 30-minute structured reflection: *What assumption have we not questioned in 90 days?*

---

## Disaster Recovery Plan

### Rollback Ladder

Four rungs, each with a clear trigger and a concrete action. The goal: no deployment decision requires more than 30 seconds of thought under pressure.

```
RUNG 4 — Feature Flag Off (< 1 min)
  Trigger: New gap_signal_type consumer misbehaves in prod
  Action:  Set FEATURE_GAP_V2=false env var; consumers fall back to legacy handler
  Risk:    None — schema already expanded, old path still valid

RUNG 3 — Revert Deploy (< 5 min)
  Trigger: UI regression, perf degradation, non-schema bug
  Action:  `git revert HEAD --no-edit && git push` → CI auto-deploys previous image
  Risk:    Low if expand/contract migrations are followed

RUNG 2 — Schema Contract Migration (< 30 min)
  Trigger: Rung 3 blocked by non-nullable column or broken FK
  Action:  Run `pnpm migrate:rollback` → applies reverse migration → redeploy
  Risk:    Medium — requires prior reverse migration to exist (enforced by CI check)

RUNG 1 — Point-in-Time Restore (< 2 hr)
  Trigger: Data corruption, ghost rows, FK violations at scale
  Action:  Restore from last clean snapshot; replay events forward if event log exists
  Risk:    High — data loss window equals snapshot interval; keep interval ≤ 4 hr
```

### Architectural Assumption Guardrails

**Assumption: personal substrate is long-lived and stable**
- Guardrail: Store `substrate_version` in every gap log entry. If substrate changes, queries can filter by version cohort. No silent cross-version data blending.

**Assumption: gap_signal_type enum is stable**
- Guardrail: Schema versioning via `signal_schema_version` integer field. Consumers read this field first and reject entries from a version they don't understand (fail-loud, not fail-silent).

**Assumption: NOTOMATION boundary holds under automation pressure**
- Guardrail: Monthly NOTOMATION audit — any workflow that auto-writes to the gap log without a human confirmation step gets flagged for review.

---

## GitHub Actions Workflows

### 1. `ci.yml` — Lint, Test, Migration Safety

```yaml
name: CI

on:
  push:
    branches: [main, 'feature/**']
  pull_request:
    branches: [main]

permissions:
  contents: read
  pull-requests: write

jobs:
  lint-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11  # v4.1.1 SHA-pinned

      - uses: actions/setup-node@60edb5dd545a775178f52524783378180af0d1f8  # v4.0.2
        with:
          node-version: '20'
          cache: 'pnpm'

      - name: Install dependencies
        run: pnpm install --frozen-lockfile

      - name: Lint
        run: pnpm lint

      - name: Type check
        run: pnpm tsc --noEmit

      - name: Exhaustive enum check
        run: pnpm lint:enums
        # Runs eslint rule: @typescript-eslint/switch-exhaustiveness-check
        # Fails if gap_signal_type switch has unhandled cases

      - name: Unit tests
        run: pnpm test

      - name: Migration safety lint
        run: pnpm migrate:lint
        # Custom script: fails if any migration removes a column or changes
        # an enum value without a preceding expand migration in the same PR

  secret-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11
      - name: Run zizmor (Actions security lint)
        run: |
          pip install zizmor --quiet
          zizmor .github/workflows/
      - name: Run actionlint
        uses: reviewdog/action-actionlint@4f6ef80f5c7c1a69e60b66c0d87af52a1f5c614e  # v1.43.0
```

### 2. `deploy.yml` — Safe Deploy with Rollback Gate

```yaml
name: Deploy

on:
  push:
    branches: [main]

permissions:
  contents: read
  deployments: write
  id-token: write   # OIDC — no long-lived secrets in env

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment: production
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11

      - uses: actions/setup-node@60edb5dd545a775178f52524783378180af0d1f8
        with:
          node-version: '20'
          cache: 'pnpm'

      - name: Install
        run: pnpm install --frozen-lockfile

      - name: Build
        run: pnpm build
        env:
          NODE_ENV: production
          # All secrets injected via OIDC/environment — never echoed

      - name: Run migrations (expand phase only)
        run: pnpm migrate:expand
        # Expand-phase migrations only: add nullable columns, add enum values
        # Contract-phase migrations are a separate, manual step post-validation

      - name: Health check pre-deploy
        run: pnpm health:check --env=production --timeout=30

      - name: Deploy
        run: pnpm deploy:production
        # Deployment command — swap for your actual deploy target

      - name: Health check post-deploy
        id: post_health
        run: pnpm health:check --env=production --timeout=60
        continue-on-error: true

      - name: Auto-rollback on health failure
        if: steps.post_health.outcome == 'failure'
        run: |
          echo "Post-deploy health check failed. Initiating Rung 3 rollback."
          pnpm deploy:rollback
          exit 1

      - name: Snapshot substrate state
        if: success()
        run: pnpm substrate:snapshot --label="post-deploy-$(date +%Y%m%d%H%M)"
        # Creates a labeled point-in-time snapshot for Rung 1 restore
```

### 3. `rollback.yml` — Manual Rollback Dispatch (Rung 1–3)

```yaml
name: Manual Rollback

on:
  workflow_dispatch:
    inputs:
      rung:
        description: 'Rollback rung (2=schema, 3=revert-deploy, 4=feature-flag)'
        required: true
        type: choice
        options: ['4', '3', '2']
      target_sha:
        description: 'Target commit SHA for Rung 3 (leave blank for Rung 4)'
        required: false

permissions:
  contents: write
  deployments: write

jobs:
  rollback:
    runs-on: ubuntu-latest
    environment: production
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11
        with:
          fetch-depth: 0

      - name: Rung 4 — Disable feature flag
        if: inputs.rung == '4'
        run: pnpm feature:disable GAP_SIGNAL_V2

      - name: Rung 3 — Revert deploy
        if: inputs.rung == '3'
        run: |
          SHA="${{ inputs.target_sha }}"
          if [ -z "$SHA" ]; then SHA=$(git rev-parse HEAD~1); fi
          git revert --no-edit "$SHA"
          git push
          pnpm deploy:production

      - name: Rung 2 — Schema rollback
        if: inputs.rung == '2'
        run: |
          pnpm migrate:rollback
          pnpm deploy:production

      - name: Post-rollback health check
        run: pnpm health:check --env=production --timeout=60

      - name: Create rollback decision receipt
        if: always()
        run: |
          echo "## Rollback Decision Receipt" >> $GITHUB_STEP_SUMMARY
          echo "- **Rung:** ${{ inputs.rung }}" >> $GITHUB_STEP_SUMMARY
          echo "- **Triggered by:** ${{ github.actor }}" >> $GITHUB_STEP_SUMMARY
          echo "- **Timestamp:** $(date -u)" >> $GITHUB_STEP_SUMMARY
          echo "- **Target SHA:** ${{ inputs.target_sha || 'auto' }}" >> $GITHUB_STEP_SUMMARY
```

### 4. `latent-risk-probe.yml` — Quarterly Audit Reminder

```yaml
name: Quarterly Latent Risk Probe

on:
  schedule:
    - cron: '0 9 1 */3 *'   # 9am on the 1st of every 3rd month
  workflow_dispatch:

permissions:
  issues: write

jobs:
  create-probe-issue:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/github-script@60a0d83039c74a4aee543508d2ffcb1c3799cdea  # v7.0.1
        with:
          script: |
            await github.rest.issues.create({
              owner: context.repo.owner,
              repo: context.repo.repo,
              title: `[ARCS] Quarterly Latent Risk Probe — ${new Date().toISOString().slice(0,7)}`,
              body: `## Latent Risk Probe Checklist\n\n` +
                `- [ ] What architectural assumption has not been questioned in 90 days?\n` +
                `- [ ] Has the NOTOMATION boundary shifted? Any automated writes to gap_log without human confirmation?\n` +
                `- [ ] Has gap_signal_type grown in ways that blur the original taxonomy?\n` +
                `- [ ] Are all third-party Action SHA pins still current?\n` +
                `- [ ] Is the substrate_version field populated in all recent gap log entries?\n` +
                `- [ ] Is the Rung 1 snapshot interval still ≤ 4 hours?\n\n` +
                `**Output:** Update \`ARCS_LOCAL_BACKLOG.md\` with findings. Close issue when probe is complete.`,
              labels: ['arcs', 'latent-risk', 'quarterly']
            });
```

---

## Decision Receipt

| Field | Value |
|---|---|
| **Question** | Should we run a pre-mortem and DRP before deployment? |
| **Horizon** | This quarter (deploy) + 1–3 years (substrate) |
| **Stakes** | HIGH |
| **Mode** | DEEP |
| **Macros run** | LAB (likelihood/impact matrix), COURT (adversarial challenge on FM-1, FM-2, FM-3), KKL (epistemic map) |
| **True North** | Yes — pre-mortem before commit is the highest-leverage moment |
| **Top risk** | FM-2 (rollback + non-nullable migration): Risk Score 20 → mitigated by expand/contract discipline + migration linter in CI |
| **Key architectural bet to watch** | gap_signal_type as stable discriminated union — requires signal_schema_version field and exhaustive consumer checks |
| **Drift check** | NOTOMATION boundary: add monthly audit step; automation must not write to gap_log without human gate |

