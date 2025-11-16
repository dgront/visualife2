# visualife/__init__.py

import sys as _sys

# This is the compiled Rust extension module
from . import visualife as _ext

# Re-export everything from the Rust extension at the package level
from .visualife import *   # ElementID, SvgDrawing, etc.

# Optionally mirror __all__ / __doc__ if you like:
__doc__ = getattr(_ext, "__doc__", None)
if hasattr(_ext, "__all__"):
    __all__ = list(_ext.__all__)  # make a copy so we can extend it
else:
    __all__ = []

# Expose submodules (these come from Rust via add_submodule)
heatmap = _ext.heatmap
styling = _ext.styling

# Make them real submodules for the import system
_sys.modules[__name__ + ".heatmap"] = heatmap
_sys.modules[__name__ + ".styling"] = styling

# If you want them in __all__ as well:
__all__ += ["heatmap", "styling"]

