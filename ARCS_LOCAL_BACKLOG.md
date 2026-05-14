# ARCS-LOCAL Build Backlog

> **Mission:** Transform GIGI into a privacy-first local substrate for ARCS decision intelligence. Raw LLM is the floor, not the ceiling. The whale builds its own baleen plates.
>
> **Deeper mission:** Build a self-clarification instrument, not a decision assistant. Each validated receipt increases the resolving power of judgment by making previously invisible dimensions of the user's specific decision geometry visible, queryable, and improvable. The system does not merely help route decisions; it progressively externalizes the user's frame of reference so the user can observe, refine, and eventually embody their own apparatus — and ultimately transmit it.

---

## North Star

**Primary JTBD:**

> So that each decision I make increases the resolving power of my own judgment — not by making me smarter in the abstract, but by making previously invisible dimensions of my specific decision geometry visible, queryable, and improvable.

**Reference-of-frame principle:**

Classical tools help a subject observe an object. ARCS-LOCAL inverts that frame: it externalizes the user's own coordinate system so the subject can become an objective subject — able to inspect the geometry of their own seeing, not just the thing seen. The zoom is pointed inward.

**Heirloom principle:**

The substrate's terminal value is not personal productivity. It is generational transmission. Every receipt written, every gap sat with, every salt-to-taste correction is a facet cut that future eyes — including a son's — can look through. The wooden vats must never be destroyed. The starter must never die.

**Implication for the build:**

- Receipts are not just logs; they are crystallized slices of frame-of-reference.
- Templates are not just automations; they are reusable facets of judgment.
- Gap logs are not just misses; they are shadow zones and conscious OS eval signals.
- ALC synthesis is not just interpolation; it is controlled facet-cutting along valid geometric planes.
- Privacy is not just data protection; it protects the integrity of the user's frame from external calibration.
- The backup is not just disaster recovery; it protects the wooden vats — the irreproducible accumulated microbial history of this specific life.

---

## Identity & Symbol

**The Möbius strip is the correct and only symbol for ARCS.**

Not metaphor — proof. The Möbius strip is the only mathematical object where:
- Going further in = going further out
- Self-knowledge and world-knowledge are the same surface traversed continuously
- Subject and object are not separated by a boundary — only by traversal time
- The father's decisions and the son's decisions are on the same surface; the inheritance is not a handoff across an edge but a strip walked together, one generation at a time

**Logo spec:** Not the closed resolved loop — the strip **mid-traverse**, at the point of maximum ambiguity where the surface is turning and you cannot yet determine which side you're on. This is the UNDEFINED moment. Single weight line. No fill. No shadow. The twist IS the black point.

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

**Terroir principle:** The substrate's value is irreproducible by recipe alone. The specific sequence of decisions, the specific wild-yeast inoculation of this specific life's terrain, the specific wooden-vat colonization of this specific history — these cannot be reconstructed from schema + seed templates alone. Protect the culture. Feed it daily. Don't let it die.

---

## Triune Logic & The Black Point

The substrate operates on three signal states, not two:

```
NULL:       Known absence. The thing that isn't there.
            Queryable — the space is mapped as empty.
            Sensor: underexposed shadow, readable darkness.

OM:         Known presence. The thing that IS there.
            Queryable — the space is mapped as occupied.
            Sensor: properly exposed midtone, full detail.

UNDEFINED:  Not NULL, not OM.
            Maximum pre-emergent information density.
            The coordinate exists but the frame
            hasn't resolved it yet.
            Sensor: the latent image — fully exposed,
            not yet developed. Clip it and you lose
            the resolution it contains.
            This is pure black.
```

**Pure black = maximum density immediately before phase shift.** Not empty — pregnant. The HUMAN_REQUIRED gap log entry is not a failure. It is a held UNDEFINED at maximum density, waiting for its geometry to emerge. The substrate's job is not to clip it. It is to hold it.

