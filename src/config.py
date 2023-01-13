"""
This file contains the configuration class for the application.
"""
import json
import os

from typing import Any


class Config:
    """
    This class is used to load the configuration file and store it in a dictionary.
    """
    __config: dict = {}

    def __init__(self, file_path: str) -> None:
        if self.__config == {}:
            self.__load_config(file_path)

    @classmethod
    def __load_config(cls, file_path: str) -> None:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                cls.__config = json.load(f)

    def __getattribute__(self, __name: str) -> Any:
        return self.__config[__name]
