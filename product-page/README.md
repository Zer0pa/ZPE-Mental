# ZPE-Mental — Product Page Build

**Lane:** ZPE-Mental
**Portfolio:** encoding
**GitHub canonical:** [Zer0pa/ZPE-Mental](https://github.com/Zer0pa/ZPE-Mental)
**HF mirror:** [Zer0pa/ZPE-Mental-lane-state](https://huggingface.co/datasets/Zer0pa/ZPE-Mental-lane-state)
**Repo HEAD this build was authored against:** `e3412bebba75` (2026-05-03)
**PyPI release:** `zpe-mental v0.1.0` (SAL-7.1) — live 2026-05-04
**Page-build date:** 2026-05-10

---

## What is in this directory

| File | Role |
|---|---|
| `index.html` | The full product page. Built from the locked ZPE-XR prototype template (`/Users/Zer0pa/Desktop/xr-product-page-build/index.html`) with lane-specific substitutions for lane-owned cells (00 hero, 04 INSIGHT, 05.0/05.1/05.2, 06/06.1, 07.1–07.5, 08/08.1/08.2). |
| `product-page-full-1440.png` | Full-page render at 1440×… 2× device scale factor. |
| `product-page-top-crop.png` | Hero + 01/02/03 row crop. |
| `product-page-insight-benchmark-crop.png` | 04 INSIGHT + 05.0/05.1/05.2 + 06/06.1 crop. |
| `product-page-possibility-crop.png` | 09 block crop (external operator-pending placeholders). |
| `product-page-mobile-414.png` | Mobile 414px full-page render. |
| `product-page-audit.json` | Audit snapshot — Pretext state, console errors, live-element targets, external-placeholder count, stale-term sweep result. |

## Story arc (12 lane-owned cells)

- **00 Hero** — `Tunnel, spiral, lattice, cobweb — replayed exactly.` (§B verbatim 6-word headline). Eyebrow `A bounded four-form visual codec, byte-exact · ZPE-Mental` (mirrors fresh-XR WHAT-KIND-codec, BRAND-CHARACTER pattern). Lede names the wedge + the non-claim.
- **04 INSIGHT** — `Four geometric forms, byte-exact across Rust and Python.`
- **05.0 CURRENT TECH** — Klüver form constants studied phenomenologically; no codec contract.
- **05.1 OUR TECH** — 20-bit wire format + COMPASS_8 / D6_12 profile-symmetry metadata + Rust/Python parity.
- **05.2 BENCHMARKS** — 4 mini-metrics {FORM_EXACT 1.00, Rust↔Python 0.00 B, Pytest 3/3, v0.1.0 SAL-7.1}, 3 mini-bars (tunnel focus + spiral + lattice all 1.00), mini-note surfaces cobweb 1.00 + parity 0.00 B + 20-bit word.
- **06 MEASUREMENT** — Every form pinned to a fixture, a parity check, and a wire-format gate.
- **06.1 COMPARATIVE** — 4 form rows all at 1.00 FORM_EXACT. Caption: `validation/corpora/endogenous_forms.json` · Rust ↔ Python byte equality (delta 0.00 B).
- **07.1–07.5 METRICS** — Form fidelity `1.00`, Rust↔Python `0.00 B`, Pytest `3/3 PASS`, PyPI `v0.1.0 SAL-7.1`, Wire format `20-bit (COMPASS_8 / D6_12)`.
- **08 DETERMINISM** — Tunnel, spiral, lattice, cobweb — byte-exact across Rust and Python.
- **08.1 WHAT DETERMINISTIC MEANS** — 4 named Klüver forms → 20-bit wire → byte-identical decode + bounded fallback + FORM_EXACT 1.00 gate.
- **08.2 FIDELITY GAP** — Non-claim boundary verbatim from §A: no cognition, no autobiographical memory recovery, no language understanding/theorem-proving/language-planning, no clinical diagnosis/therapeutic-interpretation/medical use. Release corpus is exactly four forms.

## What is operator-pending (external cells — lane agent does NOT author)

- **Cell #2 hero diagram** — pure-black 572×534 placeholder per §C.4 PLACEHOLDER status. Lane agent must NOT generate substitute.
- **01 GAP / 02 MARKETS / 03 VALUE** — external operator-supplied content. Structural h2/p placeholders match XR rhythm.
- **09 / 09.1 / 09.2 / 09.3 / 09.4–09.8** — entire 09 block external operator-supplied unlocks. Structural placeholders match XR rhythm.

## Voice

Current-reality (§0.5). Mental is a clean lane: FORM_EXACT 1.00 across the bounded four-form corpus, no failed gate, no comparator-beats-us. Mental does NOT inherit XR's failure-first voice — voice follows truth, not template.

## Truth basis

- `front-door-forensics-2026-05-04/reports/ZPE-Mental.md:13-51,65`
- `front-door-forensics-2026-05-04/falsification/ZPE-Mental.md:14-28`
- GitHub `Zer0pa/ZPE-Mental` HEAD `e3412beb` (2026-05-03)
- PyPI `zpe-mental v0.1.0` SAL-7.1 (confirmed live 2026-05-04 in same session as build; HTTP/2 200 on `https://pypi.org/pypi/zpe-mental/json`; `license_expression: LicenseRef-Zer0pa-SAL-7.1`)
- `FPO_DISPATCH_BRIEF_PRODUCT_PAGE_2026-05-09.md` §A Slot 28 + §B Slot 28 + §C Slot 28 + §C.0 + §C.1 + §C.3 + §C.4 hero diagram inventory + §C.5 mobile fold

## Reproduction (after Mac wipe)

1. Clone `Zer0pa/ZPE-Mental` at `e3412beb` or later.
2. Clone `Zer0pa/ZPE-XR` product-page-build (or equivalent locked template).
3. Apply lane-content substitutions documented in this README's "Story arc" section, against the rigid XR template's lane-owned cell anchors.
4. Verify: Pretext active, 0 console errors, 1 live-green target (LIVE LANE stamp), all external slots placeheld, no stale labels from another lane.

## Receipt

This build cleared all §C.6 visual-QA gates at 2026-05-10 21:10 SAST. Re-render any time from the `index.html` via `python3 -m http.server 8767` in this directory + a 1440px desktop + 414px mobile browser session.