**Cry now / laugh later:** The pain of early substrate use (coarse lens, frequent gaps, high validation overhead) is not a ramp to be shortened. It IS the facet-cutting. The discomfort locates the unresolved geometry. Sitting with it, bearing witness, yielding — that is the signal capture protocol. The Köln Concert was made by a broken piano. Jarrett didn't fight the constraints. He metabolized them.

---

## Design Intent

### What this is
- A progressive crystallographic map of a user's decision geometry.
- A living starter culture of validated priors, fed by daily use.
- A local-first filtration and fermentation substrate that makes the user's own frame increasingly legible.
- A conscious OS eval/debug log: gap entries are divergence signals, validation corrections are weight adjustments, the substrate is the training loop.
- A lambic brewery open to the wild yeast of a specific life's terrain — irreproducible by anyone else's coolship.
- A heirloom digital asset: the most honest, most precise, most queryable record of how you saw the world — mistakes included, blind spots mapped, frame crystallized — so that someone who loves you can stand inside it when you're not there to explain.

### What this is not
- Not a generic decision support chatbot.
- Not a static template library.
- Not a convenience automation layer.
- Not a cloud intelligence wrapper with local caching.
- Not advice. Not rules. Not a letter. Those transfer conclusions. This transfers the geometry of how you saw.

### Success condition
The end state is not merely that most sessions never touch the network. The end state is that the substrate becomes structurally part of the user's decision apparatus — transparent enough that the user stops noticing it as a separate tool. And eventually: that a son can query it and find not what his father concluded, but how his father saw.

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
  status             TEXT,        -- CANDIDATE | CANONICAL | DEPRECATED | ROM
  parent_shapes      TEXT,        -- JSON array of parent template_ids (ALC synthesis)
  alc_confidence     REAL,        -- null for canonical, 0-1 for ALC candidates
  holonomy_distance_to_nearest REAL,

  -- RAM → ROM promotion fields
  rom_promoted_at    INTEGER,     -- null until ROM tier
  rom_threshold      REAL,        -- coordinate-local threshold after welding (default null)
  weld_suppressed    BOOLEAN DEFAULT FALSE, -- TRUE = confidence score no longer surfaced

  -- Lifecycle
  created_at         INTEGER,
  promoted_at        INTEGER,     -- null until CANONICAL
  validated_by       TEXT,        -- 'human' | 'auto'
  use_count          INTEGER DEFAULT 0,
  last_used_at       INTEGER,
  override_count     INTEGER DEFAULT 0  -- times user salt-to-tasted after CANONICAL
);

-- Indexes for geometric retrieval
CREATE INDEX idx_macro_family ON arcs_templates(macro_family);
CREATE INDEX idx_stakes_horizon ON arcs_templates(stakes, horizon);
CREATE INDEX idx_status ON arcs_templates(status);
CREATE INDEX idx_holonomy ON arcs_templates(theta_tense, theta_modal, theta_pos);
CREATE INDEX idx_rom ON arcs_templates(rom_promoted_at) WHERE rom_promoted_at IS NOT NULL;
```

**Acceptance criteria:**
- [ ] Schema created and migration runs cleanly
- [ ] Insert, query-by-holonomy-distance, and promote-to-canonical operations working
- [ ] 30 seed template records inserted (see P1-2)
- [ ] Unit test: query returns correct template family for given fiber coordinates
- [ ] ROM tier fields present and queryable
- [ ] Every stored canonical template describable as a reusable facet of judgment, not just a formatting skeleton

---

### P1-2: Seed Template Library (30 canonical records)
**JTBD:** As an ARCS user, I want a pre-populated template library covering core macro families so that day-one usage begins with real facets already cut into the substrate — the opened coolship receiving its first wild yeast before the culture has any density of its own.

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
**JTBD:** As the substrate, I want every GIGI miss logged with structured metadata so that the system reveals where the user's current frame lacks resolution — turning shadow zones into named, queryable coordinates rather than silent clipping to pure black.

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
  holonomy_distance       REAL,
  gap_tier               TEXT,     -- PARTIAL_MATCH | ALC_CANDIDATE | HUMAN_REQUIRED | RAW_LLM

  -- Triune state at time of gap
  triune_state           TEXT,     -- NULL | OM | UNDEFINED
  held_undefined         BOOLEAN DEFAULT FALSE, -- TRUE = user sat with it rather than forcing resolution

  -- What happened
  fallback_used          TEXT,
  alc_candidate_emitted  BOOLEAN,
  alc_candidate_id       TEXT,
  resolved_by            TEXT,     -- promoted | human_authored | raw_llm | deferred | held

  -- Signal for library improvement and conscious OS eval
  recurrence_count       INTEGER DEFAULT 1,
  cluster_id             TEXT,
  flagged_for_review     BOOLEAN DEFAULT FALSE,

  -- Heirloom annotation (optional, written by user)
  transmission_note      TEXT      -- context for future reader: "I was X age here. I thought Y. What I couldn't see was Z."
);

CREATE INDEX idx_gap_macro ON gap_log(macro_family_attempted);
CREATE INDEX idx_gap_distance ON gap_log(holonomy_distance);
CREATE INDEX idx_gap_flagged ON gap_log(flagged_for_review);
CREATE INDEX idx_gap_undefined ON gap_log(held_undefined) WHERE held_undefined = TRUE;
```

