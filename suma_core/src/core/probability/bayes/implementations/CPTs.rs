use crate::core::probability::bayes::models::BN_base::CPTBase;

pub struct BinaryCPT {
    pub table: Vec<(Vec<bool>, f64)>,
}

impl CPTBase for BinaryCPT {
    fn get_probability(&self, parent_values: &[&str], value: &str) -> Option<f64> {
        let bool_values: Vec<bool> = parent_values.iter().map(|&v| v == "true").collect();
        let val_bool = value == "true";
        self.table
            .iter()
            .find(|(parents, _)| *parents == bool_values)
            .map(|(_, p)| if val_bool { *p } else { 1.0 - *p })
    }

    fn possible_values(&self) -> Vec<String> {
        vec!["true".into(), "false".into()]
    }

    fn parent_combinations(&self) -> Vec<Vec<String>> {
        self.table
            .iter()
            .map(|(parents, _)| parents.iter().map(|b| if *b { "true".into() } else { "false".into() }).collect())
            .collect()
    }
}
