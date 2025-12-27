# 🌐 SUMA — Sistema Unificacion de Métodos Académicos

> **Ecosistema modular para modelar, aprender y ejecutar métodos académicos de forma rigurosa, declarativa y reproducible.**

SUMA es un ecosistema académico abierto, compuesto por:

**Core en Rust** (algoritmos rigurosos) • **Módulos independientes** • **Bindings para Python** • **Lenguaje declarativo (Codex)**

</div>

---

## 🌱 Filosofía

SUMA existe para tres propósitos fundamentales:

* **✔ Uso académico:** Para enseñar estructuras, probabilidad, IA clásica, optimización, simulación y finanzas con resultados reproducibles y ejecutables.
* **✔ Uso investigativo:** Para implementar modelos desde cero, compararlos y estudiar métodos con transparencia total.
* **✔ Uso práctico:** Para que desarrolladores integren módulos de SUMA en sus proyectos (Rust, Python, Tauri, Web), manteniendo el rigor matemático y algoritmos eficientes.

---

## 🧩 Arquitectura del Ecosistema

```text
       ┌─────────────────────────────┐
       │            SUMA             │
       │     Ecosistema Completo     │
       └──────────────┬──────────────┘
                      │
     ┌────────────────▼──────────────────┐
     │              Core                 │
     │          (Rust crate)             │
     │   Estructuras, modelos, lógica    │
     └──────────────┬────────────────────┘
                    │
       ┌────────────▼──────────────┐
       │         Bindings          │
       │   Python · WebAssembly    │
       └────────────┬──────────────┘
                    │
     ┌──────────────▼──────────────┐
     │        Módulos SUMA         │
     │ boolean_algebra/            │
     │ data_structures/            │
     │ finance/                    │
     │ numerics/                   │
     └──────────────────────────────┘

## 🔢 Módulo Destacado: Álgebra Booleana

El módulo de álgebra booleana es actualmente el más completo y sirve como referencia del diseño del ecosistema: seguro, rápido, expresivo y con una API unificada.

### Características Principales

- Evaluación de expresiones complejas.
- Generación automática de tablas de verdad.
- Simplificación de expresiones.
- Verificación de tautologías, contradicciones y equivalencias.
- Exportación de datos: CSV, JSON, Polars, diccionarios y listas.

## 🧠 Operadores Soportados

| Operador | Palabras Clave (Python) | Símbolos |
|----------|--------------------------|----------|
| AND | and, & | ∧ |
| OR | or, \| | ∨ |
| NOT | not, ~ | ¬ |
| XOR | xor, ^ | ⊕ |
| IMPLICA | implies, => | → |
| EQUIVALENCIA | iff, <=> | ↔ |

## 🛠️ API Esencial

La clase BooleanExpr expone los siguientes métodos clave:

- evaluate(vars)
- truth_table()
- is_tautology() / is_contradiction()
- simplify()
- equivalent(other)
- to_normal_form()
- to_dnf(), to_cnf()

## ⚡ Instalación y Uso

⚠️ Nota: SUMA está en evolución activa. Algunos módulos pueden ser experimentales.

Instala el paquete oficial de Python:

```bash
pip install suma_ulsa
```

## 🚀 Ejemplo Rápido

```python
from suma_ulsa.boolean_algebra import BooleanExpr

# Definir una expresión
expr = BooleanExpr("(A and B) or (not C)")

# Evaluar con variables específicas
print(expr.evaluate({'A': True, 'B': False, 'C': True}))

# Generar y exportar tabla de verdad
tabla = expr.truth_table()
print(tabla.to_csv())

# Verificar propiedades lógicas
print("¿Tautología?", expr.is_tautology())
```

## 🔬 Roadmap del Ecosistema

| Módulo | Estado | Descripción |
|--------|--------|-------------|
| Core | 🟦 En desarrollo | Base en Rust, estructuras, análisis, runtime del lenguaje. |
| Boolean Algebra | 🟩 Completado | Expresiones, tablas, simplificación. |
| Data Structures | 🟧 En desarrollo | Árboles, grafos, recorridos, dependencias. |
| Numerical Methods | 🟨 Planeado | Ecuaciones, integración, derivación. |
| Finance | 🟨 Planeado | TVM (Valor del dinero en el tiempo), préstamos, inversiones. |
| SUMA Codex (DSL) | 🔵 Diseño | Lenguaje declarativo orientado a modelos. |
| SUMA CLI + REPL | 🔵 Diseño | Ejecución interactiva y consultas de modelos. |

## 👨‍💻 Desarrollo

Si deseas contribuir o ejecutar el proyecto desde la fuente:

```bash
# Clonar el repositorio
git clone https://github.com/Void-CA/suma.git
cd suma

# Instalar en modo editable
pip install -e .

# Ejecutar pruebas
cargo test  # Para el core en Rust
pytest      # Para los bindings de Python
```

## 📂 Estructura del Proyecto

```
suma/
├── src/
│   ├── lib.rs
│   ├── core/               # Motor central en Rust
│   ├── modules/
│   │   └── boolean_algebra/
│   └── bindings/           # Python, WASM
├── suma_ulsa/              # Paquete Python oficial
│   ├── boolean_algebra/
│   └── ...
```

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Las prioridades actuales son:

- Implementación del Codex (lenguaje SUMA).
- Extensiones del core.
- Nuevos módulos académicos.
- Ejemplos, documentación y benchmarks reproducibles.

## 📄 Licencia

MIT License — ver el archivo LICENSE para más detalles.

SUMA: diseñado por estudiantes, para estudiantes, para aprender de manera rigurosa, clara y reproducible.


---

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Áreas prioritarias:

- Nuevos módulos académicos
- Mejoras en álgebra booleana (Karnaugh, Quine-McCluskey)
- Documentación y ejemplos
- Tests y validación

---

## 📊 Estructura del Proyecto (resumen)

```
suma/
├── src/                   
│   ├── lib.rs
│   ├── core/    # Módulo core codificado en Rust
│   |   ├── boolean_algebra/    # Módulo de álgebra booleana
│   |   ├── ...
│   └── bindings/             # Bindings Python
├── suma_ulsa/                   # Paquete Python
│   ├── __init__.py
│   └── boolean_algebra/

```

---

## 📄 Licencia

MIT License — ver `LICENSE` para detalles.

---

_Desarrollado por estudiantes, para estudiantes, con dedicación y rigor académico._