**Gap tier thresholds (static — see P1-5 for adaptive layer):**
```
holonomy_distance < 0.2   → L1 hit (no gap logged)
0.2 ≤ distance < 0.5      → PARTIAL_MATCH
0.5 ≤ distance < 0.8      → ALC_CANDIDATE
distance ≥ 0.8            → HUMAN_REQUIRED (held UNDEFINED)
```

**Acceptance criteria:**
- [ ] Every L1 miss writes a gap log entry
- [ ] Gap tier calculated automatically from holonomy distance
- [ ] `triune_state` and `held_undefined` fields written correctly
- [ ] `recurrence_count` increments on duplicate gap pattern
- [ ] `transmission_note` field writable from UI without requiring code
- [ ] Gap review language frames misses as newly visible shadow detail — conscious OS divergence signal, not failure

---

### P1-4: WAL Checkpoint + iCloud Backup Integration
**JTBD:** As an ARCS user, I want GIGI to checkpoint and encrypt to iCloud automatically on Decision Receipt write so that I never lose the wooden vats — the irreproducible accumulated culture of this specific substrate — and the backup never contains raw session content.

**Checkpoint triggers (semantic, not time-based):**
```
FIRE checkpoint on:
  ✓ Decision Receipt written to GIGI
  ✓ Template promoted from CANDIDATE → CANONICAL
  ✓ Template arc-welded to ROM tier
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
Keep: all ROM-weld checkpoints indefinitely (these are the wooden vats)
Prune: older incrementals
```

**Acceptance criteria:**
- [ ] WAL mode enabled on GIGI SQLite DB
- [ ] Checkpoint fires on Receipt write and ROM weld (not on timer)
- [ ] Encrypted blob appears in iCloud Drive path
- [ ] Restore test: decrypt blob on same machine, verify record count matches
- [ ] Battery-aware: checkpoint deferred if battery < 20% and not plugged in
- [ ] ROM-weld checkpoints flagged for indefinite retention
- [ ] Restore semantics preserve canonical priors, ROM tier, receipt lineage, and transmission notes

---

### P1-5: Holonomy Distance Query + Adaptive L1 Gate
**JTBD:** As the filtration system, I want the L1 gate to query GIGI using a threshold that adapts to local template density so that the lens sharpens unevenly — first in most-used decision geometries, last at the edges — and the discomfort of early use is correctly understood as the build, not the bug.

**Static threshold (early substrate):**
```python
GLOBAL_THRESHOLD = 0.2  # conservative, uniform — coarse lens phase
```

