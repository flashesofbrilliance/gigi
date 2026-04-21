# ICARUS Sprint Delivery
**From:** Davis Geometric Intelligence / GIGI Engine
**To:** ICARUS Team
**Re:** 7 Sprint Asks — All Delivered + WAL Fix

---

## Summary

All 7 ICARUS sprint asks have been implemented, tested (641 tests green), and deployed to
the live service. A WAL corruption bug discovered during the sprint has also been patched.

**Live endpoint:** `https://gigi-stream.fly.dev`  
**Protocol:** `POST /v1/gql` — body `{ "query": "GQL_STATEMENT" }`  
**Reference:** See `GQL_REFERENCE.md` §XI for full syntax and response shapes.

---

## Access Pattern

```bash
curl -s -X POST https://gigi-stream.fly.dev/v1/gql \
  -H 'Content-Type: application/json' \
  -d '{"query": "HOLONOMY corpus ON FIBER (f11, f12) AROUND tense_label"}'
```

All geometric commands are GQL statements sent to `POST /v1/gql`. No authentication
required in the current configuration.

---

## Delivered Features

### 1. GAUGE … VS — Cross-bundle gauge invariance test

Tests whether two bundles share the same fiber topology in a shared subspace. The core
question: *"Are these two models encoding the same geometry, up to a gauge transformation?"*

```gql
GAUGE corpus_en VS corpus_fr ON FIBER (f11, f12) AROUND tense_label;
```

**Response:**
```json
{
  "count": 1,
  "rows": [{
    "bundle1": "corpus_en",
    "bundle2": "corpus_fr",
    "holonomy_1": 0.0,
    "holonomy_2": 0.0,
    "gauge_difference": 0.0,
    "gauge_invariant": true
  }]
}
```

`gauge_invariant: true` when `|holonomy_1 − holonomy_2| < π/10 ≈ 0.314`.  
**Bug fixed this sprint:** Cross-bundle lookup now correctly resolves `bundle2` through
the engine store rather than failing with "not supported in stream path".

---

### 2. HOLONOMY … ON FIBER — Global fiber holonomy

```gql
HOLONOMY corpus ON FIBER (f11, f12) AROUND tense_label;
```

Returns one row per `tense_label` group (centroid + transport angle) plus a summary row:

```json
{
  "count": 5,
  "rows": [
    { "transport_angle": -3.14159, "w": 0.0, "x": 1.0 },
    { "transport_angle":  1.5707,  "w": 0.5, "x": 0.5 },
    ...
    { "_type": "summary", "holonomy_angle": 0.0, "holonomy_trivial": true }
  ]
}
```

`holonomy_trivial: true` → flat connection (no torsion in the fiber).

---

### 3. HOLONOMY … NEAR — Local fiber holonomy

Scopes holonomy computation to the ε-neighbourhood of a query point. O(|N_r|) instead
of O(N). Supports Euclidean (default) and cosine metrics.

```gql
HOLONOMY corpus
  NEAR (f11=1.0, f12=0.0)
  WITHIN 0.3
  ON FIBER (f11, f12)
  AROUND tense_label;
```

```gql
HOLONOMY corpus
  NEAR (f11=1.0, f12=0.0)
  WITHIN 0.1
  METRIC cosine
  ON FIBER (f11, f12)
  AROUND tense_label;
```

**Response:** `{ "local_holonomy_angle": ..., "neighbourhood_size": ... }`

---

### 4. SPECTRAL … ON FIBER MODES k — Fiber-space Laplacian eigenmodes

Computes the k smallest non-zero eigenvalues (and inverse participation ratios) of the
normalised Laplacian of the k-NN graph in the named fiber subspace. Answers: *"How many
semantic clusters live in this fiber subspace?"*

```gql
SPECTRAL corpus ON FIBER (f11, f12) MODES 3;
```

**Response:**
```json
{
  "count": 3,
  "rows": [
    { "mode": 1, "lambda": 0.333, "ipr": 0.5   },
    { "mode": 2, "lambda": 0.128, "ipr": 0.475 },
    { "mode": 3, "lambda": 0.019, "ipr": 0.475 }
  ]
}
```

Near-zero eigenvalues indicate strong cluster boundaries. `ipr ≈ 1` = highly localised
mode; `ipr ≈ 0` = fully delocalised (diffuse).

