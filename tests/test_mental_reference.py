from __future__ import annotations

from zpe_mental.codec import decode_mental, get_mental_backend_info
from zpe_mental.pack import pack_mental_strokes, unpack_mental_words
from zpe_mental.types import DirectionProfile, DrawDir, FormClass, MentalStroke, MoveTo, SymmetryOrder

from tests.common import stroke_signature


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


def test_python_reference_round_trip_matches_public_api() -> None:
    strokes = [_stroke(DirectionProfile.COMPASS_8, 20), _stroke(DirectionProfile.D6_12, 40)]

    words = pack_mental_strokes(strokes)
    reference_meta, reference_decoded = unpack_mental_words(words)
    api_meta, api_decoded = decode_mental(words)

    assert reference_meta == api_meta
    assert [stroke_signature(stroke) for stroke in reference_decoded] == [
        stroke_signature(stroke) for stroke in api_decoded
    ]

    info = get_mental_backend_info()
    assert info["backend"] in {"python", "rust"}
