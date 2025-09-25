from typing import Dict, List, Optional
import polars as pl

class TruthTable:
    variables: List[str]
    combinations: List[List[bool]]
    columns: Dict[str, List[bool]]
    column_order: List[str]

    def __init__(self, variables: List[str], columns: Dict[str, List[bool]], column_order: List[str], combinations: List[List[bool]]) -> None:
        """
        Initialize a TruthTable.

        Args:
            variables: List of variable names.
            columns: Dictionary mapping column names to their boolean values.
            column_order: List of column names in display order.
            combinations: List of boolean combinations for each row.

        Raises:
            ValueError: If the inputs are invalid.
        """
        ...

    def to_polars(self) -> pl.DataFrame:
        """
        Convert the truth table to a Polars DataFrame.

        Returns:
            A Polars DataFrame representing the truth table.
        """
        ...

    def to_lazyframe(self) -> pl.LazyFrame:
        """
        Convert the truth table to a Polars LazyFrame.

        Returns:
            A Polars LazyFrame representing the truth table.
        """
        ...

    def to_column_dict(self) -> Dict[str, List[bool]]:
        """
        Convert the truth table to a dictionary of columns.

        Returns:
            Dictionary mapping column names to lists of boolean values.
        """
        ...

    def to_list(self) -> List[List[bool]]:
        """
        Convert the truth table to a list of boolean combinations.

        Returns:
            List of lists, where each inner list is a row of boolean values.
        """
        ...

    def to_named_rows(self) -> List[Dict[str, bool]]:
        """
        Convert the truth table to a list of dictionaries mapping column names to values.

        Returns:
            List of dictionaries, where each dictionary represents a row.
        """
        ...

    def get_row(self, index: int) -> Optional[Dict[str, bool]]:
        """
        Get a specific row by index.

        Args:
            index: The row index.

        Returns:
            A dictionary mapping column names to boolean values, or None if index is out of range.
        """
        ...

    def get_column(self, variable: str) -> Optional[List[bool]]:
        """
        Get the values for a specific column.

        Args:
            variable: The column name.

        Returns:
            A list of boolean values for the column, or None if the column does not exist.
        """
        ...

    def filter_true(self) -> 'TruthTable':
        """
        Filter rows where the last column is True.

        Returns:
            A new TruthTable containing only rows where the last column is True.

        Raises:
            ValueError: If the truth table has no columns.
        """
        ...

    def filter_false(self) -> 'TruthTable':
        """
        Filter rows where the last column is False.

        Returns:
            A new TruthTable containing only rows where the last column is False.

        Raises:
            ValueError: If the truth table has no columns.
        """
        ...

    def satisfiable_assignments(self, value: bool) -> List[Dict[str, bool]]:
        """
        Get assignments where the last column equals the specified value.

        Args:
            value: The boolean value to filter by (True or False).

        Returns:
            List of dictionaries mapping column names to boolean values.

        Raises:
            ValueError: If the truth table has no columns.
        """
        ...

    def select_columns(self, columns: List[str]) -> 'TruthTable':
        """
        Select specific columns to create a new TruthTable.

        Args:
            columns: List of column names to select.

        Returns:
            A new TruthTable containing only the specified columns.

        Raises:
            ValueError: If any specified column does not exist.
        """
        ...

    def filter(self, column: str, predicate: callable) -> 'TruthTable':
        """
        Filter rows based on a predicate applied to a specific column.

        Args:
            column: The column name to filter on.
            predicate: A callable that takes a boolean value and returns True/False.

        Returns:
            A new TruthTable containing only rows where the predicate is True.

        Raises:
            ValueError: If the column does not exist.
        """
        ...

    def equivalent_to(self, other: 'TruthTable') -> bool:
        """
        Check if this truth table is equivalent to another based on result columns.

        Args:
            other: Another TruthTable to compare with.

        Returns:
            True if the truth tables are equivalent, False otherwise.

        Raises:
            ValueError: If the comparison cannot be performed.
        """
        ...

    def to_csv(self) -> str:
        """
        Export the truth table to CSV format.

        Returns:
            A string containing the CSV representation.
        """
        ...

    def to_json(self) -> str:
        """
        Export the truth table to JSON format.

        Returns:
            A string containing the JSON representation.
        """
        ...

    def column_stats(self, column: str) -> Dict[str, float]:
        """
        Compute statistics for a specific column.

        Args:
            column: The column name.

        Returns:
            Dictionary with 'true_count', 'false_count', and 'true_percentage'.

        Raises:
            ValueError: If the column does not exist.
        """
        ...

    def summary(self) -> Dict[str, float]:
        """
        Generate a summary of the truth table.

        Returns:
            Dictionary containing:
            - num_variables: Number of variables
            - total_combinations: Number of rows
            - true_count: Number of True values in the result column
            - false_count: Number of False values in the result column
            - true_percentage: Percentage of True values in the result column
            - <variable>_true_count: Number of True values for each variable

        Raises:
            ValueError: If the truth table has no columns.
        """
        ...

    def to_pretty_string(self) -> str:
        """
        Generate a pretty-printed string representation of the truth table.

        Returns:
            A string representation of the truth table.
        """
        ...

    def __len__(self) -> int:
        """
        Get the number of rows in the truth table.

        Returns:
            The number of rows.
        """
        ...

    def __getitem__(self, index: int) -> Dict[str, bool]:
        """
        Get a row by index.

        Args:
            index: The row index.

        Returns:
            A dictionary mapping column names to boolean values.

        Raises:
            IndexError: If the index is out of range.
        """
        ...

    def __str__(self) -> str:
        """
        Return the string representation of the truth table.

        Returns:
            A pretty-printed string representation.
        """
        ...

    def __repr__(self) -> str:
        """
        Return the official string representation of the truth table.

        Returns:
            A string representation of the object.
        """
        ...

