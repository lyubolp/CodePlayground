"""
Module containing all the abstractions around languages.
"""
from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import List

from src.config import Config
from src.program_output import ProgramOutput
from src.result import Ok, Error, Result


def execute_command(command: str, args: List[str]):
    """
    Execute a command with the given arguments.
    """
    # TODO: Implement this
    pass


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
    def from_config(cls, config: dict):
        """
        Create a language from the config.
        """

    @abstractmethod
    def get_langauge_inforamtion(self) -> LanguageInformation:
        """
        Return the information about the language.
        """

    @abstractmethod
    def run(self, code: str) -> Result[ProgramOutput, Exception]:
        """
        Run the given code.
        """
