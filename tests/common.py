from __future__ import annotations

from pathlib import Path
from typing import Iterable, Sequence, Tuple


ROOT = Path(__file__).resolve().parents[1]


def flatten_points(polylines: Iterable[Sequence[Tuple[float, float]]]) -> list[Tuple[float, float]]:
    out: list[Tuple[float, float]] = []
    for poly in polylines:
        for pt in poly:
            out.append((float(pt[0]), float(pt[1])))
    return out


def stroke_signature(stroke) -> tuple[int, int, int, int, int | None, int, tuple[tuple[str, int, int] | tuple[str, int], ...]]:
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
