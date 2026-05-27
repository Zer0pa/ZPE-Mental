# ZPE-Mental

> Product-page mirror for `/encoding/ZPE-Mental/`.
> Live public repo: [Zer0pa/ZPE-Mental](https://github.com/Zer0pa/ZPE-Mental).
> GitHub Markdown cannot reproduce the website typography, CSS, JavaScript, scroll behavior, or live bento layout; this README translates the product page into GitHub-safe Markdown evidence blocks.

## 0. Install / Developer Commands

The product page is the positioning authority. This section is the only retained developer-surface material from the previous root README.

```bash
> Source: `proofs/artifacts/mental_release_matrix.json`; `validation/results/pytest.xml`; `proofs/manifests/CURRENT_FALSIFICATION_PACKET.md
python -m pip install --upgrade pip
python -m pip install ".[test]"
pytest tests -q --junitxml validation/results/pytest.xml
```

## Product Page Mirror

**Product-page title:** ZPE-Mental · Tunnel · spiral · lattice · cobweb · Zer0pa

**Product-page description:** ZPE-Mental · non-clinical codec for four endogenous visual-form examples · byte-exact across Rust and Python · 20-bit wire format · PyPI 0.1.0 stale-pending

### Hero Translation

> 00 · ZPE-MENTAL · VISUAL FORM CODECLIVE LANE · 032114Z Tracing mental images for a codec. Four-form endogenous-visual codec · ZPE-Mental · PyPI zpe-mental v0.1.0 · github.com/Zer0pa/ZPE-Mental Perceptual science has described tunnel, spiral, lattice, and cobweb for nearly a century — the four Klüver form constants people see in the mind's eye — but no one had ever pinned them to an exact, replayable shape. ZPE-Mental writes each form into a 20-bit packet and brings it back byte-identical across a Rust-native fast path and a Python reference. Non-visual prompts collapse to a bounded fallback. No cognition, clinical, diagnostic, or autobiographical claim is made — just the four forms, replayed exactly.

## Positioning

| Field | Value |
| --- | --- |
| Section | encoding |
| Product route | /encoding/ZPE-Mental/ |
| Live public repository | https://github.com/Zer0pa/ZPE-Mental |
| Repo identity used here | ZPE-Mental |
| Website display identity | ZPE-Mental |
| Verdict | STAGED |
| Posture | always_in_beta |
| Headline metric | FORM_EXACT: 1.00. ZPE-Mental canonical authority surface; useful now, improving continuously. |
| Honest blocker | Broad cognition decoding or any general decoding of non-visual mental content.; Recovery of autobiographical memory, legal or moral reasoning, theorem proving, or language planning.; Clinical diagnosis, therapeutic interpretation, or any medical use. |
| Mechanics asset from product page | MENTAL.gif |

## Key Metrics

| Metric | Value | Baseline |
| --- | --- | --- |
| FORM_EXACT | 1.00 | ref |
| BASELINE_DELTA | 0.00 | py-ref |
| NON_VISUAL_ALIAS | 1.00 | bounded |
| NON_VISUAL_SEMANTIC_RETENTION | 0.00 | bounded |

## Proof Anchors

| Path | State |
| --- | --- |
| proofs/manifests/CURRENT_FALSIFICATION_PACKET.md | VERIFIED |
| proofs/artifacts/mental_release_matrix.json | VERIFIED |
| validation/results/pytest.xml | VERIFIED |
| docs/BOUNDED_RELEASE_SCOPE.md | VERIFIED |

## What We Prove

- Exact round-trip preservation for the bounded endogenous visual form corpus in `validation/corpora/endogenous_forms.json` (4 forms: tunnel, spiral, lattice, cobweb).
- Parity between the Rust-native encoder/decoder and the pure-Python reference implementation on the same bounded corpus (`BASELINE_DELTA = 0.00`; byte-level word equality asserted in CI).
- Stable profile and symmetry handling for both `COMPASS_8` (8 directions at 45° increments) and `D6_12` (12 directions at 30° increments, enabling exact D6 rotational symmetry) direction surfaces included in the release corpus.
- Deterministic collapse of non-visual prompts to a bounded fallback form instead of broad-cognition retention (`NON_VISUAL_SEMANTIC_RETENTION = 0.00`; `NON_VISUAL_ALIAS = 1.00`).
- Compact 20-bit wire format supporting raw and RLE direction encodings, per-stroke frame-index and delta-time metadata, all within the same word space. RLE is the default transport (Augmentation Phase 2). Proof: `zpe_mental/pack.py`, `src/lib.rs`.

## What We Do Not Claim

- Broad cognition decoding or any general decoding of non-visual mental content.
- Recovery of autobiographical memory, legal or moral reasoning, theorem proving, or language planning.
- Clinical diagnosis, therapeutic interpretation, or any medical use.
- A broader shared stack or any decoding coverage outside endogenous visual forms.

## Blockers / Failures

> Broad cognition decoding or any general decoding of non-visual mental content.; Recovery of autobiographical memory, legal or moral reasoning, theorem proving, or language planning.; Clinical diagnosis, therapeutic interpretation, or any medical use.

## Verification Surface

| Code | Check | Verdict |
| --- | --- | --- |
| V_01 | `python -m pip install ".[test]"` builds and imports the Rust-native extension as `zpe_mental._native` | PASS |
| V_02 | `python scripts/validate_release.py --output proofs/artifacts/mental_release_matrix.json` reproduces the bounded release metrics | PASS |
| V_03 | `pytest tests -q --junitxml validation/results/pytest.xml` passes the public regression suite | PASS |

## License

| Field | Value |
| --- | --- |
| License | SAL-7.0 |
| Authority source | proofs/artifacts/mental_release_matrix.json |

## Upcoming Workstreams

| Category | Summary |
| --- | --- |
| Zero-Base Scientific Thinking — GPD Research and Planning Pending | Klüver-form codec — market and admission path. Lane has no incumbent codec market for endogenous visual forms. Active research into market definition and certified-subset admission path. |
| Active Engineering | Continue current authority-packet refinement on ZPE-Mental; surface new receipts as they land. |

## Related Repos

No related repos are declared on the product page frontmatter.

<details>
<summary>Full Visible Product-Page Bento Translation</summary>

This section preserves the product page cells as Markdown text blocks. It intentionally omits shared site navigation, footer chrome, CSS, and scripts.

### Bento Cell 1

> 00 · ZPE-MENTAL · VISUAL FORM CODECLIVE LANE · 032114Z Tracing mental images for a codec. Four-form endogenous-visual codec · ZPE-Mental · PyPI zpe-mental v0.1.0 · github.com/Zer0pa/ZPE-Mental Perceptual science has described tunnel, spiral, lattice, and cobweb for nearly a century — the four Klüver form constants people see in the mind's eye — but no one had ever pinned them to an exact, replayable shape. ZPE-Mental writes each form into a 20-bit packet and brings it back byte-identical across a Rust-native fast path and a Python reference. Non-visual prompts collapse to a bounded fallback. No cognition, clinical, diagnostic, or autobiographical claim is made — just the four forms, replayed exactly.

### Bento Cell 2

> 01 · THE GAPDESCRIBED, NOT CAPTURED These forms have been described in words for a century. None captured exactly.

### Bento Cell 3

> 02 · MARKETSADJACENT FORECASTS NIH BRAIN Initiative FY24$0.75B/yr Perceptual/cognitive neuroscience software ’30est. $0.8B Open-science replication infrastructure ’28est. $0.3B Psychedelic research clinical trials ’30est. $0.5B Computational neuroscience tooling ’30est. $1.1B Adjacent research-infrastructure estimates only; ZPE-Mental is a small replay tool for four named visual forms, not a market claim.

### Bento Cell 4

> 03 · VALUE $0.75B/yr NIH BRAIN Initiative annual funding; ZPE-Mental serves perceptual research as a four-form fixture tool.

### Bento Cell 5

> 04 · INSIGHT Fix the form. Tunnel, spiral, lattice, cobweb — the same shape every time.

### Bento Cell 6

> 05.1 · CURRENT TECHDESCRIBED, NOT PINNED Klüver named these four forms in 1928, and perceptual science has studied them ever since. Yet every reference depends on prose, sketches, or rendered images that drift between researchers, devices, and decades. The form itself was never standardised.

### Bento Cell 7

> 05.2 · OUR TECHPIN THE FORM ZPE-Mental writes each of the four forms into a 20-bit packet with profile and symmetry metadata, then returns the exact same bytes whether it runs in Rust or in Python. Non-visual prompts collapse to a documented fallback rather than producing a plausible-looking form. Two profiles — COMPASS_8 and D6_12 — are represented; coverage stays narrow on purpose.

### Bento Cell 8

> 05.3 · BENCHMARKSFOUR-FORM CORPUS FORM_EXACT1.00fidelity Rust ↔ Python0.00bytes Δ Pytest3/3PASS PyPI0.1.0stale tunnel1.00 spiral1.00 lattice1.00 Scope: four named forms only · 20-bit packet · cobweb 1.00

### Bento Cell 9

> 06 · MEASUREMENTCORPUS PARITY CHECK Each form measured against a known reference, replayed identically every run.

### Bento Cell 10

> 06.1 · COMPARATIVE PERFORMANCEFOUR-FORM FORM_EXACT Tunnel1.00 Spiral1.00 Lattice1.00 Cobweb1.00 FORM_EXACT fidelity · ZPE-Mental 0.1.0 · validation/corpora/endogenous_forms.json · Rust ↔ Python byte equality (0-byte delta); four named forms only, no external clinical comparator.

### Bento Cell 11

> 07 · KEY METRICSFOUR-FORM AGGREGATE

### Bento Cell 12

> 07.1 · FORM FIDELITY 1.00 FORM_EXACT · four named forms

### Bento Cell 13

> 07.2 · RUST ↔ PYTHON 0.00B Byte delta · fast path = reference

### Bento Cell 14

> 07.3 · PYTEST SUITE 3 / 3 PASS · v0.1.0 commit e3412beb

### Bento Cell 15

> 07.4 · PYPI STATE 0.1.0 CONNECTED · STALE PENDING 0.1.1

### Bento Cell 16

> 07.5 · WIRE FORMAT 20-bit Four fixtures · two profiles represented

### Bento Cell 17

> 08 · DETERMINISMBYTE-EXACT FOUR-FORM REPLAY Tunnel, spiral, lattice, cobweb —byte-exact across Rust and Python.

### Bento Cell 18

> 08.1 · WHAT DETERMINISTIC MEANSFOUR-FORM SURFACE Each of the four named forms — tunnel, spiral, lattice, cobweb — encodes into a 20-bit wire format and decodes to byte-identical bytes across the Rust-native fast path and the Python reference. FORM_EXACT = 1.00 is the falsification check. Non-visual prompts do not silently produce a plausible form: they collapse to a documented fallback (NON_VISUAL_SEMANTIC_RETENTION 0.00, NON_VISUAL_ALIAS 1.00) so that any non-visual input is recoverable as such. The COMPASS_8 and D6_12 profiles are represented, not exhaustive.

### Bento Cell 19

> 08.2 · THE FIDELITY GAP Honest Blocker · No cognition decoding, autobiographical recovery, language understanding, theorem proving, legal or moral reasoning, clinical diagnosis, therapy, medical-device, regulatory, mental-health-assessment, or prosthetics use. The release corpus is exactly four endogenous visual-form examples — tunnel, spiral, lattice, cobweb. No all-forms/all-profiles coverage; PyPI remains 0.1.0 stale pending 0.1.1 metadata release.

### Bento Cell 20

> 09 FOUR FORMS, ONE CITABLE SHAPE.

### Bento Cell 21

> 09.1 · THE AMBITION ZPE-Mental gives perceptual science its first deterministic packet for the Klüver form constants. Tunnel, spiral, lattice, and cobweb become stable fixtures a psychophysics lab can specify, share, and replay without arguing over rendering or wording — a small, exact contribution to a question that has resisted exact treatment since 1928.

### Bento Cell 22

> 09.2 · WHAT WORKS NOWEXTERNAL Working today: four named forms at FORM_EXACT 1.00; Rust and Python decode identically; three of three tests pass.

### Bento Cell 23

> 09.3 · WHAT'S STILL OPENEXTERNAL Still open: broader form coverage, full profile range, and the 0.1.1 metadata release that retires the stale PyPI 0.1.0.

### Bento Cell 24

> 09.4 · REPLICATION · NEAR-TERM (12–24 MO) Perceptual experiments get a shared vocabulary A psychophysics group that ran a tunnel-stimulus study in 2024 can hand a 20-bit packet to a 2026 replication team and know both labs are testing the exact same shape — not a screenshot, not a verbal description.

### Bento Cell 25

> 09.5 · STIMULUS LIBRARIES · NEAR-TERM (12–24 MO) Stimulus libraries get a fidelity floor A perception lab maintaining a long-running stimulus library no longer drifts as workstations, GPUs, and rendering toolchains turn over. Tunnel, spiral, lattice, and cobweb resolve to identical bytes across devices and years, so longitudinal studies stop fighting their own equipment.

### Bento Cell 26

> 09.6 · PSYCHEDELIC RESEARCH · MID-TERM (24–48 MO) Clinical trials use a controlled stimulus vocabulary Researchers studying psychedelic-induced visual experience cite Klüver form constants constantly but compare them with prose. A four-form packet vocabulary lets multi-site trials, IRB submissions, and published results refer to the same stimulus identity without ambiguity.

### Bento Cell 27

> 09.7 · BOUNDARY DISCIPLINE · MID-TERM (24–48 MO) Replication studies acquire an audit instrument Because non-visual prompts collapse to a documented fallback rather than producing a plausible form, a reviewer can probe a published endogenous-visual claim against a bounded reference. Replication science gains a small, exact tool for separating claimed visual content from prompt artefact.

### Bento Cell 28

> 09.8 · CITABLE FORMS · PARADIGM (48 MO+) Perceptual forms become citable objects A spiral or lattice referenced in a paper carries an identifier the next reader can retrieve and verify, the way DOIs anchor text. Perceptual research gains the citation infrastructure that text and data already enjoy — a foundation a century of Klüver scholarship has lacked.

</details>

---

Source mapping: product route `/encoding/ZPE-Mental/` -> live public repo `Zer0pa/ZPE-Mental`. README generated from product-page authority plus retained install/dev commands only.
