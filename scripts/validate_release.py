from __future__ import annotations

import argparse
import json
from pathlib import Path

from zpe_mental import decode_mental, encode_mental, get_mental_backend_info
from zpe_mental.ingest import map_prompt_dataset
from zpe_mental.pack import pack_mental_strokes, unpack_mental_words
from zpe_mental.types import DirectionProfile, DrawDir, FormClass, MentalStroke, MoveTo, SymmetryOrder


REPO_ROOT = Path(__file__).resolve().parents[1]
CORPORA_ROOT = REPO_ROOT / "validation" / "corpora"


def _load_json(path: Path) -> object:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def _build_stroke(spec: dict[str, object]) -> MentalStroke:
    profile = DirectionProfile[str(spec["direction_profile"])]
    commands = [MoveTo(int(spec["move"][0]), int(spec["move"][1]))]
    commands.extend(DrawDir(int(direction), profile=profile) for direction in spec["directions"])
    return MentalStroke(
        commands=commands,
        form_class=FormClass[str(spec["form_class"])],
        symmetry=SymmetryOrder[str(spec["symmetry"])],
        direction_profile=profile,
        spatial_frequency=int(spec["spatial_frequency"]),
        drift_speed=int(spec["drift_speed"]),
        frame_index=spec["frame_index"],
        delta_ms=int(spec["delta_ms"]),
    )


def _stroke_signature(stroke: MentalStroke) -> tuple[int, int, int, int, int | None, int, tuple[tuple[str, int, int] | tuple[str, int], ...]]:
    commands = []
    for command in stroke.commands:
        if hasattr(command, "x") and hasattr(command, "y") and not hasattr(command, "direction"):
            commands.append(("m", int(command.x), int(command.y)))
            continue
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


def build_payload() -> dict[str, object]:
    endogenous_specs = _load_json(CORPORA_ROOT / "endogenous_forms.json")
    non_visual_prompts = _load_json(CORPORA_ROOT / "non_visual_prompts.json")

    strokes = [_build_stroke(spec) for spec in endogenous_specs["strokes"]]
    native_words = encode_mental(strokes)
    python_words = pack_mental_strokes(strokes)
    native_meta, native_decoded = decode_mental(native_words)
    python_meta, python_decoded = unpack_mental_words(python_words)

    out_of_scope_results = map_prompt_dataset(non_visual_prompts["entries"])
    out_of_scope_signatures = {_stroke_signature(result.stroke) for result in out_of_scope_results}

    return {
        "repo": "zpe-mental",
        "release_scope": "bounded endogenous visual forms",
        "backend": get_mental_backend_info(),
        "inputs": {
            "endogenous_forms": "validation/corpora/endogenous_forms.json",
            "non_visual_prompts": "validation/corpora/non_visual_prompts.json",
        },
        "metrics": {
            "form_exact_rate": float(
                [_stroke_signature(stroke) for stroke in native_decoded]
                == [_stroke_signature(stroke) for stroke in strokes]
            ),
            "baseline_delta": 0.0 if native_words == python_words and native_meta == python_meta else 1.0,
            "non_visual_prompt_alias_rate": 1.0 if len(out_of_scope_signatures) == 1 else 0.0,
            "non_visual_semantic_retention_rate": 0.0,
        },
        "checks": {
            "native_words_match_reference": native_words == python_words,
            "native_metadata_match_reference": native_meta == python_meta,
            "decoded_signatures_match_reference": [_stroke_signature(stroke) for stroke in native_decoded]
            == [_stroke_signature(stroke) for stroke in python_decoded],
            "native_backend_active": get_mental_backend_info()["backend"] == "rust",
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--output",
        type=Path,
        default=REPO_ROOT / "proofs" / "artifacts" / "mental_release_matrix.json",
    )
    args = parser.parse_args()

    payload = build_payload()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(payload, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
