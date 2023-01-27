"""
Module containing the Executor class.
"""
import os

from src.config import Config
from src.language import Language
from src.program_output import ProgramOutput
from src.result import Result, Ok, Error


class Executor:
    """
    Executor class is responsible for running code in a given language.
    """
    def __init__(self, language: Language) -> None:
        self.__language = language

    def run(self, code: str) -> Result[ProgramOutput, str]:
        """
        Run a give piece of code in a given language.
        """
        save_code_result = self.__save_code_to_temp_file(code)
        if not save_code_result:
            return Error(save_code_result.error())

        return self.__language.run(save_code_result.value())

    @property
    def language(self) -> Language:
        """
        Get the language of the executor.
        """
        return self.__language

    def __save_code_to_temp_file(self, code: str) -> Result[str, str]:
        full_path = os.path.join(Config().work_dir, self.__generate_file_name())

        try:
            with open(full_path, 'w', encoding='utf-8') as file_descriptor:
                file_descriptor.write(code)
        except OSError as exception:
            return Error(str(exception))

        return Ok(full_path)

    def __generate_file_name(self) -> str:
        return os.path.join('code', self.__language.get_language_information().file_extension)
