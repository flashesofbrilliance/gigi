# ARCS-LOCAL Build Backlog

> **Mission:** Transform GIGI into a privacy-first local substrate for ARCS decision intelligence. Raw LLM is the floor, not the ceiling. The whale builds its own baleen plates.
>
> **Deeper mission:** Build a self-clarification instrument, not a decision assistant. Each validated receipt should increase the resolving power of judgment by making previously invisible dimensions of the user's specific decision geometry visible, queryable, and improvable. The system does not merely help route decisions; it progressively externalizes the user's frame of reference so the user can observe, refine, and eventually embody their own apparatus.

---

## North Star

**Primary JTBD:**

> So that each decision I make increases the resolving power of my own judgment — not by making me smarter in the abstract, but by making previously invisible dimensions of my specific decision geometry visible, queryable, and improvable.

**Reference-of-frame principle:**

Classical tools help a subject observe an object. ARCS-LOCAL inverts that frame: it externalizes the user's own coordinate system so the subject can become an objective subject — able to inspect the geometry of their own seeing, not just the thing seen.

**Implication for the build:**

- Receipts are not just logs; they are crystallized slices of frame-of-reference.
- Templates are not just automations; they are reusable facets of judgment.
- Gap logs are not just misses; they are shadow zones in the current coordinate system.
- ALC synthesis is not just interpolation; it is controlled facet-cutting along valid geometric planes.
- Privacy is not just data protection; it protects the integrity of the user's frame from external calibration.

---

## Architecture Overview

```
L0 Raw signal
  → L1 GIGI exact/holonomy match         (~80% at maturity)
  → L2 ALC + VCE structural synthesis    (variant gaps)
  → L3 Human authorship                  (truly novel shapes)
  → L4 Controlled LLM (structure only)   (novel macro families)
  → L5 Raw LLM (floor, not ceiling)      (what nothing else caught)
```

**Privacy model:** Local-first. iCloud ADP as encrypted backup. State actor = named edge case, accepted.

**Hardware target:** M1 MacBook Pro 16GB. GIGI Rust binary = CPU-only, ~0W. LLM compute = graceful cloud fallback with explicit user approval only.

---

## Design Intent

### What this is
- A progressive crystallographic map of a user's decision geometry.
- A living starter culture of validated priors, fed by daily use.
- A local-first filtration and fermentation substrate that makes the user's own frame increasingly legible.

### What this is not
- Not a generic decision support chatbot.
- Not a static template library.
- Not a convenience automation layer.
- Not a cloud intelligence wrapper with local caching.

### Success condition
The end state is not merely that most sessions never touch the network. The end state is that the substrate becomes structurally part of the user's decision apparatus — transparent enough that the user stops noticing it as a separate tool.

---

## Phase 0 — Machine Hardening (Do First, No Code)

### P0-1: Enable Advanced Data Protection
- [ ] System Settings → Apple ID → iCloud → Advanced Data Protection → **Enable**
- [ ] Verify: end-to-end encryption active, Apple holds no key
- [ ] **Acceptance:** ADP status shows enabled

### P0-2: Enable FileVault
- [ ] System Settings → Privacy & Security → FileVault → **Enable**
- [ ] **Acceptance:** Local disk encrypted at rest

### P0-3: Enable iCloud Keychain
- [ ] System Settings → Apple ID → iCloud → Keychain → **Enable**
- [ ] **Acceptance:** Keychain syncing across devices under ADP

> ⚠️ **Residual threat vector (named + accepted):** State-level legal compulsion or cryptanalysis. Out of scope for current threat model. Upgrade path: migrate to YubiKey 5 hardware key + disable iCloud Keychain sync if threat model escalates.

---

## Phase 1 — Foundation

### P1-1: GIGI Schema for ARCS Templates
**JTBD:** As an ARCS user, I want a GIGI bundle schema that stores ARCS macro templates as fiber-bundle records so that my repeated decisions become reusable facets of judgment, queryable by geometry rather than reconstructed from scratch each time.

