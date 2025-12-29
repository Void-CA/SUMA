#[macro_export]
macro_rules! matrix {
    // $(...);* -> Repite bloques separados por punto y coma (Filas)
    // $(...),* -> Repite elementos separados por comas (Columnas)
    ( $( $( $x:expr ),+ );* $(;)? ) => {
        {
            let mut data = Vec::new();
            let mut rows = 0;
            let mut cols = 0;
            let mut consistent = true;

            // Iteramos sobre las filas (meta-variable de repetición externa)
            $(
                rows += 1;
                let mut current_cols = 0;
                
                // Iteramos sobre las columnas (meta-variable interna)
                $(
                    data.push($x); 
                    current_cols += 1;
                )+

                if cols == 0 {
                    cols = current_cols;
                } else if cols != current_cols {
                    panic!("Error en matrix!: La fila {} tiene {} columnas, se esperaban {}.", 
                           rows, current_cols, cols);
                }
            )*

            // Retornamos la estructura construida
            // Nota: Asume que DenseMatrix está accesible o usa la ruta completa
            crate::linear_algebra::DenseMatrix::new(rows, cols, data)
        }
    };
}

#[macro_export]
macro_rules! zeros {
    ($rows:expr, $cols:expr) => {
        {
            let total_elements = $rows * $cols;
            let data = vec![0.0; total_elements];
            crate::linear_algebra::DenseMatrix::new($rows, $cols, data)
        }
    };
}