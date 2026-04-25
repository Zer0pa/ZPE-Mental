# Reproducibility

## Canonical Inputs

- `validation/corpora/endogenous_forms.json`
- `validation/corpora/non_visual_prompts.json`

## Golden-Bundle Hash

This value will be populated by the `receipt-bundle.yml` workflow in Wave 3.

## Verification Command

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

## Supported Runtimes

- Python 3.11+
- Python 3.12
- Rust toolchain available via `cargo` for building `zpe_mental._native`
