# ZPE-Mental

[![License: SAL v7.1](https://img.shields.io/badge/license-SAL%20v7.1-blue.svg)](LICENSE)

## What This Is

Bounded endogenous-form codec. Tunnels, spirals, lattices, symmetry metadata, and profile fields round-trip exactly within the declared release corpus.

ZPE-Mental is one of 17 independent encoding products in the Zer0pa portfolio, each scoped to its own domain; Mental covers the endogenous visual modality and makes no claim beyond it.

The product is useful now and improving continuously. The current surface is prepared for exact bounded-form replay on the corpus committed here and for fresh falsification from a clean outsider shell. It does not claim broad cognition decoding, language understanding, autobiographical memory recovery, or general decoding of non-visual mental content.

## Codec Mechanics

<p>
  <img src=".github/assets/readme/lane-mechanics/MENTAL.gif" alt="ZPE-Mental Codec Mechanics animation" width="100%">
</p>

| Field | Value |
| ------- | ------- |
| Architecture | VISUAL_FORM_STREAM |
| Encoding | MENTAL_ENDOGENOUS_FORM_V1 |
| Mechanics Asset | `.github/assets/readme/lane-mechanics/MENTAL.gif` |

## Key Metrics

| Metric | Value | Baseline |
| -------- | ------- | ---------- |
| FORM_EXACT | 1.00 | ref |
| BASELINE_DELTA | 0.00 | py-ref |
| NON_VISUAL_ALIAS | 1.00 | bounded |
| NON_VISUAL_SEMANTIC_RETENTION | 0.00 | bounded |

> Source: `proofs/artifacts/mental_release_matrix.json`; `validation/results/pytest.xml`; `proofs/manifests/CURRENT_FALSIFICATION_PACKET.md`

## Repo Identity

| Field | Value |
| ------- | ------- |
| Identifier | ZPE-Mental |
| Repository | https://github.com/Zer0pa/ZPE-Mental |
| Section | encoding |
| Visibility | PUBLIC |
| Architecture | VISUAL_FORM_STREAM |
| Encoding | MENTAL_ENDOGENOUS_FORM_V1 |
| Commit SHA | 7d4668156676 |
| License | SAL-7.1 |
| Authority Source | proofs/artifacts/mental_release_matrix.json |

## Readiness

| Field | Value |
| ------- | ------- |
| Verdict | STAGED |
| Checks | 3/3 |
| Anchors | 4 display anchors |
| Commit | 1fc027756da4 |
| Authority | proofs/artifacts/mental_release_matrix.json |

### Honest Blocker

Broad cognition decoding or any general decoding of non-visual mental content.; Recovery of autobiographical memory, legal or moral reasoning, theorem proving, or language planning.; Clinical diagnosis, therapeutic interpretation, or any medical use.

## What We Prove

- Exact round-trip preservation for the bounded endogenous visual form corpus in `validation/corpora/endogenous_forms.json` (4 forms: tunnel, spiral, lattice, cobweb).
- Parity between the Rust-native encoder/decoder and the pure-Python reference implementation on the same bounded corpus (`BASELINE_DELTA = 0.00`; byte-level word equality asserted in CI).
- Stable profile and symmetry handling for both `COMPASS_8` (8 directions at 45° increments) and `D6_12` (12 directions at 30° increments, enabling exact D6 rotational symmetry) direction surfaces included in the release corpus.
- Deterministic collapse of non-visual prompts to a bounded fallback form instead of broad-cognition retention (`NON_VISUAL_SEMANTIC_RETENTION = 0.00`; `NON_VISUAL_ALIAS = 1.00`).
- Compact 20-bit wire format supporting raw and RLE direction encodings, per-stroke frame-index and delta-time metadata, all within the same word space. RLE is the default transport (Augmentation Phase 2). Proof: `zpe_mental/pack.py`, `src/lib.rs`.

## What We Don't Claim

- Broad cognition decoding or any general decoding of non-visual mental content.
- Recovery of autobiographical memory, legal or moral reasoning, theorem proving, or language planning.
- Clinical diagnosis, therapeutic interpretation, or any medical use.
- A broader shared stack or any decoding coverage outside endogenous visual forms.

## Verification Status

| Code | Check | Verdict |
| ------ | ------- | --------- |
| V_01 | `python -m pip install \".[test]\"` builds and imports the Rust-native extension as `zpe_mental._native` | PASS |
| V_02 | `python scripts/validate_release.py --output proofs/artifacts/mental_release_matrix.json` reproduces the bounded release metrics | PASS |
| V_03 | `pytest tests -q --junitxml validation/results/pytest.xml` passes the public regression suite | PASS |

## Proof Anchors

| Path | State |
| ------ | ------- |
| `proofs/manifests/CURRENT_FALSIFICATION_PACKET.md` | VERIFIED |
| `proofs/artifacts/mental_release_matrix.json` | VERIFIED |
| `validation/results/pytest.xml` | VERIFIED |
| `docs/BOUNDED_RELEASE_SCOPE.md` | VERIFIED |

## Repo Shape

| Field | Value |
| ------- | ------- |
| Proof Anchors | 4 display anchors |
| Modality Lanes | 1 |
| Architecture | VISUAL_FORM_STREAM |
| Encoding | MENTAL_ENDOGENOUS_FORM_V1 |
| Verification | 3/3 checks |
| Authority Source | proofs/artifacts/mental_release_matrix.json |

## Extended Metrics

Rows retained from the previous expanded `## Key Metrics` table. The public product page uses the first four rows only.

| Metric | Value | Baseline | Proof | CI Test |
|-------|-------|----------|-------|---------|
| PYTEST_PASS | 3/3 | pytest | `validation/results/pytest.xml` | `pytest tests -q` (CI: Run tests) |

## Competitive Benchmarks

ZPE-Mental encodes endogenous visual forms (tunnels, spirals, lattices, cobwebs per the Klüver form taxonomy). This modality has no incumbent codec market and no established external encoder/decoder. The closest domain — fractal geometry encoders or L-system compressors — generate fractals from rules rather than encode/decode closed perceptual forms; the comparison would be apples-to-oranges. The lane's product claim is exact roundtrip fidelity on the bounded form corpus (FORM_EXACT = 1.00), not byte compression.

## Quick Start

```bash
python3 - <<'PY'
import sys
raise SystemExit(0 if sys.version_info >= (3, 11) else "Python 3.11+ is required")
PY
cargo --version
python3 -m venv .venv
. .venv/bin/activate
python -m pip install --upgrade pip
python -m pip install ".[test]"
python scripts/validate_release.py --output proofs/artifacts/mental_release_matrix.json
pytest tests -q --junitxml validation/results/pytest.xml
```

## Upcoming Workstreams

This section captures the active lane priorities — what the next agent or contributor picks up, and what investors should expect. Cadence is continuous, not milestoned.

- **Klüver-form codec — market and admission path** — Zero-Base Scientific Thinking — GPD Research and Planning Pending. Lane has no incumbent codec market for endogenous visual forms. Active research into market definition (perceptual encoding, neuroscience tooling, altered-states documentation, art-tech) and the certified-subset admission path before corpus expansion past the four bounded forms.