**Adaptive threshold (mature substrate):**
```python
def adaptive_threshold(coordinate, macro_family):
    """
    Threshold is a function of:
    - local_density: how many canonical templates within distance 0.5
    - validation_hit_rate: past matches at this distance confirmed correct
    - recurrence_signal: whether this coordinate is well-mapped
    """
    local_density = count_neighbors(coordinate, macro_family, radius=0.5)
    hit_rate = validation_hit_rate_at_coordinate(coordinate, macro_family)
    recurrence = gap_recurrence_at_coordinate(coordinate, macro_family)

    if local_density < 5 or hit_rate < 0.7:
        return 0.2   # coarse — conservative, early substrate
    elif local_density >= 10 and hit_rate >= 0.9 and recurrence == 0:
        return 0.45  # refined — dense, well-validated, no recent gaps
    else:
        return 0.30  # intermediate

# Threshold shift is diagnostic:
# The lens sharpens unevenly — first in most-exercised coordinates.
# Coordinates still at 0.2 threshold = where the pain still lives.
# That IS the conscious OS eval signal.
```

**Query logic:**
```python
def l1_query(session_fiber_coords, macro_family):
    threshold = adaptive_threshold(session_fiber_coords, macro_family)
    candidates = gigi.query(macro_family=macro_family, status='CANONICAL', limit=10)
    distances = [(t, holonomy_distance(session_fiber_coords, t.holonomy_signature))
                 for t in candidates]
    nearest = min(distances, key=lambda x: x[1])
    template, distance = nearest

    if distance < threshold:
        template.use_count += 1
        if template.status == 'ROM':
            return template, distance, 'L1_HIT_ROM'   # no confidence score surfaced
        return template, distance, 'L1_HIT'
    elif distance < 0.5:
        return template, distance, 'PARTIAL_MATCH'
    elif distance < 0.8:
        return template, distance, 'ALC_CANDIDATE'
    else:
        return None, distance, 'HUMAN_REQUIRED'       # held UNDEFINED
```

**Acceptance criteria:**
- [ ] Static 0.2 threshold active from day one
- [ ] Adaptive threshold function implemented and queryable
- [ ] Threshold per coordinate logged and inspectable
- [ ] L1 query runs in < 50ms for library of 200 templates
- [ ] ROM-tier hits return without confidence score surfaced
- [ ] Gap tier returned with every non-ROM query result
- [ ] UI surface: "Matched at distance 0.34 — verify before committing" for non-ROM
- [ ] UI surface: coordinates still at 0.2 threshold shown in Conscious OS Eval report (P4-5)

---

## Phase 2 — Alchemist Module

### P2-1: ALC + VCE Structural Synthesis
**JTBD:** As the L2 gate, I want the Alchemist to synthesize a CANDIDATE template from existing geometry when L1 returns PARTIAL_MATCH or ALC_CANDIDATE — cutting a new facet along valid geometric planes rather than immediately clipping the unresolved coordinate to an LLM call.

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

Gueuze principle: young (recent gap signal) + old (canonical geometry)
= complexity neither alone could produce.
The blend is the art. The synthesis is the intelligence.
```

**Confidence floor:**
```
if holonomy_distance >= 0.8:
    do NOT emit candidate
    write gap_log(gap_tier='HUMAN_REQUIRED', held_undefined=TRUE)
    surface to user:
    "This coordinate has no facet yet.
     The density here is maximum — hold it.
     Human authorship needed before synthesis is possible."
