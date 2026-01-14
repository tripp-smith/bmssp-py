"""BMSSP: Fast single-source shortest paths for grid and pipeline networks."""

from _bmssp import *  # noqa: F401,F403

from .graph import Graph
from .sssp import SSSPResult, multi_sink_costs, reconstruct_path, sssp

_bmssp_all = list(globals().get("__all__", []))

__all__ = list(
    dict.fromkeys(
        _bmssp_all
        + [
            "Graph",
            "sssp",
            "SSSPResult",
            "reconstruct_path",
            "multi_sink_costs",
        ]
    )
)

__version__ = "0.1.0"