**Schema fields:**
```sql
CREATE TABLE arcs_templates (
  -- Identity
  template_id        TEXT PRIMARY KEY,
  macro_family       TEXT,        -- BAR | KKL | LAB | RECEIPT | DRIFT | KINTSUGI
  version_tag        TEXT,        -- v8.5 | v9 | ...
  
  -- Fiber coordinates (routing geometry)
  stakes             TEXT,        -- low | medium | high | ambiguous
  horizon            TEXT,        -- today | this-week | this-quarter | 1-3yr
  mode               TEXT,        -- QUICKMOVE | PLAN | COURT
  domain             TEXT,        -- general | product | financial | relational
  
  -- Holonomy signature (13D path-state float array)
  theta_person       REAL,
  theta_tense        REAL,
  theta_modal        REAL,
  theta_pos          REAL,
  animacy_avg        REAL,
  base_0 REAL, base_1 REAL, base_2 REAL, base_3 REAL,
  base_4 REAL, base_5 REAL, base_6 REAL, base_7 REAL,
  
  -- Template content
  skeleton           TEXT,        -- Mad Libs string with {slot_name} placeholders
  slot_names         TEXT,        -- JSON array of slot names
  example_fill       TEXT,        -- JSON example fill (abstract, never personal)
  
  -- Provenance
  source             TEXT,        -- canonical | alchemist_candidate | human_authored | llm_structural
  status             TEXT,        -- CANDIDATE | CANONICAL | DEPRECATED
  parent_shapes      TEXT,        -- JSON array of parent template_ids (ALC synthesis)
  alc_confidence     REAL,        -- null for canonical, 0-1 for ALC candidates
  holonomy_distance_to_nearest REAL,
  
  -- Lifecycle
  created_at         INTEGER,
  promoted_at        INTEGER,     -- null until CANONICAL
  validated_by       TEXT,        -- 'human' | 'auto'
  use_count          INTEGER DEFAULT 0,
  last_used_at       INTEGER
);

-- Indexes for geometric retrieval
CREATE INDEX idx_macro_family ON arcs_templates(macro_family);
CREATE INDEX idx_stakes_horizon ON arcs_templates(stakes, horizon);
CREATE INDEX idx_status ON arcs_templates(status);
CREATE INDEX idx_holonomy ON arcs_templates(theta_tense, theta_modal, theta_pos);
```

**Acceptance criteria:**
- [ ] Schema created and migration runs cleanly
- [ ] Insert, query-by-holonomy-distance, and promote-to-canonical operations working
- [ ] 30 seed template records inserted (see P1-2)
- [ ] Unit test: query returns correct template family for given fiber coordinates
- [ ] Every stored canonical template can be described as a reusable facet of judgment, not just a formatting skeleton

---

### P1-2: Seed Template Library (30 canonical records)
**JTBD:** As an ARCS user, I want a pre-populated template library covering core macro families so that day-one usage begins with real facets already cut into the substrate, reducing reconstruction overhead and making judgment more immediately legible.

**Seed matrix:**

| Family | Variants | Total |
|--------|----------|-------|
| Decision Receipt | low/med/high stakes × today/this-week | 6 |
| BAR | med/high stakes × today/this-quarter | 6 |
| KKL | med/high stakes × today/this-quarter | 6 |
| LAB | med/high stakes × today/this-quarter | 6 |
| Drift Check | land-plane / stay-exploring / hard-stop | 3 |
| Kintsugi | repair/acknowledge/reframe | 3 |
| **Total** | | **30** |

**Example seed record (BAR, high stakes, today):**
```json
{
  "template_id": "BAR_high_today_v85",
  "macro_family": "BAR",
  "stakes": "high",
  "horizon": "today",
  "mode": "COURT",
  "skeleton": "Bet: {bet} · Against: {counterparty} · At stake: {downside} · Stop-loss: {exit_condition} · Time-box: {timeframe}",
  "slot_names": ["bet", "counterparty", "downside", "exit_condition", "timeframe"],
  "source": "canonical",
  "status": "CANONICAL"
}
```

**Acceptance criteria:**
- [ ] All 30 records inserted with valid fiber coordinates
- [ ] Query by `macro_family=BAR, stakes=high` returns correct variants
- [ ] No two canonical templates have holonomy distance < 0.1 (distinct facets)
- [ ] Seed set spans multiple horizons and stakes clearly enough to expose frame differences, not just output differences

---

### P1-3: Gap Log Schema
**JTBD:** As the substrate, I want every GIGI miss logged with structured metadata so that the system can reveal where the user's current frame lacks resolution, turning unknown terrain into visible shadow zones rather than silent failure.

