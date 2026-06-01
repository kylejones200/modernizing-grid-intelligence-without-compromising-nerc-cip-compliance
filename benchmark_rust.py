#!/usr/bin/env python3
"""Python vs Rust kernel benchmark."""

from __future__ import annotations

import time
import sys
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "src"))
from compute_kernel import asset_risk_scores  # noqa: E402

def main() -> None:
    n = 5000
    age = np.ascontiguousarray(np.arange(n) % 40 + 1, dtype=float)
    criticality = np.ascontiguousarray((np.arange(n) % 10) * 0.1)
    exposure = np.ascontiguousarray((np.arange(n) % 7) * 0.12)
    t0 = time.perf_counter()
    for _ in range(200):
        asset_risk_scores(age, criticality, exposure)
    py_s = time.perf_counter() - t0
    try:
        import modernizing_grid_intelligence_without_compromising_nerc_cip_compliance_rs as rs
    except ImportError:
        print("Build: maturin develop --release -m rust/py/Cargo.toml")
        print(f"Python {py_s:.3f}s")
        return
    rs_s = rs.bench_kernel_py(age, criticality, exposure, 10000)
    print(f"Python {py_s:.3f}s Rust {rs_s:.3f}s speedup {py_s / max(rs_s, 1e-9):.1f}x")
    np.testing.assert_allclose(
        asset_risk_scores(age, criticality, exposure),
        np.asarray(rs.asset_risk_scores_py(age, criticality, exposure)),
        rtol=1e-10,
    )
    print("Correctness: OK")

if __name__ == "__main__":
    main()
