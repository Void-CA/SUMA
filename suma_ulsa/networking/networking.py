"""
Networking module bindings
"""
try:
    # Import from Rust extension
    from ..suma_ulsa.networking import *
except ImportError:
    # Fallback for type checkers
    pass

__all__ = [
    "FLSMCalculator",
    "SubnetRow",
    "VLSMCalculator",
    "compress_ipv6",
    "expand_ipv6",
]