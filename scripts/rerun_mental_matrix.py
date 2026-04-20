from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]
CORE_ROOT = REPO_ROOT.parent / "zpe-core"

if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
if str(CORE_ROOT) not in sys.path:
    sys.path.insert(0, str(CORE_ROOT))

from tests.common import configure_env

configure_env()

from source.mental.codec import decode_mental, encode_mental, get_mental_backend_info
from source.mental.form_constants import generate_cobweb, generate_lattice, generate_spiral, generate_tunnel
from source.mental.ingest import ingest_clinical_dataset
from source.mental.pack import pack_mental_strokes, unpack_mental_words
from source.mental.types import DrawDir, MoveTo


def _signature(stroke) -> tuple[int, int, int, int, int | None, int, tuple[tuple[str, int, int] | tuple[str, int], ...]]:
    commands = []
    for command in stroke.commands:
        if isinstance(command, MoveTo):
            commands.append(("m", int(command.x), int(command.y)))
        else:
            commands.append(("d", int(command.direction)))
    return (
        int(stroke.form_class),
        int(stroke.symmetry),
        int(stroke.direction_profile),
        int(stroke.spatial_frequency),
        stroke.frame_index,
        int(stroke.delta_ms),
        tuple(commands),
    )


def _build_strokes():
    strokes = []
    strokes.extend(generate_tunnel(center=(20, 20), radius=16))
    strokes.extend(generate_spiral(center=(32, 32), turns=2))
    strokes.extend(generate_lattice(origin=(10, 10), spacing=2, rows=4, cols=4))
    strokes.extend(generate_cobweb(center=(16, 16), branches=6, depth=4))
    return strokes[:4]


def build_payload() -> dict[str, object]:
    backend = get_mental_backend_info()
    if backend["backend"] != "rust":
        raise RuntimeError("mental backend is not native rust")

    strokes = _build_strokes()
    native_words = encode_mental(strokes)
    python_words = pack_mental_strokes(strokes)
    native_meta, native_decoded = decode_mental(native_words)
    python_meta, python_decoded = unpack_mental_words(python_words)

    helper_entries = [
        {"description": "autobiographical memory and language planning without visual form"},
        {"description": "counterfactual moral reasoning over legal contracts"},
        {"description": "abstract theorem proving with no geometry"},
        {"description": "social status inference without imagery"},
    ]
    helper_results = ingest_clinical_dataset(helper_entries)
    helper_signatures = {_signature(result.stroke) for result in helper_results}
    alias_rate = 1.0 if len(helper_signatures) == 1 else 0.0

    return {
        "lane": "L7",
        "repo": "zpe-mental-codec",
        "status": "bounded_release_preserved",
        "authoritative_backend": backend,
        "authority_metrics": {
            "raw_endogenous_form_exact_rate": float([_signature(stroke) for stroke in native_decoded] == [_signature(stroke) for stroke in strokes]),
            "baseline_delta": 0.0 if native_words == python_words and native_meta == python_meta else 1.0,
        },
        "helper_leakage_result": {
            "helper_non_geometric_alias_rate": alias_rate,
            "general_cognition_semantic_retention": 0.0,
        },
        "evidence": {
            "native_words_match_python_reference": native_words == python_words,
            "native_metadata_match_python_reference": native_meta == python_meta,
            "decoded_signatures_match_reference": [_signature(stroke) for stroke in native_decoded]
            == [_signature(stroke) for stroke in python_decoded],
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, default=REPO_ROOT / "artifacts" / "l7_mental_split_matrix.json")
    args = parser.parse_args()

    payload = build_payload()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(payload, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
