"""
Module containing the Executor class.
"""
import os

from subprocess import CompletedProcess

from src.config import Config
from src.language import Language


class Executor:
    """
    Executor class is responsible for running code in a given language.
    """
    def __init__(self, language: Language) -> None:
        self.__language = language

    def run(self, code: str) -> CompletedProcess:
        """
        Run a give piece of code in a given language.
        """
        path = self.__save_code_to_temp_file(code)

        return self.__language.run(path)

    @property
    def language(self) -> Language:
        """
        Get the language of the executor.
        """
        return self.__language

    def __save_code_to_temp_file(self, code: str) -> str:
        work_dir = Config().get_item('work_dir')

        full_path = os.path.join(work_dir.value(), self.__generate_file_name())

        with open(full_path, 'w', encoding='utf-8') as file_descriptor:
            file_descriptor.write(code)

        return full_path

    def __generate_file_name(self) -> str:
        file_extension = self.__language.get_language_information().file_extension

        return 'code' + file_extension
