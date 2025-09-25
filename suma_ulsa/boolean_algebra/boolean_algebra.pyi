from typing import Dict, List, Optional, Tuple
from typing_extensions import Any
import polars as pl

class FullTruthTable:
    base_variables: List[str]
    step_labels: List[str]
    step_results: List[List[bool]]
    
    def __init__(self, base_variables: List[str], step_labels: List[str], step_results: List[List[bool]]) -> None: ...
    
    @property
    def base_combinations(self) -> List[List[bool]]: ...
    @property
    def final_results(self) -> List[bool]: ...
    
    def num_rows(self) -> int: ...
    def num_steps(self) -> int: ...
    def to_polars(self) -> pl.DataFrame: ...
    def to_pretty_string(self) -> str: ...
    def get_row(self, index: int) -> Optional[List[bool]]: ...
    def get_step_results(self, step_index: int) -> Optional[List[bool]]: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __len__(self) -> int: ...
    
class TruthTable:
    variables: List[str]
    combinations: List[List[bool]]
    results: List[bool]
    
    def to_polars(self) -> pl.DataFrame: ...
    def to_lazyframe(self) -> pl.LazyFrame: ...
    def to_dict(self) -> Dict[str, Any]: ...
    def to_list(self) -> List[Dict[str, bool]]: ...
    def filter_true(self) -> 'TruthTable': ...
    def summary(self) -> Dict[str, Any]: ...
    def to_string(self) -> str: ...
    def __len__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BooleanExpr:
    def __init__(self, expr: str) -> None:
        """
        Initialize a Boolean expression parser.
        
        Args:
            expr: String representing a boolean expression using variables,
                  AND, OR, NOT operations. Example: "(A AND B) OR NOT C"
                  
        Raises:
            ValueError: If the expression is empty, too complex, or contains invalid syntax
        """
        ...
    
    def evaluate(self, values: dict[str, bool]) -> bool:
        """
        Evaluate the boolean expression with the given variable values.
        
        Args:
            values: Dictionary mapping variable names to their boolean values.
            
        Returns:
            The result of evaluating the expression.
            
        Raises:
            ValueError: If any variable in the expression is missing from the values dict.
        """
        ...
    
    def evaluate_with_defaults(self, values: dict[str, bool], default: bool) -> bool:
        """
        Evaluate the boolean expression with default values for missing variables.
        
        Args:
            values: Dictionary mapping variable names to their boolean values.
            default: Default value to use for any variables missing from the values dict.
            
        Returns:
            The result of evaluating the expression.
        """
        ...
    
    def truth_table(self) -> list[tuple[dict[str, bool], bool]]:
        """
        Generate the complete truth table for the expression.
        
        Returns:
            List of tuples where each tuple contains:
            - A dictionary of variable values for the combination
            - The result of evaluating the expression for those values
        """
        ...

    def truth_table_pretty(self) -> str:
        """
        Generate a pretty-printed truth table for the expression.
        
        Returns:
            A string representation of the truth table.
        """
        ...
    
    def to_string(self) -> str:
        """Convert the expression to string (infix notation)."""
        ...
    
    def to_prefix_notation(self) -> str:
        """Convert the expression to prefix notation (for debugging)."""
        ...
    
    def complexity(self) -> int:
        """Get the complexity of the expression (number of operators)."""
        ...
    
    def is_tautology(self) -> bool:
        """Check if the expression is a tautology (always true)."""
        ...
    
    def is_contradiction(self) -> bool:
        """Check if the expression is a contradiction (always false)."""
        ...
    
    def equivalent_to(self, other: 'BooleanExpr') -> bool:
        """
        Check if two expressions are logically equivalent.
        
        Args:
            other: Another BooleanExpr to compare with.
            
        Returns:
            True if both expressions produce the same results for all variable combinations.
        """
        ...

    def complexity(self) -> int:
        """Get the complexity of the expression (number of operators)."""
        ...
    
    
    @property
    def variables(self) -> list[str]:
        """
        Get the list of unique variables used in the expression.
        
        Returns:
            List of variable names used in the expression, sorted alphabetically.
        """
        ...
    
    def __str__(self) -> str:
        """Return the string representation."""
        ...
    
    def __repr__(self) -> str:
        """Return the official string representation."""
        ...
    
    def __eq__(self, other: object) -> bool:
        """Check if two expressions are identical."""
        ...
    
    def __and__(self, other: 'BooleanExpr') -> 'BooleanExpr':
        """Return a new BooleanExpr representing the AND of this and another expression."""
        ...
    
    def __or__(self, other: 'BooleanExpr') -> 'BooleanExpr':
        """Return a new BooleanExpr representing the OR of this and another expression."""
        ...
    
    def __invert__(self) -> 'BooleanExpr':
        """Return a new BooleanExpr representing the NOT of this expression."""
        ...

def parse_expression_debug(expression: str) -> str:
    """
    Parse an expression and return its AST in prefix notation (for debugging).
    
    Args:
        expression: Boolean expression string to parse.
        
    Returns:
        The expression in prefix (Polish) notation.
        
    Raises:
        ValueError: If the expression is invalid.
    """
    ...

def truth_table_from_expr(variables: List[str], results: List[bool]) -> BooleanExpr:
    """
    Create a BooleanExpr from a truth table specification.
    
    Args:
        variables: List of variable names.
        results: List of boolean results for each combination (in standard binary order).
        
    Returns:
        A BooleanExpr that matches the specified truth table.
        
    Raises:
        ValueError: If the inputs are invalid.
    """
    ...