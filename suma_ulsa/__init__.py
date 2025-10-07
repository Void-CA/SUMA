"""
SUMA ULSA - Sistema Unificado de Métodos Avanzados
"""
__version__ = "0.1.10"

try:
    from . import suma_ulsa as _native
except ImportError as e:
    raise ImportError(
        "No se pudo cargar el módulo nativo 'suma_ulsa'. "
        "Asegúrate de instalar el paquete compilado correctamente."
    ) from e

# Exporta todo del módulo nativo si lo tiene
if hasattr(_native, "__all__"):
    from .suma_ulsa import *