class BooleanExpr:
    def __init__(self, expr: str) -> None:
        """
        Initialize a Boolean expression parser.

        Args:
            expr: String representing a boolean expression using variables,
                  AND, OR, NOT operations. Example: "(A AND B) OR NOT C"

        Raises:
            ValueError: If the expression is empty, too complex, or contains invalid syntax.
        """
        ...

    def evaluate(self, values: Dict[str, bool]) -> bool:
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

    def evaluate_with_defaults(self, values: Dict[str, bool], default: bool) -> bool:
        """
        Evaluate the boolean expression with default values for missing variables.

        Args:
            values: Dictionary mapping variable names to their boolean values.
            default: Default value to use for any variables missing from the values dict.

        Returns:
            The result of evaluating the expression.
        """
        ...

    def truth_table(self) -> TruthTable:
        """
        Generate the truth table for the expression.

        Returns:
            A TruthTable object representing the truth table.
        """
        ...

    def full_truth_table(self) -> TruthTable:
        """
        Generate the complete truth table for the expression.

        Returns:
            A TruthTable object representing the complete truth table.
        """
        ...

    def to_prefix_notation(self) -> str:
        """
        Convert the expression to prefix notation (for debugging).

        Returns:
            The expression in prefix (Polish) notation.
        """
        ...

    def is_tautology(self) -> bool:
        """
        Check if the expression is a tautology (always true).

        Returns:
            True if the expression is a tautology, False otherwise.
        """
        ...

    def is_contradiction(self) -> bool:
        """
        Check if the expression is a contradiction (always false).

        Returns:
            True if the expression is a contradiction, False otherwise.
        """
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

    @property
    def variables(self) -> List[str]:
        """
        Get the list of unique variables used in the expression.

        Returns:
            List of variable names used in the expression, sorted alphabetically.
        """
        ...

    @property
    def complexity(self) -> int:
        """
        Get the complexity of the expression (number of operators).

        Returns:
            The number of operators in the expression.
        """
        ...

    def __str__(self) -> str:
        """
        Return the string representation of the expression.

        Returns:
            The expression in infix notation.
        """
        ...

    def __repr__(self) -> str:
        """
        Return the official string representation of the expression.

        Returns:
            A string representation of the object.
        """
        ...

    def __and__(self, other: 'BooleanExpr') -> 'BooleanExpr':
        """
        Return a new BooleanExpr representing the AND of this and another expression.

        Args:
            other: Another BooleanExpr to combine with.

        Returns:
            A new BooleanExpr representing the AND operation.
        """
        ...

    def __or__(self, other: 'BooleanExpr') -> 'BooleanExpr':
        """
        Return a new BooleanExpr representing the OR of this and another expression.

        Args:
            other: Another BooleanExpr to combine with.

        Returns:
            A new BooleanExpr representing the OR operation.
        """
        ...

    def __invert__(self) -> 'BooleanExpr':
        """
        Return a new BooleanExpr representing the NOT of this expression.

        Returns:
            A new BooleanExpr representing the NOT operation.
        """
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