"""Module to represent the output of a program."""
from dataclasses import dataclass


@dataclass
class ProgramOutput:
    """Class to represent the output of a program.

    Attributes:
        output: The output of the program.
        error: The error of the program.
        return_code: The return code of the program.
    """
    stdout: str
    stderr: str
    return_code: int

    def __str__(self):
        return f"stdout: {self.stdout}"
