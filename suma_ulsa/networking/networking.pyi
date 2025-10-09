"""
Subnet Calculator Module

This module provides classes and functions for IP subnet calculations,
including subnetting, network address calculation, and host range determination.
"""

from typing import Dict, List, Any, Optional

class SubnetCalculator:
    """
    A calculator for IP subnet operations.
    
    This class helps calculate subnet information including network addresses,
    broadcast addresses, and host ranges for a given IP and subnet quantity.
    
    Args:
        ip: The IP address in CIDR notation or with subnet mask (e.g., "192.168.1.0/24" or "192.168.1.0 255.255.255.0")
        subnet_quantity: The number of subnets to create
    
    Example:
        >>> calculator = SubnetCalculator("192.168.1.0/24", 4)
        >>> calculator.print_summary()
    """
    
    def __init__(self, ip: str, subnet_quantity: int) -> None: ...
    
    def summary(self) -> str:
        """
        Returns a formatted summary of the subnet calculation.
        
        Returns:
            A string containing the IP, class, subnet mask, new mask, net jump, and hosts per subnet.
            
        Example:
            >>> summary = calculator.summary()
            >>> print(summary)
        """
        ...
    
    def print_summary(self) -> None:
        """
        Prints the subnet calculation summary to stdout.
        
        This method displays a nicely formatted summary with box-drawing characters.
        
        Example:
            >>> calculator.print_summary()
            ┌──────────────────────────────────────────┐
            │           SUBNET CALCULATOR SUMMARY      │
            ├──────────────────────────────────────────┤
            │ IP:            192.168.1.0/24            │
            │ Class:         C                         │
            │ Subnet Mask:   255.255.255.0             │
            ...
        """
        ...
    
    def table(self, show_binary: Optional[bool] = None) -> str:
        """
        Returns a formatted table showing all subnet information.
        
        Args:
            show_binary: If True, includes binary representation of IP addresses. Defaults to False.
            
        Returns:
            A string containing a formatted table with subnet details.
            
        Example:
            >>> table = calculator.table()
            >>> print(table)
            ┌─────────────────────────────────────────────────────────────────┐
            │                         SUBNET TABLE                            │
            ├───────┬─────────────────┬─────────────────┬─────────────────┬───┤
            │Subnet │ Network Address │   First Host    │    Last Host    │...│
            ...
        """
        ...
    
    def print_table(self, show_binary: Optional[bool] = None) -> None:
        """
        Prints the subnet table to stdout.
        
        Args:
            show_binary: If True, includes binary representation of IP addresses. Defaults to False.
            
        Example:
            >>> calculator.print_table()
            >>> calculator.print_table(show_binary=True)
        """
        ...
    
    def compact_table(self) -> str:
        """
        Returns a compact version of the subnet table.
        
        This provides a simpler, more concise table format without box-drawing characters.
        
        Returns:
            A string containing a compact table with subnet details.
            
        Example:
            >>> compact = calculator.compact_table()
            >>> print(compact)
            Subnet │ Network       │ First Host    │ Last Host     │ Broadcast
            ───────┼───────────────┼───────────────┼───────────────┼───────────────
            1      │ 192.168.1.0   │ 192.168.1.1   │ 192.168.1.62  │ 192.168.1.63
            ...
        """
        ...
    
    @property
    def original_ip(self) -> str:
        """The original IP address provided during initialization."""
        ...
    
    @property 
    def net_class(self) -> str:
        """The network class (A, B, C, etc.) of the IP address."""
        ...
    
    @property
    def subnet_mask(self) -> str:
        """The original subnet mask of the network."""
        ...
    
    @property
    def new_subnet_mask(self) -> str:
        """The new subnet mask after subnetting."""
        ...
    
    @property
    def net_jump(self) -> str:
        """The network jump between subnets."""
        ...
    
    @property
    def hosts_quantity(self) -> int:
        """The number of hosts per subnet."""
        ...
    
    @property
    def subnet_quantity(self) -> int:
        """The total number of subnets created."""
        ...
    
    def generate_rows(self) -> List['SubnetRow']:
        """
        Generates all subnet rows as SubnetRow objects.
        
        Returns:
            A list of SubnetRow objects containing detailed subnet information.
            
        Example:
            >>> rows = calculator.generate_rows()
            >>> for row in rows:
            ...     print(row.direccion_red)
        """
        ...
    
    def get_row(self, subnet_number: int) -> 'SubnetRow':
        """
        Gets a specific subnet row by subnet number.
        
        Args:
            subnet_number: The subnet number (starting from 1)
            
        Returns:
            A SubnetRow object for the specified subnet.
            
        Raises:
            IndexError: If subnet_number is out of range
            
        Example:
            >>> row = calculator.get_row(1)
            >>> print(row.direccion_red)
        """
        ...
    
    def to_dict(self) -> Dict[str, Any]:
        """
        Returns the calculator configuration as a dictionary.
        
        Returns:
            A dictionary containing the main configuration parameters.
            
        Example:
            >>> config = calculator.to_dict()
            >>> print(config['original_ip'])
        """
        ...
    
    def __str__(self) -> str:
        """
        Returns the summary string representation.
        
        This is called by the built-in str() function and print().
        
        Returns:
            The same string as summary() method.
        """
        ...
    
    def __repr__(self) -> str:
        """
        Returns the official string representation.
        
        This is called by the built-in repr() function.
        
        Returns:
            A string that could be used to recreate the object.
        """
        ...


