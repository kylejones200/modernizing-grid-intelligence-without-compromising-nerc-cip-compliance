"""Composite asset risk score from age, criticality, and exposure vectors."""

from __future__ import annotations

import numpy as np


def asset_risk_scores(
    age: np.ndarray, criticality: np.ndarray, exposure: np.ndarray
) -> np.ndarray:
    a = np.asarray(age, dtype=float)
    c = np.asarray(criticality, dtype=float)
    e = np.asarray(exposure, dtype=float)
    return (a / 40.0) * 0.4 + c * 0.35 + e * 0.25
