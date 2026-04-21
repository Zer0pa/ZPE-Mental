from __future__ import annotations

import pytest

from tests.common import stroke_signature
from zpe_mental.codec import decode_mental, encode_mental, get_mental_backend_info
from zpe_mental.pack import pack_mental_strokes, unpack_mental_words
from zpe_mental.types import DirectionProfile, DrawDir, FormClass, MentalStroke, MoveTo, SymmetryOrder


if get_mental_backend_info()["backend"] != "rust":
    pytest.skip("zpe_mental native extension is not installed", allow_module_level=True)


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


def test_mental_native_backend_reports_rust() -> None:
    info = get_mental_backend_info()

    assert info["backend"] == "rust"
    assert info["native"] is True
    assert info["fallback_used"] is False
    assert info["module_name"] == "zpe_mental._native"


def test_mental_native_matches_python_reference_words_and_decode() -> None:
    strokes = [_stroke(DirectionProfile.COMPASS_8, 20), _stroke(DirectionProfile.D6_12, 40)]

    native_words = encode_mental(strokes)
    python_words = pack_mental_strokes(strokes)
    native_meta, native_decoded = decode_mental(native_words)
    python_meta, python_decoded = unpack_mental_words(python_words)

    assert native_words == python_words
    assert native_meta == python_meta
    assert [stroke_signature(stroke) for stroke in native_decoded] == [
        stroke_signature(stroke) for stroke in python_decoded
    ]