---

### 5. TRANSPORT … FROM / TO — Parallel transport between records

Computes the rotation matrix encoding how the fiber has rotated when moving along the
geodesic from record A to record B.

```gql
TRANSPORT corpus FROM (token_id=42) TO (token_id=99) ON FIBER (f11, f12);
```

**Response:**
```json
{
  "count": 1,
  "rows": [{
    "displacement_0": -0.293,
    "displacement_1":  0.707,
    "displacement_2":  0.0,
    "displacement_3":  0.0,
    "q0": 0.707, "q1": 0.707, "q2": 0.0, "q3": 0.0,
    "transport_angle": 1.5707963
  }]
}
```

`transport_angle ≈ π/2` → 90° rotation between these two tokens in fiber space.

---

### 6. DIVERGENCE … VS — Distribution divergence between two bundles

GQL surface for KL and Jensen–Shannon divergence between two bundle distributions.

```gql
DIVERGENCE corpus_en VS corpus_fr;
```

**Response:**
```json
{
  "bundle_a":       "corpus_en",
  "bundle_b":       "corpus_fr",
  "fields_compared": 4,
  "jensen_shannon":  0.216,
  "kl_forward":      0.696,
  "kl_reverse":      1.551,
  "per_field":       "w=0.003,x=0.224,y=0.468,z=0.000"
}
```

Also available via REST: `POST /v1/divergence` with body `{ "from": "...", "to": "..." }`.

---

### 7. INVARIANT constraint in BUNDLE definition

Bundles may now declare gauge invariants — runtime constraints that GIGI enforces on every
insert and upsert.

```gql
BUNDLE corpus
  BASE (token_id NUMERIC)
  FIBER (
    tense_label CATEGORICAL INDEX,
    f11 NUMERIC, f12 NUMERIC
  )
  INVARIANT f11 * f11 + f12 * f12 = 1.0 +/- 0.05;
```

Any section that violates a declared invariant is rejected with an explanatory error.
Multiple constraints are supported; each one binds to the named field in the FIBER block.

---

### 8. WAL Fix — Gauge key persistence

A bug in the WAL replay path caused `gauge_key` values to be silently dropped on restart,
leading to decryption failures for ENCRYPTED fields on warm restart. Fixed: gauge keys now
round-trip correctly through WAL serialisation and replay.

---

## Running All 5 ICARUS Commands Against the Live DB

```bash
BASE=https://gigi-stream.fly.dev
GQL="$BASE/v1/gql"

# 1. Global holonomy
curl -s -X POST $GQL -H 'Content-Type: application/json' \
  -d '{"query":"HOLONOMY corpus ON FIBER (f11, f12) AROUND tense_label"}' | jq .

# 2. Local holonomy (cosine)
curl -s -X POST $GQL -H 'Content-Type: application/json' \
  -d '{"query":"HOLONOMY corpus NEAR (f11=0.5, f12=0.866) WITHIN 0.2 METRIC cosine ON FIBER (f11, f12) AROUND tense_label"}' | jq .

# 3. Fiber eigenmodes
curl -s -X POST $GQL -H 'Content-Type: application/json' \
  -d '{"query":"SPECTRAL corpus ON FIBER (f11, f12) MODES 3"}' | jq .

# 4. Parallel transport
curl -s -X POST $GQL -H 'Content-Type: application/json' \
  -d '{"query":"TRANSPORT corpus FROM (token_id=0) TO (token_id=1) ON FIBER (f11, f12)"}' | jq .

# 5. Gauge invariance test
curl -s -X POST $GQL -H 'Content-Type: application/json' \
  -d '{"query":"GAUGE corpus VS corpus ON FIBER (f11, f12) AROUND tense_label"}' | jq .
```

---

## Test Coverage

| Category | Tests |
|---|---|
| Parser unit tests (all statements) | 598 |
| Binary integration tests (gigi-stream, gigi-edge, bench_ingest) | 43 |
| **Total** | **641** |

All 641 tests pass on CI and locally (`cargo test --all`).

---

## Reference

- Full GQL syntax: `GQL_REFERENCE.md` §XI (Geometric Operations)
- API reference: `GIGI_API.md` (GQL section)
- Live service health: `GET https://gigi-stream.fly.dev/v1/health`
