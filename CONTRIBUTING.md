# Contributing

## Scope Discipline

This repo covers endogenous visual forms only. Contributions must keep broad cognition, general decoding claims about non-visual mental content, and any broader-stack framing out of scope unless the authority surface is expanded in a reviewed future wave.

## Pull Requests

- keep changes atomic and easy to audit
- update proof anchors when a public claim changes
- do not remove adverse results or non-claims to flatter the surface
- do not add internal-only cross-links, private tooling references, or internal research vocabulary

## Verification

Before opening a pull request, run:

```bash
python -m pip install ".[test]"
python scripts/validate_release.py --output proofs/artifacts/mental_release_matrix.json
pytest tests -q --junitxml validation/results/pytest.xml
```
