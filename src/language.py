"""
Module containing all the abstractions around languages.
"""
import subprocess

from abc import ABC, abstractmethod
from typing import List

from subprocess import CompletedProcess

from src.language_information import LanguageInformation


def execute_command(command: str, args: List[str]) -> CompletedProcess:
    """
    Execute a command with the given arguments.
    """
    result = subprocess.run([command, *args],
                            stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE,
                            encoding="utf-8",
                            check=True)

    return result


class Language(ABC):
    """
    Abstract class for a language.
    """

    @abstractmethod
    def get_language_information(self) -> LanguageInformation:
        """
        Return the information about the language.
        """

    @abstractmethod
    def run(self, code_path: str) -> CompletedProcess:
        """
        Run the given code.
        """
