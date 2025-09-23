
<div align="center">
   <img src="https://img.shields.io/badge/SUMA-v0.1.0-blue.svg" alt="Versión" />
   <img src="https://img.shields.io/badge/build-passing-brightgreen.svg" alt="Build Status" />
   <img src="https://img.shields.io/badge/license-MIT-lightgrey.svg" alt="Licencia" />
</div>


# SUMA - Sistema Universitario de Métodos Académicos


<p align="center">
   <b>Librería académica para el aprendizaje y resolución rigurosa de métodos universitarios, con un enfoque práctico y accesible.</b>
</p>

---


SUMA es una librería académica orientada a estudiantes universitarios y autodidactas con conocimientos básicos de programación en Python. Permite:

- Resolver problemas y ejercicios de diversas materias (álgebra booleana, estructuras de datos, finanzas, entre otras).
- Verificar y validar resultados de ejercicios realizados manualmente.
- Comprender y aplicar métodos académicos mediante herramientas computacionales confiables.

---


## Objetivos

- Proveer un núcleo Rust eficiente y confiable para cálculos académicos
- Exponer funciones accesibles desde Python mediante bindings
- Mantener una arquitectura modular para facilitar la expansión de nuevas materias


## Secciones iniciales

| Materia              | Funcionalidades principales                |
|----------------------|-------------------------------------------|
| Álgebra Booleana     | Tablas de verdad, Mapas de Karnaugh, Simplificación de expresiones |
| (Próximamente)       | Estructuras de datos, Finanzas computacionales, Otros métodos universitarios |

---


## Instalación (en desarrollo)

```bash
pip install suma
```

---


## Ejemplo de uso rápido

```python
import suma

# Ejemplo: Generar una tabla de verdad
tabla = suma.booleana.tabla_verdad('A and B')
print(tabla)
```

---


## Contribuciones


Las contribuciones de la comunidad son bienvenidas. Si tienes sugerencias, mejoras o encuentras algún error, no dudes en abrir un issue o pull request.

---


## Licencia

Este proyecto está bajo la licencia MIT.

---


<div align="center">
   Desarrollado por estudiantes, para estudiantes, con dedicación y rigor académico.
</div>