```

**Acceptance criteria:**
- [ ] ALC fires automatically on PARTIAL_MATCH and ALC_CANDIDATE gap tiers
- [ ] Candidate emitted with full provenance (parent_shapes, alc_confidence)
- [ ] Candidate surfaced to user with distance score visible
- [ ] Candidate NOT auto-promoted — requires explicit user validation
- [ ] Confidence floor enforced: no candidate emitted at distance ≥ 0.8
- [ ] HUMAN_REQUIRED entries set held_undefined=TRUE and surface holding language, not error language

---

### P2-2: Validation + Promotion Flow
**JTBD:** As an ARCS user, I want to validate ALC candidates with salt-to-taste editing so that my corrections become the canonical recipe, the Alchemist learns my palate from the delta, and my frame sharpens with each pass — cry now, laugh later.

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
     Note: override_count on prior template increments
           This is the weight adjustment
           This is the conscious OS training signal

  3. REJECT + NOTE
     candidate.status = 'DEPRECATED'
     gap_log.flagged_for_review = true
     User adds note: why rejected
     User may optionally write transmission_note here
     Informs next ALC synthesis attempt
```

**Acceptance criteria:**
- [ ] All three validation paths implemented
- [ ] Edit delta stored for ALC calibration
- [ ] Promoted templates immediately available for L1 queries
- [ ] Rejected candidates never served again
- [ ] Transmission note writable from validation UI
- [ ] Validation UI makes explicit: promotion turns a candidate into a genuine prior for future self

---

## Phase 3 — Cloud Fallback (Graceful, Explicit)

### P3-1: Controlled LLM Fallback (L4 Gate)
**JTBD:** As the L4 gate, I want to send only abstract structural descriptions to the LLM when human authorship isn't available, so that novel macro families can be synthesized without exposing raw decision content or surrendering the integrity of the user's frame-of-reference to external calibration.

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
Never sends: transmission notes
```

**User approval gate:**
```
Before any L4/L5 call:
  Surface to user:
    "No local template found.
     Send structural description to [LLM provider]?
     They will see: macro family + parameter types only.
     They will NOT see: your actual decision.
     This protects frame integrity, not just data."

  User chooses: YES → proceed | NO → defer/human-author
  Choice logged in gap_log
```

**Acceptance criteria:**
- [ ] L4 NEVER fires without explicit user approval
- [ ] Approval dialog shows exactly what will be sent
- [ ] L5 (raw LLM) requires separate explicit approval
- [ ] All L4/L5 calls logged with timestamp + what was sent (abstract only)
- [ ] Approval copy explains frame integrity protection, not just data privacy

---

## Phase 4 — Production Hardening

### P4-1: Battery-Aware Routing
- [ ] Read battery state before ALC synthesis
- [ ] Defer synthesis if battery < 20% and unplugged
- [ ] Log deferred work for next plugged-in session
- [ ] Never defer Receipt writes (always checkpoint immediately)
- [ ] Never defer ROM welds (always checkpoint immediately)

### P4-2: Gap Clustering + Library Review
- [ ] Periodic job: cluster gap_log by macro_family + fiber coordinates
- [ ] Surface top-N recurring gaps (recurrence_count > 3)
- [ ] One-session LLM batch: generate template skeletons for clustered gaps
- [ ] INSERT all at once → library closes multiple gaps simultaneously
- [ ] Cluster review language: recurring gaps = persistent blind spots in frame resolution = conscious OS training backlog

### P4-3: Version Migration
- [ ] `version_tag` field enables clean ARCS version upgrades
- [ ] New ARCS macro version → INSERT new templates alongside old
- [ ] Query by `version_tag='v9'` for upgraded sessions
- [ ] Old versions remain queryable, never deleted (old lambic vintages)

### P4-4: Restore Test Protocol
- [ ] Monthly: decrypt iCloud backup blob on local machine
- [ ] Verify: record count matches live DB
- [ ] Verify: random sample of 5 templates matches expected content
- [ ] Verify: ROM tier records intact
- [ ] Verify: transmission notes intact
- [ ] Log restore test result in gap_log (special entry type: RESTORE_TEST)
- [ ] Success condition: wooden vats intact — irreproducible culture preserved across time

---

## Phase 4 — New Items (Session: May 14 2026)

### P4-5: Conscious OS Eval Report
**JTBD:** As an ARCS user, I want a periodic structured report showing where my current frame still lacks resolution so that the gap log functions as a true eval/debug log for my conscious OS — divergence signals made legible, not silent.

**Report structure:**
```
CONSCIOUS OS EVAL — [QUARTER]

