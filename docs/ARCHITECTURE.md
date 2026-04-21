# Architecture

## Runtime Surface

The public package lives in `zpe_mental/`.

- `zpe_mental/codec.py` exposes the public encode/decode API and prefers the Rust-native extension when available.
- `zpe_mental/pack.py` is the pure-Python reference encoder/decoder used for parity checks.
- `zpe_mental/types.py` defines the bounded form types, direction profiles, and symmetry enums.
- `src/lib.rs` implements the Rust-native payload packer/unpacker exposed as `zpe_mental._native`.

## Verification Surface

- `validation/corpora/endogenous_forms.json` commits the bounded release corpus.
- `validation/corpora/non_visual_prompts.json` commits the out-of-scope prompt set used to confirm non-visual collapse.
- `scripts/validate_release.py` reproduces the release metrics from those corpora.
- `tests/` holds the public regression suite.

## Boundary Discipline

The repo is intentionally single-product and single-surface. It does not depend on sibling repos at runtime, and it does not claim any broader shared stack.
