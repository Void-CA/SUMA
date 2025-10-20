from typing import Any, Dict, List

# Aliases
IPAddress = str
SubnetMask = str
NetworkClass = str
ExportFormat = str  # "json", "csv", "yaml", "yml", "xml", "md", "markdown", "txt", "text", "xlsx", "excel"

class SubnetRow:
    """Representación de una fila de subred calculada.

    Atributos
    ---------
    subred:
        Índice de la subred (1-based) dentro del cálculo.
    direccion_red:
        Dirección de red en formato decimal punteado (p. ej. "192.168.1.0").
    primera_ip:
        Primera IP usable dentro de la subred.
    ultima_ip:
        Última IP usable dentro de la subred.
    broadcast:
        Dirección de broadcast de la subred.
    hosts_per_net:
        Número de hosts utilizables en esta subred.
    """
    subred: int
    direccion_red: str
    primera_ip: str
    ultima_ip: str
    broadcast: str
    hosts_per_net: int

    def to_dict(self) -> Dict[str, Any]:
        """Devuelve un diccionario simple con claves para cada campo (útil desde Python)."""
    def to_pretty_string(self) -> str:
        """Devuelve una cadena legible en una sola línea con un resumen de la fila."""
    def to_json(self) -> str:
        """Serializa la fila a una cadena JSON."""
    def to_csv(self) -> str:
        """Serializa la fila como una línea CSV (útil para añadir a archivos CSV)."""
    def to_yaml(self) -> str:
        """Serializa la fila a un fragmento YAML."""
    def __str__(self) -> str:
        """Equivalente a :meth:`to_pretty_string` (representación amigable)."""
    def __repr__(self) -> str: ...

class FLSMCalculator:
    """Calculadora FLSM (Fixed-Length Subnet Mask).

    Crea una instancia con una IP base y el número deseado de subredes de igual tamaño.
    Expone utilidades para obtener filas estructuradas, tablas formateadas y exportar datos.

    Ejemplo
    -------
    >>> calc = FLSMCalculator("192.168.1.0", 4)
    >>> print(calc.summary())
    """
    def __init__(self, ip: IPAddress, subnet_count: int) -> None: ...

    def summary(self) -> str:
        """Devuelve un resumen legible (múltiples líneas) del cálculo."""
    def print_summary(self) -> None:
        """Imprime el resumen por stdout (conveniencia)."""
    def subnets_table(self) -> str:
        """Devuelve una tabla monoespaciada (string) con todas las subredes calculadas."""
    def print_table(self) -> None:
        """Imprime la tabla por stdout (conveniencia)."""

    def get_subnets(self) -> List[SubnetRow]:
        """Devuelve una lista de :class:`SubnetRow` con los datos estructurados."""
    def get_subnet(self, subnet_number: int) -> SubnetRow:
        """Devuelve una sola fila de subred (índice 1-based). Levanta IndexError si el índice es inválido."""

    def to_dict(self) -> Dict[str, Any]:
        """Serializa el cálculo completo a un dict anidado (listo para JSON/YAML)."""
    def to_json(self) -> str:
        """Devuelve la representación JSON del cálculo."""
    def to_csv(self) -> str:
        """Devuelve una representación CSV de las subredes (cabecera + filas)."""
    def to_markdown(self) -> str:
        """Devuelve una tabla en formato Markdown representando las subredes."""
    def to_excel(self, path: str) -> None:
        """Escribe un archivo Excel (.xlsx) con la tabla de subredes en `path`."""
    def export_to_file(self, filename: str, format: ExportFormat) -> None:
        """Exporta los datos a un archivo en el formato solicitado.

        Formatos soportados: json, csv, md, txt, xlsx.
        """

    @property
    def base_ip(self) -> IPAddress:
        """IP base original usada para calcular las subredes."""
    @property
    def base_cidr(self) -> int: ...
    @property
    def subnet_count(self) -> int: ...
    @property
    def network_class(self) -> NetworkClass: ...
    @property
    def new_cidr(self) -> int: ...
    @property
    def subnet_mask(self) -> SubnetMask: ...
    @property
    def subnet_size(self) -> int: ...
    @property
    def hosts_per_subnet(self) -> int: ...
    @property
    def utilization_percentage(self) -> float: ...
    @property
    def total_hosts(self) -> int: ...

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...




class VLSMCalculator:
    """Calculadora VLSM (Variable-Length Subnet Mask).

    Crea una instancia a partir de una red base y una lista de requisitos de hosts
    por subred. La calculadora asigna subredes con el tamaño adecuado según
    los requisitos y proporciona utilidades de visualización y exportación.

    Ejemplo
    -------
    >>> calc = VLSMCalculator("10.0.0.0/8", [100, 50, 10])
    >>> print(calc.summary())
    """

    def __init__(self, ip: IPAddress, host_requirements: List[int]) -> None: ...

    def summary(self) -> str:
        """Devuelve un resumen legible de la asignación VLSM (subredes, eficiencia, uso)."""
    def print_summary(self) -> None:
        """Imprime el resumen en stdout (conveniencia)."""
    def subnets_table(self) -> str:
        """Devuelve una tabla monoespaciada (string) con las subredes asignadas."""
    def print_table(self) -> None:
        """Imprime la tabla de subredes en stdout (conveniencia)."""

    def get_subnets(self) -> List[SubnetRow]:
        """Devuelve la lista de subredes calculadas como objetos :class:`SubnetRow`."""
    def get_subnet(self, subnet_number: int) -> SubnetRow:
        """Devuelve una subred concreta (índice 1-based). Levanta IndexError si no existe."""

    def to_dict(self) -> Dict[str, Any]:
        """Serializa el resultado a un diccionario anidado (fácil de convertir a JSON/YAML)."""
    def to_json(self) -> str:
        """Devuelve la representación JSON del resultado."""
    def to_csv(self) -> str:
        """Devuelve una representación CSV de las subredes."""
    def to_markdown(self) -> str:
        """Devuelve una tabla en formato Markdown con la asignación de subredes."""
    def to_excel(self, path: str) -> None:
        """Escribe un archivo Excel (.xlsx) con la información de subredes en `path`."""
    def export_to_file(self, filename: str, format: ExportFormat) -> None:
        """Exporta los datos a un archivo en el formato pedido.

        Formatos soportados: json, csv, md, txt, xlsx.
        """

    @property
    def base_ip(self) -> IPAddress:
        """Red base o dirección utilizada para el cálculo."""

    @property
    def base_cidr(self) -> int: ...

    @property
    def network_class(self) -> NetworkClass: ...

    @property
    def host_requirements(self) -> List[int]:
        """Lista de requisitos de hosts usada para la asignación (orden original)."""

    @property
    def efficiency(self) -> float:
        """Porcentaje de eficiencia del empaquetamiento de hosts (menor desperdicio mejor)."""

    @property
    def utilization_percentage(self) -> float:
        """Porcentaje de utilización de los hosts asignados respecto al total disponible."""

    @property
    def total_hosts(self) -> int:
        """Total de hosts disponibles en la red base (antes de subnetear)."""

    @property
    def subnet_count(self) -> int:
        """Número total de subredes generadas por la calculadora."""

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