**Schema:**
```sql
CREATE TABLE gap_log (
  gap_id             TEXT PRIMARY KEY,
  session_id         TEXT,         -- hash only, never content
  timestamp          INTEGER,
  
  -- What was attempted
  macro_family_attempted  TEXT,
  stakes_attempted        TEXT,
  horizon_attempted       TEXT,
  
  -- What was found
  nearest_template_id     TEXT,
  holonomy_distance       REAL,     -- distance to nearest match
  gap_tier               TEXT,     -- PARTIAL_MATCH | ALC_CANDIDATE | HUMAN_REQUIRED | RAW_LLM
  
  -- What happened
  fallback_used          TEXT,     -- which gate fired
  alc_candidate_emitted  BOOLEAN,
  alc_candidate_id       TEXT,     -- null if no candidate
  resolved_by            TEXT,     -- promoted | human_authored | raw_llm | deferred
  
  -- Signal for library improvement
  recurrence_count       INTEGER DEFAULT 1,
  cluster_id             TEXT,     -- null until gap clustering runs
  flagged_for_review     BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_gap_macro ON gap_log(macro_family_attempted);
CREATE INDEX idx_gap_distance ON gap_log(holonomy_distance);
CREATE INDEX idx_gap_flagged ON gap_log(flagged_for_review);
```

**Gap tier thresholds:**
```
holonomy_distance < 0.2   → L1 hit (no gap logged)
0.2 ≤ distance < 0.5      → PARTIAL_MATCH (Scenario 2)
0.5 ≤ distance < 0.8      → ALC_CANDIDATE (ALC fires)
distance ≥ 0.8            → HUMAN_REQUIRED (too far for synthesis)
```

**Acceptance criteria:**
- [ ] Every L1 miss writes a gap log entry
- [ ] Gap tier calculated automatically from holonomy distance
- [ ] `recurrence_count` increments on duplicate gap pattern
- [ ] Periodic review query: `SELECT * FROM gap_log WHERE recurrence_count > 3 ORDER BY recurrence_count DESC`
- [ ] Gap review UI language frames misses as newly visible shadow detail, not simple errors

---

### P1-4: WAL Checkpoint + iCloud Backup Integration
**JTBD:** As an ARCS user, I want GIGI to checkpoint and encrypt to iCloud automatically on Decision Receipt write so that I never lose validated slices of my frame-of-reference, and the backup never contains raw session content.

**Checkpoint triggers (semantic, not time-based):**
```
FIRE checkpoint on:
  ✓ Decision Receipt written to GIGI
  ✓ Template promoted from CANDIDATE → CANONICAL
  ✓ Session explicitly closed
  ✓ Gap log entry flagged high-stakes

DO NOT fire on:
  ✗ Timer alone
  ✗ ALC candidate emitted (still Golden Thread)
  ✗ Mid-session gap log entry
  ✗ Battery < 20% (defer to next trigger)
```

**Backup flow:**
```
Checkpoint trigger fires
  → SQLite WAL checkpoint (changed pages only)
  → AES-256-GCM encrypt (key: M1 Secure Enclave)
  → Write .gigi.enc to ~/Library/Mobile Documents/
    (iCloud Drive path, ADP-protected)
  → Log: timestamp, pages_written, backup_size
```

**Retention policy:**
```
Keep: last 30 daily snapshots
Keep: all Receipt-triggered checkpoints for 90 days
Prune: older incrementals
```

**Acceptance criteria:**
- [ ] WAL mode enabled on GIGI SQLite DB
- [ ] Checkpoint fires on Receipt write (not on timer)
- [ ] Encrypted blob appears in iCloud Drive path
- [ ] Restore test: decrypt blob on same machine, verify record count matches
- [ ] Battery-aware: checkpoint deferred if battery < 20% and not plugged in
- [ ] Restore semantics preserve canonical priors and receipt lineage, not just file integrity

---

### P1-5: Holonomy Distance Query + L1 Gate
**JTBD:** As the filtration system, I want the L1 gate to query GIGI for the nearest template by holonomy distance so that close-enough matches are surfaced as already-cut facets of judgment, reducing conscious reconstruction load before L2+ is needed.

**Query logic:**
```python
def l1_query(session_fiber_coords, macro_family, threshold=0.2):
    """
    Returns nearest canonical template if distance < threshold.
    Returns None + gap_tier if no match within threshold.
    """
    candidates = gigi.query(
        macro_family=macro_family,
        status='CANONICAL',
        limit=10
    )
    
    distances = [
        (t, holonomy_distance(session_fiber_coords, t.holonomy_signature))
        for t in candidates
    ]
    
    nearest = min(distances, key=lambda x: x[1])
    template, distance = nearest
    
    if distance < 0.2:
        template.use_count += 1
        return template, distance, 'L1_HIT'
    elif distance < 0.5:
        return template, distance, 'PARTIAL_MATCH'  # Scenario 2
    elif distance < 0.8:
        return template, distance, 'ALC_CANDIDATE'  # Scenario 3, ALC fires
    else:
        return None, distance, 'HUMAN_REQUIRED'
```

