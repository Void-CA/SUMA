use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum DecisionNode<T, A: Eq + std::hash::Hash> {
    Decision {
        outcome: T,
    },
    Question {
        question: String,
        branches: HashMap<A, Box<DecisionNode<T, A>>>,
    },
}

impl<T, A: Clone + Eq + std::hash::Hash> DecisionNode<T, A> {
    pub fn decision(outcome: T) -> Self {
        DecisionNode::Decision { outcome }
    }

    pub fn question(question: impl Into<String>, branches: HashMap<A, DecisionNode<T, A>>) -> Self {
        DecisionNode::Question {
            question: question.into(),
            branches: branches.into_iter().map(|(k, v)| (k, Box::new(v))).collect(),
        }
    }

    /// Evaluate the decision tree by following answers.
    /// Returns the outcome if a leaf is reached, or an error listing unanswered questions.
    pub fn evaluate(&self, answers: &HashMap<String, A>) -> Result<&T, String>
    where
        A: Display,
    {
        match self {
            DecisionNode::Decision { outcome } => Ok(outcome),
            DecisionNode::Question { question, branches } => {
                match answers.get(question.as_str()) {
                    Some(answer) => {
                        match branches.get(answer) {
                            Some(next) => next.evaluate(answers),
                            None => Err(format!("Invalid answer '{}' for question '{}'", answer, question)),
                        }
                    }
                    None => Err(format!("Question '{}' not answered", question)),
                }
            }
        }
    }

    /// Returns all question paths in the tree.
    pub fn questions(&self) -> Vec<String> {
        let mut result = Vec::new();
        self.collect_questions(&mut result);
        result
    }

    fn collect_questions(&self, acc: &mut Vec<String>) {
        match self {
            DecisionNode::Decision { .. } => {}
            DecisionNode::Question { question, branches } => {
                acc.push(question.clone());
                for (_, child) in branches {
                    child.collect_questions(acc);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_weather_tree() -> DecisionNode<&'static str, &'static str> {
        let mut sunny_branches = HashMap::new();
        sunny_branches.insert("yes", DecisionNode::decision("Play tennis"));
        sunny_branches.insert("no", DecisionNode::decision("Stay home"));

        let mut weather_branches = HashMap::new();
        weather_branches.insert("sunny", DecisionNode::question("Is it hot?", sunny_branches));
        weather_branches.insert("rainy", DecisionNode::decision("Stay home"));
        weather_branches.insert("cloudy", DecisionNode::decision("Go for a walk"));

        DecisionNode::question("What's the weather like?", weather_branches)
    }

    #[test]
    fn test_sunny_hot_play_tennis() {
        let tree = setup_weather_tree();
        let mut answers = HashMap::new();
        answers.insert("What's the weather like?".to_string(), "sunny");
        answers.insert("Is it hot?".to_string(), "yes");

        assert_eq!(tree.evaluate(&answers), Ok(&"Play tennis"));
    }

    #[test]
    fn test_sunny_not_hot_stay_home() {
        let tree = setup_weather_tree();
        let mut answers = HashMap::new();
        answers.insert("What's the weather like?".to_string(), "sunny");
        answers.insert("Is it hot?".to_string(), "no");

        assert_eq!(tree.evaluate(&answers), Ok(&"Stay home"));
    }

    #[test]
    fn test_rainy_stay_home() {
        let tree = setup_weather_tree();
        let mut answers = HashMap::new();
        answers.insert("What's the weather like?".to_string(), "rainy");

        assert_eq!(tree.evaluate(&answers), Ok(&"Stay home"));
    }

    #[test]
    fn test_missing_answer_returns_error() {
        let tree = setup_weather_tree();
        let answers = HashMap::new();

        assert!(tree.evaluate(&answers).is_err());
        assert!(tree.evaluate(&answers).unwrap_err().contains("not answered"));
    }

    #[test]
    fn test_invalid_answer_returns_error() {
        let tree = setup_weather_tree();
        let mut answers = HashMap::new();
        answers.insert("What's the weather like?".to_string(), "hurricane");

        assert!(tree.evaluate(&answers).is_err());
        assert!(tree.evaluate(&answers).unwrap_err().contains("Invalid"));
    }

    #[test]
    fn test_single_decision() {
        let tree = DecisionNode::<&str, &str>::decision("Always do this");
        let answers = HashMap::new();
        assert_eq!(tree.evaluate(&answers), Ok(&"Always do this"));
    }

    #[test]
    fn test_collect_questions() {
        let tree = setup_weather_tree();
        let questions = tree.questions();
        assert!(questions.contains(&"What's the weather like?".to_string()));
        assert!(questions.contains(&"Is it hot?".to_string()));
        assert_eq!(questions.len(), 2);
    }
}
