# Current Falsification Packet

This packet defines the bounded local falsification surface for the current repo state.

## Governing files

- `proofs/artifacts/mental_release_matrix.json`
- `validation/results/pytest.xml`
- `docs/BOUNDED_RELEASE_SCOPE.md`

## Governing claim under test

ZPE-Mental exactly round-trips the committed endogenous visual form corpus and keeps broad cognition explicitly out of scope.

## Governing metrics

- `FORM_EXACT = 1.00`
- `BASELINE_DELTA = 0.00`
- `NON_VISUAL_ALIAS = 1.00`
- `PYTEST_PASS = 3/3`

## Required reproduction

```bash
python -m pip install ".[test]"
python scripts/validate_release.py --output proofs/artifacts/mental_release_matrix.json
pytest tests -q --junitxml validation/results/pytest.xml
```
