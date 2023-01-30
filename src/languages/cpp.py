"""
Module containing the C++ language class.
"""
import os
from subprocess import CompletedProcess

from src.compiled_language import CompiledLanguage
from src.config import Config
from src.language import execute_command
from src.language_information import LanguageInformation


class Cpp(CompiledLanguage):
    """
    Class representing the C++ language.
    """
    def __init__(self, config: Config):

        language_info = config.create_language_information('cpp')

        self.__info: LanguageInformation = language_info
        self.__compiled_output = config.get_item('languages')['cpp']['run']

    def get_language_information(self) -> LanguageInformation:
        return self.__info

    def run(self, code_path: str) -> CompletedProcess:
        self.compile(code_path)

        return execute_command(self.get_executalbe_path(), [])

    def compile(self, code_path: str) -> CompletedProcess:
        """
        Compiles the code at the given path

        Returns:
            CompletedProcess: The result of the compilation
        """
        return execute_command(self.__info.executable,
                               [code_path, '-o', self.get_executalbe_path()])

    def get_executalbe_path(self) -> str:
        """
        Returns the path to the executable
        """
        return os.path.join(Config().get_item('work_dir'), self.__compiled_output)
