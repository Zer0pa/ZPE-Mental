from __future__ import annotations

import importlib.util
from pathlib import Path
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))


def _has_native_mental_module() -> bool:
    for name in ("zpe_mental_codec.zpe_mental_codec", "zpe_mental_codec"):
        try:
            if importlib.util.find_spec(name) is not None:
                return True
        except ModuleNotFoundError:
            continue
    return False


if not _has_native_mental_module():
    pytest.skip("zpe-mental-codec wheel is not installed", allow_module_level=True)

from tests.common import configure_env

configure_env()

from source.mental.codec import decode_mental, encode_mental, get_mental_backend_info
from source.mental.pack import pack_mental_strokes, unpack_mental_words
from source.mental.types import DirectionProfile, DrawDir, FormClass, MentalStroke, MoveTo, SymmetryOrder


def _stroke(profile: DirectionProfile, delta_ms: int) -> MentalStroke:
    directions = [0, 2, 4, 6] if profile == DirectionProfile.COMPASS_8 else [0, 2, 4, 6, 8, 10]
    return MentalStroke(
        commands=[MoveTo(10, 10)] + [DrawDir(direction, profile=profile) for direction in directions],
        form_class=FormClass.LATTICE,
        symmetry=SymmetryOrder.D6 if profile == DirectionProfile.D6_12 else SymmetryOrder.D4,
        direction_profile=profile,
        spatial_frequency=4,
        drift_speed=1,
        frame_index=3,
        delta_ms=delta_ms,
    )


def _signature(stroke: MentalStroke) -> tuple[int, int, int, int, int | None, int, tuple[tuple[str, int, int] | tuple[str, int], ...]]:
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


def test_mental_native_backend_reports_rust() -> None:
    info = get_mental_backend_info()

    assert info["backend"] == "rust"
    assert info["native"] is True
    assert info["fallback_used"] is False
    assert info["module_file"]


def test_mental_native_matches_python_reference_words_and_decode() -> None:
    strokes = [_stroke(DirectionProfile.COMPASS_8, 20), _stroke(DirectionProfile.D6_12, 40)]

    native_words = encode_mental(strokes)
    python_words = pack_mental_strokes(strokes)
    native_meta, native_decoded = decode_mental(native_words)
    python_meta, python_decoded = unpack_mental_words(python_words)

    assert native_words == python_words
    assert native_meta == python_meta
    assert [_signature(stroke) for stroke in native_decoded] == [_signature(stroke) for stroke in python_decoded]