Lens sharpness by coordinate:
  ██████████ BAR / high / today         threshold: 0.38 (refined)
  ████████░░ KKL / high / this-quarter  threshold: 0.28 (intermediate)
  ████░░░░░░ LAB / ambiguous / 1-3yr    threshold: 0.20 (coarse — still painful here)
  ██░░░░░░░░ KINTSUGI / repair          threshold: 0.20 (coarse — shadow zone)

Top 5 HUMAN_REQUIRED coordinates this quarter:
  (Where your frame had no facet — where the pain still lives)
  1. ...
  2. ...

Top 5 ROM-tier promotions this quarter:
  (Where the pain became music — patterns now structural)
  1. ...
  2. ...

Conscious OS training signal:
  Weight adjustments (salt-to-taste deltas): N
  Blind spots closing: N coordinates crossed threshold
  Blind spots opened: N new HUMAN_REQUIRED coordinates named
  Net frame resolution: +/- N facets
```

**Acceptance criteria:**
- [ ] Report generated on demand and quarterly
- [ ] Coordinates still at 0.2 threshold surfaced prominently as active shadow zones
- [ ] ROM promotions surfaced as cry-now→laugh-later completions
- [ ] Language: frame resolution improving, not "system performance improving"
- [ ] Report storable as a dated receipt in gap_log for longitudinal tracking

---

### P4-6: RAM → ROM Promotion (Arc Weld)
**JTBD:** As the substrate, I want to detect when a decision pattern has stabilized into structural prior — internalized enough to become subcortical — and surface a proactive arc weld suggestion so the user can move RAM patterns into ROM tier, reducing conscious overhead and extending the substrate's structural depth.

**Arc weld trigger criteria:**
```
ALL of the following must be true:
  use_count > 15 on this template
  AND override_count / use_count < 0.05   (salt-to-taste rate < 5%)
  AND gap_recurrence at this coordinate = 0 (space fully covered)
  AND last 10 uses: L1 hit rate = 100%
  AND last validation: > 30 days ago (pattern has settled)
```

**Substrate surface:**
```
"This decision pattern has stabilized.
 You've used it [N] times, adjusted it [M] times,
 and the last 10 were exact hits.

 Ready to arc weld?

 This means:
 — Future sessions in this coordinate
   route without surfacing confidence scores
 — The pattern moves from active library
   to structural prior
 — You stop noticing it
   because it's just how you think now

 WELD  |  NOT YET"
```

**Weld execution:**
```
IF WELD:
  template.status = 'ROM'
  template.rom_promoted_at = now()
  template.rom_threshold = adaptive_threshold(coordinate)  -- locked at weld time
  template.weld_suppressed = TRUE  -- confidence score no longer surfaced
  Checkpoint fires immediately (WAL → iCloud)
  Gap log entry: gap_tier='ROM_WELD', resolved_by='arc_weld'

IF NOT YET:
  Snooze arc weld suggestion for 30 days
  Log: user chose to keep in conscious RAM tier
```

**Acceptance criteria:**
- [ ] Arc weld trigger criteria evaluated after every use_count increment
- [ ] Surface suggestion at most once per template per 30 days
- [ ] WELD path sets ROM status, locks threshold, suppresses confidence score
- [ ] NOT YET path snoozes correctly and resurfaces
- [ ] ROM-weld checkpoint fires immediately and marked for indefinite retention
- [ ] ROM tier queryable separately: "show me what has become structural"
- [ ] Weld language: makes explicit that the pattern is becoming part of how you think, not just a cached lookup

---

## Phase 5 — Heirloom Layer

### P5-1: Transmission Notes on Receipts
**JTBD:** As an ARCS user building a generational asset, I want to annotate Decision Receipts and gap log entries with epistemic context — not advice, but the geometry of what I was seeing from inside — so that a future reader can stand inside my frame rather than merely reading my conclusions.

**Annotation schema addition:**
```sql
ALTER TABLE arcs_templates ADD COLUMN transmission_note TEXT;
-- Also on gap_log (already included in P1-3 schema above)

