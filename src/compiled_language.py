"""
Module containing the interface for compiled languages
"""
from abc import abstractmethod

from src.language import Language
from src.program_output import ProgramOutput
from src.result import Result


class CompiledLanguage(Language):
    """
    Interface for compiled languages
    """
    @abstractmethod
    def compile(self, code_path: str) -> Result[ProgramOutput, str]:
        """
        Compiles the code at the given path

        Returns:
            Result[ProgramOutput, str]: The result of the compilation
        """

    @abstractmethod
    def get_executalbe_path(self) -> str:
        """
        Returns the path to the executable
        """
