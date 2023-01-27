"""
This file contains the configuration class for the application.
"""
import json
import os

from typing import Any, Dict, Optional

from src.language_information import LanguageInformation
from src.result import Result, Ok, Error


class Config:
    """
    This class is used to load the configuration file and store it in a dictionary.
    """
    __config: dict = {}

    def __init__(self, file_path: Optional[str] = '') -> None:
        if self.__config == {} and file_path is not None:
            self.__load_config(file_path)

    @classmethod
    def __load_config(cls, file_path: str) -> None:
        if os.path.exists(file_path):
            with open(file_path, 'r', encoding='utf-8') as file_handler:
                cls.__config = json.load(file_handler)

    @classmethod
    def reset_config(cls) -> None:
        """
        Reset the config to an empty dictionary.
        """
        cls.__config = {}

    def get_item(self, key: str) -> Result[Any, str]:
        """
        Get an item from the config.

        :param key: The key to get the value for.
        :return: The value or an error.
        """
        if self.__config == {}:
            return Error('Config is not loaded')

        if key not in self.__config:
            return Error('Key not found')

        return Ok(self.__config[key])

    def create_language_information(self, language: str) -> Result[LanguageInformation, str]:
        """
        Create a LanguageInformation object from the config.

        :param language: The language to create the object for.
        :return: The LanguageInformation object or an error.
        """
        if self.__config == {}:
            return Error('Config is not loaded')

        languages: Dict[str, dict] = self.__config['languages']

        if language not in languages:
            return Error('Language not found')

        current_language: Dict[str, str] = languages[language]

        return Ok(LanguageInformation(language, current_language['executable'],
                                      current_language['extension'], current_language['compiled']))