class SubnetRow:
    """
    Represents a single subnet calculation result.
    
    This class contains all the information for a specific subnet including
    network address, host ranges, and broadcast address.
    
    Attributes:
        subred: The subnet number (starting from 1)
        direccion_red: The network address of the subnet
        primera_ip: The first usable host IP in the subnet
        ultima_ip: The last usable host IP in the subnet  
        broadcast: The broadcast address of the subnet
    """
    
    subred: int
    direccion_red: str
    primera_ip: str
    ultima_ip: str
    broadcast: str
    
    def to_dict(self) -> Dict[str, Any]:
        """
        Converts the subnet row to a dictionary.
        
        Returns:
            A dictionary with all subnet row attributes.
            
        Example:
            >>> row_dict = row.to_dict()
            >>> print(row_dict['direccion_red'])
        """
        ...
    
    def to_pretty_string(self) -> str:
        """
        Returns a formatted string representation of the subnet row.
        
        Returns:
            A nicely formatted string with box-drawing characters.
            
        Example:
            >>> pretty = row.to_pretty_string()
            >>> print(pretty)
            ┌─────────────┐
            │ Subnet   1  │
            ├─────────────┤
            │ Network:   192.168.1.0 │
            │ First:     192.168.1.1 │
            ...
        """
        ...
    
    def __str__(self) -> str:
        """
        Returns the string representation.
        
        Returns:
            A string containing all subnet row information.
        """
        ...
    
    def __repr__(self) -> str:
        """
        Returns the official string representation.
        
        Returns:
            A string that could be used to recreate the object.
        """
        ...


def create_subnet_calculator(ip: str, subnet_quantity: int) -> SubnetCalculator:
    """
    Creates a new SubnetCalculator instance.
    
    This is a convenience function for creating subnet calculators.
    
    Args:
        ip: The IP address in CIDR notation or with subnet mask
        subnet_quantity: The number of subnets to create
        
    Returns:
        A new SubnetCalculator instance.
        
    Example:
        >>> calculator = create_subnet_calculator("192.168.1.0/24", 4)
    """
    ...


def calculate_subnets(ip: str, subnet_quantity: int) -> List[Dict[str, str]]:
    """
    Quickly calculate subnets and return results as dictionaries.
    
    This function provides a quick way to get subnet information without
    creating a SubnetCalculator instance.
    
    Args:
        ip: The IP address in CIDR notation or with subnet mask
        subnet_quantity: The number of subnets to create
        
    Returns:
        A list of dictionaries, each representing a subnet with keys:
        - 'subred': subnet number as string
        - 'direccion_red': network address
        - 'primera_ip': first host IP
        - 'ultima_ip': last host IP  
        - 'broadcast': broadcast address
        
    Example:
        >>> subnets = calculate_subnets("192.168.1.0/24", 4)
        >>> for subnet in subnets:
        ...     print(f"Subnet {subnet['subred']}: {subnet['direccion_red']}")
    """
    ...


# Module-level attributes
__version__: str
"""The version of the networking module."""

__author__: str  
"""The author of the networking module."""

__doc__: str
"""The documentation string for the networking module."""