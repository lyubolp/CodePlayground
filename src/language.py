"""
Module containing all the abstractions around languages.
"""
import subprocess

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import List

from src.config import Config
from src.program_output import ProgramOutput
from src.result import Ok, Error, Result


def execute_command(command: str, args: List[str]) -> Result:
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

    return Ok(result.stdout)


@dataclass
class LanguageInformation:
    """
    Information about a language.
    """
    name: str
    executable: str
    file_extension: str
    is_compiled: str


class Language(ABC):
    """
    Abstract class for a language.
    """

    @abstractmethod
    @classmethod
    def from_config(cls, config: Config):
        """
        Create a language from the config.
        """

    @abstractmethod
    def get_language_information(self) -> LanguageInformation:
        """
        Return the information about the language.
        """

    @abstractmethod
    def run(self, code_path: str) -> Result[ProgramOutput, str]:
        """
        Run the given code.
        """
