```
// ==========================================
// DOMINIO: MATEMÁTICAS (Estricto)
// ==========================================

// Definición del Modelo
LinearSystem "Problema_Tarea_1" {
    coefficients: [4, 7; 2, 6]
    constants:    [10; 20]
}

// Consulta / Proyección
Analysis "Revision_Resultados" {
    target: "Problema_Tarea_1"
    calculate: [ determinant, solution, inverse ]
}

// ==========================================
// DOMINIO: INFRAESTRUCTURA (Descriptivo)
// ==========================================

// Definición
Subnet "Red_Principal" {
    cidr: "192.168.1.0/24"
    gateway: "192.168.1.1"
}

// Consulta
Inspect "Validacion_IPs" {
    target: "Red_Principal"
    show: [ range, netmask, broadcast ]
}

// ==========================================
// DOMINIO: OPTIMIZACIÓN (Expresivo)
// ==========================================

Optimization "Max_Produccion" {
    maximize: 5x + 3y
    subject_to {
        x + y <= 100
        x >= 0
    }
}

Audit "Analisis_Sensibilidad" {
    target: "Max_Produccion"
    check: [ shadow_prices ]
}
```