**Acceptance criteria:**
- [ ] L1 query runs in < 50ms for library of 200 templates
- [ ] Holonomy distance function matches pure-fiber paper spec (§2.4 weighted ℓ2)
- [ ] Gap tier returned with every query result
- [ ] Confidence score surfaced in UI: "Matched at distance 0.34 — verify before committing"
- [ ] UI copy makes explicit when a match is a frame-of-reference aid versus a high-confidence canonical hit

---

## Phase 2 — Alchemist Module

### P2-1: ALC + VCE Structural Synthesis
**JTBD:** As the L2 gate, I want the Alchemist to synthesize a CANDIDATE template from existing geometry when L1 returns PARTIAL_MATCH or ALC_CANDIDATE, so that variant gaps become new candidate facets cut along valid geometric planes instead of being offloaded immediately to an LLM.

**ALC operation:**
```
Input:  nearest template (right shape, wrong elements)
        target fiber coordinates (what was actually needed)
        
VCE collision:
  1. Extract holonomy signature from nearest template (preserve geometry)
  2. Transport semantic coordinates to target fiber position
  3. Fork skeleton: preserve structure, replace miscalibrated slots
  4. Emit CANDIDATE with provenance tag
  
Output: new template at target coordinate
        source: 'alchemist_candidate'
        status: 'CANDIDATE'
        alc_confidence: 0.0-1.0
        parent_shapes: [nearest_template_id]
```

**Confidence floor:**
```
if holonomy_distance >= 0.8:
    do NOT emit candidate
    write gap_log(gap_tier='HUMAN_REQUIRED')
    surface to user: "No synthesis possible — human authorship needed"
```

**Acceptance criteria:**
- [ ] ALC fires automatically on PARTIAL_MATCH and ALC_CANDIDATE gap tiers
- [ ] Candidate emitted with full provenance (parent_shapes, alc_confidence)
- [ ] Candidate surfaced to user with distance score visible
- [ ] Candidate NOT auto-promoted — requires explicit user validation
- [ ] Confidence floor enforced: no candidate emitted at distance ≥ 0.8
- [ ] Candidate explanation language describes synthesis as controlled facet-cutting, not black-box generation

---

### P2-2: Validation + Promotion Flow
**JTBD:** As an ARCS user, I want to validate ALC candidates with salt-to-taste editing so that my corrections become the canonical recipe, the Alchemist learns my palate from the delta, and my frame becomes more precise through use.

**Validation flow:**
```
ALC candidate surfaced
  → User reviews skeleton
  → Three options:
  
  1. PROMOTE AS-IS
     candidate.status = 'CANONICAL'
     candidate.validated_by = 'human'
     candidate.promoted_at = now()
     gap_log.resolved_by = 'promoted'
     
  2. EDIT THEN PROMOTE (salt to taste)
     Record delta between ALC output and user edit
     Store delta as ALC calibration signal
     Promote edited version as canonical
     
  3. REJECT + NOTE
     candidate.status = 'DEPRECATED'
     gap_log.flagged_for_review = true
     User adds note: why rejected
     Informs next ALC synthesis attempt
```

**Acceptance criteria:**
- [ ] All three validation paths implemented
- [ ] Edit delta stored for ALC calibration
- [ ] Promoted templates immediately available for L1 queries
- [ ] Rejected candidates never served again
- [ ] Validation UI makes explicit that promotion turns a candidate into a genuine prior for future self

---

## Phase 3 — Cloud Fallback (Graceful, Explicit)

### P3-1: Controlled LLM Fallback (L4 Gate)
**JTBD:** As the L4 gate, I want to send only abstract structural descriptions to the LLM when human authorship isn't available, so that novel macro families can be synthesized without exposing raw decision content or surrendering the integrity of the user's frame-of-reference.

**What gets sent (abstract structure only):**
```
Sends:      "I need a [MACRO_FAMILY] variant for 
             [STAKES_LEVEL] stakes, [HORIZON] horizon, 
             [MODE] mode. Structural skeleton only, 
             no example content."
             
Never sends: actual decision content
Never sends: Decision Receipt history  
Never sends: session context
Never sends: personal identifiers
```

