"""
Module containing the interface for compiled languages
"""
from abc import abstractmethod
from subprocess import CompletedProcess

from src.language import Language


class CompiledLanguage(Language):
    """
    Interface for compiled languages
    """
    @abstractmethod
    def compile(self, code_path: str) -> CompletedProcess:
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