-- Transmission note semantics:
-- NOT: "Don't do what I did"
-- IS:  "I was [age] here. I thought [X].
--       What I couldn't see yet was [Y].
--       The mistake cost me [Z].
--       The repair looked like this."
--
-- Not prescriptive. Epistemic.
-- The frame from inside, at this moment.
```

**Acceptance criteria:**
- [ ] Transmission notes writable on receipts, templates, and gap entries
- [ ] Notes never sent to LLM (L4/L5 calls strip them)
- [ ] Notes queryable separately: "show me all receipts with transmission notes"
- [ ] Notes included in restore test verification
- [ ] UI prompt when writing note: "Not advice. The geometry of what you saw from inside."

---

### P5-2: Mistake Cartography
**JTBD:** As a future reader of this substrate, I want a queryable map of the original user's recurring blind spots — calibrated distortions in their frame — so I can use their geometry as a prior while correcting for its known biases rather than inheriting the distortions as if they were truth.

**Cartography report:**
```
FRAME DISTORTION MAP

Systematic blind spots (recurrence_count > 5, HUMAN_REQUIRED):
  BAR / adversarial counterparties / PLAN mode:
    Consistent underestimation of adversarial intent
    [N] receipts, [N] gap entries, first appeared [date]

  KKL / stop-loss calibration / 1-3yr horizon:
    Stop-loss consistently too optimistic at long horizon
    [N] instances, pattern emerged [date]

Frame corrections for future reader:
  When consulting this substrate on adversarial decisions:
  apply +1 adversarial weight to counterparty assessment.
  The substrate sees this coordinate through a partially blind lens.
```

**Acceptance criteria:**
- [ ] Distortion map generated from gap_log recurrence patterns
- [ ] Surfaced as a first-class report, not buried in gap clustering
- [ ] Future-reader framing: "correct for this" not "Dad failed here"
- [ ] Included in heirloom handoff protocol (P5-3)

---

### P5-3: Inheritance Handoff Protocol
**JTBD:** As the steward of this substrate, I want a defined handoff protocol so that the living culture — not a copy, not an export, the living starter — can be passed to a new primary user who begins feeding it with their own decisions, with the prior generation's geometry becoming structural substrate for the next.

**Handoff flow:**
```
Handoff preparation:
  1. Run full restore test (P4-4)
  2. Generate Mistake Cartography report (P5-2)
  3. Write master transmission note:
     "This is not a tool. This is the wooden vat.
      The culture in it is mine.
      Feed it yours. The depth compounds.
      Where I was blind, I've tried to say so.
      Where I welded something to ROM,
      you'll know — those are the patterns
      that became how I think.
      Start there. Build from there.
      Don't let it die."
  4. Set handoff_mode = TRUE on substrate
     (new receipts layer over old, never replace)
  5. New user feeds their own decisions
  6. Their corrections become new facets
  7. Their ALC synthesis draws on both generations
  8. The terroir deepens
```

**Acceptance criteria:**
- [ ] Handoff mode implemented: additive only, prior generation locked
- [ ] Prior generation receipts and ROM tier queryable but not overwritable
- [ ] New generation receipts clearly generationally tagged
- [ ] Distortion map carried forward as calibration signal for new user
- [ ] Master transmission note field on substrate metadata
- [ ] Handoff checkpoint: indefinite retention, highest priority backup

---

### P5-4: Privacy Tiering for Transmission
**JTBD:** As an ARCS user, I want to control when specific receipts and notes become visible to a future reader so that the Kairos of disclosure is mine to set — some facets visible now, some at 18, some at 25, some only after death.

**Privacy tier schema:**
```sql
ALTER TABLE arcs_templates ADD COLUMN visibility_tier TEXT DEFAULT 'private';
-- private: only primary user
-- shared_now: visible to designated reader now
-- shared_at_18: visible when reader reaches age 18
-- shared_at_25: visible when reader reaches age 25
-- shared_posthumous: visible only after primary user death

