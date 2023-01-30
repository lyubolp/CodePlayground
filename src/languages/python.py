"""
Module containing the Python language class.
"""
from subprocess import CompletedProcess

from src.config import Config
from src.interpreted_language import InterpretedLanguage
from src.language import execute_command
from src.language_information import LanguageInformation
from src.result import Result, Ok, Error


class Python(InterpretedLanguage):
    """
    Class representing the Python language.
    """
    def __init__(self, config: Config):

        language_info_result = config.create_language_information('python3')

        if not language_info_result:
            raise ValueError(language_info_result.value())

        language_info: LanguageInformation = language_info_result.value()

        self.__info: LanguageInformation = language_info

    def get_language_information(self) -> LanguageInformation:
        return self.__info

    def run(self, code_path: str) -> Result[CompletedProcess, str]:
        return execute_command(self.__info.executable, [code_path])
