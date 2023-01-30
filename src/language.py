"""
Module containing all the abstractions around languages.
"""
import subprocess

from abc import ABC, abstractmethod
from typing import List

from subprocess import CompletedProcess

from src.language_information import LanguageInformation
from src.result import Ok, Error, Result


def execute_command(command: str, args: List[str]) -> Result[CompletedProcess, str]:
    """
    Execute a command with the given arguments.
    """
    try:
        result = subprocess.run([command, *args],
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE,
                                encoding="utf-8",
                                check=True)
    except subprocess.CalledProcessError as exception:
        return Error(exception.stderr)

    return Ok(result)


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
    def run(self, code_path: str) -> Result[CompletedProcess, str]:
        """
        Run the given code.
        """