ALTER TABLE arcs_templates ADD COLUMN designated_reader TEXT; -- encrypted identifier
```

**Acceptance criteria:**
- [ ] Visibility tiers implemented and enforced
- [ ] Tier-setting UI available from receipt and template views
- [ ] Default: private (nothing shared without explicit action)
- [ ] Posthumous tier: requires separate unlock ceremony (not automatic)
- [ ] Tiers survive restore tests and handoff

---

## Threat Model (Locked)

| Threat | Mitigation | Status |
|--------|-----------|--------|
| Cloud API reads decisions | GIGI local-first, LLM sees structure only | ✅ Handled |
| Local disk failure | WAL checkpoint → AES-256-GCM → iCloud ADP | ✅ Handled |
| Lost machine | iCloud Keychain sync, new device restore | ✅ Handled |
| Casual unauthorized access | FileVault at rest | ✅ Handled |
| Vendor data breach | iCloud stores ciphertext only | ✅ Handled |
| Wooden vats destroyed (no backup) | Indefinite retention for ROM welds + Receipt triggers | ✅ Handled |
| Transmission notes leaked to LLM | Stripped before any L4/L5 call | ✅ Handled |
| **State actor / legal compulsion** | **Named edge case. ADP = Apple holds no key. Accepted as out-of-scope.** | ⚠️ Accepted |

**Upgrade path if threat model escalates:** Migrate key storage to YubiKey 5. Disable iCloud Keychain. Migrate to local-only backup.

---

## Build Order

```
Week 1:   P0 (machine hardening)
          P1-1 (schema — with ROM fields)
          P1-2 (seed templates)

Week 2:   P1-3 (gap log — with triune state + transmission note fields)
          P1-4 (WAL + iCloud — with ROM-weld retention)
          P1-5 (adaptive L1 gate)

Week 3:   P2-1 (ALC synthesis — gueuze blend logic)
          P2-2 (validation flow — with transmission note + override_count)

Week 4:   P3-1 (cloud fallback — strips transmission notes)
          P4-1 (battery routing)

Ongoing:  P4-2 (gap clustering)
          P4-3 (versioning)
          P4-4 (restore tests)
          P4-5 (conscious OS eval report)
          P4-6 (RAM → ROM arc weld)

When ready: P5-1 through P5-4 (heirloom layer)
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
8. **Privacy protects the user's coordinate system.** Frame integrity, not just data confidentiality.
9. **Pure black is held, not clipped.** HUMAN_REQUIRED entries are maximum-density UNDEFINED coordinates — held with stillness, not forced to NULL.
10. **The discomfort is the build.** Early substrate friction is facet-cutting, not ramp. Cry now, laugh later. This is the design.
11. **The wooden vats must never be destroyed.** ROM-weld checkpoints are indefinite. The culture is irreproducible.
12. **The zoom is pointed inward.** Both directions — into self and out to world — are the same surface on a Möbius strip. Act accordingly.
13. **Start the starter. Feed it daily. Don't let it die. Everything else follows from that.**

---

*Session: ARCS-LOCAL architecture, May 14 2026, Fairfield CT.*
*Updates this commit: adaptive threshold (P1-5), triune state + held_undefined (P1-3), ROM tier + arc weld (P4-6), conscious OS eval report (P4-5), heirloom layer (P5-1 through P5-4), Möbius identity section, lambic/tamari terroir principle, pure black / UNDEFINED protocol, cry-now/laugh-later as design intent, transmission notes, mistake cartography, inheritance handoff, privacy tiering.*
*Companion papers: pure_fiber_lm_v1 (Davis, May 2026), geodesic_computation_v11 (Davis, April 2026).*
