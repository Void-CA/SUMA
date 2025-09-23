pub fn generate_truth_table(variables: Vec<String>) -> Vec<Vec<bool>> {
    let num_vars = variables.len();
    let num_rows = 1 << num_vars; // 2^num_vars
    let mut table = Vec::new();

    for i in 0..num_rows {
        let mut row = Vec::new();
        for j in (0..num_vars).rev() {
            let value = (i & (1 << j)) != 0;
            row.push(value);
        }
        table.push(row);
    }
    table
}