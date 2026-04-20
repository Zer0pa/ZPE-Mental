# L7 Mental Bounded Release Scope

Date: 2026-04-20
Lane: `L7`
Release class: bounded adopter
Backend: Rust authoritative path with Python regression fallback

## Honest Claim

This release ships the mental codec only for bounded endogenous visual forms on the live authoritative path.

The release claim is:

- exact preservation of bounded endogenous geometric form
- exact preservation of profile and symmetry invariants for the frozen endogenous-form object class
- explicit non-closure for broad non-geometric cognition

This is not a general cognition codec.

## Authoritative Backend

- Rust crate: `v0.0/code/rust/zpe_mental_codec`
- Live wrapper: `v0.0/code/zpe_multimodal/mental/codec.py`
- Source mirror wrapper: `v0.0/code/source/mental/codec.py`

When the native module is installed, the authoritative packing and unpacking path is executed in Rust. The Python implementation remains as regression control only.

## Bounded Pass Surface

Retained on the authoritative path:

- endogenous geometric form
- shape identity
- profile identity
- symmetry identity

Machine-validated authority metrics:

- `raw_shape_exact_rate = 1.0`
- `raw_profile_exact_rate = 1.0`
- `raw_form_exact_rate = 1.0`
- `raw_symmetry_exact_rate = 1.0`
- `metric_structure_rate = 1.0`
- `topological_structure_rate = 1.0`
- `graph_relational_structure_rate = 1.0`
- `baseline_delta = 0.0`

## Explicit Non-Claims

This release does not claim authoritative preservation of:

- autobiographical memory
- language planning
- moral/legal reasoning
- broad abstract cognition
- any helper-ingest-only cognition surface

Those cases remain authentic negatives on the authoritative path:

- `general_cognition_semantic_retention = 0.0`
- `helper_non_geometric_alias_rate = 1.0`

This release therefore preserves the narrow-scope truth and refuses broad-cognition inflation.

## Reproducibility Artifact

- Artifact: `v0.0/proofs/artifacts/geogram3/geogram3_native_falsifier_matrix.json`
- Native test: `v0.0/code/tests/test_mental_native_optional.py`

Minimal reproduction:

```bash
source .venv-gf/bin/activate
python -m maturin build --release --manifest-path v0.0/code/rust/zpe_mental_codec/Cargo.toml --interpreter "$(which python)" --out /tmp/geogram3-dist
python -m pip install --force-reinstall /tmp/geogram3-dist/zpe_mental_codec-*.whl
pytest v0.0/code/tests/test_mental_native_optional.py v0.0/code/tests/test_sensation_regression.py v0.0/code/tests_phase3/mental/test_mental_augmented.py -q
PYTHONPATH=v0.0/code python v0.0/code/scripts/geogram3_native_falsifier_matrix.py --output v0.0/proofs/artifacts/geogram3/geogram3_native_falsifier_matrix.json
```

## Release Judgement

`L7` is publishable now as a bounded endogenous-visual-form codec with an explicit authentic negative for broad cognition. Any wider cognitive claim would be false.
