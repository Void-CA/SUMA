# Networking API Documentation

This document provides comprehensive documentation for the networking module, generated from the `networking.pyi` stub file.

## Aliases

- `IPAddress = str`
- `SubnetMask = str`
- `NetworkClass = str`
- `ExportFormat = str`  # "json", "csv", "yaml", "yml", "xml", "md", "markdown", "txt", "text", "xlsx", "excel"

## Classes

### SubnetRow

Representación de una fila de subred calculada.

#### Atributos

- `subred`: Índice de la subred (1-based) dentro del cálculo.
- `direccion_red`: Dirección de red en formato decimal punteado (p. ej. "192.168.1.0").
- `primera_ip`: Primera IP usable dentro de la subred.
- `ultima_ip`: Última IP usable dentro de la subred.
- `broadcast`: Dirección de broadcast de la subred.
- `hosts_per_net`: Número de hosts utilizables en esta subred.

#### Methods

##### `to_dict() -> Dict[str, Any]`

Devuelve un diccionario simple con claves para cada campo (útil desde Python).

##### `to_pretty_string() -> str`

Devuelve una cadena legible en una sola línea con un resumen de la fila.

##### `to_json() -> str`

Serializa la fila a una cadena JSON.

##### `to_csv() -> str`

Serializa la fila como una línea CSV (útil para añadir a archivos CSV).

##### `to_yaml() -> str`

Serializa la fila a un fragmento YAML.

##### `__str__() -> str`

Equivalente a `to_pretty_string` (representación amigable).

##### `__repr__() -> str`

### FLSMCalculator

Calculadora FLSM (Fixed-Length Subnet Mask).

Crea una instancia con una IP base y el número deseado de subredes de igual tamaño.
Expone utilidades para obtener filas estructuradas, tablas formateadas y exportar datos.

#### Ejemplo

```python
>>> calc = FLSMCalculator("192.168.1.0", 4)
>>> print(calc.summary())
```

#### Constructor

##### `__init__(self, ip: IPAddress, subnet_count: int) -> None`

#### Methods

##### `summary() -> str`

Devuelve un resumen legible (múltiples líneas) del cálculo.

##### `print_summary() -> None`

Imprime el resumen en stdout (conveniencia).

##### `subnets_table() -> str`

Devuelve una tabla monoespaciada (string) con todas las subredes calculadas.

##### `print_table() -> None`

Imprime la tabla de subredes en stdout (conveniencia).

##### `get_subnets() -> List[SubnetRow]`

Devuelve una lista de `SubnetRow` con los datos estructurados.

##### `get_subnet(subnet_number: int) -> SubnetRow`

Devuelve una sola fila de subred (índice 1-based). Levanta IndexError si el índice es inválido.

##### `to_dict() -> Dict[str, Any]`

Serializa el cálculo completo a un dict anidado (listo para JSON/YAML).

##### `to_json() -> str`

Devuelve la representación JSON del cálculo.

##### `to_csv() -> str`

Devuelve una representación CSV de las subredes (cabecera + filas).

##### `to_markdown() -> str`

Devuelve una tabla en formato Markdown representando las subredes.

##### `to_excel(path: str) -> None`

Escribe un archivo Excel (.xlsx) con la tabla de subredes en `path`.

##### `export_to_file(filename: str, format: ExportFormat) -> None`

Exporta los datos a un archivo en el formato solicitado.

Formatos soportados: json, csv, md, txt, xlsx.

##### `__str__() -> str`

##### `__repr__() -> str`

#### Properties

##### `base_ip -> IPAddress`

IP base original usada para calcular las subredes.

##### `base_cidr -> int`

##### `subnet_count -> int`

##### `network_class -> NetworkClass`

##### `new_cidr -> int`

##### `subnet_mask -> SubnetMask`

##### `subnet_size -> int`

##### `hosts_per_subnet -> int`

##### `utilization_percentage -> float`

##### `total_hosts -> int`

### VLSMCalculator

Calculadora VLSM (Variable-Length Subnet Mask).

Crea una instancia a partir de una red base y una lista de requisitos de hosts
por subred. La calculadora asigna subredes con el tamaño adecuado según
los requisitos y proporciona utilidades de visualización y exportación.

#### Ejemplo

```python
>>> calc = VLSMCalculator("10.0.0.0/8", [100, 50, 10])
>>> print(calc.summary())
```

#### Constructor

##### `__init__(self, ip: IPAddress, host_requirements: List[int]) -> None`

#### Methods

##### `summary() -> str`

Devuelve un resumen legible de la asignación VLSM (subredes, eficiencia, uso).

##### `print_summary() -> None`

Imprime el resumen en stdout (conveniencia).

##### `subnets_table() -> str`

Devuelve una tabla monoespaciada (string) con las subredes asignadas.

##### `print_table() -> None`

Imprime la tabla de subredes en stdout (conveniencia).

##### `get_subnets() -> List[SubnetRow]`

Devuelve la lista de subredes calculadas como objetos `SubnetRow`.

##### `get_subnet(subnet_number: int) -> SubnetRow`

Devuelve una subred concreta (índice 1-based). Levanta IndexError si no existe.

##### `to_dict() -> Dict[str, Any]`

Serializa el resultado a un diccionario anidado (fácil de convertir a JSON/YAML).

##### `to_json() -> str`

Devuelve la representación JSON del resultado.

##### `to_csv() -> str`

Devuelve una representación CSV de las subredes.

##### `to_markdown() -> str`

Devuelve una tabla en formato Markdown con la asignación de subredes.

##### `to_excel(path: str) -> None`

Escribe un archivo Excel (.xlsx) con la información de subredes en `path`.

##### `export_to_file(filename: str, format: ExportFormat) -> None`

Exporta los datos a un archivo en el formato pedido.

Formatos soportados: json, csv, md, txt, xlsx.

##### `__str__() -> str`

##### `__repr__() -> str`

#### Properties

##### `base_ip -> IPAddress`

Red base o dirección utilizada para el cálculo.

##### `base_cidr -> int`

##### `network_class -> NetworkClass`

##### `host_requirements -> List[int]`

Lista de requisitos de hosts usada para la asignación (orden original).

##### `efficiency -> float`

Porcentaje de eficiencia del empaquetamiento de hosts (menor desperdicio mejor).

##### `utilization_percentage -> float`

Porcentaje de utilización de los hosts asignados respecto al total disponible.

##### `total_hosts -> int`

Total de hosts disponibles en la red base (antes de subnetear).

##### `subnet_count -> int`

Número total de subredes generadas por la calculadora.

## Functions

### `compress_ipv6(ipv6_address: str) -> str`

Función para comprimir una dirección IPv6 de su forma larga a su forma corta.

### `expand_ipv6(ipv6_address: str) -> str`

Función para comprimir una dirección IPv6 expandida a su forma corta.