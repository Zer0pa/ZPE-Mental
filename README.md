# ZPE-Mental

## What This Is
ZPE-Mental is a bounded encoding product for endogenous visual forms: tunnels, spirals, lattices, cobweb-like geometry, and the profile and symmetry metadata needed to round-trip those forms exactly. It ships a public Python API backed by a Rust-native fast path, with a pure-Python reference path kept in-repo for parity checks.

The current repo surface is prepared for exact bounded-form replay on the corpus committed here and for fresh falsification from a clean outsider shell. It does not claim broad cognition decoding, language understanding, autobiographical memory recovery, or general decoding of non-visual mental content.

| Field | Value |
|-------|-------|
| Architecture | VISUAL_FORM_STREAM |
| Encoding | MENTAL_ENDOGENOUS_FORM_V1 |

## Key Metrics
| Metric | Value | Baseline |
|-------|-------|----------|
| FORM_EXACT | 1.00 | ref |
| BASELINE_DELTA | 0.00 | py-ref |
| NON_VISUAL_ALIAS | 1.00 | bounded |
| PYTEST_PASS | 3/3 | pytest |

> Source: `proofs/artifacts/mental_release_matrix.json`; `validation/results/pytest.xml`

## What We Prove
- Exact round-trip preservation for the bounded endogenous visual form corpus in `validation/corpora/endogenous_forms.json`.
- Parity between the Rust-native encoder/decoder and the pure-Python reference implementation on the same bounded corpus.
- Stable profile and symmetry handling for the local `DirectionProfile.COMPASS_8` and `DirectionProfile.D6_12` enums included in the release corpus.
- Deterministic collapse of non-visual prompts to a bounded fallback form instead of broad-cognition retention.

## What We Don't Claim
- Broad cognition decoding or any general decoding of non-visual mental content.
- Recovery of autobiographical memory, legal or moral reasoning, theorem proving, or language planning.
- Clinical diagnosis, therapeutic interpretation, or any medical use.
- A broader shared stack or any decoding coverage outside endogenous visual forms.

## Commercial Readiness
This release candidate is restamped to the verified source commit below.

| Field | Value |
|-------|-------|
| Verdict | STAGED |
| Commit SHA | 1fc027756da4 |
| Confidence | 100% |
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
