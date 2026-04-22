# ZPE-Mental Novelty Card

**Product:** ZPE-Mental
**Domain:** Bounded endogenous visual form encoding — tunnels, spirals, lattices, cobweb-like geometry; discrete direction-sequence strokes with symmetry and spatial-frequency metadata.
**What we sell:** Exact round-trip serialization of endogenous visual forms for research and archival use, with a Rust-native fast path and an in-repo pure-Python reference path for parity verification.

## Novel contributions

1. **Bounded endogenous visual form codec with explicit direction profiles** — A 20-bit word encoding scheme where each stroke is represented as a sequence of discrete direction steps under one of two enumerated profiles (COMPASS_8 = 8 directions at 45-degree increments; D6_12 = 12 directions at 30-degree increments), plus per-stroke form class, symmetry order, spatial frequency, and drift speed fields. The wire format supports raw, RLE, frame-indexed, and delta-time word variants, all within the same 20-bit word space. Code: [`zpe_mental/pack.py:1-460`](../../../../../../zpe_mental/pack.py), [`src/lib.rs:1-700`](../../../../../../src/lib.rs). Nearest prior art: general polyline or stroke serialization formats (e.g. SVG path data, InkML). What is genuinely new here: the combination of explicit symmetry order, direction-profile discriminant, and form-class enumeration in a compact 20-bit word enables exact lossless round-trip of endogenous form metadata at a granularity not found in general vector formats.

2. **Rust-native fast path with pure-Python reference parity guarantee** — The public API (`zpe_mental/codec.py`) prefers the compiled Rust extension (`zpe_mental._native` via PyO3/maturin) but falls back to the pure-Python reference implementation transparently. The in-repo regression suite asserts byte-level parity between the two paths on the bounded release corpus. Code: [`zpe_mental/codec.py:1-120`](../../../../../../zpe_mental/codec.py), [`zpe_mental/pack.py`](../../../../../../zpe_mental/pack.py), [`src/lib.rs`](../../../../../../src/lib.rs).

3. **Non-visual collapse contract** — Out-of-scope (non-visual) prompts are deterministically collapsed to a bounded fallback form rather than silently misencoded or rejected with an exception. This makes the encoding contract explicit and falsifiable. Code: [`zpe_mental/ingest.py:89-105`](../../../../../../zpe_mental/ingest.py), validated by [`validation/corpora/non_visual_prompts.json`](../../../../../../validation/corpora/non_visual_prompts.json).

## Standard techniques used (explicit, not novel)

- Run-length encoding (RLE) for repeated direction steps within a stroke word
- Delta encoding for per-stroke timing metadata (`delta_ms`)
- Frame indexing for multi-frame stroke sequences
- PyO3 / maturin for Python-Rust binding (standard toolchain)
- pytest for regression testing

## Compass-8 / 8-primitive architecture

NO — ZPE-Mental does NOT implement the Compass-8 portfolio-level architecture.

`DirectionProfile.COMPASS_8` (defined at [`zpe_mental/types.py:21-23`](../../../../../../zpe_mental/types.py)) is a 2-value internal enum discriminant distinguishing the 8-direction (45-degree) and 12-direction (30-degree) direction profiles used within the endogenous visual form encoding. It is a data-format selector internal to this codec, not a portfolio-level Compass-8 architecture claim. The LICENSE §7.8 confirms Compass-8 = NO for this product.

What this codec actually uses: a discrete direction-sequence stroke model with two enumerated direction profiles (8-step and 12-step), explicit symmetry orders, and a compact 20-bit word wire format. This is unrelated to the Compass-8 primitive architecture defined at the portfolio level.

## Open novelty questions for the license agent

- None. All claims are grounded in code citations above.
