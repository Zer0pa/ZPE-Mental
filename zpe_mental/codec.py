from __future__ import annotations

import importlib.machinery
import importlib.util
from pathlib import Path
import sys
from typing import Iterable, List, Optional, Tuple

from .pack import pack_mental_strokes, unpack_mental_words
from .types import MentalStroke


PACKAGE_ROOT = Path(__file__).resolve().parent


def _load_installed_native_module():
    module_name = "zpe_mental._native"
    for entry in sys.path:
        try:
            entry_path = Path(entry).resolve()
        except OSError:
            continue
        candidate_dir = entry_path / "zpe_mental"
        if candidate_dir == PACKAGE_ROOT or not candidate_dir.is_dir():
            continue
        for suffix in importlib.machinery.EXTENSION_SUFFIXES:
            for candidate in candidate_dir.glob(f"_native*{suffix}"):
                spec = importlib.util.spec_from_file_location(module_name, candidate)
                if spec is None or spec.loader is None:
                    continue
                module = importlib.util.module_from_spec(spec)
                sys.modules[module_name] = module
                spec.loader.exec_module(module)
                return module
    return None


try:
    from . import _native
except ImportError:  # pragma: no cover - exercised via native-optional test gate.
    _native = _load_installed_native_module()


def _normalize_metadata(metadata: dict | None) -> dict | None:
    if metadata is None:
        return None
    normalized = dict(metadata)
    delta_ms = normalized.get("delta_ms")
    if isinstance(delta_ms, (bytes, bytearray)):
        normalized["delta_ms"] = [int(value) for value in delta_ms]
    return normalized


def _stroke_to_payload(stroke: MentalStroke) -> dict[str, int | list[int] | None]:
    move_x = 0
    move_y = 0
    directions: list[int] = []
    for command in stroke.commands:
        if hasattr(command, "x") and hasattr(command, "y") and not hasattr(command, "direction"):
            move_x = int(command.x)
            move_y = int(command.y)
            continue
        if hasattr(command, "direction"):
            directions.append(int(command.direction))
            continue
        raise TypeError(f"unsupported stroke command: {command!r}")
    return {
        "move_x": move_x,
        "move_y": move_y,
        "directions": directions,
        "form_class": int(stroke.form_class),
        "symmetry": int(stroke.symmetry),
        "direction_profile": int(stroke.direction_profile),
        "spatial_frequency": int(stroke.spatial_frequency),
        "drift_speed": int(stroke.drift_speed),
        "frame_index": stroke.frame_index,
        "delta_ms": int(stroke.delta_ms),
    }


def _payload_to_stroke(payload: dict) -> MentalStroke:
    from .types import DirectionProfile, DrawDir, FormClass, MoveTo, SymmetryOrder

    commands = [MoveTo(int(payload["move_x"]), int(payload["move_y"]))]
    profile = DirectionProfile(int(payload["direction_profile"]))
    commands.extend(
        DrawDir(int(direction), profile=profile) for direction in payload["directions"]
    )
    return MentalStroke(
        commands=commands,
        form_class=FormClass(int(payload["form_class"])),
        symmetry=SymmetryOrder(int(payload["symmetry"])),
        direction_profile=profile,
        spatial_frequency=int(payload["spatial_frequency"]),
        drift_speed=int(payload["drift_speed"]),
        frame_index=payload.get("frame_index"),
        delta_ms=int(payload.get("delta_ms", 0)),
    )


def encode_mental(
    strokes: Iterable[MentalStroke], metadata: Optional[dict] = None
) -> List[int]:
    """High-level encode helper for mental modality streams."""

    if _native is not None:
        payloads = [_stroke_to_payload(stroke) for stroke in strokes]
        return list(_native.pack_mental_strokes_payload(payloads, metadata))
    return pack_mental_strokes(strokes, metadata=metadata)


def decode_mental(words: Iterable[int]) -> Tuple[dict | None, List[MentalStroke]]:
    """High-level decode helper for mental modality streams."""

    if _native is not None:
        metadata, payloads = _native.unpack_mental_words_payload(list(words))
        return _normalize_metadata(dict(metadata)), [_payload_to_stroke(payload) for payload in payloads]
    return unpack_mental_words(words)


def get_mental_backend_info() -> dict[str, object]:
    if _native is None:
        return {
            "backend": "python",
            "native": False,
            "fallback_used": True,
            "module_name": "zpe_mental.pack",
            "crate_name": None,
            "version": None,
        }
    return dict(_native.backend_info())
