#!/usr/bin/env python3
"""
Demo Python script for batless syntax highlighting demonstration.

This file showcases various Python language features that should be
highlighted properly by batless when using syntax highlighting mode.
"""

import os
import sys
from typing import List, Dict, Optional, Any
from dataclasses import dataclass
from datetime import datetime


@dataclass
class Person:
    """A simple person class to demonstrate class syntax."""

    name: str
    age: int
    email: Optional[str] = None

    def __post_init__(self):
        if self.age < 0:
            raise ValueError("Age cannot be negative")

    def greet(self) -> str:
        """Return a greeting message."""
        return f"Hello, my name is {self.name} and I'm {self.age} years old."


class DatabaseManager:
    """Demonstrates various Python syntax elements."""

    def __init__(self, connection_string: str):
        self.connection = connection_string
        self._cache: Dict[str, Any] = {}
        self.is_connected = False

    async def connect(self) -> bool:
        """Async method example with error handling."""
        try:
            # Simulate connection logic
            print(f"Connecting to: {self.connection}")
            self.is_connected = True
            return True
        except ConnectionError as e:
            print(f"Failed to connect: {e}")
            return False
        finally:
            print("Connection attempt completed")

    def query(self, sql: str, params: Optional[List[str]] = None) -> Dict:
        """Execute a query with optional parameters."""
        if not self.is_connected:
            raise RuntimeError("Not connected to database")

        # String interpolation and f-strings
        cache_key = f"query_{hash(sql)}_{hash(str(params))}"

        if cache_key in self._cache:
            print("Cache hit!")
            return self._cache[cache_key]

        # List comprehension and lambda
        cleaned_params = [p.strip() for p in (params or []) if p]
        result = {
            "sql": sql,
            "params": cleaned_params,
            "timestamp": datetime.now().isoformat(),
            "rows_affected": len(cleaned_params),
        }

        # Dictionary operations
        self._cache[cache_key] = result
        return result


def process_data(items: List[Dict[str, Any]]) -> List[str]:
    """Process a list of data items with various Python features."""
    results = []

    for i, item in enumerate(items):
        # Multiple assignment and walrus operator (Python 3.8+)
        if (value := item.get("value")) is not None:
            # Match-case statement (Python 3.10+)
            match item.get("type"):
                case "string":
                    processed = f"String: {value.upper()}"
                case "number":
                    processed = f"Number: {value * 2}"
                case "boolean":
                    processed = f"Boolean: {not value}"
                case _:
                    processed = f"Unknown: {value}"

            results.append(f"Item {i}: {processed}")
        else:
            # Multi-line string with triple quotes
            error_msg = """
            Error: Item missing required 'value' field.
            This is a multi-line string to demonstrate
            syntax highlighting for different string types.
            """
            print(error_msg.strip())

    return results


def main():
    """Main function demonstrating various Python constructs."""
    # Raw strings and regex-like patterns
    pattern = r"^\d{3}-\d{2}-\d{4}$"

    # Dictionary with various value types
    config = {
        "debug": True,
        "max_retries": 3,
        "timeout": 30.5,
        "endpoints": ["api.example.com", "backup.example.com"],
        "credentials": None,
        "pattern": pattern,
    }

    # Creating instances
    person = Person("Alice", 30, "alice@example.com")
    db = DatabaseManager("postgresql://localhost:5432/mydb")

    # Sample data for processing
    sample_data = [
        {"type": "string", "value": "hello world"},
        {"type": "number", "value": 42},
        {"type": "boolean", "value": True},
        {"type": "unknown", "value": [1, 2, 3]},
        {"type": "string"},  # Missing value
    ]

    # Process the data
    try:
        results = process_data(sample_data)
        for result in results:
            print(result)
    except Exception as e:
        print(f"Error processing data: {e}")

    # Context manager and file operations
    output_file = "demo_output.txt"
    with open(output_file, "w") as f:
        f.write("# Demo Output\n")
        f.write(f"Generated at: {datetime.now()}\n")
        f.write(f"Person: {person.greet()}\n")

        # Generator expression
        squared = (x**2 for x in range(10) if x % 2 == 0)
        f.write(f"Even squares: {list(squared)}\n")

    print(f"Output written to {output_file}")


# Module-level code with __name__ check
if __name__ == "__main__":
    # Command line argument handling
    if len(sys.argv) > 1 and sys.argv[1] == "--help":
        print(__doc__)
        sys.exit(0)

    main()
