"""
Module containing the Python language class.
"""
from subprocess import CompletedProcess

from src.config import Config
from src.interpreted_language import InterpretedLanguage
from src.language import execute_command
from src.language_information import LanguageInformation


class Python(InterpretedLanguage):
    """
    Class representing the Python language.
    """
    def __init__(self, config: Config):

        language_info = config.create_language_information('python3')

        self.__info: LanguageInformation = language_info

    def get_language_information(self) -> LanguageInformation:
        return self.__info

    def run(self, code_path: str) -> CompletedProcess:
        return execute_command(self.__info.executable, [code_path])
