# Bounded Release Scope

Date: 2026-04-20
Release class: bounded adopter
Scope: endogenous visual forms only

## Honest Claim

This release covers one narrow surface: endogenous visual forms represented as bounded geometric trajectories with explicit profile and symmetry metadata.

The release claim is limited to:

- exact round-trip preservation for the bounded form corpus committed in `validation/corpora/endogenous_forms.json`
- exact parity between the Rust-native encoder/decoder and the in-repo Python reference path on that corpus
- explicit exclusion of broad cognition, general semantic reasoning, and non-visual mental content

This is not a general cognition codec and not a general decoder of non-visual mental content.

## Authoritative Backend

- Public package: `zpe_mental`
- Native module: `zpe_mental._native`
- Python reference path: `zpe_mental.pack`
- Verification entry point: `scripts/validate_release.py`

## Bounded Pass Surface

Retained on the authoritative path:

- endogenous geometric form
- shape identity
- direction profile identity
- symmetry identity
- frame and delta metadata

Verified release metrics:

- `form_exact_rate = 1.0`
- `baseline_delta = 0.0`
- `non_visual_prompt_alias_rate = 1.0`
- `non_visual_semantic_retention_rate = 0.0`

## Explicit Non-Claims

This release does not claim authoritative preservation of:

- autobiographical memory
- language planning
- moral or legal reasoning
- broad abstract cognition
- any broader decoding surface outside endogenous visual forms

## Reproducibility Artifact

- Corpus: `validation/corpora/endogenous_forms.json`
- Out-of-scope prompts: `validation/corpora/non_visual_prompts.json`
- Release artifact: `proofs/artifacts/mental_release_matrix.json`
- Regression suite: `tests/`

Minimal reproduction:

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

## Release Judgement

This repo is prepared for fresh falsification as a bounded endogenous visual form encoder with an explicit non-claim for broad cognition. Any wider cognition claim would exceed the current evidence surface.