**User approval gate:**
```
Before any L4/L5 call:
  Surface to user:
    "No local template found.
     Send structural description to [LLM provider]?
     They will see: macro family + parameter types only.
     They will NOT see: your actual decision."
     
  User chooses: YES → proceed | NO → defer/human-author
  Choice logged in gap_log
```

**Acceptance criteria:**
- [ ] L4 NEVER fires without explicit user approval
- [ ] Approval dialog shows exactly what will be sent
- [ ] L5 (raw LLM) requires separate explicit approval
- [ ] All L4/L5 calls logged with timestamp + what was sent (abstract only)
- [ ] Approval copy explains that privacy protects frame integrity, not only content secrecy

---

## Phase 4 — Production Hardening

### P4-1: Battery-Aware Routing
- [ ] Read battery state before ALC synthesis
- [ ] Defer synthesis if battery < 20% and unplugged
- [ ] Log deferred work for next plugged-in session
- [ ] Never defer Receipt writes (always checkpoint immediately)

### P4-2: Gap Clustering + Library Review
- [ ] Periodic job: cluster gap_log by macro_family + fiber coordinates
- [ ] Surface top-N recurring gaps (recurrence_count > 3)
- [ ] One-session LLM batch: generate template skeletons for clustered gaps
- [ ] INSERT all at once → library closes multiple gaps simultaneously
- [ ] Cluster review language frames recurring gaps as persistent blind spots in frame resolution

### P4-3: Version Migration
- [ ] `version_tag` field enables clean ARCS version upgrades
- [ ] New ARCS macro version → INSERT new templates alongside old
- [ ] Query by `version_tag='v9'` for upgraded sessions
- [ ] Old versions remain queryable, never deleted

### P4-4: Restore Test Protocol
- [ ] Monthly: decrypt iCloud backup blob on local machine
- [ ] Verify: record count matches live DB
- [ ] Verify: random sample of 5 templates matches expected content
- [ ] Log restore test result in gap_log (special entry type)
- [ ] Verify that restored state preserves continuity of priors across time, not just bytes on disk

---

## Threat Model (Locked)

| Threat | Mitigation | Status |
|--------|-----------|--------|
| Cloud API reads decisions | GIGI local-first, LLM sees structure only | ✅ Handled |
| Local disk failure | WAL checkpoint → AES-256-GCM → iCloud ADP | ✅ Handled |
| Lost machine | iCloud Keychain sync, new device restore | ✅ Handled |
| Casual unauthorized access | FileVault at rest | ✅ Handled |
| Vendor data breach | iCloud stores ciphertext only | ✅ Handled |
| **State actor / legal compulsion** | **Named edge case. ADP = Apple holds no key. Accepted as out-of-scope.** | ⚠️ Accepted |

**Upgrade path if threat model escalates:** Migrate key storage to YubiKey 5. Disable iCloud Keychain. Migrate to local-only backup.

---

## Build Order

```
Week 1:  P0 (machine hardening) + P1-1 (schema) + P1-2 (seed templates)
Week 2:  P1-3 (gap log) + P1-4 (WAL + iCloud) + P1-5 (L1 gate)
Week 3:  P2-1 (ALC synthesis) + P2-2 (validation flow)
Week 4:  P3-1 (cloud fallback) + P4-1 (battery routing)
Ongoing: P4-2 (gap clustering) + P4-3 (versioning) + P4-4 (restore tests)
```

---

## Key Principles (Non-Negotiable)

1. **Raw LLM is the floor, not the ceiling.** Every session starts at L1, not L5.
2. **Explicit approval for every cloud call.** No silent network access.
3. **WAL checkpoints are semantic, not time-based.** Receipt write = checkpoint trigger.
4. **ALC candidates never auto-promote.** You are always the validator.
5. **Holonomy distance always visible.** No confident-looking miscalibrated output.
6. **NOTOMATION intact.** The substrate scaffolds. You decide.
7. **The substrate externalizes frame-of-reference, not just output structure.** Build for legibility of judgment, not convenience alone.
8. **Privacy protects the user's coordinate system.** Protect frame integrity, not just data confidentiality.

---

*Generated from ARCS-LOCAL architecture session, May 14 2026.*
*Updated to include the “reference of frame” principle and self-clarification JTBD, May 14 2026.*
*Companion papers: pure_fiber_lm_v1 (Davis, May 2026), geodesic_computation_v11 (Davis, April 2026).*
