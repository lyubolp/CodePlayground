"""
This file contains the configuration class for the application.
"""
import json
import os

from typing import Any, Optional


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

    def __getattribute__(self, __name: str) -> Any:
        return self.__config[__name]
