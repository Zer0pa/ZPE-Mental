# ZPE-Mental

[![License: SAL v7.0](https://img.shields.io/badge/license-SAL%20v7.0-blue.svg)](LICENSE)

## What This Is
ZPE-Mental is a bounded encoding product for endogenous visual forms: tunnels, spirals, lattices, cobweb-like geometry, and the profile and symmetry metadata needed to round-trip those forms exactly. It ships a public Python API backed by a Rust-native fast path, with a pure-Python reference path kept in-repo for parity checks. The strongest CI-anchored result is `FORM_EXACT = 1.00` — every form in the bounded release corpus survives a full encode/decode round-trip with zero loss, verified in CI against committed proof artifacts.

ZPE-Mental is one of 17 independent encoding products in the Zer0pa portfolio, each scoped to its own domain; Mental covers the endogenous visual modality and makes no claim beyond it.

The product is useful now and improving continuously. The current surface is prepared for exact bounded-form replay on the corpus committed here and for fresh falsification from a clean outsider shell. It does not claim broad cognition decoding, language understanding, autobiographical memory recovery, or general decoding of non-visual mental content.

| Field | Value |
|-------|-------|
| Architecture | VISUAL_FORM_STREAM |
| Encoding | MENTAL_ENDOGENOUS_FORM_V1 |

## Key Metrics
| Metric | Value | Baseline | Proof | CI Test |
|-------|-------|----------|-------|---------|
| FORM_EXACT | 1.00 | ref | `proofs/artifacts/mental_release_matrix.json` → `form_exact_rate` | `tests/test_mental_native_optional.py::test_mental_native_matches_python_reference_words_and_decode` |
| BASELINE_DELTA | 0.00 | py-ref | `proofs/artifacts/mental_release_matrix.json` → `baseline_delta` | `tests/test_mental_reference.py::test_python_reference_round_trip_matches_public_api` |
| NON_VISUAL_ALIAS | 1.00 | bounded | `proofs/artifacts/mental_release_matrix.json` → `non_visual_prompt_alias_rate` | `scripts/validate_release.py` (CI: Rebuild release artifact) |
| NON_VISUAL_SEMANTIC_RETENTION | 0.00 | bounded | `proofs/artifacts/mental_release_matrix.json` → `non_visual_semantic_retention_rate` | `scripts/validate_release.py` (CI: Rebuild release artifact) |
| PYTEST_PASS | 3/3 | pytest | `validation/results/pytest.xml` | `pytest tests -q` (CI: Run tests) |

`NON_VISUAL_SEMANTIC_RETENTION = 0.00` means non-visual prompts carry zero semantic content into the encoded output — they are deterministically collapsed to a bounded fallback form. `NON_VISUAL_ALIAS = 1.00` means every out-of-scope prompt maps to the same bounded canonical form.

> Source: `proofs/artifacts/mental_release_matrix.json`; `validation/results/pytest.xml`; `proofs/manifests/CURRENT_FALSIFICATION_PACKET.md`

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

## Commercial Readiness
ZPE-Mental is useful now and improving continuously. The bounded scope is deliberate: the product ships what it can prove, and proof coverage will expand as the corpus and techniques mature. `STAGED` reflects that this surface is public and adopter-ready within its bounded scope — not that it is incomplete or hedged.

| Field | Value |
|-------|-------|
| Verdict | STAGED |
| Commit SHA | db4a57fefe28 |
| Source | proofs/artifacts/mental_release_matrix.json |

## Tests and Verification
| Code | Check | Verdict |
|------|-------|---------|
| V_01 | `python -m pip install ".[test]"` builds and imports the Rust-native extension as `zpe_mental._native` | PASS |
| V_02 | `python scripts/validate_release.py --output proofs/artifacts/mental_release_matrix.json` reproduces the bounded release metrics | PASS |
| V_03 | `pytest tests -q --junitxml validation/results/pytest.xml` passes the public regression suite | PASS |

## Proof Anchors
| Path | State |
|------|-------|
| `proofs/manifests/CURRENT_FALSIFICATION_PACKET.md` | VERIFIED |
| `proofs/artifacts/mental_release_matrix.json` | VERIFIED |
| `validation/results/pytest.xml` | VERIFIED |
| `docs/BOUNDED_RELEASE_SCOPE.md` | VERIFIED |

## Comp Benchmarks

ZPE-Mental encodes endogenous visual forms (tunnels, spirals, lattices, cobwebs per the Klüver form taxonomy). This modality has no incumbent codec market and no established external encoder/decoder. The closest domain — fractal geometry encoders or L-system compressors — generate fractals from rules rather than encode/decode closed perceptual forms; the comparison would be apples-to-oranges. The lane's product claim is exact roundtrip fidelity on the bounded form corpus (FORM_EXACT = 1.00), not byte compression.

## Repo Shape
| Field | Value |
|-------|-------|
| Proof Anchors | 4 |
| Modality Lanes | 1 |
| Authority Source | `proofs/artifacts/mental_release_matrix.json` |
| Public State | bounded adopter |
| Native Backend | Rust via `zpe_mental._native` |

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
