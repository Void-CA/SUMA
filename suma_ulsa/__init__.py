# suma_ulsa/__init__.py
"""
Expose the compiled PyO3 extension and register its submodules in
sys.modules so both attribute access (suma_ulsa.boolean_algebra) and
`import suma_ulsa.boolean_algebra` work.

Behavior:
- Try relative import of the compiled extension (common with maturin).
- Fall back to importlib importing the top-level extension.
- For each public attribute on the extension that is a module-like object,
  insert it into sys.modules under the fully-qualified name and set it on
  the package namespace.
"""

from __future__ import annotations

import importlib
import sys
import types

try:
    # usual case: the compiled extension lives as suma_ulsa.suma_ulsa
    from . import suma_ulsa as _suma_ext 
except Exception:
    # fallback: try importing the extension as a top-level module
    _suma_ext = importlib.import_module("suma_ulsa") 


def _register_submodules():
    """Register submodules from the compiled extension into sys.modules.

    This makes `import suma_ulsa.submod` succeed and keeps attribute access
    working.
    """
    pkg_name = __name__
    for name in dir(_suma_ext):
        if name.startswith("_"):
            continue
        try:
            attr = getattr(_suma_ext, name)
        except Exception:
            continue
        # Accept real module objects or extension submodules (ModuleType)
        if isinstance(attr, types.ModuleType):
            fqname = f"{pkg_name}.{name}"
            # Inject into sys.modules if missing or not the same object
            if fqname not in sys.modules or sys.modules[fqname] is not attr:
                sys.modules[fqname] = attr
            # Ensure attribute visible on package
            if not hasattr(sys.modules[pkg_name], name):
                setattr(sys.modules[pkg_name], name, attr) 


# Register discovered submodules now
_register_submodules()


__all__ = [n for n in dir(_suma_ext) if not n.startswith("_")]


def __getattr__(name: str):
    """Dynamic attribute access: forward to the compiled extension."""
    if hasattr(_suma_ext, name):
        return getattr(_suma_ext, name)
    raise AttributeError(f"module {__name__} has no attribute {name}")


def __dir__():
    return sorted(list(globals().keys()) + [n for n in dir(_suma_ext) if not n.startswith("_")])
