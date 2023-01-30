"""
Module for the LanguageInformation class.
"""
from dataclasses import dataclass


@dataclass
class LanguageInformation:
    """
    Information about a language.
    """
    name: str
    executable: str
    file_extension: str
    is_compiled: str
